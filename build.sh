#!/bin/bash

# Build the project
echo "Building the project..."

export COMPONENT_ADAPTER=$(pwd)/wasi_preview1_component_adapter.wasm

# Build guest components
pushd guest || exit

pushd go/gopher || exit
go generate
tinygo build -target=wasi gopher.go
wasm-tools print gopher.wasm -o gopher.wat

# adapt to component
# this step is needed to embed wit exports into the wasm file
wasm-tools component embed --world critter ./gopher.wit gopher.wasm -o gopher.embed.wasm
wasm-tools component new -o gopher.component.wasm --adapt wasi_snapshot_preview1="$COMPONENT_ADAPTER" gopher.embed.wasm

# validate
wasm-tools validate gopher.component.wasm --features component-model 
wasm-tools print gopher.component.wasm -o gopher.component.wat
wasm-tools component wit gopher.component.wasm

popd || exit # go/gopher

pushd rust/crates || exit

pushd rustacean || exit

cargo component build --release

popd || exit # rustacean

pushd app || exit

cargo component build --release

popd || exit # app

popd || exit # rust/crates

pushd js/dino || exit

npx jco componentize dino.js --wit dino.wit -o dino.component.wasm

popd || exit # js/dino

wasm-tools compose --verbose --output linked.wasm rust/crates/app/target/wasm32-wasi/release/app.wasm --config config.yaml
wasm-tools component wit linked.wasm

popd || exit # guest
