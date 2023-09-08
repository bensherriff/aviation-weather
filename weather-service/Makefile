#!make

include .env

SHELL := /bin/bash

.PHONY: help build start stop lint

help: ## This info
	@echo
	@cat Makefile | grep -E '^[a-zA-Z\/_-]+:.*?## .*$$' | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
	@echo

up:
	docker compose up -d

down:
	docker compose down

connect:
	docker exec -it aviation_weather_db psql -U postgres

lint:  ## Run the linter
	npm run lint

clean-db:  ## Remove database and Cargo packages
	docker exec -i ${DATABASE_HOST} sh -c 'PGPASSWORD=${DATABASE_PASSWORD} psql -U ${DATABASE_USER} -d postgres -c "DROP DATABASE IF EXISTS \"${DATABASE_NAME}\";"'  || true

	