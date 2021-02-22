IMAGE ?= acria-network
IMAGE_DEV ?= $(IMAGE)-dev

.PHONY: init
init:
	./scripts/init.sh

.PHONY: check
check:
	SKIP_WASM_BUILD=1 cargo check

.PHONY: test
test:
	SKIP_WASM_BUILD=1 cargo test --release --all

.PHONY: pallettest
pallettest:
	cd pallet/acria/
	cargo  test
	cd ../..

.PHONY: run
run:
	cargo  build --release --all
	target/release/acria-node --dev --tmp

.PHONY: build
build:
	cargo  build --release --all

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

