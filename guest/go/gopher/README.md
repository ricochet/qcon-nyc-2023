# Gopher says hey

```bash
# Build a WASI reactor module
go generate
tinygo build -target=wasi gopher.go
wasm-tools print gopher.wasm -o gopher.wat

# Adapt to a Component and WASI preview 2
# this step is needed to embed wit exports into the wasm file
COMPONENT_ADAPTER=../../../wasi_preview1_component_adapter.wasm
wasm-tools component embed --world critter ./gopher.wit gopher.wasm -o gopher.embed.wasm
wasm-tools component new -o gopher.component.wasm --adapt wasi_snapshot_preview1="$COMPONENT_ADAPTER" gopher.embed.wasm

# validate
wasm-tools validate gopher.component.wasm --features component-model 
wasm-tools print gopher.component.wasm -o gopher.component.wat
wasm-tools component wit gopher.component.wasm
```
