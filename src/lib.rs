use wasm_bindgen::prelude::*;
use threejs_sys::core::Object3D;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_str(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_object(object: &Object3D);
}


// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let myobj = Object3D::new();   
    myobj.translate_x(0.2);
    log_object(&myobj);
    Ok(())
}
