use wasmer::{imports, Store, Value, Module, Instance};
use wasmer_compiler_cranelift::Cranelift;

pub fn run_wasm(binary: String, params: &[Value]) -> Result<Box<[wasmer::Value]>, &'static str> {
  let compiler = Cranelift::new();
  let mut store = Store::new(compiler);
  let file = std::fs::read(&binary).unwrap();
  let module = Module::new(&store, file).unwrap();
  let imports = imports! {

  };

  let instance = Instance::new(&mut store, &module, &imports).unwrap();

  if instance.exports.contains("run") {
    let main_func = instance.exports.get_function("run");
    match main_func {
      Ok(function) => {
        let y = function.call(&mut store, params).unwrap();
        return Ok(y);
      },
      Err(_error) => {
        return Err("Wat.")
      }
    }
    } else {
      return Err("An error occured: WASM binary does not contain 'run'")
  }
}
