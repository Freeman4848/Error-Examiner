use super::{ProviderKind, ProviderSettings};
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue},
};
use serde_json::Value;
use std::time::Duration;

pub(super) fn list(settings: &ProviderSettings, api_key: &str) -> Result<Vec<String>, String> {
    if !matches!(
        settings.provider,
        ProviderKind::Mock | ProviderKind::LmStudio
    ) && api_key.trim().is_empty()
    {
        return Err("Enter the API key first.".to_owned());
    }
    if settings.provider == ProviderKind::Mock {
        return Ok(vec!["mock".to_owned()]);
    }
    let client = Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|error| error.to_string())?;
    let (url, headers) = request_parts(settings, api_key)?;
    let response = client
        .get(url)
        .headers(headers)
        .send()
        .map_err(|error| format!("Model list request failed: {error}"))?;
    if !response.status().is_success() {
        return Err(format!("Model list failed: HTTP {}", response.status()));
    }
    let value: Value = response
        .json()
        .map_err(|error| format!("Model list decode failed: {error}"))?;
    let mut models = parse_models(settings.provider, &value);
    models.sort();
    models.dedup();
    if models.is_empty() {
        Err("Provider returned no generative models.".to_owned())
    } else {
        Ok(models)
    }
}

fn request_parts(
    settings: &ProviderSettings,
    api_key: &str,
) -> Result<(String, HeaderMap), String> {
    let base = settings.base_url.trim_end_matches('/');
    let mut headers = HeaderMap::new();
    let url = match settings.provider {
        ProviderKind::Gemini => {
            headers.insert(
                "x-goog-api-key",
                HeaderValue::from_str(api_key.trim()).map_err(|error| error.to_string())?,
            );
            format!("{base}/models?pageSize=1000")
        }
        ProviderKind::Anthropic => {
            headers.insert(
                "x-api-key",
                HeaderValue::from_str(api_key.trim()).map_err(|error| error.to_string())?,
            );
            headers.insert("anthropic-version", HeaderValue::from_static("2023-06-01"));
            format!("{base}/v1/models")
        }
        ProviderKind::LmStudio => {
            let root = base.trim_end_matches("/v1").trim_end_matches("/api/v1");
            format!("{root}/api/v1/models")
        }
        _ => {
            headers.insert(
                "authorization",
                HeaderValue::from_str(&format!("Bearer {}", api_key.trim()))
                    .map_err(|error| error.to_string())?,
            );
            format!("{base}/models")
        }
    };
    Ok((url, headers))
}

fn parse_models(provider: ProviderKind, value: &Value) -> Vec<String> {
    match provider {
        ProviderKind::Gemini => value["models"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|model| {
                model["supportedGenerationMethods"]
                    .as_array()
                    .is_some_and(|methods| {
                        methods
                            .iter()
                            .any(|method| method.as_str() == Some("generateContent"))
                    })
            })
            .filter_map(|model| model["name"].as_str())
            .map(|name| name.trim_start_matches("models/").to_owned())
            .collect(),
        ProviderKind::LmStudio => value["models"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|model| model["type"].as_str() == Some("llm"))
            .filter_map(|model| model["key"].as_str())
            .map(str::to_owned)
            .collect(),
        _ => value["data"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|model| model["id"].as_str())
            .map(str::to_owned)
            .collect(),
    }
}
