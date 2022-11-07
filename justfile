default:
    run
start:
  cargo run -p oxa

run:
    cargo run -p oxa

generate:
    cargo run -p ast_generator

fmt:
    cargo fmt --all -- --check

clippy:
    cargo clippy --all-targets --release -- -D warnings

test:
    cargo test --all-targets --workspace

build:
    cargo build --all-targets --workspace

bench:
    # Docker copy code and install dependencies
    # Docker run the code
    echo "Run code bench markings"

coverage:
    # Docker copy code and install dependencies
    # Docker run the code
    docker build . -t oxa-coverage:build
    docker run oxa-coverage:build