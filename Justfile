help:
    cargo run -- help

build p="dev":
    cargo build {{ if p == "release" { "--release" } else { "" } }}

install p="dev": (build p)
    sudo cp target/{{ if p == "release" { "release" } else { "debug" } }}/credential_truth \
        /usr/bin/docker-credential-truth
