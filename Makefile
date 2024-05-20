MAKE=make
CARGO=cargo

BENCHES=

all: fmt lint bench test

fmt:
	@echo "Runing cargo fmt..."
	@$(CARGO) fmt --all -v

lint:
	@echo "Runing cargo check & clippy..."
	@cargo check && cargo clippy --all-targets --all-features --tests --benches -- -D warnings

bench: $(BENCHES)

$(BENCHES):
	@$(CARGO) bench --bench $@

test:
	@echo "Runing rust test ..."
	@$(CARGO) test

build:
	@echo "Build🔧..."
	@$(CARGO) build --release
