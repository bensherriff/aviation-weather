#!make

include .env

SHELL := /bin/bash

.PHONY: help build start stop lint

help: ## This info
	@echo
	@cat Makefile | grep -E '^[a-zA-Z\/_-]+:.*?## .*$$' | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
	@echo

build: ## Build Docker containers
	docker compose build

up: ## Start Docker containers
	docker compose up -d

down: ## Stop Docker containers
	docker compose down

clean: ## Cleanup Docker containers
	docker compose down && \
	docker image rm weather-ui || \
	docker image rm weather-service || \
	docker network rm weather-frontend || \
	docker network rm weather-backend

generate: ## Generate RSA keys
	mkdir keys
	openssl genrsa -out keys/access_private_key.pem 4096
	openssl rsa -in keys/access_private_key.pem -pubout -outform PEM -out keys/access_public_key.pem
	openssl genrsa -out keys/refresh_private_key.pem 4096
	openssl rsa -in keys/refresh_private_key.pem -pubout -outform PEM -out keys/refresh_public_key.pem
