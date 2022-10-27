#!/usr/bin/make -f
CONTAINER_RUNTIME := $(shell which docker 2>/dev/null || which podman 2>/dev/null)

### Use cosmwasm/rust-optimizer-arm64 on M1 Macs (https://hub.docker.com/r/cosmwasm/rust-optimizer-arm64)
OPTIMIZER_IMAGE := cosmwasm/rust-optimizer
### 0.12.9 is the latest tag (https://hub.docker.com/r/cosmwasm/rust-optimizer/tags)
OPTIMIZER_DOCKER_TAG := 0.12.9

.PHONY: all
all: clean fmt lint test schema optimize

.PHONY: dev
dev: fmt lint test schema

.PHONY: clean
clean:
	rm -rf artifacts/ schema/
	@cargo clean

.PHONY: fmt
fmt:
	@cargo fmt --all -- --check

.PHONY: lint
lint:
	@cargo clippy -- -D warnings

.PHONY: build
build:
	@cargo wasm

.PHONY: test
test:
	@cargo test

### TODO: Add cargo docs rule

.PHONY: schema
schema:
	@cargo schema

.PHONY: optimize
optimize:
	$(CONTAINER_RUNTIME) run --rm -v $(CURDIR):/code:Z \
		--mount type=volume,source=validation_oracle_smart_contract_cache,target=/code/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		$(OPTIMIZER_IMAGE):$(OPTIMIZER_DOCKER_TAG)
