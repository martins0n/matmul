data_prep:
	poetry run python -m utils.data_gen

generate_md:
	poetry run jupyter nbconvert notebooks/results.ipynb --to markdown --no-input --output BENCHMARK.md

build_python:
	cd python && poetry install

run_python: build_python
	cd python && poetry run python -m src $(ARGS)

build_rust:
	cd rust && cargo build --release

run_rust: build_rust
	cd rust && target/release/matmul $(ARGS)


build_rust_nighty:
	cd rust && RUSTFLAGS='-C target-feature=+avx2 -C target-cpu=native' cargo +nightly build --release

run_rust_nightly:
	cd rust && target/release/matmul $(ARGS)