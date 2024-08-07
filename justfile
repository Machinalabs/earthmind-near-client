set dotenv-load

run_miner:
    rm -rf data
    cargo run -- --mode miner --account-id "$ACCOUNT" --private-key "$SECRET_KEY" --network testnet

run_validator:
    rm -rf data
    cargo run -- --mode validator --account-id "$ACCOUNT" --private-key "$SECRET_KEY" --network testnet
