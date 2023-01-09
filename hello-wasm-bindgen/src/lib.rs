/* hello world Rust webassembly*/
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

//export the function to javascript
#[wasm_bindgen]
pub fn marco_polo(s: &str) {
    //if the string is "Marco" return "Polo"
    if s == "Marco" {
        alert("Polo");
    }
    //if the string is anything else return "Not Marco"
    else {
        alert("Not Marco");
    }
}
