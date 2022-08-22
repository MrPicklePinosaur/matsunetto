
default: client

client:
    cargo run --bin mnet-client

devsetup:
    cp dev/hooks/* .git/hooks

fmt:
    cargo +nightly fmt --all

lint:
    cargo clippy
