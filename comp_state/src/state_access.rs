use crate::state_functions::*;
use std::marker::PhantomData;

///  Accessor struct that provides access to getting and setting the
///  state of the stored type
#[derive(Debug)]
pub struct StateAccess<T> {
    pub id: topo::Id,
    _phantom_data: PhantomData<T>,
}

impl<T> Copy for StateAccess<T> {}
impl<T> Clone for StateAccess<T> {
    fn clone(&self) -> StateAccess<T> {
        StateAccess::<T> {
            id: self.id,
            _phantom_data: PhantomData::<T>,
        }
    }
}

impl<T> StateAccess<T>
where
    T: 'static,
{
    pub fn new(id: topo::Id) -> StateAccess<T> {
        StateAccess {
            id,
            _phantom_data: PhantomData,
        }
    }

    // stores a value of type T in a backing Store
    pub fn set(self, value: T) {
        set_state_with_topo_id(value, self.id);
    }

    pub fn remove(self) -> Option<T> {
        remove_state_with_topo_id(self.id)
    }

    /// updates the stored state in place
    /// using the provided function
    pub fn update<F: FnOnce(&mut T) -> ()>(self, func: F) {
        update_state_with_topo_id(self.id, func);
    }
}

trait CloneForStateAccess<T>
where
    T: Clone + 'static,
{
    fn clone_state(&self) -> Option<T>;
    fn hard_clone(&self) -> T;
}

impl<T> CloneForStateAccess<T> for StateAccess<T>
where
    T: Clone + 'static,
{
    fn clone_state(&self) -> Option<T> {
        clone_state_with_topo_id::<T>(self.id)
    }

    /// returns a clone of the stored state panics if not stored.
    fn hard_clone(&self) -> T {
        clone_state_with_topo_id::<T>(self.id).unwrap()
    }
}
