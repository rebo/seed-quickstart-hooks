#![feature(track_caller)]

use comp_state::{topo, use_istate, use_lstate, use_state, CloneState, StateAccess};
use seed::{prelude::*, *};
use seed_bind::*;

mod seed_bind;

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::JsCast;

#[derive(Default)]
struct Model {}

enum Msg {
    NoOp,
}

fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

impl Default for Msg {
    fn default() -> Msg {
        Msg::NoOp
    }
}

fn update(msg: Msg, _model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::NoOp => (),
    }
}

fn view(_model: &Model) -> impl View<Msg> {
    root_view()
}

#[topo::nested]
fn root_view() -> Node<Msg> {
    div![
        "Clone Example:",
        div![
            my_button(),
            my_button(),
            my_button(),
            my_button(),
            my_button(),
        ],
        "None Clone:",
        div![
            my_button_non_clone(),
            my_button_non_clone(),
            my_button_non_clone(),
            my_button_non_clone(),
            my_button_non_clone(),
        ],
        numberbind(),
        dispatch_test(),
        todos(),
        after_example(),
    ]
}

fn get_html_element_by_id(id: &str) -> Option<web_sys::HtmlElement> {
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
fn after_example() -> Node<Msg> {
    after_render(false, || {
        document().set_title("The Page has been rendered");
        if let Some(my_div) = get_html_element_by_id("my_div") {
            my_div.set_inner_text("This div has been rendered");
        }
    });
    div![id!("my_div"), "Not Rendered"]
}

#[topo::nested]
fn my_button() -> Node<Msg> {
    let count = use_state(|| 3);

    div![
        button![
            "-",
            mouse_ev(Ev::Click, move |_| {
                count.update(|v| *v -= 1);
                Msg::NoOp
            }),
        ],
        count.get().to_string(),
        button![
            "+",
            mouse_ev(Ev::Click, move |_| {
                count.update(|v| *v += 1);
                Msg::NoOp
            }),
        ],
    ]
}

#[derive(Default)]
struct NonCloneI32(i32);

#[topo::nested]
fn my_button_non_clone() -> Node<Msg> {
    let count = use_state(NonCloneI32::default);

    div![
        button![
            "-",
            mouse_ev(Ev::Click, move |_| {
                count.update(|item| item.0 -= 1);
                Msg::NoOp
            }),
        ],
        count.get_with(|item| item.0.to_string()),
        button![
            "+",
            mouse_ev(Ev::Click, move |_| {
                count.update(|item| item.0 += 1);
                Msg::NoOp
            }),
        ]
    ]
}

//
// Effective clone of Reacts useReducer.
// Locally adjust state depending on a Message.
//
enum ComponentMsg {
    Increment,
    Decrement,
}

fn dispatch(state: StateAccess<i32>, msg: ComponentMsg) {
    match msg {
        ComponentMsg::Increment => state.update(|v| *v += 1),
        ComponentMsg::Decrement => state.update(|v| *v -= 1),
    }
}

#[topo::nested]
fn dispatch_test() -> Node<Msg> {
    let val = use_state(|| 0);
    div![
        button![
            "-",
            mouse_ev(Ev::Click, move |_| {
                dispatch(val, { ComponentMsg::Decrement });
                Msg::NoOp
            })
        ],
        format!("{}", val.get()),
        button![
            "+",
            mouse_ev(Ev::Click, move |_| {
                dispatch(val, { ComponentMsg::Increment });
                Msg::NoOp
            })
        ]
    ]
}

#[topo::nested]
fn numberbind() -> Node<Msg> {
    let a = use_istate(|| 0);
    let b = use_istate(|| 0);

    div![
        input![attrs![At::Type=>"number"], bind(At::Value, a)],
        input![attrs![At::Type=>"number"], bind(At::Value, b)],
        p![format!("{} + {} = {}", a.get(), b.get(), a.get() + b.get())]
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}

#[rustfmt::skip]
#[topo::nested]
fn todos() -> Node<Msg> {
    let todos = use_state(|| vec![use_istate(String::new)]);  
    div![
        todos.get().iter().enumerate().map(|(idx, todo)| {
            vec![
                input![bind(At::Value, *todo)],
                button![ "X", mouse_ev(Ev::Click, move |_| 
                    { todos.update(|t| {t.remove(idx);}); Msg::NoOp }) ],
                br![],]
        }),
        button![
            mouse_ev(Ev::Click, move |_| {
                todos.update(|t| t.push(use_lstate(String::new))); Msg::NoOp }),
            "Add"]
    ]
}
//

#[topo::nested]
fn after_render<F: Fn() -> () + 'static>(rerun: bool, func: F) {
    let already_triggered = use_state(|| false);
    if rerun {
        already_triggered.set(false);
    }
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
