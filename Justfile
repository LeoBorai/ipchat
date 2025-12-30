# https://just.systems/man/en

# Lists available commands
default:
	just --list

# Builds both client and server
build: build-client build-server
	@echo "Build complete."

# Builds the client (Force)
build-client:
	cd src/client && bun run build:f

# Builds the server (Without optimizations)
build-server:
	cargo b

# Run clippy and format the code
clippy:
	cargo clippy --workspace --fix --allow-dirty --allow-staged
	cargo fmt --all

# Starts the development container
dev:
	docker compose -f dev/docker-compose.dev.yml up --build --detach

# Generates the OpenAPI specification file
gen-openapi:
	curl http://localhost:4724/api-docs/openapi.json > ./src/client/assets/openapi.json

# Builds the client for Release
release-client:
	cd ./src/ui && trunk build --release

# Builds the server for Release
release-server:
	cargo build --release

# Release build
release: release-client release-server
	@echo "Release build complete."

# Starts the server for Development
run: build
	./target/debug/ipchat start

# Runs the Client UI for Development
run-client:
	cd ./src/ui && trunk serve

# Stops the development container
undev:
	docker compose -f dev/docker-compose.dev.yml down
