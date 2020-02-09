#![feature(track_caller)]

use comp_state::{topo, use_state, CloneState};
use seed::{prelude::*, *};

mod ev_handlers_for_state_access;
use ev_handlers_for_state_access::StateAccessEventHandlers;

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
        my_button(),
        my_button(),
        my_button(),
        my_button(),
        my_button(),
        my_ev_button(),
        my_ev_input(),
    ]
}

#[topo::nested]
fn my_button() -> Node<Msg> {
    let count_access = use_state(|| 0);
    div![button![
        format!("Clicked {} times", count_access.get()),
        mouse_ev(Ev::Click, move |_| {
            count_access.update(|count| *count += 1);
            Msg::NoOp
        })
    ]]
}

#[topo::nested]
fn my_ev_button() -> Node<Msg> {
    let count_access = use_state(|| 0);
    div![button![
        format!("Clicked {} times", count_access.get()),
        count_access.mouse_ev(Ev::Click, |count, _| *count += 1),
    ]]
}
fn my_ev_input() -> Node<Msg> {
    let input_access = use_state(|| "".to_string());

    div![
        input![
            attrs![ At::Value => input_access.get()],
            input_access.input_ev(Ev::Input, |input, text| *input = text),
        ],
        format!("Text inputted: {}", input_access.get())
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
