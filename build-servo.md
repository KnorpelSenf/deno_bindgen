# Notes on how to build libsimpleservo from servo fork with deno_bindgen

Use this branch to create a custom version with different cargo build parameter
```sh
cargo build --release
```
Alias this for handy access
```sh
alias servo_deno_bindgen="$(pwd)/target/release/deno_bindgen"
```

The info how to build simple servo is in servo/ports/libsimpleservo/README.md
```sh
cd servo/ports/libsimpleservo/capi
export CARGO_TARGET_DIR=./target
<path to your deno_bindgen release>/deno_bindgen --release
```
