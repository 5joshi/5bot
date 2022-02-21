use serde::Deserialize;

#[derive(Deserialize)]
pub struct ValueRange {
    range: String,
    majorDimension: String,
    pub values: Vec<Vec<String>>,
}

#[derive(Deserialize)]
pub struct BatchGetResponse {
    spreadsheetId: String,
    pub valueRanges: Vec<ValueRange>,
}
