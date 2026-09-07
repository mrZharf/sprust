#[derive(Debug)]
pub enum Error {
	Http(reqwest::Error),
	Json(serde_json::Error),
}
