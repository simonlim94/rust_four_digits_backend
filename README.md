# Rust For Digits Backend
A Rust hobby project that mimics lottery system/4 digits. So what is 4 digits? [4-Digits](https://en.wikipedia.org/wiki/4-Digits)

---
This project is using serverless framework approach to deploy Rust code to AWS Lambda platform. 

[AWS Lambda Rust Runtime](https://github.com/awslabs/aws-lambda-rust-runtime)

[Serverless Rust](https://github.com/softprops/serverless-rust)

---
## Current feature
1. Generating 19 ramdom numbers that are between 0000 and 9999 and storing the results into db (mongoDB is my selection). Cron job is scheduled at 8pm everyday for result of the day.
2. Getting result for the specific day from db through Rest HTTP API.

---
## Pending feature
1. Storing user's betting into db.
2. Matching user's betting against result of the day and calculating winning prize.

---
## Improvement
1. Need to implement proper error handling in this project.
2. Need to change synchoronous approach to asynchoronous approach

---
## How to Deploy if interested. It is free to deploy anyway.
1. Having AWS account. And setup it for serverless deployment credential. Sample:
```
serverless config credentials --provider aws --key AKIAIOSFODNN7EXAMPLE --secret wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY
```
[Refer here for more info](https://www.serverless.com/framework/docs/providers/aws/guide/credentials/)

2. Having mongo Atlas account. And create a mongo Atlas project for this. Creating one db and one collection for the project.

Name of db: `four_digits`

Name of collection (only one is needed for now): `draw_results`

[Refer here for more info](https://docs.atlas.mongodb.com/getting-started/)

3. Setup the database connection credential into AWS Secret Manager for following secret values:

`db_username`: username of db user

`db_password`: password of db user

`db_host`: host url of db

4. Change arn of the secret manager to arn of your own setup secret manager in serverless.yml file. 

`arn:aws:secretsmanager:*region*:*account-id*:secret:*resource-id*`

5. Perform serverless deployment command.

`sudo sls deploy -v`

---
Want to try out my deployed API? More than welcome to try:

Get Result:
```
curl --request POST \
  --url https://doagccylv7.execute-api.us-east-1.amazonaws.com/dev/result \
  --header 'content-type: application/json' \
  --data '{
	"date": "2020-05-18 00:00:00"
}'
```

---
I am still a ametuer Rustacean. Trying to learning Rust through some coding projects like this. This is my first try.
