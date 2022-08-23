
default: client

client:
    cargo run --bin mnet_client

server:
    cargo run --bin mnet_server

cli:
    cargo run --bin mnet_cli

devsetup:
    cp dev/hooks/* .git/hooks

fmt:
    cargo +nightly fmt --all

lint:
    cargo clippy -- -W clippy::unwrap_used
