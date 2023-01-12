use ::wasmtime::*;
use anyhow::{Result};
use wasmtime_wasi::{sync::WasiCtxBuilder};

pub fn run_wasi(binary_loc: String) -> Result<()> {
  // Define the WASI functions globally on the `Config`.
  let engine = Engine::default();
  let mut linker = Linker::new(&engine);
  wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

  // Create a WASI context and put it in a Store; all instances in the store
  // share this context. `WasiCtxBuilder` provides a number of ways to
  // configure what the target program will have access to.
  let wasi = WasiCtxBuilder::new()
      .inherit_stdio()
      .inherit_args()?
      .build();
  let mut store = Store::new(&engine, wasi);

  // Instantiate our module with the imports we've created, and run it.
  let module = Module::from_file(&engine, binary_loc);
  match module {
    Ok(module) => {
      linker.module(&mut store, "", &module)?;
      linker
        .get_default(&mut store, "")?
        .typed::<(), (), _>(&store)?
        .call(&mut store, ())?;
      Ok(())
    }
  Err(err) => {
    eprintln!("Wasm module instantiation failed: {}", err);
    err.chain().skip(1).for_each(|cause| eprintln!("because: {}", cause));
    return Err(err);
  }
  }
}