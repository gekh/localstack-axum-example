# README

This project provides a minimal example of an AWS Lambda function written in Rust, designed to run locally using Localstack on macOS.

## INSTALL

- Install docker

- Install [localstack](https://www.localstack.cloud) (needs docker)

  ```sh
  brew install localstack/tap/localstack-cli
  localstack start -d
  localstack status services
  ```

- Install awslocal
  https://github.com/localstack/awscli-local

  ```
  brew install awscli-local
  ```

- Install compiler for aarch64-unknown-linux-gnu
  https://rustup.rs/
  ```sh
  brew tap messense/macos-cross-toolchains
  brew install aarch64-unknown-linux-gnu
  brew install FiloSottile/musl-cross/musl-cross
  ```

## USAGE

> **Tip:** If you don't have `awslocal` installed, simply replace `awslocal` at the start of each command with:
>
> ```
> aws --endpoint-url=http://localhost:4566
> ```

### CREATE

```sh
rustup target add aarch64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu
cp target/aarch64-unknown-linux-gnu/release/basic-axum-project bootstrap
chmod +x bootstrap
zip -9 lambda.zip bootstrap
awslocal lambda create-function \
  --function-name basic-axum-project \
  --runtime provided.al2023 \
  --architectures arm64 \
  --role arn:aws:iam::000000000000:role/lambda-role \
  --handler bootstrap \
  --zip-file fileb://lambda.zip
rm lambda.zip bootstrap response.json
```

_(press "q" to exit status message when lambda is created)_

## CHECK

```sh
awslocal lambda get-function \
  --function-name basic-axum-project
```

You should see this when lambda is created/updated successfully:

```json
{
  // ...
  "State": "Active",
  "LastUpdateStatus": "Successful"
  // ...
}
```

## UPDATE

```
rm lambda.zip bootstrap response.json
cargo build --release --target aarch64-unknown-linux-gnu
cp target/aarch64-unknown-linux-gnu/release/basic-axum-project bootstrap
chmod +x bootstrap
zip -9 lambda.zip bootstrap
awslocal lambda update-function-code \
  --function-name basic-axum-project \
  --zip-file fileb://lambda.zip
rm lambda.zip bootstrap
```

_(press "q" to exit status message after lambda is updated)_

## INVOKE

```sh
awslocal lambda invoke \
  --function-name basic-axum-project \
  --payload file://invoke_events/root.json \
  --cli-binary-format raw-in-base64-out \
  response.json
```

In `--payload file://invoke_events/root.json` you can change the payload to test different endpoints. Like `--payload file://invoke_events/hello_get.json`

Print result:

```sh
cat response.json
#or
cat response.json | jq .body | tr -d '\\\\' | sed 's/^\"//;s/\"$//' | jq
```

_(install jq for second option)_

(Optional) Remove response.json

```sh
rm response.json
```

## DESTROY

```
awslocal lambda delete-function \
  --function-name basic-axum-project
```

# NOTES

To install jq on macOS:

```sh
brew install jq
```
