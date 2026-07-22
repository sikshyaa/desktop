lint-rust:
    cargo fmt --all

prettify:
    pnpx prettier -w src docs *.yaml *.json

test-coverage:
    cd src-tauri && cargo llvm-cov --workspace
