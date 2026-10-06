release platform="default":
    @just release-{{platform}}

release-default:
    cargo build --release

release-linux:
    cargo build --target x86_64-unknown-linux-gnu --release

release-windows:
    cargo build --target x86_64-pc-windows-gnu --release

run-android:
    cargo apk2 run --manifest-path android/Cargo.toml

clean:
    cargo clean
    cargo clean --manifest-path android/Cargo.toml
