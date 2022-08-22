
default: client

client:
    cargo run --bin mnet-client

server:
    cargo run --bin mnet-server

devsetup:
    cp dev/hooks/* .git/hooks

fmt:
    cargo +nightly fmt --all

lint:
    cargo clippy
