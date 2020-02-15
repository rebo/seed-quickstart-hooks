#![feature(track_caller)]

use comp_state::{topo, use_istate, use_lstate, use_state, CloneState, StateAccess};
use seed::{prelude::*, *};

use comp_state_seed_extras::*;

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
        deps_example(),
        focus_example(),
        // other_examples(),
        "Clone Example:",
        div![my_button(), my_button(),],
        "None Clone:",
        div![my_button_non_clone(), my_button_non_clone(),],
        "Simplified:",
        div![my_ev_button2(), my_ev_button2(),],
        "Bind number to inputs:",
        numberbind(),
        "Use a function to dispatch",
        dispatch_test(),
        "React useEffect Clone",
        // after_example(),
        "simplified state accessor event handlers:",
        my_ev_input(),
        my_ev_button(),
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

#[topo::nested]
fn my_ev_button2() -> Node<Msg> {
    let count = use_state(|| 3);

    div![
        button!["-", count.mouse_ev(Ev::Click, |count, _| *count -= 1)],
        count.get().to_string(),
        button!["+", count.mouse_ev(Ev::Click, |count, _| *count += 1)],
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

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}

#[rustfmt::skip]

//

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

#[topo::nested]
fn todos() -> Node<Msg> {
    let todos = use_state(|| vec![use_istate(String::new)]);
    div![
        todos.get().iter().enumerate().map(|(idx, todo)| {
            vec![
                input![bind(At::Value, *todo)],
                button![
                    "X",
                    todos.mouse_ev(Ev::Click, move |todo, _| {
                        todo.remove(idx);
                    })
                ],
                br![],
            ]
        }),
        button![
            mouse_ev(Ev::Click, move |_| {
                todos.update(|t| t.push(use_lstate(String::new)));
                Msg::NoOp
            }),
            "Add"
        ]
    ]
}

#[topo::nested]
fn focus_example() -> Node<Msg> {
    let input_string = use_state(String::new);

    after_render_once(move || {
        if let Some(elem) = get_html_element_by_id(&input_string.identity()) {
            let _ = elem.focus();
        }
    });

    input![id!(input_string.identity())]
}

#[topo::nested]
fn deps_example() -> Node<Msg> {
    use std::cmp::Ordering;
    let input_a = use_istate(String::new);
    let input_b = use_istate(String::new);

    after_render_deps(&[input_a, input_b], move || {
        if let (Ok(a), Ok(b)) = (input_a.get().parse::<i32>(), input_b.get().parse::<i32>()) {
            let smallest = match a.cmp(&b) {
                Ordering::Less => "<li>A is the smallest</li>",
                Ordering::Greater => "<li>B is the smallest</li>",
                Ordering::Equal => "<li>Neither is the smallest</li>",
            };

            if let Some(elem) = get_html_element_by_id("list") {
                let _ = elem.insert_adjacent_html("beforeend", smallest);
            }
        }
    });

    div![
        "A:",
        input![bind(At::Value, input_a)],
        "B:",
        input![bind(At::Value, input_b)],
        ul![id!("list"), "Smallest Log:"],
    ]
}

trait StateAccessAsString {
    fn identity(self) -> String;
}

impl<T> StateAccessAsString for StateAccess<T> {
    fn identity(self) -> String {
        format!("{}", self)
    }
}

// #[topo::nested]
// fn after_example() -> Node<Msg> {
//     after_render(false, || {
//         document().set_title("The Page has been rendered");
//         if let Some(my_div) = get_html_element_by_id("my_div") {
//             my_div.set_inner_text("This div has been rendered");
//         }
//     });
//     div![id!("my_div"), "Not Rendered"]
// }

// #[topo::nested]
// fn other_examples() -> Node<Msg> {
//     let recalculate_width = use_state(|| false);

//     after_render(recalculate_width.get(), move || {
//         if let Some(my_div) = get_html_element_by_id("my_div2") {
//             if let Ok(Some(style)) = window().get_computed_style(&my_div) {
//                 my_div.set_inner_text(&format!(
//                     "width of this div = {}",
//                     style.get_property_value("width").unwrap()
//                 ))
//             }
//         }
//         recalculate_width.set(false);
//     });

//     div![
//         div![id!("my_div2")],
//         button![
//             "Calculate Width of div",
//             recalculate_width.mouse_ev(Ev::Click, |recalc, _| *recalc = !*recalc)
//         ],
//     ]
// }
