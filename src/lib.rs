#![feature(track_caller)]

use comp_state::{
    do_once, topo, use_istate, use_lstate, use_state, ChangedState, CloneState, StateAccess,
};
use ev_handlers::StateAccessEventHandlers;
use seed::{prelude::*, *};
use seed_bind::*;
use utils::*;

mod ev_handlers;
mod seed_bind;
mod utils;

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
        parent_comp_example(),
        if_example(),
        focus_example(),
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
    let input = use_state(ElRef::default);

    do_once(|| {
        after_render(move |_| {
            let input_elem: web_sys::HtmlElement = input.get().get().expect("input element");
            input_elem.focus().expect("focus input");
        });
    });
    input![el_ref(&input.get())]
}

#[topo::nested]
fn if_example() -> Node<Msg> {
    use std::cmp::Ordering;
    let input_a = use_istate(String::new);
    let input_b = use_istate(String::new);

    if input_a.changed() || input_b.changed() {
        after_render(move |_| {
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
    }

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

use slotmap::{DefaultKey, DenseSlotMap};

#[derive(Clone)]
struct IdTreeNode {
    key: DefaultKey,
    child_keys: Vec<DefaultKey>,
    id: topo::Id,
}

#[derive(Clone, Default)]
struct IdTree {
    tree: DenseSlotMap<DefaultKey, IdTreeNode>,
    root_key: Option<DefaultKey>,
}

impl IdTree {
    fn root_with(&mut self, id: topo::Id) {
        let key = self.tree.insert_with_key(|key| IdTreeNode {
            child_keys: vec![],
            id,
            key,
        });
        self.root_key = Some(key);
    }
    fn destroy_all(&mut self) {
        for node in self.tree.values() {
            log!(format!("destroying {:#?}", node.id))
        }
    }

    fn get_branch_keys(&self, key: DefaultKey, collected_keys: &mut Vec<DefaultKey>) {
        let node = self.tree.get(key).unwrap();

        for key in &node.child_keys {
            collected_keys.push(*key);
            self.get_branch_keys(*key, collected_keys);
        }
    }

    fn destroy_from_id(&mut self, id: topo::Id) {
        let mut collected_keys = vec![];
        if let Some(node) = self.tree.values().find(|node| node.id == id) {
            collected_keys.push(node.key);
            self.get_branch_keys(node.key, &mut collected_keys);
        }
        for key in collected_keys {
            if let Some(node) = &self.tree.get(key) {
                log!(format!("Destroying: {:#?}", node.id));
            }
        }
    }

    fn push_child(&mut self, parent_id: topo::Id, child_id: topo::Id) {
        if self.tree.values().any(|node| node.id == child_id) {
            return;
        }

        let child_key = self.tree.insert_with_key(|key| IdTreeNode {
            key,
            id: child_id,
            child_keys: vec![],
        });

        let parent_key = self
            .tree
            .values()
            .find(|node| node.id == parent_id)
            .unwrap()
            .key;

        let parent = self.tree.get_mut(parent_key).unwrap();
        parent.child_keys.push(child_key);
    }
}

fn create_id_tree() -> StateAccess<IdTree> {
    use_state(|| {
        let mut id_tree = IdTree::default();
        id_tree.root_with(topo::Id::current());
        id_tree
    })
}

trait StateAccessIdTree {
    fn add_current(self, parent_id: topo::Id);
    fn add_id(self, parent_id: topo::Id, child_id: topo::Id);
}

impl StateAccessIdTree for StateAccess<IdTree> {
    fn add_current(self, parent_id: topo::Id) {
        self.update(|tree| tree.push_child(parent_id, topo::Id::current()));
    }
    fn add_id(self, parent_id: topo::Id, child_id: topo::Id) {
        self.update(|tree| tree.push_child(parent_id, child_id));
    }
}

#[topo::nested]
fn parent_comp_example() -> Node<Msg> {
    let tree_destroyed = use_istate(|| false);

    let id_tree = create_id_tree();

    div![
        if !tree_destroyed.get() {
            child_comp(id_tree, topo::Id::current())
        } else {
            empty![]
        },
        button![
            "Destroy",
            id_tree.mouse_ev(Ev::Click, move |tree, _| {
                tree.destroy_all();
                tree_destroyed.set(true);
            })
        ]
    ]
}

#[topo::nested]
fn child_comp(id_tree: StateAccess<IdTree>, parent_id: topo::Id) -> Node<Msg> {
    id_tree.add_current(parent_id);
    let current_id = topo::Id::current();
    div![
        button![
            "Destroy",
            id_tree.mouse_ev(Ev::Click, move |tree, _| {
                tree.destroy_from_id(current_id);
            })
        ],
        format!("{:#?}", topo::Id::current()),
        child_comp_a(id_tree, topo::Id::current()),
        child_comp_a(id_tree, topo::Id::current()),
        child_comp_a(id_tree, topo::Id::current()),
        child_comp_b(id_tree, topo::Id::current()),
    ]
}

#[topo::nested]
fn child_comp_a(id_tree: StateAccess<IdTree>, parent_id: topo::Id) -> Node<Msg> {
    id_tree.add_current(parent_id);
    let current_id = topo::Id::current();
    div![
        button![
            "Destroy",
            id_tree.mouse_ev(Ev::Click, move |tree, _| {
                tree.destroy_from_id(current_id);
            })
        ],
        format!("{:#?}", topo::Id::current())
    ]
}

#[topo::nested]
fn child_comp_b(id_tree: StateAccess<IdTree>, parent_id: topo::Id) -> Node<Msg> {
    id_tree.add_current(parent_id);
    let current_id = topo::Id::current();
    div![
        button![
            "Destroy",
            id_tree.mouse_ev(Ev::Click, move |tree, _| {
                tree.destroy_from_id(current_id);
            })
        ],
        format!("{:#?}", topo::Id::current()),
        child_comp_c(id_tree, topo::Id::current())
    ]
}

#[topo::nested]
fn child_comp_c(id_tree: StateAccess<IdTree>, parent_id: topo::Id) -> Node<Msg> {
    id_tree.add_current(parent_id);
    let current_id = topo::Id::current();

    div![
        button![
            "Destroy",
            id_tree.mouse_ev(Ev::Click, move |tree, _| {
                tree.destroy_from_id(current_id);
            })
        ],
        format!("{:#?}", topo::Id::current())
    ]
}

// #[topo::nested]
// fn cleanup_example() {}
