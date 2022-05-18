High-level overview:
1. Generate a link for the user to click on.
2. Listen for the response from Twitter to get an auth code
3. Use that auth token within 30 seconds to get an access token
4. Make a request with the access token to get user information
5. Make a request with the access token to post a tweet

The following crates are used:
- Reqwest to make network requests
- Actix-web to receive data from Twitter's servers
- Serde and serde_json to parse all the JSON data
