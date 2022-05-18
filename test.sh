# Load env variables
source ./env.sh



# Create a server on localhost:8080 and listen
#key=$(nc -l 8080 | grep -oP '(?<=code=).*?(?=\s)')
#echo $key

#curl --location --request POST 'https://api.twitter.com/2/oauth2/token' \
#--header 'Content-Type: application/x-www-form-urlencoded' \
#--data-urlencode 'code='$key \
#--data-urlencode 'grant_type=authorization_code' \
#--data-urlencode 'client_id='$client_id \
#--data-urlencode 'redirect_uri=https://www.localhost.com:8080' \
#--data-urlencode 'code_verifier=challenge'


curl --location --request POST 'http://localhost.com:3000' \
--header 'Content-Type: application/x-www-form-urlencoded' \
--data-urlencode 'code=testkey' \
--data-urlencode 'grant_type=authorization_code' \
--data-urlencode 'client_id='$client_id \
--data-urlencode 'redirect_uri=https://www.localhost.com:8080' \
--data-urlencode 'code_verifier=challenge'
