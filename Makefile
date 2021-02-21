IMAGE ?= acria-network
IMAGE_DEV ?= $(IMAGE)-dev

.PHONY: init
init:
	rustup self update
	rustup update stable
	rustup update nightly-2020-10-05
	rustup target add wasm32-unknown-unknown --toolchain nightly-2020-10-05
	rustup default nightly-2020-10-05

.PHONY: check
check:
	SKIP_WASM_BUILD=1 cargo check

.PHONY: test
test:
	SKIP_WASM_BUILD=1 cargo test --release --all

.PHONY: pallettest
pallettest:
	cd pallet/acria/
	cargo +nightly-2020-10-05 test
	cd ../..

.PHONY: run
run:
	cargo +nightly-2020-10-05 build --release --all
	target/release/acria-node --dev --tmp

.PHONY: build
build:
	cargo +nightly-2020-10-05 build --release --all

.PHONY: release
release:
	@$(DOCKER) build --no-cache --squash -t $(IMAGE) .

.PHONY: dev-docker-build
dev-docker-build:
	@$(DOCKER) build -t $(IMAGE_DEV) .

.PHONY: dev-docker-run
dev-docker-run:
	@$(DOCKER) run --net=host -it --rm $(IMAGE_DEV) --dev --tmp

.PHONY: dev-docker-inspect
dev-docker-inspect:
	@$(DOCKER) run --net=host -it --rm --entrypoint /bin/bash $(IMAGE_DEV)
