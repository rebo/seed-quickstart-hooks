use crate::state_access::CloneState;
use crate::state_functions::use_state;

/// call the provided function once and once only
///
/// Example
///
/// do_once(||{
///     println!("This will print only once");
/// });
pub fn do_once<F: Fn() -> ()>(func: F) {
    topo::call(|| {
        let has_done = use_state(|| false);
        if !has_done.get() {
            func();
            has_done.set(true);
        }
        58
    });
}
