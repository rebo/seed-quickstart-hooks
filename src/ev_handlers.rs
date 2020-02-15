use comp_state::StateAccess;
use seed::prelude::*;
// use web_sys;

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
