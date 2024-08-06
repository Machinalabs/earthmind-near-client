set dotenv-load

run_miner:
    cargo run -- --mode miner --account-id "$ACCOUNT" --private-key "$SECRET_KEY" --network testnet

run_validator:
    cargo run -- --mode validator --account-id "$ACCOUNT" --private-key "$SECRET_KEY" --network testnet
