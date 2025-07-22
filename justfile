# List all available recipes
default:
    @just --list

# Generate documentation
doc:
    cargo doc --open

# Download Starlink Protobuf spec
download-protobuf-spec:
    curl -L https://raw.githubusercontent.com/SpaceExplorationTechnologies/enterprise-api/31197c8016bab0e0aa4d70ac45a28b3f7fd7fb8d/device-api/device.proto -o vendor/device.proto

# Download Starlink OpenAPI spec
download-openapi-spec:
    curl -L https://web-api.starlink.com/enterprise/swagger/v1/swagger.json -o vendor/swagger.json

generate-openapi lang="rust":
    openapi-generator-cli generate -i ./vendor/swagger.json  -g {{lang}} -o ./openapi
