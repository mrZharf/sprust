// mrZharf
use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::future::Future;
#[derive(Debug, Deserialize)]
pub struct User {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    #[serde(default)]
    pub username: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct ApiResp<T> {
    pub ok: bool,
    pub result: T,
}
#[derive(Clone)]
pub struct Client {
    token: String,
    http: reqwest::Client,
}

#[derive(Debug, Serialize, Default)]
pub struct GetUpdatesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub time_out: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Chat {
    pub id: i64,
    pub r#type: String,

    #[serde(default)]
    pub title: Option<String>,

    #[serde(default)]
    pub username: Option<String>,

    #[serde(default)]
    pub first_name: Option<String>,

    #[serde(default)]
    pub last_name: Option<String>,

    #[serde(default)]
    pub photo: Option<ChatPhoto>,

    #[serde(default)]
    pub bio: Option<String>,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub invite_link: Option<String>,

    #[serde(default)]
    pub pinned_message: Option<Box<Message>>,

    #[serde(default)]
    pub slow_mode_delay: Option<i64>,

    #[serde(default)]
    pub message_auto_delete_time: Option<i64>,

    #[serde(default)]
    pub has_protected_content: Option<bool>,

    #[serde(default)]
    pub sticker_set_name: Option<String>,

    #[serde(default)]
    pub can_set_sticker_set: Option<bool>,

    #[serde(default)]
    pub linked_chat_id: Option<i64>,

    #[serde(default)]
    pub location: Option<Location>,

    #[serde(default)]
    pub all_members_are_administrators: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ChatPhoto {
    pub small_file_id: String,
    pub small_file_unique_id: String,
    pub big_file_id: String,
    pub big_file_unique_id: String,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub message_id: i64,

    #[serde(default)]
    pub from: Option<User>,

    pub date: i64,

    pub chat: Chat,

    #[serde(default)]
    pub text: Option<String>,

    #[serde(default)]
    pub entities: Option<Vec<MessageEntity>>,
}

#[derive(Debug, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Deserialize)]
pub struct Update {
    pub update_id: i64,
    #[serde(default)]
    pub message: Option<Message>,
    #[serde(default)]
    pub edited_message: Option<Message>,
    //#[serde(default)]
    //pub callback_query: Option<CallbackQuery>,
}

#[derive(Debug, Deserialize)]
pub struct MessageEntity {
    pub offset: i64,
    pub length: i64,
    pub r#type: String,
}

impl Client {
    pub fn new(token: String) -> Self {
        Self {
            token,
            http: reqwest::Client::new(),
        }
    }
    async fn request<T>(&self, method: &str) -> Result<T, Error>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = format!("https://api.splus.ir/bot{}/{}", self.token, method);

        let response = self.http.get(&url).send().await.map_err(Error::Http)?;

        let text = response.text().await.map_err(Error::Http)?;

        let data: ApiResp<T> = serde_json::from_str(&text).map_err(Error::Json)?;

        Ok(data.result)
    }
    async fn request_param<T, P>(&self, method: &str, params: &P) -> Result<T, Error>
    where
        T: for<'de> Deserialize<'de>,
        P: Serialize,
    {
        let url = format!("https://api.splus.ir/bot{}/{}", self.token, method);

        let response = self
            .http
            .get(&url)
            .query(params)
            .send()
            .await
            .map_err(Error::Http)?;

        let text = response.text().await.map_err(Error::Http)?;

        let data: ApiResp<T> = serde_json::from_str(&text).map_err(Error::Json)?;

        Ok(data.result)
    }
    pub async fn get_me(&self) -> Result<User, Error> {
        self.request::<User>("GetMe").await
    }
    pub async fn get_updates(&self, params: &GetUpdatesParams) -> Result<Vec<Update>, Error> {
        self.request_param::<Vec<Update>, _>("GetUpdates", params)
            .await
    }
    pub async fn send_message(&self, chat_id: i64, text: &str) -> Result<Message, Error> {
        #[derive(Serialize)]
        struct SendMessageParams<'a> {
            chat_id: i64,
            text: &'a str,
        }

        let params = SendMessageParams { chat_id, text };

        self.request_param::<Message, _>("sendMessage", &params)
            .await
    }
    pub async fn poll<F, Fut>(&self, mut handler: F) -> Result<(), Error>
    where
        F: FnMut(Update) -> Fut,
        Fut: Future<Output = ()>,
    {
        let mut offset: Option<i64> = None;

        loop {
            let params = GetUpdatesParams {
                offset,
                time_out: Some(30),
                ..Default::default()
            };

            let updates = self.get_updates(&params).await?;

            for update in updates {
                offset = Some(update.update_id + 1);

                handler(update).await;
            }
        }
    }
}
