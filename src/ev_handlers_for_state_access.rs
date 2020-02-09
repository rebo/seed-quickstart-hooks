use comp_state::StateAccess;
use seed::prelude::*;
use web_sys;

pub trait StateAccessEventHandlers<T>
where
    T: 'static,
{
    fn input_ev<F: Fn(&mut T, String) -> () + 'static + Clone, Ms: Default + 'static>(
        &self,
        event: Ev,
        func: F,
    ) -> seed::Listener<Ms>;

    fn mouse_ev<F: Fn(&mut T, web_sys::MouseEvent) -> () + 'static + Clone, Ms: Default + 'static>(
        &self,
        event: Ev,
        func: F,
    ) -> seed::Listener<Ms>;
}

impl<T> StateAccessEventHandlers<T> for StateAccess<T>
where
    T: 'static,
{
    fn input_ev<F: Fn(&mut T, String) -> () + 'static + Clone, Ms: Default + 'static>(
        &self,
        event: Ev,
        func: F,
    ) -> seed::Listener<Ms> {
        let accessor = *self;
        input_ev(event, move |text| {
            accessor.update(|val| func(val, text));
            Ms::default()
        })
    }

    fn mouse_ev<
        F: Fn(&mut T, web_sys::MouseEvent) -> () + 'static + Clone,
        Ms: Default + 'static,
    >(
        &self,
        event: Ev,
        func: F,
    ) -> seed::Listener<Ms> {
        let accessor = *self;
        mouse_ev(event, move |m_ev| {
            accessor.update(|val| func(val, m_ev));
            Ms::default()
        })
    }
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
