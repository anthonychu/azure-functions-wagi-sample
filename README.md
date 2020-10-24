# Azure Functions WAGI custom handler sample

Run WebAssembly with [WAGI](https://github.com/deislabs/wagi) on Azure Functions.

```bash
rustup target add wasm32-wasi
cargo build --target wasm32-wasi --release
func start
```

wagi binary built from [WAGI](https://github.com/deislabs/wagi).