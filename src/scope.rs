use wasm_bindgen::JsCast;
use web_sys;

pub fn get_scope() -> web_sys::WorkerGlobalScope {
    js_sys::global()
        .dyn_into::<web_sys::WorkerGlobalScope>()
        .unwrap()
}
