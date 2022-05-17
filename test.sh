# Load env variables
source ./env.sh


# Test to just get a couple of tweets
curl "https://api.twitter.com/2/tweets?ids=1261326399320715264,1278347468690915330" \
  -H "Authorization: Bearer $bearer_token"


