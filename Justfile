# https://just.systems/man/en

# Lists available commands
default:
	just --list

# Starts the development container
dev:
	docker compose -f dev/docker-compose.dev.yml up --build --detach
