# GJK Symposion Web
This is a website for the GJK Symposion event.

With ❤️ for the 🌍 written in Rust and C++, ones of the most [energy efficient](https://haslab.github.io/SAFER/scp21.pdf) programming languages.

Warning: The current API implementation crashed on a segfault and we weren't able to identify the cause as of now. However, it didn't crash since then, so it should be safe to use, especially if you use the `repeat_run.sh` script to launch it, since it will restart it if it crashes.

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
1. download [asio](https://think-async.com/Asio/Download.html), copy(/move) contents of asio's include folder into `api_server/include/`
2. run the build script - `api_server/sh_build.sh` - from it's directory (it uses g++)
3. wait untill it finishes compiling :)
4. (Optional) change the config file - `api_server/bin/api_server.cfg`; the certificate chain and private key are not used (because it uses http by default)
5. run the `api_server/bin/repeat_run.sh` script from it's directory (you can provide log filename as argument)
