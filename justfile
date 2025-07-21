# List all available recipes
default:
    @just --list

# Build the project
build:
    cargo build

# Build in release mode
build-release:
    cargo build --release

# Run the project
run:
    cargo run

# Run tests
test:
    cargo test

# Run the basic usage example
example:
    cargo run --example basic_usage

# Format code
fmt:
    cargo fmt

# Check formatting
fmt-check:
    cargo fmt -- --check

# Run clippy
clippy:
    cargo clippy -- -D warnings

# Clean build artifacts
clean:
    cargo clean

# Check project (format, clippy, test)
check: fmt-check clippy test

# Generate documentation
doc:
    cargo doc --open

# Download Starlink Protobuf spec
download-protobuf-spec:
    curl -L https://raw.githubusercontent.com/SpaceExplorationTechnologies/enterprise-api/31197c8016bab0e0aa4d70ac45a28b3f7fd7fb8d/device-api/device.proto -o vendor/device.proto

# Download Starlink OpenAPI spec
download-openapi-spec:
    curl -L https://web-api.starlink.com/enterprise/swagger/v1/swagger.json -o vendor/swagger.json

