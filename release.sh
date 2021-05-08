# Building binaries
cross build --release --target=x86_64-unknown-linux-gnu
cargo build --release --target=x86_64-apple-darwin

# Moving binaries
mkdir -p dist/darwin_amd64 && cp target/x86_64-apple-darwin/release/resin dist/darwin_amd64
mkdir -p dist/linux_amd64 && cp target/x86_64-unknown-linux-gnu/release/resin dist/linux_amd64
