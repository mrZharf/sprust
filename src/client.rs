// mrZharf
use serde::{Deserialize, de::DeserializeOwned};
use crate::error::Error;


#[derive(Debug , Deserialize)]
pub struct User {
	pub id: i64,
	pub is_bot: bool,
	pub first_name: String,
	pub username: String,
}
#[derive(Debug , Deserialize)]
pub struct ApiResp<T> {
	pub ok: bool,
	pub result: T,
}
pub struct Client {
	token : String,
	http : reqwest::Client,
}

impl Client {
	pub fn new(token:String) -> Self {
		Self {
			token,
			http : reqwest::Client::new(),
		}
	}
	async fn request<T>(&self, method: &str) -> Result<T, Error>
	where 
		T : DeserializeOwned,
	{
    	let url = format!("https://api.splus.ir/bot{}/{}", self.token, method);

    	let response = self
        	.http
        	.get(&url)
        	.send()
        	.await
        	.map_err(Error::Http)?;

    	let text = response
        	.text()
        	.await
        	.map_err(Error::Http)?;

    	let data: ApiResp<T> =
        	serde_json::from_str(&text)
            	.map_err(Error::Json)?;

    	Ok(data.result)
	}
	pub async fn get_me(&self) -> Result<User, Error> {
    	self.request::<User>("GetMe").await
	}
}
