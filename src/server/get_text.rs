use dioxus::prelude::*;

#[get("/api/v1/text")]
pub async fn get_text() -> Result<String> {
    Ok("Hi".to_string())
}
