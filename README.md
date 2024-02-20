### Enable one mod
Enable one of the `mod` lines in `src/lib.rs`.


### Compile the crate to wasm:
```bash
cargo build --target wasm32-unknown-unknown --release
```


### Check the binary output size.

```bash
stat -c %s ./target/wasm32-unknown-unknown/release/file_bloat.wasm
```