//use wasmer::{RuntimeError};
//use std::sync::{Arc, Mutex};
//
//fn print_str(input: Arc<Mutex<String>>) -> Result<Vec<wasmer::Value>, RuntimeError> {
//  println!("{:?}", input);
//  return Ok(Vec::new());
//}
//
//fn return_bool(input: bool) -> Result<Vec<wasmer::Value>, RuntimeError> {
//  return Ok(Vec::new());
//}

use std::panic;

use wasmer::{imports, Store, Value, Imports, Module, Instance, RuntimeError};
use wasmer_compiler_cranelift::Cranelift;


pub fn run_wasm(binary: String, params: &[Value]) -> Result<Box<[wasmer::Value]>, RuntimeError> {
  let compiler = Cranelift::new();
  let mut store = Store::new(compiler);

  let file = std::fs::read(&binary).unwrap();

  let module = Module::new(&store, file).unwrap();

  let imports = imports! {

  };

  let instance = Instance::new(&mut store, &module, &imports).unwrap();

  if instance.exports.contains("main") {
    let main_func = instance.exports.get_function("main").unwrap();
    return main_func.call(&mut store, params)
  } else {
    panic!("WASM module \"{}\" doesn't contain \"main\" function", binary)
  }
}
