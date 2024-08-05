set dotenv-load

run:
    cargo run -- --mode validator --account-id "$ACCOUNT" --private-key "$SECRET_KEY" --network testnet
