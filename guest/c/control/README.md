# Building with C

## Usage

```bash
wit-bindgen c ./wit
/opt/wasi-sdk/bin/clang control.c power_component_type.o power.c -o power.wasm -mexec-model=reactor
wasm-tools component new power.wasm -o power.component.wasm
```

[WAMR C/C++ Doc](https://github.com/bytecodealliance/wasm-micro-runtime/blob/main/doc/build_wasm_app.md)
