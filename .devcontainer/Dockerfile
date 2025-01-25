FROM mcr.microsoft.com/devcontainers/base:bookworm

# Install common utilities and development tools
RUN apt-get update && apt-get upgrade -y \
    && apt-get install -y \
    pkg-config \
    xdg-utils \
    curl \
    git \
    build-essential \
    pkg-config \
    libglib2.0-dev \
    libgtk-3-dev \
    libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libxdo-dev \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    javascriptcoregtk-4.1 \
    libsoup-3.0 \
    webkit2gtk-4.1 \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install cargo-binstall
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

# Install cargo-run-bin
RUN cargo binstall cargo-run-bin --no-confirm

# Install Bun
RUN curl -fsSL https://bun.sh/install | bash
ENV PATH="/root/.bun/bin:${PATH}"

# Install pnpm and Node.js
RUN curl -fsSL https://get.pnpm.io/install.sh | env ENV="/root/.bashrc" SHELL="/bin/bash" PNPM_VERSION=10.0.0 sh -
ENV PATH="/root/.local/share/pnpm:${PATH}"

# Set working directory
WORKDIR /workspaces/task-stack
