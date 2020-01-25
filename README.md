# Seed Quickstart (React Style Hook Example)

**Introduction**

This is an evolving example demonstrating how React style hooks might integrate into Seed.

It requires nightly due to reliance on the [track_caller feature ](https://github.com/rust-lang/rust/issues/47809) and currently the [topo](https://crates.io/crates/topo) crate.

**To get started:**

- Clone this repo: `git clone https://github.com/rebo/seed-quickstart-hooks.git`

- The key difference between this quickstart and the normal example app is that this makes use of the `use_state()` api in order to store state that persists over renders and is associated with an identifiable component.

- `use_state` takes a closure that returns type T it returns a tuple of the type's current value and an accessor struct that can be used to modify that value. Often you would use the accessor struct to modify the value from a javascript callback such a `Ev::Click` event. See the`my_button()` function.

- Component's need their functions annoted with `#[topo::nested]` and there needs to be an ultimate root component usually called in the main app view root.

- Building this example app will create a div containing 5 buttons each with their own internal state.

- If you don't have Rust and cargo-make installed, [Download it](https://www.rust-lang.org/tools/install), and run the following commands:

`rustup update`

`rustup target add wasm32-unknown-unknown`

`cargo install --force cargo-make`

Run `cargo make build` in a terminal to build the app, and `cargo make serve` to start a dev server
on `127.0.0.1:8000`.

If you'd like the compiler automatically check for changes, recompiling as
needed, run `cargo make watch` instead of `cargo make build`.
