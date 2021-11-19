# cargo install just
# More info: https://github.com/casey/just

# Set *default* profile. Use environment variable to override per-run.
profile := "debug"

# Shouldn't be overridden.
_profile_flag := if profile == "release" {"--release"} else {""}

help:
    cargo run -- help

build:
    cargo build {{_profile_flag}}
    mv target/{{profile}}/credential_truth target/{{profile}}/docker-credential-truth
    chmod +x target/{{profile}}/docker-credential-truth

install: build
    sudo cp target/{{profile}}/docker-credential-truth \
        /usr/bin/

doc open="":
    cargo doc {{_profile_flag}} \
        --lib --document-private-items --no-deps \
        {{ if open == "open" {"--open"} else {""} }}
