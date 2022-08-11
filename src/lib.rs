extern crate isaribi;
extern crate js_sys;
extern crate kagura;
extern crate nusa;
extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;
extern crate web_sys;

mod components;
mod model;

use components::app::{self, App};
use nusa::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    wasm_bindgen_futures::spawn_local(async {
        kagura::Runtime::run(nusa::dom_node::BasicDomNode::new(entry_point(), |this| {
            vec![App::empty(this, None, app::Props {}, Sub::none())]
        }))
        .await;
    });
}

fn entry_point() -> web_sys::Node {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("app")
        .unwrap()
        .into()
}
