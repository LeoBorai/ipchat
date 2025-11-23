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

# Starts the development container
dev:
	docker compose -f dev/docker-compose.dev.yml up --build --detach

# Starts the server for Development
run: build
	./target/debug/ipchat start
