# Essentially the same as mcr.microsoft.com/devcontainers/rust:1-bullseye
# But added some commands

# FROM rustlang/rust:nightly-slim
# FROM rust:1-bullseye
FROM mcr.microsoft.com/devcontainers/rust:1-bullseye

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    # Remove imagemagick due to https://security-tracker.debian.org/tracker/CVE-2019-10131
    && apt-get purge -y imagemagick imagemagick-6-common

RUN apt-get install binaryen
RUN (curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh)

# [Optional] Uncomment this section to install additional OS packages.
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends <your-package-list-here>