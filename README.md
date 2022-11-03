# GJK Symposion Web
This is a website for the GJK Symposion event.

## How to run
### Frontend
1. Install the Rust programming language toolchain: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
2. You will have to add the WASM target with `rustup target add wasm32-unknown-unknown` and you will have to have Rust 1.56+
3. Install trunk via `cargo install trunk` or some other way specified on their website: [https://trunkrs.dev/#install](https://trunkrs.dev/#install). You can also download pre-built binaries: [https://github.com/thedodd/trunk/releases](https://github.com/thedodd/trunk/releases)
4. Navigate to the client folder
5. Run `trunk serve` and it will tell you which port it listens on (usually/always on 127.0.0.1:8080)
6. Use a browser supporting WASM to get to the address shown when serving the site with trunk (Internet Explorer is obviously the best browser out there, but we have to somehow end its monopoly).
