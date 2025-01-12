from mcr.microsoft.com/devcontainers/rust:1-1-bullseye

# Install additional packages
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
    # Install additional packages
    git \
    # Install Rust components
    && rustup component add rust-src rls rust-analysis rustfmt clippy