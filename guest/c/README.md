# Building with C

## Usage

```bash
/opt/wasi-sdk/bin/clang critter.c critter_component_type.o component.c -o critter.wasm -mexec-model=reactor
wasm-tools component new critter.wasm -o critter.component.wasm
```

[WAMR C/C++ Doc](https://github.com/bytecodealliance/wasm-micro-runtime/blob/main/doc/build_wasm_app.md)
