#![feature(track_caller)]

use comp_state::{topo, use_istate, use_lstate, use_state, CloneState, StateAccess};
use seed::{prelude::*, *};
use seed_bind::*;

mod seed_bind;

#[derive(Default)]
struct Model {}

enum Msg {
    NoOp,
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
    ]
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
