profile := "debug"
# profile := "release"

# Shouldn't be overridden.
_profile_flag := if profile == "release" {"--release"} else {""}

help:
    cargo run -- help

build:
    cargo build {{_profile_flag}}

install: build
    sudo cp target/{{profile}}/credential_truth \
        /usr/bin/docker-credential-truth

doc open="_":
    cargo doc {{_profile_flag}} \
        --lib --document-private-items --no-deps \
        {{ if open == "open" {"--open"} else {""} }}
