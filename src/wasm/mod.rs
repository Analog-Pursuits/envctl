mod imports;

use wasmtime::*;
use wasmtime_wasi::{WasiCtx, sync::WasiCtxBuilder};

use anyhow::{Result};

struct MyState {
  wasi: WasiCtx,
}

pub fn run_wasm(binary: String) -> Result<()> {
    // Enable epoch interruption code via `Config` which means that code will
    // get interrupted when `Engine::increment_epoch` happens.
    let engine = Engine::new(Config::new().epoch_interruption(true)).unwrap();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |state: &mut MyState| &mut state.wasi)?;

    // Create a WASI context and put it in a Store; all instances in the store
    // share this context. `WasiCtxBuilder` provides a number of ways to
    // configure what the target program will have access to.
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args().unwrap()
        .build();
    let mut store = Store::new(&engine, MyState { wasi: wasi });
    store.set_epoch_deadline(1);

    // Compile and instantiate a small example with an infinite loop.
    let module = Module::from_file(&engine, binary)?;
    let instance = linker.instantiate(&mut store, &module);

  match instance {
    Ok(_x) => {
      let main_func = linker
      .get_default(&mut store, "");
      match main_func {
        Ok(x) => {
          let func_type = x.typed::<(), (), _>(&store);
          match func_type {
            Ok(x) => {
              let called_func = x.call(store, ());
              match called_func {
                Ok(x) => {
                  return Ok(x);
                },
                Err(err) => {
                  eprintln!("Something went wrong in the called function: {:?}", err)
                }
              }
              return Ok(());
            },
            Err(err) => {
                eprintln!("An error occured: {}", err);
              return Err(err);
            }
          }
        },
        Err(err) => {
          return Err(err);
        }
      }
    
    }
  
    Err(err) => {
      eprintln!("An Error occured in the root match: {}", err);
      return Err(err);
      //return Err("There was an error instantiating the WASM instance.".to_string())
    }
  }

}
