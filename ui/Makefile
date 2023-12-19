#!make
SHELL := /bin/bash

GIT_HASH ?= $(shell git log --format="%h" -n 1)

include .env

.PHONY: help build start stop lint

help: ## This info
	@echo
	@cat Makefile | grep -E '^[a-zA-Z\/_-]+:.*?## .*$$' | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
	@echo

build:  ## Install the dependencies and build
	export TAG=${GIT_HASH} && docker compose build

up:  ## Start the dev instance
	docker compose up -d

down:  ## Stop the dev instance
	docker compose down

lint:  ## Run the linter
	npm run lint

clean:  ## Remove node modules
	docker compose down && \
	docker image rm aviation-ui