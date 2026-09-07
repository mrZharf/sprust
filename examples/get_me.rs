use sprust::Client;

#[tokio::main]
async fn main() {
	dotenvy::dotenv().ok();
    let token = std::env::var("SP_TOKEN").unwrap();

    let bot = Client::new(token);

    let user = bot.get_me().await.unwrap();

    println!("{:?}",user);
}
