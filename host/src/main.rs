use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};
use wasmtime_wasi::preview2::{self, Table, WasiCtx, WasiCtxBuilder, WasiView};

struct ReactorCtx {
    table: Table,
    wasi: WasiCtx,
}

impl WasiView for ReactorCtx {
    fn table(&self) -> &Table {
        &self.table
    }
    fn table_mut(&mut self) -> &mut Table {
        &mut self.table
    }
    fn ctx(&self) -> &WasiCtx {
        &self.wasi
    }
    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}


#[tokio::main]
async fn main() -> wasmtime::Result<()> {
    println!("Hello, from the Rust Host!");

    let mut config = Config::new();
    config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
    config.wasm_component_model(true);
    config.async_support(true);

    // An engine stores and configures global compilation settings like
    // optimization level, enabled wasm features, etc.
    let engine = Engine::new(&config).unwrap();
    let component = Component::from_file(&engine, "../guest/linked.wasm")?;
    // let component = Component::from_file(&engine, "../guest/wasmbuilder.wasm")?;


    // A `Store` is what will own instances, functions, globals, etc. All wasm
    // items are stored within a `Store`, and it's what we'll always be using to
    // interact with the wasm world. Custom data can be stored in stores but for
    // now we just use `()`.
    let mut table = Table::new();
    let wasi = WasiCtxBuilder::new().build(&mut table)?;

    let mut store: Store<ReactorCtx> = Store::new(&engine, ReactorCtx { table, wasi });
    let mut linker: Linker<ReactorCtx> = Linker::new(&engine);

    // preview2::wasi::filesystem::filesystem::add_to_linker(&mut linker, |x| x)?;
    // preview2::wasi::io::streams::add_to_linker(&mut linker, |x| x)?;
    // preview2::wasi::cli_base::environment::add_to_linker(&mut linker, |x| x)?;
    // preview2::wasi::cli_base::preopens::add_to_linker(&mut linker, |x| x)?;
    // preview2::wasi::cli_base::exit::add_to_linker(&mut linker, |x| x)?;
    // preview2::wasi::cli_base::stdin::add_to_linker(&mut linker, |x| x)?;
    // preview2::wasi::cli_base::stdout::add_to_linker(&mut linker, |x| x)?;
    // preview2::wasi::cli_base::stderr::add_to_linker(&mut linker, |x| x)?;

    let instance = linker.instantiate_async(&mut store, &component).await?;
    let run = instance.get_typed_func::<(), (String,)>(&mut store, "run")?;

    let ret = run.call_async(&mut store, ()).await?;
    println!("result of run:\n{}", ret.0);

    Ok(())

}
