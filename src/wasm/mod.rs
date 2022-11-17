mod imports;

use wasmtime::*;
use wasmtime_wasi::{WasiCtx, sync::WasiCtxBuilder};
use anyhow::{Result, Context};

struct MyState {
  wasi: WasiCtx,
}

pub fn run_wasm(binary: String, name: Option<&str>) -> Result<()> {
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
          let ty = x.ty(&store);
          let values = Vec::new();
          //let mut results = vec![Val::null(); ty.results().len()];
          let mut results = Vec::new();
          let function = x.call(store, &values, &mut results).with_context(|| {
            if let Some(name) = name {
                format!("failed to invoke `{}`", name)
            } else {
                format!("failed to invoke command default")
            }
          });
          if !&results.is_empty() {
            eprintln!(
                "warning: using `--invoke` with a function that returns values \
                 is experimental and may break in the future"
            );
        }
          match function {
            Ok(x) => {
              println!("Function returned successfully.");
              println!("{:?}", results);
              for result in results {
                println!("Internal to value loop");
                match result {
                    Val::I32(i) => println!("{}", i),
                    Val::I64(i) => println!("{}", i),
                    Val::F32(f) => println!("{}", f32::from_bits(f)),
                    Val::F64(f) => println!("{}", f64::from_bits(f)),
                    Val::ExternRef(_) => println!("<externref>"),
                    Val::FuncRef(_) => println!("<funcref>"),
                    Val::V128(i) => println!("{}", i),
                }
              }
            },
            Err(err) => {
              println!("Err: {}", err)
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
      eprintln!("An Error occured in the root match: {}", err);
      return Err(err);
    }
  }
}
