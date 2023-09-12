use whistle_common::{Tip, Span};

use crate::{Compiler, Function};


pub fn compile_tip_wasm_bytes(_compiler: &mut Compiler, function: &mut Function, tip: Tip, _span: Span) {
    let raw_data = tip.value.split(",").map(|s| s.trim()).collect::<Vec<&str>>();
    let data = raw_data.iter().map(|s| s.parse::<u8>().unwrap()).collect::<Vec<u8>>();
    function.raw(data);
}