# GJK Symposion Web
This is a website for the GJK Symposion event.

## How to run
### Frontend
1. Install the Rust programming language toolchain: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
2. You will have to add the WASM target with `rustup target add wasm32-unknown-unknown` and you will have to have Rust 1.56+
3. Install trunk via `cargo install trunk` or some other way specified on their website: [https://trunkrs.dev/#install](https://trunkrs.dev/#install). You can also download pre-built binaries: [https://github.com/thedodd/trunk/releases](https://github.com/thedodd/trunk/releases)
4. Navigate to the client folder
5. (Optional) Change the config.json file to use a different API link
6. Run `trunk serve` and it will tell you which port it listens on (usually/always on 127.0.0.1:8080)
7. Use a browser supporting WASM to get to the address shown when serving the site with trunk (Internet Explorer is obviously the best browser out there, but we have to somehow end its monopoly).

### API Server
#### Linux
1. run the build script - `api_server/sh_build.sh` - from it's directory (it uses g++)
2. wait untill it finishes compiling :)
3. (Optional) change the config file - `api_server/bin/api_server.cfg`; the certificate chain and private key are not used (because it uses http by default)
4. run the `api_server/bin/api_server` executable from it's directory
