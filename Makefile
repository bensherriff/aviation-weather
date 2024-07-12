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

build: ## Build Docker containers
	docker compose build

tag: ## Tag Docker images
	docker tag aviation-ui:latest aviation-ui:${GIT_HASH}
	docker tag aviation-service:latest aviation-service:${GIT_HASH}

up: ## Start Docker containers
	docker compose up -d

down: ## Stop Docker containers
	docker compose down

clean: ## Cleanup Docker containers
	docker compose down && \
	docker image rm aviation-ui || \
	docker image rm aviation-service || \
	docker network rm aviation-frontend || \
	docker network rm aviation-backend

generate: ## Generate RSA keys
	mkdir keys
	openssl genrsa -out keys/access_private_key.pem 4096
	openssl rsa -in keys/access_private_key.pem -pubout -outform PEM -out keys/access_public_key.pem
	openssl genrsa -out keys/refresh_private_key.pem 4096
	openssl rsa -in keys/refresh_private_key.pem -pubout -outform PEM -out keys/refresh_public_key.pem
