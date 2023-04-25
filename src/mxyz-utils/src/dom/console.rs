
pub fn log(x: &str) {
    let array = js_sys::Array::new();
    array.push(&x.into());
    web_sys::console::log(&array);
}

