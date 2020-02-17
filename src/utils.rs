use anymap::any::Any;
use comp_state::{do_once, topo, use_state, CloneState, StateAccess};
use seed::{prelude::*, *};
use slotmap::{DefaultKey, DenseSlotMap};
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

// thread_local! {
//     static CLEAN_UP_CLOSURES: RefCell<Vec<Box<dyn Fn()->()>>> = RefCell::new(vec![]);
// }

// thread_local! {
//     static CLEAN_UP: RefCell<CleanUp> = RefCell::new(CleanUp::default());
// }

// #[derive(Default)]
// struct CleanUp {
//     marked_for_cleanup: DenseSlotMap<DefaultKey, CleanUpGroup>,
//     protected_for_cleanup: DenseSlotMap<DefaultKey, CleanUpGroup>,
// }

// struct CleanUpGroup {
//     id: topo::Id,
//     anymap: anymap::Map<dyn Any>,
// }

// fn cleanup() {
//     CLEAN_UP.with(|clean_up| {
//         // this drains the marked for clean up slotmap
//         // destroying each accessor as we go.

//         // it then copies over the protected to the new marked for clean up.
//         // if a
//         let mut clean_up = clean_up.borrow_mut();

//         for (key, clean_up_group) in clean_up.marked_for_cleanup.drain() {
//             clean_up_group
//             log!("Desotrying {#:?}", accessor.id());
//         }

//         // for (key, accessor) in clean_up.protected_for_cleanup.drain() {
//         //     clean_up.marked_for_cleanup.insert(accessor);
//         // }
//         let empty_slotmap: DenseSlotMap<DefaultKey, Box<dyn StateAccessMarkForCleanUp>> =
//             DenseSlotMap::default();
//         let protected = std::mem::replace(&mut clean_up.protected_for_cleanup, empty_slotmap);
//         std::mem::replace(&mut clean_up.marked_for_cleanup, protected);

//         // clean_up.protected_for_cleanup.replace(vec![]);
//     })
// }

trait StateAccessMarkForCleanUp {
    fn mark_for_cleanup(&self);
    fn destroy(&self);
    fn equals<T>(&self, other: StateAccess<T>);
    fn id(&self) -> topo::Id;
}

// impl<T> StateAccessMarkForCleanUp for StateAccess<T>
// where
//     T: 'static,
// {
//     fn equals<R>(&self, other: StateAccess<R>) {
//         *self == other
//     }

//     fn id(&self) -> topo::Id {
//         self.id
//     }
//     fn mark_for_cleanup(&self) {
//         CLEAN_UP.with(|clean_up| {
//             let clean_up = clean_up.borrow_mut();

//             if !clean_up
//                 .marked_for_cleanup
//                 .iter()
//                 .any(|(k, a)| a.equals(self))
//             {
//                 clean_up.marked_for_cleanup.insert(Box::new(self));
//             }
//         });
//     }

//     fn destroy(&self) {
//         log!("Destroying {:#?}", self.id);
//     }
// }

// #[topo::nested]
// pub fn clean_up<F: Fn() -> () + 'static>(func: F) {
//     let closure_already_added = use_state(|| false);
//     if !closure_already_added.get() {
//         CLEAN_UP_CLOSURES.with(|clean_up_vec| clean_up_vec.borrow_mut().push(Box::new(func)));
//         CLEAN_UP_CLOSURES.with(|clean_up_vec| {
//             clean_up_vec
//                 .borrow_mut()
//                 .push(Box::new(|| closure_already_added.destroy()))
//         });
//         closure_already_added.set(true);
//     }
// }

// pub fn run_clean_up() {
//     CLEAN_UP_CLOSURES.with(|clean_up_vec| {
//         for closure in clean_up_vec.borrow().iter() {
//             closure();
//         }
//     });
// }
// #[topo::nested]
// pub fn after_render_once<F: Fn() -> () + 'static>(func: F) {
//     let already_triggered = use_state(|| false);
//     if already_triggered.get() {
//         return;
//     }

//     already_triggered.set(true);
//     let f = Rc::new(RefCell::new(None));
//     let g = f.clone();

//     // let mut i = 0;
//     *g.borrow_mut() = Some(Closure::wrap(Box::new(move |_| {
//         func();
//         f.borrow_mut().take();
//     }) as Box<dyn FnMut(f64)>));
//     request_animation_frame(g.borrow().as_ref().unwrap());
// }

pub fn after_render<F: Fn(f64) -> () + 'static>(func: F) {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    // let mut i = 0;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move |delta| {
        func(delta);
        f.borrow_mut().take();
    }) as Box<dyn FnMut(f64)>));
    request_animation_frame(g.borrow().as_ref().unwrap());
}

#[topo::nested]
pub fn after_render_once<F: Fn(f64) -> () + 'static + Clone>(func: F) {
    do_once(move || after_render(func.clone()));
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
