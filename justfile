[private]
default:
    @just --list --unsorted

# format with nightly rustfmt
fmt:
    cargo +nightly fmt

generate:
    cargo run --bin crdgen > yaml/crd.yaml
