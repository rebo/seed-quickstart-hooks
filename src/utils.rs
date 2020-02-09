use comp_state::{topo, use_state, CloneState};
use seed::{prelude::*, *};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys;
pub fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub fn get_html_element_by_id(id: &str) -> Option<web_sys::HtmlElement> {
    let maybe_elem = document()
        .get_element_by_id(id)
        .map(wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlElement>);

    if let Some(Ok(elem)) = maybe_elem {
        Some(elem)
    } else {
        None
    }
}

#[topo::nested]
pub fn after_render<F: Fn() -> () + 'static>(rerun: bool, func: F) {
    let already_triggered = use_state(|| false);
    if rerun {
        already_triggered.set(false);
    } else if already_triggered.get() {
        return;
    }
    already_triggered.set(true);
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    // let mut i = 0;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |_| {
        func();
        f.borrow_mut().take();
    }) as Box<dyn FnMut(f64)>));
    request_animation_frame(g.borrow().as_ref().unwrap());
}
