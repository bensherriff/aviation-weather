#!make

include .env

SHELL := /bin/bash

.PHONY: help build start stop lint

help: ## This info
	@echo
	@cat Makefile | grep -E '^[a-zA-Z\/_-]+:.*?## .*$$' | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
	@echo

build: ## Build Docker service container
	docker compose build

up: ## Start Docker service containers
	docker compose up -d

down: ## Stop Docker service containers
	docker compose down

connect: ## Connect to the Weather DB
	docker exec -it ${DATABASE_CONTAINER} psql -U postgres

clean: ## Cleanup Docker containers
	docker compose down && \
	docker image rm weather-ui || \
	docker image rm weather-service || \
	docker network rm weather-frontend || \
	docker network rm weather-backend

clean-db:  ## Remove database
	docker exec -i ${DATABASE_CONTAINER} sh -c 'PGPASSWORD=${DATABASE_PASSWORD} psql -U ${DATABASE_USER} -d postgres -c "DROP DATABASE IF EXISTS \"${DATABASE_NAME}\";"'
	docker exec -i ${DATABASE_CONTAINER} sh -c 'PGPASSWORD=${DATABASE_PASSWORD} psql -U ${DATABASE_USER} -d postgres -c "CREATE DATABASE \"${DATABASE_NAME}\";"'  || true

	