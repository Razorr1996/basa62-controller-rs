[private]
default: list-recipes fmt generate

[private]
list-recipes:
    @just --list --unsorted

# install crd into the cluster
install-crd context='minikube': generate
    kubectl --context {{context}} apply -f yaml/crd.yaml

# generate yaml files for CRD and example instance
generate:
    cargo run --bin crdgen > yaml/crd.yaml
    cargo run --bin example > yaml/instance-example.yaml

# format with nightly rustfmt
fmt:
    cargo +nightly fmt

# generate report for unused features
unused-features:
    unused-features analyze
    unused-features build-report
