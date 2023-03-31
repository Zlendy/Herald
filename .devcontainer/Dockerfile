# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="bullseye"
FROM mcr.microsoft.com/vscode/devcontainers/rust:1-${VARIANT}

# [Optional] Uncomment this section to install additional packages.
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends librust-gio-dev librust-gdk-pixbuf-dev librust-pangocairo-dev libgraphene-1.0-dev

# This will not work, gtk4 does not exist in Debian 11