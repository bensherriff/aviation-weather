#!make

SHELL := /bin/bash

.PHONY: help build start stop lint

help: ## This info
	@echo
	@cat Makefile | grep -E '^[a-zA-Z\/_-]+:.*?## .*$$' | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
	@echo

install:  ## Install the dependencies
	npm install

build:  ## Install the dependencies and build
	npm run build

start:  ## Start the dev instance
	npm run dev

lint:  ## Run the linter
	npm run lint

clean:  ## Remove node modules
	rm -rf node_modules