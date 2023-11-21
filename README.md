# Aviation Weather

## Makefile
`make help` to list all commands

## Setup

1. Copy `.env.TEMPLATE` to `.env`
2. Generate JWT RS256 (RSA Signature with SHA-256) Private/Public keys with `make generate`
3. Build the service and ui images with `make build`
4. Run the application with `make up`