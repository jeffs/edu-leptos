#!/usr/bin/env -S zsh -euo pipefail

cargo install trunk
cargo init leptos-tutorial

cd leptos-tutorial

>README.md <<EOF 
[Getting Started](https://leptos-rs.github.io/leptos/02_getting_started.html)
EOF

>rust-toolchain.toml <<EOF
[toolchain]
channel = "nightly"
EOF

cargo add leptos --features=csr,nightly
rustup target add wasm32-unknown-unknown

>index.html <<EOF
<!DOCTYPE html>
<html>
  <head></head>
  <body></body>
</html>
EOF

>src/main.rs <<EOF
use leptos::*;

fn main() {
    mount_to_body(|| view! { <p>"Hello, world!"</p> });
}
EOF

# trunk serve --open
