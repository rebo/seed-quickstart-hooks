use comp_state::{topo, use_state, CloneState, StateAccess};
use seed::{prelude::*, *};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;

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
pub fn after_render_deps<F: Fn() -> () + 'static>(
    dependencies: &[impl StateChangedTrait],
    func: F,
) {
    if dependencies
        .iter()
        .all(|dependency| !dependency.state_changed())
    {
        return;
    }

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    // let mut i = 0;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |_| {
        func();
        f.borrow_mut().take();
    }) as Box<dyn FnMut(f64)>));
    request_animation_frame(g.borrow().as_ref().unwrap());
}

#[topo::nested]
pub fn after_render_once<F: Fn() -> () + 'static>(func: F) {
    let already_triggered = use_state(|| false);
    if already_triggered.get() {
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

#[topo::nested]
pub fn after_render_always<F: Fn() -> () + 'static>(func: F) {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    // let mut i = 0;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |_| {
        func();
        f.borrow_mut().take();
    }) as Box<dyn FnMut(f64)>));
    request_animation_frame(g.borrow().as_ref().unwrap());
}

pub trait StateChangedTrait {
    fn state_changed(&self) -> bool;
}

impl<T> StateChangedTrait for StateAccess<T>
where
    T: Clone + PartialEq + 'static,
{
    fn state_changed(&self) -> bool {
        let old_state: StateAccess<Option<T>> = topo::call_in_slot(self.id, || use_state(|| None));

        if Some(self.get()) == old_state.get() {
            false
        } else {
            old_state.set(Some(self.get()));
            true
        }
    }
}

// #[topo::nested]
// fn focus_example() -> Node<Msg> {
//     let input_string = use_state(String::new);

//     after_render_once(move || {
//         if let Some(elem) = get_html_element_by_id(&input_string.identity()) {
//             let _ = elem.focus();
//         }
//     });

//     input![id!(input_string.identity())]
// }

// #[topo::nested]
// fn deps_example() -> Node<Msg> {
//     use std::cmp::Ordering;
//     let input_a = use_istate(String::new);
//     let input_b = use_istate(String::new);

//     after_render_deps(&[input_a, input_b], move || {
//         if let (Ok(a), Ok(b)) = (input_a.get().parse::<i32>(), input_b.get().parse::<i32>()) {
//             let smallest = match a.cmp(&b) {
//                 Ordering::Less => "<li>A is the smallest</li>",
//                 Ordering::Greater => "<li>B is the smallest</li>",
//                 Ordering::Equal => "<li>Neither is the smallest</li>",
//             };

//             if let Some(elem) = get_html_element_by_id("list") {
//                 let _ = elem.insert_adjacent_html("beforeend", smallest);
//             }
//         }
//     });

//     div![
//         "A:",
//         input![bind(At::Value, input_a)],
//         "B:",
//         input![bind(At::Value, input_b)],
//         ul![id!("list"), "Smallest Log:"],
//     ]
// }

// trait StateAccessAsString {
//     fn identity(self) -> String;
// }

// impl<T> StateAccessAsString for StateAccess<T> {
//     fn identity(self) -> String {
//         format!("{}", self)
//     }
// }
