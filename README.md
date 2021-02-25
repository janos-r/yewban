<p align="center">
    <img src="assets/logo.png" width="60%" />
    <a href="https://radim.xyz/yewban/"><h2 align="center">🚀️Running example on my web🚀️<h2></a>
</p>

# Yewban

A simple frontend in Rust🦀️ for a goban (go game board).

## Install

### install the build tools

```
cargo install trunk wasm-bindgen-cli
```

- Trunk is really amazing. At the time of writing it is recommended only in the non-stable version of the yew documentation. Trunk mentions that in the future, the `wasm-bindgen-cli` package will be installed as part of trunk.

### install the rust wasm toolchain

```
rustup target add wasm32-unknown-unknown
```

- Otherwise `trunk build` won't go through.

## Development

### Build

```
trunk build
```
- This creates the `/dist` folder. Under the hood it also runs `cargo build` that creates the `/target` folder. So you can still normally use cargo's check, build or clippy for checking code. 
- #### Static files:
  The folder `/static` and its contents are copied via trunk automatically to `/dist` thanks to this line in `index.html`:\
  "`<link data-trunk rel="copy-dir" href="static"/>`"

### Build, view and rebuild code on the fly

```
trunk serve
```

## Sharing the result

- To cut the total size, get a clean output folder (if previous builds were made). This deletes the `./dist` folder.

  ```
  trunk clean
  ```

- Make a new optimized build.

  ```
  trunk build --release
  ```
If you want to run the html directly, for example on a different system, just launching `index.html` from `./dist/` won't work! If you need to use the `index.html` directly (for example in a link on your web), you have to change the 3 addresses for the .css, .js and .wasm inside. Otherwise by default it is searching for them in root.

But locally the simplest way is to serve the folder:

- take the content of `./dist/` and serve it with any of these:
  - rust tool:

    ```
    miniserve ./dist --index index.html
    ```

  - npm (nodejs) tool:

    ```
    http-server ./dist
    ```

  - usually present by default on linux (run from inside the folder):
    ```
    python3 -m http.server
    ```
