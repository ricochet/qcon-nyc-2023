use anyhow::Result;

use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};
use wasmtime_wasi::preview2::{self, Table, WasiCtx, WasiCtxBuilder, WasiView};
// use wasmtime_wasi_http::WasiHttp;

use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::body::Frame;
use hyper::server::conn::http1;
use hyper::{body::Bytes, service::service_fn, Method, Request, Response, StatusCode};
use std::{net::SocketAddr};
use tokio::net::TcpListener;

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

    let component = r#"
    (component
      (type $t (func (param "a" u32) (result u32)))
      (component $a
        (core module $m
          (func (export "add-five") (param i32) (result i32)
            local.get 0
            i32.const 5
            i32.add)
        )
        (core instance $m (instantiate $m))
        (func (export "add-five") (type $t) (canon lift (core func $m "add-five")))
      )
      (component $b
        (import "interface-v1" (instance $i
          (export "add-five" (func (type $t)))))
        (core module $m
          (func $add-five (import "interface-0.1.0" "add-five") (param i32) (result i32))
          (func) ;; causes index out of bounds
          (func (export "run") (result i32) i32.const 0 call $add-five)
        )
        (core func $add-five (canon lower (func $i "add-five")))
        (core instance $i (instantiate 0
          (with "interface-0.1.0" (instance
            (export "add-five" (func $add-five))
          ))
        ))
        (func (result u32) (canon lift (core func $i "run")))
        (export "run" (func 1))
      )
      (instance $a (instantiate $a))
      (instance $b (instantiate $b (with "interface-v1" (instance $a))))
      (export "run" (func $b "run"))
    )
    "#;

    let mut config = Config::new();
    config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
    config.wasm_component_model(true);
    config.async_support(true);

    // An engine stores and configures global compilation settings like
    // optimization level, enabled wasm features, etc.
    let engine = Engine::new(&config).unwrap();
    let component = Component::from_file(&engine, "/Users/bhayes/repos/ricochet/qcon-nyc-2023/guest/linked.wasm")?;
    // let component = Component::new(&engine, component)?;

    // A `Store` is what will own instances, functions, globals, etc. All wasm
    // items are stored within a `Store`, and it's what we'll always be using to
    // interact with the wasm world. Custom data can be stored in stores but for
    // now we just use `()`.
    let mut table = Table::new();
    let wasi = WasiCtxBuilder::new().build(&mut table)?;

    let mut store: Store<ReactorCtx> = Store::new(&engine, ReactorCtx { table, wasi });
    let mut linker: Linker<ReactorCtx> = Linker::new(&engine);

    preview2::wasi::filesystem::filesystem::add_to_linker(&mut linker, |x| x)?;
    preview2::wasi::io::streams::add_to_linker(&mut linker, |x| x)?;
    preview2::wasi::cli_base::environment::add_to_linker(&mut linker, |x| x)?;
    preview2::wasi::cli_base::preopens::add_to_linker(&mut linker, |x| x)?;
    preview2::wasi::cli_base::exit::add_to_linker(&mut linker, |x| x)?;
    preview2::wasi::cli_base::stdin::add_to_linker(&mut linker, |x| x)?;
    preview2::wasi::cli_base::stdout::add_to_linker(&mut linker, |x| x)?;
    preview2::wasi::cli_base::stderr::add_to_linker(&mut linker, |x| x)?;

    let instance = linker.instantiate_async(&mut store, &component).await?;



    // let run = instance.get_typed_func::<(), (u32,)>(&mut store, "run")?;
    // let ret = run.call_async(&mut store, ()).await?;
    // println!("result of run:\n{}", ret.0);



    let run = instance.get_typed_func::<(), (String,)>(&mut store, "run")?;
    let ret = run.call_async(&mut store, ()).await?;
    println!("result of run:\n{}", ret.0);












    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(stream, service_fn(echo))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

/// This is our service handler. It receives a Request, routes on its
/// path, and returns a Future of a Response.
async fn echo(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => Ok(Response::new(full(
            "Try POSTing data to /echo such as: `curl localhost:3000/echo -XPOST -d \"hello world\"`",
        ))),

        // Simply echo the body back to the client.
        (&Method::POST, "/echo") => Ok(Response::new(req.into_body().boxed())),

        // Convert to uppercase before sending back to client using a stream.
        (&Method::POST, "/echo/uppercase") => {
            let frame_stream = req.into_body().map_frame(|frame| {
                let frame = if let Ok(data) = frame.into_data() {
                    data.iter()
                        .map(|byte| byte.to_ascii_uppercase())
                        .collect::<Bytes>()
                } else {
                    Bytes::new()
                };

                Frame::data(frame)
            });
            Ok(Response::new(frame_stream.boxed()))
        }

        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::new(empty());
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}
