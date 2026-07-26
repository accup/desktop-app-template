FROM mcr.microsoft.com/devcontainers/base:ubuntu24.04

# Configuration
## Environment
ARG USERNAME=vscode
## Rust
ARG RUSTUP_VERSION=1.29.0
ARG RUSTUP_INIT_SHA256=4acc9acc76d5079515b46346a485974457b5a79893cfb01112423c89aeb5aa10
ARG RUST_ARCH=x86_64-unknown-linux-gnu
ARG RUST_VERSION=1.97.1
## Node.js
ARG PNPM_VERSION=11.14.0
ARG NODE_VERSION=24.18.0
## just
ARG JUST_VERSION=1.56.0
ARG JUST_SHA256=fa2a8ec1015d9df5330941ade12437488fc40d33f9c9f8cd4eb70a26de11b639

# Root-level setup
ENV PATH="/usr/local/cargo/bin:$PATH"

## Install Rust
ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ARG RUSTUP_INIT_URL="https://static.rust-lang.org/rustup/archive/${RUSTUP_VERSION}/${RUST_ARCH}/rustup-init"

RUN TMP_DIR="$(mktemp -d)" && \
    TMP_RUSTUP_INIT="${TMP_DIR}/rustup-init" && \
    curl -fsSL "${RUSTUP_INIT_URL}" -o "${TMP_RUSTUP_INIT}" && \
    echo "${RUSTUP_INIT_SHA256} *${TMP_RUSTUP_INIT}" | sha256sum -c - && \
    chmod +x "${TMP_RUSTUP_INIT}" && \
    "${TMP_RUSTUP_INIT}" -y --no-modify-path --profile minimal --default-toolchain "${RUST_VERSION}" --default-host "${RUST_ARCH}" && \
    rm -rf "${TMP_DIR}" && \
    chmod -R a+w "${RUSTUP_HOME}" "${CARGO_HOME}"

## Install graphics libraries
### GPU パススルーのない WSLg 上でも wgpu がサーフェスを作成できるよう、
### Vulkan ローダーと Mesa のソフトウェアレンダリングドライバーを導入する。
### libxcursor1・libxi6・libxkbcommon0・libxkbcommon-x11-0 は
### winit の X11 バックエンドが実行時に動的ロードするため必要
### desktop-file-utils と file は AppImage の生成時に appimagetool が必要とする
RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
      desktop-file-utils \
      file \
      libegl1 \
      libgl1 \
      libgles2 \
      libvulkan1 \
      libxcursor1 \
      libxi6 \
      libxkbcommon-x11-0 \
      libxkbcommon0 \
      mesa-vulkan-drivers && \
    rm -rf /var/lib/apt/lists/*

# User-level setup
USER "${USERNAME}"
ENV PNPM_HOME="/home/${USERNAME}/.local/share/pnpm"
ARG LOCAL_BIN_DIR="/home/${USERNAME}/.local/bin"
ENV PATH="${PNPM_HOME}/bin:${PATH}"
ENV PATH="${LOCAL_BIN_DIR}:$PATH"

## Ensure local bin directory exists
RUN mkdir -p "${LOCAL_BIN_DIR}"

## Install pnpm
ARG PNPM_INSTALL_URL="https://get.pnpm.io/install.sh"
RUN curl -fsSL "${PNPM_INSTALL_URL}" \
      | ENV="$HOME/.bashrc" SHELL="$(which bash)" PNPM_VERSION="${PNPM_VERSION}" bash - && \
    pnpm config set store-dir ~/.pnpm-store --global

## Install Node.js
RUN pnpm runtime set node "${NODE_VERSION}" --global

## Install just
ARG JUST_URL="https://github.com/casey/just/releases/download/${JUST_VERSION}/just-${JUST_VERSION}-x86_64-unknown-linux-musl.tar.gz"
RUN TMP_DIR="$(mktemp -d)" && \
    TMP_JUST_ARCHIVE="${TMP_DIR}/just.tar.gz" && \
    curl -fsSL "${JUST_URL}" -o "${TMP_JUST_ARCHIVE}" && \
    echo "${JUST_SHA256} *${TMP_JUST_ARCHIVE}" | sha256sum -c - && \
    tar -xzf "${TMP_JUST_ARCHIVE}" -C "${TMP_DIR}" just && \
    mv "${TMP_DIR}/just" "${LOCAL_BIN_DIR}/just" && \
    rm -rf "${TMP_DIR}"

## Install Claude Code
ARG CLAUDE_INSTALL_URL="https://claude.ai/install.sh"
RUN curl -fsSL "${CLAUDE_INSTALL_URL}" | bash
