data_prep:
	poetry run python -m utils.data_gen

generate_md:
	poetry run jupyter nbconvert notebooks/results.ipynb --to markdown --no-input --output BENCHMARK.md

build_python:
	cd python && poetry install

run_python: build_python
	cd python && poetry run python -m src $(ARGS)

build_rust:
	cd rust && RUSTFLAGS='-C target-feature=+avx2 -C target-cpu=native --emit asm -C llvm-args=-x86-asm-syntax=intel' cargo build --release

run_rust: build_rust
	cd rust && target/release/matmul $(ARGS)

build_rust_nighty:
	cd rust && RUSTFLAGS='--emit asm -C target-feature=+avx2 -C target-cpu=native --emit asm -C llvm-args=-x86-asm-syntax=intel' cargo +nightly build --release

run_rust_nightly:
	cd rust && target/release/matmul $(ARGS)