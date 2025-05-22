VERSION 0.8

deps-system:
    FROM fedora:42
    RUN dnf install -y gtk3-devel clang cmake git which ninja rustup

    RUN rustup-init -y
    ENV PATH="$HOME/.cargo/bin:$PATH"

    WORKDIR /flutter_sdk
    COPY flutter_linux_*-stable.tar.xz /flutter_sdk.tar.xz
    RUN tar -xf /flutter_sdk.tar.xz -C .
    RUN chown -R root:root /flutter_sdk
    RUN rm /flutter_sdk.tar.xz
    ENV PATH="/flutter_sdk/flutter/bin:$PATH"

    WORKDIR /project

rust-src:
    FROM ./invoy/rust+copy
    SAVE ARTIFACT /src /src

deps-rust:
    FROM +deps-system
    COPY +rust-src/src .
    RUN cargo fetch
    SAVE ARTIFACT . /project
    SAVE ARTIFACT /root/.cargo /deps_cache

deps-flutter:
    FROM +deps-system
    COPY invoy/pubspec.yaml .
    COPY invoy/pubspec.lock .
    COPY invoy/rust_builder rust_builder
    RUN flutter pub get
    COPY invoy .
    COPY build build
    SAVE ARTIFACT . /project

build-linux:
    FROM +deps-flutter

    COPY +deps-rust/project rust
    COPY +deps-rust/deps_cache /root/.cargo

    RUN flutter build linux --release

    SAVE ARTIFACT build/* AS LOCAL build/