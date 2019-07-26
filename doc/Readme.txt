rustup update stable
cargo update stable
cargo install cargo-update

cargo new hello-world
cargo new hello-world --bin
cargo new hello-world --lib --vcs none
cargo build
cargo build --release
cargo run --release
cargo update
cargo update -p rand   # updates just “rand”
cargo test
cargo test foo

cargo -V
rustc --version