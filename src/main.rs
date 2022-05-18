use actix_web::{web, App, HttpServer};
use serde::Deserialize;
use serde_json::Value;


// High-level overview
// Step 1. Generate a link for the user to click on.
// Step 2. Listen for the response from Twitter to get an auth code
// Step 3. Use that auth token within 30 seconds to get an access token
// Step 4. Make a request with the access token to get user information
// Step 5. Make a request with the access token to post a tweet



// Used to generate the link will take the user to Twitter.
// I've made it more verbose than it needs to be so it can be configured easily.
struct AuthLink {
	endpoint     : &'static str,
	redirect_uri : &'static str,      // Twitter redirects the user after login
	scope        : Vec<&'static str>, // Permissions needed
	client_id    : &'static str,
	challenge    : &'static str,
}
impl AuthLink {
	fn new() -> Self {
		Self {
			endpoint     : "https://twitter.com/i/oauth2/authorize",
			redirect_uri : "http://localhost.com:8080",
			scope        : vec!["tweet.read","tweet.write","users.read","offline.access"],
			client_id    : env!("client_id"),
			challenge    : "makethisrandomdata", // Should be random data in final implementation
		}
	}
}
impl std::fmt::Display for AuthLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f,"{endpoint}?response_type=code&client_id={client_id}&redirect_uri={redirect_uri}&scope={scope}&state=state&code_challenge={challenge}&code_challenge_method=plain",
			endpoint     = self.endpoint,
			client_id    = self.client_id,
			scope        = self.scope.join("%20"),
			redirect_uri = self.redirect_uri,
			challenge    = self.challenge,
		)
	}
}



// Built when this server receives a auth response from Twitter servers
#[derive(Deserialize)]
#[allow(dead_code)]
struct AuthResponse {
	state:String,
	code:String,
}


// Built when we try to convert the Auth code into an access token
#[derive(Deserialize)]
#[allow(dead_code)]
struct AccessResponse {
	token_type:String,
	expires_in:u32,
	access_token:String,
	scope:String,
	refresh_token:String,
}



#[tokio::main]
async fn main() -> std::io::Result<()> {
	
	// Generate auth link and display it in the terminal
	let auth_link = AuthLink::new();
	println!("Navigate to this link to authorize: {auth_link}");
	

	// Start listening, Twitter will be sending the auth code to us soon.
	// We need to convert it to an access code within 30 seconds or it will expire.
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|auth: web::Query<AuthResponse>| async move {

				// We get the auth code and immediately try to convert it into an acces token
				let access_response = transform_auth_to_access(&auth.code).await.unwrap();
				let access_token = access_response.access_token;


				// Use the access token to get info on the user
				let user_info = get_user_info(&access_token).await.unwrap();
				let name = match user_info.get("data").unwrap().get("name").unwrap(){
					Value::String(n) => n.clone(),
					_                => "Unknown Username".to_string(),
				};
				println!("Username is {name}");
				
				// Use access token to post a tweet
				post_tweet(&access_token,"Example tweet text").await.unwrap();
				
				format!("Tweet posted, {}",name)
			}))
    })
    .bind(("127.0.0.1", 8080))?.run().await
}


// Converts an auth code into an access code by Twitter.
async fn transform_auth_to_access(auth_code:&str) -> Result<AccessResponse,Box<dyn std::error::Error>>{
	let access_response = reqwest::Client::new()
		.post("https://api.twitter.com/2/oauth2/token")
		.form(&[
			("code", auth_code),
			("grant_type", "authorization_code"),
			("client_id", env!("client_id")),
			("redirect_uri", "http://localhost.com:8080"),
			("code_verifier", "makethisrandomdata"),
		])
		.send().await?
		.json::<AccessResponse>().await?;
	Ok(access_response)
}



// Get basic information from the user
async fn get_user_info(access_code:&str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
	let resp = reqwest::Client::new()
		.get("https://api.twitter.com/2/users/me")
		.header("AUTHORIZATION",format!("Bearer {}",access_code))
		.send()
        .await?
        .json::<serde_json::Value>()
		.await?;
	Ok(resp)
}



// Post a tweet for the user
async fn post_tweet(access_code:&str,text:&str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
	
	let mut post_data = std::collections::HashMap::new();
	post_data.insert("text",text);
	
	let resp = reqwest::Client::new()
		.post("https://api.twitter.com/2/tweets")
		.header("AUTHORIZATION",format!("Bearer {}",access_code))
		.json(&post_data)
		.send()
        .await?
        .json::<serde_json::Value>()
		.await?;
	Ok(resp)
}
