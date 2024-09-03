#!make
SHELL := /bin/bash

GIT_HASH ?= $(shell git log --format="%h" -n 1)

include .env
-include .env.local
export

.PHONY: help build start stop lint

help: ## This info
	@echo
	@cat Makefile | grep -E '^[a-zA-Z\/_-]+:.*?## .*$$' | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
	@echo

format: ## Format code
	@echo "Formatting code..."
	@cargo fmt
	@echo "Format complete"

run: ## Run the service
	@cargo run

clean: ## Cleanup
	@echo "Cleaning up..."
	@cargo clean
	@rm -rf ../keys
	@echo "Cleanup complete"

up: ## Start the Docker containers
	@docker compose --profile backend up -d

down: ## Stop the Docker containers
	@docker compose --profile backend down

connect: ## Connect to the PSQL DB
	@docker exec -it ${DATABASE_CONTAINER} psql -U postgres

docker-build: ## Build the Docker image
	@docker compose build

docker-tag: ## Tag the Docker image
	@docker tag aviation-service:latest aviation-service:${GIT_HASH}

docker-run: ## Start the service
	@docker compose --profile service up -d

docker-clean: ## Cleanup Docker containers
	@docker compose --profile backend --profile service down -v

clean-db:  ## Remove database
	@docker exec -i ${DATABASE_CONTAINER} sh -c 'PGPASSWORD=${DATABASE_PASSWORD} psql -U ${DATABASE_USER} -d postgres -c "DROP DATABASE IF EXISTS \"${DATABASE_NAME}\";"'
	@docker exec -i ${DATABASE_CONTAINER} sh -c 'PGPASSWORD=${DATABASE_PASSWORD} psql -U ${DATABASE_USER} -d postgres -c "CREATE DATABASE \"${DATABASE_NAME}\";"'  || true

generate-keys: ## Generate RSA keys
	@mkdir ../keys/
	@openssl genrsa -out ../keys/access_private_key.pem 4096
	@openssl rsa -in ../keys/access_private_key.pem -pubout -outform PEM -out ../keys/access_public_key.pem
	@openssl genrsa -out ../keys/refresh_private_key.pem 4096
	@openssl rsa -in ../keys/refresh_private_key.pem -pubout -outform PEM -out ../keys/refresh_public_key.pem

psql: ## Connect to the PSQL DB
	@docker exec -it ${DATABASE_CONTAINER} psql -U ${DATABASE_USER}