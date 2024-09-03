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
	@cd api && cargo fmt
	@cd ui && npm run format

backend-up: ## Start Docker containers
	@docker compose --profile backend up -d

up-backend: backend-up

backend-down: ## Stop Docker containers
	@docker compose --profile backend down

down-backend: backend-down

run: ## Run the api
	@cd api && cargo run

frontend-up: ## Start Docker containers
	@docker compose --profile frontend up -d

up-frontend: frontend-up

frontend-down: ## Stop Docker containers
	@docker compose --profile frontend down

down-frontend: frontend-down

docker-clean: ## Stop the docker containers and remove volumes
	@echo "Stopping docker container and removing volumes..."
	@docker compose --profile frontend --profile api --profile backend down -v
	@echo "Docker container stopped and volumes removed"

docker-refresh: docker-clean up-backend ## Refresh the database

psql: ## Connect to the PSQL DB
	@docker exec -it ${DATABASE_CONTAINER} psql -U ${DATABASE_USER} -P pager=off