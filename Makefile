include .env

GREEN  := $(shell tput -Txterm setaf 2)
YELLOW := $(shell tput -Txterm setaf 3)
WHITE  := $(shell tput -Txterm setaf 7)
CYAN   := $(shell tput -Txterm setaf 6)
RESET  := $(shell tput -Txterm sgr0)
HELPER_VERSION=$(shell cat version.txt)
BUILD_DATE=$(shell date --rfc-3339=seconds)

.PHONY: build build-linux-x86 help test

## Build
build-dev: ## Build application in debug mode; executable will be placed in target/debug
	cargo build

build: ## Build application with release optimizations; executable will be placed in target/release
	cargo build --release

build-linux-x86: ## Build application with release optimizations cross-compiled to linux x86_64, to be used when build env is different os or arch
	@echo "Build and cross compile release for linux x86 with glibc"
	RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc' cargo build --target x86_64-unknown-linux-gnu --release;

	@echo "Push to Artifactory"
	@curl -u $(ARTIFACTORY_USER):$(ARTIFACTORY_PASS) -X PUT https://room303.jfrog.io/artifactory/rust-binary/cli-cross-x86_64-1.1.3 -T /app/target/x86_64-unknown-linux-gnu/release/cli-cross-x86

## Run
run: ## Run application
	cargo run

## Push
push: ## Push to artifactory
	curl -u $(USERNAME) -X PUT "https://artifacts.mastercard.int/artifactory/snapshots/com/mastercard/mipv/certgen" -T target/release/certgen -k

## Test:
test: ## Run the tests of the project
	cargo test

## Lint:
lint: ## Run clippy linter on project with expect and unwrap used checks enabled
	cargo clippy -- -D clippy::expect_used -D clippy::unwrap_used

## Help:
help: ## Show this help.
	@echo ''
	@echo 'Usage:'
	@echo '  ${YELLOW}make${RESET} ${GREEN}<target>${RESET}'
	@echo ''
	@echo 'Targets:'
	@awk 'BEGIN {FS = ":.*?## "} { \
		if (/^[a-zA-Z0-9_-]+:.*?##.*$$/) {printf "    ${YELLOW}%-20s${GREEN}%s${RESET}\n", $$1, $$2} \
		else if (/^## .*$$/) {printf "  ${CYAN}%s${RESET}\n", substr($$1,4)} \
		}' $(MAKEFILE_LIST)
