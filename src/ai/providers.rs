use super::*;

pub(super) fn ask_openai(
    client: &Client,
    request: &AiRequest,
    messages: &[ChatMessage],
) -> Result<AiAnswer, String> {
    let input: Vec<Value> = messages.iter().map(openai_response_message).collect();
    let mut payload = json!({
        "model": request.settings.model,
        "instructions": system_prompt(request),
        "input": input,
        "max_output_tokens": request.settings.max_output_tokens,
        "store": false
    });
    if request.settings.model.starts_with("gpt-5") {
        payload["reasoning"] = json!({"effort": "low"});
    }
    let value = post_json(
        client,
        &endpoint(&request.settings.base_url, "responses"),
        bearer_headers(&request.api_key)?,
        &payload,
        &request.api_key,
        request.settings.retries,
    )?;
    let text = value
        .get("output_text")
        .and_then(Value::as_str)
        .map(str::to_owned)
        .or_else(|| {
            value["output"].as_array()?.iter().find_map(|item| {
                item["content"].as_array()?.iter().find_map(|content| {
                    content
                        .get("text")
                        .and_then(Value::as_str)
                        .map(str::to_owned)
                })
            })
        })
        .ok_or_else(|| "OpenAI response contained no output text.".to_owned())?;
    Ok(answer_with_usage(text, &value))
}

pub(super) fn ask_openai_compatible(
    client: &Client,
    request: &AiRequest,
    messages: &[ChatMessage],
) -> Result<AiAnswer, String> {
    let mut api_messages = vec![json!({"role": "system", "content": system_prompt(request)})];
    api_messages.extend(messages.iter().map(openai_chat_message));
    let payload = json!({
        "model": request.settings.model,
        "messages": api_messages,
        "max_tokens": request.settings.max_output_tokens
    });
    let headers = if request.api_key.trim().is_empty() {
        Vec::new()
    } else {
        bearer_headers(&request.api_key)?
    };
    let value = post_json(
        client,
        &endpoint(&request.settings.base_url, "chat/completions"),
        headers,
        &payload,
        &request.api_key,
        request.settings.retries,
    )?;
    let text = value["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| "Compatible API response contained no message text.".to_owned())?
        .to_owned();
    Ok(answer_with_usage(text, &value))
}

pub(super) fn ask_lm_studio(
    client: &Client,
    request: &AiRequest,
    messages: &[ChatMessage],
) -> Result<AiAnswer, String> {
    let root = lm_studio_root(&request.settings.base_url);
    let model = discover_lm_studio_model(client, &root, &request.api_key)?;
    if messages.iter().any(|message| message.image.is_some()) {
        return ask_lm_studio_vision(client, request, messages, &root, model);
    }
    let transcript = messages
        .iter()
        .rev()
        .find(|message| message.role == "user")
        .map(|message| message.content.as_str())
        .unwrap_or_default();
    let payload = json!({
        "model": model,
        "input": transcript,
        "system_prompt": system_prompt(request),
        "max_output_tokens": request.settings.max_output_tokens,
        "store": false
    });
    let headers = optional_bearer_headers(&request.api_key)?;
    let value = post_json(
        client,
        &format!("{root}/api/v1/chat"),
        headers,
        &payload,
        &request.api_key,
        request.settings.retries,
    )?;
    let text = value["output"]
        .as_array()
        .and_then(|items| {
            items.iter().find_map(|item| {
                (item["type"].as_str() == Some("message"))
                    .then(|| item["content"].as_str().map(str::to_owned))
                    .flatten()
            })
        })
        .ok_or_else(|| "LM Studio returned no message output.".to_owned())?;
    Ok(AiAnswer {
        text: text.trim().to_owned(),
        input_tokens: value["stats"]["input_tokens"].as_u64(),
        output_tokens: value["stats"]["total_output_tokens"].as_u64(),
        tokens_per_second: value["stats"]["tokens_per_second"].as_f64(),
        time_to_first_token_seconds: value["stats"]["time_to_first_token_seconds"].as_f64(),
        resolved_model: Some(model),
    })
}

fn discover_lm_studio_model(client: &Client, root: &str, api_key: &str) -> Result<String, String> {
    let mut request = client.get(format!("{root}/api/v1/models"));
    for (name, value) in optional_bearer_headers(api_key)? {
        request = request.header(name, value);
    }
    let response = request
        .send()
        .map_err(|error| format!("LM Studio model discovery failed: {error}"))?;
    if !response.status().is_success() {
        return Err(format!(
            "LM Studio model discovery failed: HTTP {}",
            response.status()
        ));
    }
    let value: Value = response
        .json()
        .map_err(|error| format!("LM Studio model list decode failed: {error}"))?;
    let models = value["models"]
        .as_array()
        .ok_or_else(|| "LM Studio returned no model list.".to_owned())?;
    models
        .iter()
        .filter(|model| model["type"].as_str() == Some("llm"))
        .flat_map(|model| model["loaded_instances"].as_array().into_iter().flatten())
        .min_by_key(|instance| instance.get("remaining_ttl_seconds").is_some())
        .and_then(|instance| instance["id"].as_str())
        .map(|id| id.trim().to_owned())
        .ok_or_else(|| "LM Studio has no loaded LLM. Load one and retry.".to_owned())
}

pub(super) fn ask_gemini(
    client: &Client,
    request: &AiRequest,
    messages: &[ChatMessage],
) -> Result<AiAnswer, String> {
    let contents: Vec<Value> = messages
        .iter()
        .map(|message| {
            let role = if message.role == "assistant" {
                "model"
            } else {
                "user"
            };
            let mut parts = vec![json!({"text": message.content})];
            if let Some(image) = &message.image {
                parts.push(json!({
                    "inlineData": {
                        "mimeType": image.mime_type,
                        "data": image.data_base64
                    }
                }));
            }
            json!({"role": role, "parts": parts})
        })
        .collect();
    let payload = json!({
        "systemInstruction": {"parts": [{"text": system_prompt(request)}]},
        "contents": contents,
        "generationConfig": {"maxOutputTokens": request.settings.max_output_tokens}
    });
    let model = request.settings.model.trim_start_matches("models/");
    let value = post_json(
        client,
        &endpoint(
            &request.settings.base_url,
            &format!("models/{model}:generateContent"),
        ),
        vec![(
            HeaderName::from_static("x-goog-api-key"),
            secret_header(&request.api_key)?,
        )],
        &payload,
        &request.api_key,
        request.settings.retries,
    )?;
    let text = value["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .ok_or_else(|| "Gemini response contained no candidate text.".to_owned())?
        .to_owned();
    let input_tokens = value["usageMetadata"]["promptTokenCount"].as_u64();
    let output_tokens = value["usageMetadata"]["candidatesTokenCount"].as_u64();
    Ok(AiAnswer {
        text,
        input_tokens,
        output_tokens,
        tokens_per_second: None,
        time_to_first_token_seconds: None,
        resolved_model: None,
    })
}

pub(super) fn ask_anthropic(
    client: &Client,
    request: &AiRequest,
    messages: &[ChatMessage],
) -> Result<AiAnswer, String> {
    let api_messages: Vec<Value> = messages
        .iter()
        .map(|message| {
            let mut content = vec![json!({"type": "text", "text": message.content})];
            if let Some(image) = &message.image {
                content.push(json!({
                    "type": "image",
                    "source": {
                        "type": "base64",
                        "media_type": image.mime_type,
                        "data": image.data_base64
                    }
                }));
            }
            json!({
                "role": if message.role == "assistant" {"assistant"} else {"user"},
                "content": content
            })
        })
        .collect();
    let payload = json!({
        "model": request.settings.model,
        "system": system_prompt(request),
        "messages": api_messages,
        "max_tokens": request.settings.max_output_tokens
    });
    let headers = vec![
        (
            HeaderName::from_static("x-api-key"),
            secret_header(&request.api_key)?,
        ),
        (
            HeaderName::from_static("anthropic-version"),
            HeaderValue::from_static("2023-06-01"),
        ),
    ];
    let value = post_json(
        client,
        &endpoint(&request.settings.base_url, "v1/messages"),
        headers,
        &payload,
        &request.api_key,
        request.settings.retries,
    )?;
    let text = value["content"]
        .as_array()
        .and_then(|content| {
            content
                .iter()
                .find_map(|part| part["text"].as_str().map(str::to_owned))
        })
        .ok_or_else(|| "Anthropic response contained no text.".to_owned())?;
    Ok(AiAnswer {
        text,
        input_tokens: value["usage"]["input_tokens"].as_u64(),
        output_tokens: value["usage"]["output_tokens"].as_u64(),
        tokens_per_second: None,
        time_to_first_token_seconds: None,
        resolved_model: None,
    })
}

fn openai_response_message(message: &ChatMessage) -> Value {
    let mut content = vec![json!({"type": "input_text", "text": message.content})];
    if let Some(image) = &message.image {
        content.push(json!({
            "type": "input_image",
            "image_url": format!("data:{};base64,{}", image.mime_type, image.data_base64)
        }));
    }
    json!({"role": message.role, "content": content})
}

fn openai_chat_message(message: &ChatMessage) -> Value {
    let Some(image) = &message.image else {
        return json!({"role": message.role, "content": message.content});
    };
    let mut content = vec![json!({"type": "text", "text": message.content})];
    content.push(json!({
        "type": "image_url",
        "image_url": {
            "url": format!("data:{};base64,{}", image.mime_type, image.data_base64)
        }
    }));
    json!({"role": message.role, "content": content})
}

fn ask_lm_studio_vision(
    client: &Client,
    request: &AiRequest,
    messages: &[ChatMessage],
    root: &str,
    model: String,
) -> Result<AiAnswer, String> {
    let mut api_messages = vec![json!({"role": "system", "content": system_prompt(request)})];
    api_messages.extend(messages.iter().map(openai_chat_message));
    let payload = json!({
        "model": model,
        "messages": api_messages,
        "max_tokens": request.settings.max_output_tokens
    });
    let value = post_json(
        client,
        &format!("{root}/v1/chat/completions"),
        optional_bearer_headers(&request.api_key)?,
        &payload,
        &request.api_key,
        request.settings.retries,
    )?;
    let text = value["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| "LM Studio vision response contained no text.".to_owned())?;
    let mut answer = answer_with_usage(text.trim().to_owned(), &value);
    answer.resolved_model = Some(model);
    Ok(answer)
}

fn post_json(
    client: &Client,
    url: &str,
    headers: Vec<(HeaderName, HeaderValue)>,
    payload: &Value,
    secret: &str,
    retries: u8,
) -> Result<Value, String> {
    let retries = retries.min(2);
    for attempt in 0..=retries {
        let mut header_map = HeaderMap::new();
        for (name, value) in &headers {
            header_map.insert(name.clone(), value.clone());
        }
        let response = client.post(url).headers(header_map).json(payload).send();
        match response {
            Ok(response) if response.status().is_success() => {
                return response
                    .json::<Value>()
                    .map_err(|error| format!("Response JSON decode failed: {error}"));
            }
            Ok(response) => {
                let status = response.status();
                let transient = matches!(
                    status,
                    StatusCode::TOO_MANY_REQUESTS
                        | StatusCode::BAD_GATEWAY
                        | StatusCode::SERVICE_UNAVAILABLE
                        | StatusCode::GATEWAY_TIMEOUT
                );
                let body = response.text().unwrap_or_default();
                if transient && attempt < retries {
                    std::thread::sleep(Duration::from_millis(300 * (attempt as u64 + 1)));
                    continue;
                }
                return Err(format_api_error(status, &body, secret));
            }
            Err(error) if attempt < retries => {
                std::thread::sleep(Duration::from_millis(300 * (attempt as u64 + 1)));
                let _ = error;
            }
            Err(error) => return Err(format!("Request failed: {error}")),
        }
    }
    Err("Request failed after retries.".to_owned())
}

fn answer_with_usage(text: String, value: &Value) -> AiAnswer {
    AiAnswer {
        text,
        input_tokens: value["usage"]["input_tokens"]
            .as_u64()
            .or_else(|| value["usage"]["prompt_tokens"].as_u64()),
        output_tokens: value["usage"]["output_tokens"]
            .as_u64()
            .or_else(|| value["usage"]["completion_tokens"].as_u64()),
        tokens_per_second: None,
        time_to_first_token_seconds: None,
        resolved_model: None,
    }
}

fn format_api_error(status: StatusCode, body: &str, secret: &str) -> String {
    let parsed = serde_json::from_str::<Value>(body).ok();
    let message = parsed
        .as_ref()
        .and_then(|value| value["error"]["message"].as_str())
        .unwrap_or(body);
    let redacted = if secret.is_empty() {
        message.to_owned()
    } else {
        message.replace(secret, "[REDACTED]")
    };
    let short: String = redacted.chars().take(500).collect();
    format!("HTTP {status}: {short}")
}

fn bearer_headers(api_key: &str) -> Result<Vec<(HeaderName, HeaderValue)>, String> {
    Ok(vec![(
        HeaderName::from_static("authorization"),
        HeaderValue::from_str(&format!("Bearer {api_key}"))
            .map_err(|_| "API key contains invalid header characters.".to_owned())?,
    )])
}

fn optional_bearer_headers(api_key: &str) -> Result<Vec<(HeaderName, HeaderValue)>, String> {
    if api_key.trim().is_empty() {
        Ok(Vec::new())
    } else {
        bearer_headers(api_key)
    }
}

fn secret_header(api_key: &str) -> Result<HeaderValue, String> {
    HeaderValue::from_str(api_key)
        .map_err(|_| "API key contains invalid header characters.".to_owned())
}

pub(super) fn endpoint(base_url: &str, suffix: &str) -> String {
    let base = base_url.trim_end_matches('/');
    if base.ends_with(suffix) {
        base.to_owned()
    } else {
        format!("{base}/{}", suffix.trim_start_matches('/'))
    }
}

fn lm_studio_root(base_url: &str) -> String {
    base_url
        .trim_end_matches('/')
        .trim_end_matches("/api/v1")
        .trim_end_matches("/v1")
        .to_owned()
}

pub(super) fn mock_answer(messages: &[ChatMessage]) -> AiAnswer {
    let last = messages
        .iter()
        .rev()
        .find(|message| message.role == "user")
        .map(|message| message.content.as_str())
        .unwrap_or("");
    let text = format!(
        "Summary\nOffline demo analyzed {} characters.\n\nSeverity & confidence\nUnknown · low confidence (no live model).\n\nLikely root cause\nThe demo provider cannot infer a real cause. Connect an AI provider in Settings.\n\nVerification\n1. Confirm the complete error and stack trace are present.\n2. Reproduce once and capture the first failure.\n3. Retry with a configured provider.",
        last.chars().count()
    );
    AiAnswer {
        text,
        input_tokens: Some(estimate_tokens(last) as u64),
        output_tokens: Some(78),
        tokens_per_second: None,
        time_to_first_token_seconds: None,
        resolved_model: None,
    }
}
