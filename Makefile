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
	$(SILENCE)cargo build --profile wasm-release --target wasm32-unknown-unknown
	$(SILENCE)wasm-bindgen --no-typescript --target web \
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
	$(SILENCE)cargo clippy -q -- -W clippy::use_self $(CLIPPY_OPTS)

.PHONY: lint-pedantic
lint-pedantic:
	$(SILENCE)cargo clippy -- -W clippy::pedantic -W clippy::nursery -W clippy::clippy::restriction \
	  -A clippy::cast_precision_loss -A clippy::cast_possible_truncation -A clippy::module_name_repetitions

.PHONY: ci
ci:
	$(SILENCE)make format
	$(SILENCE)make build
	$(SILENCE)make lint CLIPPY_OPTS="-D warnings"
	$(SILENCE)make check