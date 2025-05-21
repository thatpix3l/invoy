VERSION 0.8

install-system-deps:
    FROM fedora:42
    RUN dnf install -y gtk3-devel clang cmake git which ninja rustup
    RUN rustup-init -y
    RUN . "$HOME/.cargo/env"

install-flutter:
    FROM +install-system-deps
    COPY flutter_linux_*-stable.tar.xz /flutter_sdk.tar.xz
    WORKDIR /flutter_sdk
    RUN tar -xf /flutter_sdk.tar.xz -C .
    RUN chown -R root:root /flutter_sdk
    ENV PATH="/flutter_sdk/flutter/bin:$PATH"

install-project-deps:
    FROM +install-flutter
    WORKDIR /project
    COPY invoy .
    RUN flutter pub get

build-linux:
    FROM +install-project-deps
    RUN flutter build linux --release
    SAVE ARTIFACT build/* AS LOCAL build/