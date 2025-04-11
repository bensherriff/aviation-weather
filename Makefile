#!make
SHELL := /bin/bash

export API_VERSION = $(shell awk -F ' = ' '$$1 ~ /package.version/ { gsub(/[\"]/, "", $$2); printf("%s",$$2) }' api/Cargo.toml)
export UI_VERSION := $(shell awk -F'"' '/"version"/ { print $$4 }' ui/package.json)

include .env
-include .env.local
export

.PHONY: help build start stop lint

help: ## This info
	@echo
	@cat Makefile | grep -E '^[a-zA-Z\/_-]+:.*?## .*$$' | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
	@echo

format: format-api format-ui ## Format code

psql: ## Connect to the PSQL DB
	@docker exec -it aviation-postgres psql -U ${POSTGRES_USER} -P pager=off

#################
# API Commands  #
#################

format-api: ## Format code
	@cd api && cargo fmt

build-api: ## Build the project
	@cd api && cargo build

run-api: ## Run the API project
	@cd api && cargo run -p api

#################
#  UI Commands  #
#################

lint-ui:  ## Run the linter
	@cd ui && npm run lint

format-ui: ## Run the formatter
	@cd ui && npm run format

build-ui:  ## Build the UI app
	@cd ui && npm install && npm run build

clean-ui:  ## Remove UI build files
	@cd ui && rm -rf node_modules dist

run-ui:  ## Run the UI app
	@cd ui && npm install && npm run dev

###################
# Docker Commands #
###################

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

docker-prune: ## Prune the docker system
	@docker system prune -a

docker-clean: ## Stop the docker containers and remove volumes
	@docker compose --profile frontend --profile api --profile backend down -v

docker-down: ## Stop the docker container
	@docker compose --profile frontend --profile api --profile backend down

docker-up: ## Start the docker container
	@docker compose --profile backend --profile api --profile frontend up -d

docker-refresh: docker-clean up-backend ## Refresh the database

refresh: docker-refresh

build: version=$(if $(v),$(v),latest)
build: folder=$(if $(f),$(f),httpd)
build: image=aviation-${folder}:${version}
build: ## Build a specific docker image (`make build f=httpd`)
	docker buildx build \
	-f ${folder}/Dockerfile \
	-t ${image} \
	--load \
	--build-arg BUILD_DATE=$$(date -u +'%Y-%m-%dT%H:%M:%SZ') \
	--build-arg BUILD_VERSION=${version} \
	--build-arg VCS_REF=$$(git rev-parse head) \
	${folder}

docker-build: build
