use crate::ai::ImageAttachment;
use base64::{engine::general_purpose::STANDARD, Engine};
use image::GenericImageView;
use std::{io::Cursor, path::Path};

pub(crate) enum InputFile {
    Image(ImageAttachment),
    Log { name: String, text: String },
}

pub(crate) fn pick_input_file() -> Result<Option<InputFile>, String> {
    let Some(path) = rfd::FileDialog::new()
        .add_filter(
            "Logs and screenshots",
            &["log", "txt", "json", "out", "png", "jpg", "jpeg", "webp"],
        )
        .pick_file()
    else {
        return Ok(None);
    };
    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    if matches!(extension.as_str(), "png" | "jpg" | "jpeg" | "webp") {
        Ok(Some(InputFile::Image(read_image(&path)?)))
    } else {
        let size = std::fs::metadata(&path)
            .map_err(|error| error.to_string())?
            .len();
        if size > 20 * 1024 * 1024 {
            return Err("File exceeds the 20 MB preprocessing limit.".to_owned());
        }
        let text = std::fs::read_to_string(&path).map_err(|error| error.to_string())?;
        Ok(Some(InputFile::Log {
            name: file_name(&path, "log"),
            text,
        }))
    }
}

fn read_image(path: &Path) -> Result<ImageAttachment, String> {
    let bytes = std::fs::read(path).map_err(|error| error.to_string())?;
    let image = image::load_from_memory(&bytes).map_err(|error| error.to_string())?;
    let image = if image.width() > 1920 || image.height() > 1920 {
        image.resize(1920, 1920, image::imageops::FilterType::Lanczos3)
    } else {
        image
    };
    let (width, height) = image.dimensions();
    let mut png = Cursor::new(Vec::new());
    image
        .write_to(&mut png, image::ImageFormat::Png)
        .map_err(|error| error.to_string())?;
    Ok(ImageAttachment {
        name: file_name(path, "screenshot.png"),
        mime_type: "image/png".to_owned(),
        data_base64: STANDARD.encode(png.into_inner()),
        width,
        height,
    })
}

fn file_name(path: &Path, fallback: &str) -> String {
    path.file_name()
        .and_then(|value| value.to_str())
        .unwrap_or(fallback)
        .to_owned()
}
