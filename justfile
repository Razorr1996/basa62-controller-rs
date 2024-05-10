[private]
default:
    @just --list --unsorted

# install crd into the cluster
install-crd: generate
    kubectl apply -f yaml/crd.yaml

generate:
    cargo run --bin crdgen > yaml/crd.yaml
    cargo run --bin example > yaml/instance-example.yaml

# format with nightly rustfmt
fmt:
    cargo +nightly fmt

unused-features:
    unused-features analyze
    unused-features build-report
