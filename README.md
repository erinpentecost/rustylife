# Rustlife

This is a learning experiment for Rust, Nodejs, and WebAssembly. It's based on this [excellent book](https://rustwasm.github.io/docs/book/). The game runs in a web worker that writes to an [OffscreenCanvas](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas). Because of that, it won't work on Firefox. Probably the most interesting code here is how I handle input buffering, since the game's frames per second are so terrible. This isn't production ready code by any means.


## Folder Organization

* `.\node_modules` contains packages installed by NPM.
* `.\pkg` is the built WASM file (and helper js and ts files) that is generated by the Rust code in `.\src`. These files are built via a plugin defined in `webpack.config.js` that calls the `wasm-pack` tool. Files in this directory are made available in the website output by `webpack.config.js`.
* `.\src` contains all the source files for the project. Javascript, HTML, and Rust code are all intermixed in this directory. `webpack.config.js` sorts it all out.
* `.\Cargo.toml` defines dependencies and other build configuration information for the Rust code.
* `.\package.json` definses dependencies and other information for node\webpack\etc. It also contains the build script definitions. Call them like `npm run start` or `npm run lint`.
* `.\src\games\demo.rs` writes an image embedded into the binary onto the canvas.
* `.\src\games\life.rs` contains an interactive, wrapped-space version of Life.

## How to Build

1. Install Rust and NPM.
2. `npm run start`
