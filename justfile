appimagetool_version := "1.9.0"
appimagetool_sha256 := "46fdd785094c7f6e545b61afcfb0f3d98d8eab243f644b4b17698c01d06083d1"

default:
    just --list

format:
    cargo fmt --all
    pnpm run format

check:
    cargo check --workspace --all-targets

test:
    cargo test --workspace

lint:
    cargo clippy --workspace --all-targets -- -D warnings

doc:
    RUSTDOCFLAGS='-D warnings' cargo doc --workspace

build:
    cargo build --release --workspace

[linux]
bundle: build
    #!/usr/bin/env bash
    set -euo pipefail
    tool=target/tools/appimagetool
    if [ ! -x "$tool" ]; then
        mkdir -p target/tools
        curl -fsSL "https://github.com/AppImage/appimagetool/releases/download/{{ appimagetool_version }}/appimagetool-x86_64.AppImage" -o "$tool"
        echo "{{ appimagetool_sha256 }} *$tool" | sha256sum -c -
        chmod +x "$tool"
    fi
    appdir=target/bundle/winker.AppDir
    rm -rf target/bundle
    mkdir -p "$appdir/usr/bin"
    cp target/release/app-desktop "$appdir/usr/bin/winker"
    cp bundle/winker.desktop bundle/winker.svg "$appdir/"
    ln -s usr/bin/winker "$appdir/AppRun"
    # FUSE のない環境でも appimagetool を実行できるよう自己展開モードを使う
    APPIMAGE_EXTRACT_AND_RUN=1 ARCH={{ arch() }} "$tool" "$appdir" "target/bundle/winker-{{ arch() }}-unknown-linux-gnu.AppImage"

[macos]
bundle: build
    #!/usr/bin/env bash
    set -euo pipefail
    version=$(sed -n 's/^version = "\(.*\)"$/\1/p' Cargo.toml)
    app=target/bundle/winker.app
    rm -rf target/bundle
    mkdir -p "$app/Contents/MacOS"
    cp target/release/app-desktop "$app/Contents/MacOS/winker"
    sed "s/BUNDLE_VERSION/$version/g" bundle/Info.plist > "$app/Contents/Info.plist"
    cd target/bundle && zip -ry "winker-{{ arch() }}-apple-darwin.zip" winker.app

[windows]
bundle: build
    #!/usr/bin/env bash
    set -euo pipefail
    stage=target/bundle/winker
    rm -rf target/bundle
    mkdir -p "$stage"
    cp target/release/app-desktop.exe "$stage/winker.exe"
    cd target/bundle && 7z a "winker-{{ arch() }}-pc-windows-msvc.zip" winker

ready: format check test lint doc
