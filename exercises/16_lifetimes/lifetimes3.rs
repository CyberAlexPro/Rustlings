// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.

struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}


// Le paramètre de l"ifetime 'a" indique que les champs author et title dans Book sont des références qui vivent aussi longtemps que 'a.
// Cela assure que Book ne survit pas à ses références, prévenant ainsi les erreurs de dangling references (références pendantes).
// Dans main, les références à name et title sont passées à Book. Ces références sont valides pendant toute la durée de vie de name et title dans main, ce qui correspond au lifetime spécifié.