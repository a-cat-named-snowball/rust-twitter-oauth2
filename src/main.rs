use std::collections::HashMap;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	
	let config = AuthConfig::new();
	println!("Navigate to this link to authorize: {}",config.get_authorize_link());

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
	scope:Vec<&'static str>,
}
impl AuthConfig {
	fn new() -> Self {
		Self {
			endpoint:"https://twitter.com/i/oauth2/authorize",
			redirect_uri:"http://localhost.com:8080",
			scope:vec!["tweet.write","users.read"],
		}
	}
	fn get_authorize_link(&self) -> String {
		format!(
			"{endpoint}?response_type=code&client_id={client_id}&redirect_uri={redirect_uri}&scope={scope}&state=state&code_challenge={challenge}&code_challenge_method=plain",
			endpoint     = self.endpoint,
			client_id    = env!("client_id"),
			scope        = self.scope.join("%20"),
			redirect_uri = self.redirect_uri,
			challenge    = "makethisrandomdata",
		)

	}
}

//https://twitter.com/i/oauth2/authorize?response_type=code&client_id=dGotejkwLTMtT0s4LXEzNFl6REU6MTpjaQ&redirect_uri=http://localhost&scope=tweet.write%20users.read&state=state&code_challenge=makethisrandomdata&code_challenge_method=plain
