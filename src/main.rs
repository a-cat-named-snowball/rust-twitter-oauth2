use std::collections::HashMap;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}



struct AuthConfig {
	endpoint:&'static str,
	redirect_uri:&'static str,
	scopes:Vec<&'static str>,
}
impl AuthConfig {
	fn new() -> Self {
		Self {
			endpoint:"https://twitter.com/i/oauth2/authorize",
			redirect_uri:"http://localhost",
			scopes:vec!["tweet.write","users.read"],
		}
	}
	fn get_authorize_link(&self) -> String {
		format!(
			"{endpoint}?response_type=code&client_id={client_id}&redirect_uri={redirect_uri}&state=state&code_challenge={challenge}&code_challenge_method=plain",
			endpoint     = self.endpoint,
			client_id    = env!("client_id"),
			redirect_uri = self.redirect_uri,
			challenge    = "makethisrandomdata",
		)

	}
}
