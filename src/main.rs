use std::collections::HashMap;

// Actix and serde are only requried to get the auth code from twitter
use actix_web::{get, web, App, HttpServer, HttpRequest, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct TwitterAuthResponse{
	state:String,
	code:String,
}


#[tokio::main]
async fn main() -> std::io::Result<()> {
	
	// Generate auth link
	let config = AuthConfig::new();
	println!("Navigate to this link to authorize: {}",config.get_authorize_link());
	
	// Start listening, Twitter will send the auth code to us.
	// We need to convert it to an access code within 30 seconds or it will expire.
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|auth: web::Query<TwitterAuthResponse>| async move {
				let access_code = transform_auth_to_access(&auth.code).await.unwrap();
				""
			}))
    })
    .bind(("127.0.0.1", 8080))?.run().await
}

// The auth code needs converted into an access code.
async fn transform_auth_to_access(auth_code:&str) -> Result<String,Box<dyn std::error::Error>>{
	let access_code = reqwest::Client::new()
		//.post("https://api.twitter.com/2/oauth2/token")
		.post("http://localhost.com:3000")
		.form(&[
			("code", auth_code),
			("grant_type", "authorization_code"),
			("client_id", env!("client_id")),
			("redirect_uri", "http://localhost.com:8080"),
			("code_verifier", "challenge"),
		])
		//.body("Testbody")
		.send()
		.await?;
	dbg!(&access_code);
	Ok("access_code".to_owned())
}


async fn access(access_code:&str) -> Result<(), Box<dyn std::error::Error>> {

	// Use the acces code to get some information 
	let client = reqwest::Client::new();
	let resp = client
		.get("https://api.twitter.com/2/users/me")
		.header("AUTHORIZATION",format!("Bearer {}",access_code))
		.send()
        .await?;
        //.json::<HashMap<String, String>>()
        //.await?;
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
			scope:vec!["tweet.read","tweet.write","users.read","offline.access"],
		}
	}

	// This is the link that the user will have to click on to start the oauth process
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

