#!make
SHELL := /bin/bash

GIT_HASH ?= $(shell git log --format="%h" -n 1)

include .env

.PHONY: help build start stop lint

help: ## This info
	@echo
	@cat Makefile | grep -E '^[a-zA-Z\/_-]+:.*?## .*$$' | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
	@echo

build: ## Build the Docker image
	export TAG=${GIT_HASH} && docker compose build

utils: ## Start the utils
	docker compose up -d db
	docker compose up -d redis
	docker compose up -d minio

up: ## Start the Docker containers
	docker compose up -d

down: ## Stop the Docker containers
	docker compose down

connect: ## Connect to the PSQL DB
	docker exec -it ${DATABASE_CONTAINER} psql -U postgres

clean: ## Cleanup Docker containers
	docker compose down && \
	docker image rm aviation-service || \
	docker network rm aviation-frontend || \
	docker network rm aviation-backend

clean-db:  ## Remove database
	docker exec -i ${DATABASE_CONTAINER} sh -c 'PGPASSWORD=${DATABASE_PASSWORD} psql -U ${DATABASE_USER} -d postgres -c "DROP DATABASE IF EXISTS \"${DATABASE_NAME}\";"'
	docker exec -i ${DATABASE_CONTAINER} sh -c 'PGPASSWORD=${DATABASE_PASSWORD} psql -U ${DATABASE_USER} -d postgres -c "CREATE DATABASE \"${DATABASE_NAME}\";"'  || true

generate: ## Generate RSA keys
	mkdir ../keys/
	openssl genrsa -out ../keys/access_private_key.pem 4096
	openssl rsa -in ../keys/access_private_key.pem -pubout -outform PEM -out ../keys/access_public_key.pem
	openssl genrsa -out ../keys/refresh_private_key.pem 4096
	openssl rsa -in ../keys/refresh_private_key.pem -pubout -outform PEM -out ../keys/refresh_public_key.pem
	