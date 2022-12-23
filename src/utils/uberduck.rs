use serde::Deserialize;

#[derive(Deserialize)]
pub struct SpeakResponse {
    pub uuid: String,
}

#[derive(Deserialize)]
pub struct SpeakStatusResponse {
    pub failed_at: Option<String>,
    pub finished_at: Option<String>,
    pub meta: Option<String>,
    pub path: Option<String>,
    pub started_at: Option<String>,
}
