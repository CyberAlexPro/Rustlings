// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.

#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}

// La macro "my_macro" est flexible et peut être appelée soit sans arguments pour un message générique, soit avec un argument pour un message personnalisé.
// Cela montre comment utiliser les macros pour créer des fonctionnalités réutilisables et polyvalentes.