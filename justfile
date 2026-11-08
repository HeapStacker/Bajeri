[windows]
set shell := ["powershell.exe", "-Command"]

[unix]
set shell := ["sh", "-c"]

default:
    just setup

setup:
    cargo install cargo-binstall
    cargo binstall -y agent cargo-nextest cargo-llvm-cov cargo-deny cargo-machete cargo-about

fmt:
    cargo fmt

lint:
    cargo clippy --all-targets -- -D warnings

test:
    cargo nextest run
    cargo test --doc

coverage:
    cargo llvm-cov --fail-under-lines 80 --html

check:
    cargo machete
    cargo deny check
    just lint
    just test
    just coverage

clean:
    cargo clean
