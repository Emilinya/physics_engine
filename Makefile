VERBOSE ?=
RELEASE ?=

SILENCE = @
ifeq ($(VERBOSE), 1)
	SILENCE =
endif

TARGET = debug
CARGO_OPTS ?=
CLIPPY_OPTS ?=
ifeq ($(RELEASE), 1)
	override CARGO_OPTS += --release
	TARGET = release
endif

.PHONY: build
build:
	$(SILENCE)cargo build $(CARGO_OPTS)
	$(SILENCE)cp  ./target/$(TARGET)/physics_engine .

.PHONY: build-wasm
build-wasm:
	RUSTFLAGS='--cfg getrandom_backend="wasm_js"' \
		cargo build --target wasm32-unknown-unknown \
		--profile wasm-release --no-default-features
	wasm-bindgen --no-typescript --target web \
		--out-dir ./wasm/ --out-name "physics_engine" \
		./target/wasm32-unknown-unknown/wasm-release/physics_engine.wasm

.PHONY: format
format:
	$(SILENCE)cargo fmt

.PHONY: check
check:
	$(SILENCE)cargo test -q

.PHONY: lint
lint:
	$(SILENCE)cargo clippy --all-targets -q -- $(CLIPPY_OPTS)

.PHONY: ci
ci:
	$(SILENCE)make format
	$(SILENCE)make build
	$(SILENCE)make lint CLIPPY_OPTS="-D warnings"
	$(SILENCE)make check
