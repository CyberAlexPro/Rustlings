// traits5.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits5` or use the `hint` watch subcommand for a
// hint.


pub trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

pub trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct {}
struct OtherStruct {}

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// YOU MAY ONLY CHANGE THE NEXT LINE
fn some_func<T: SomeTrait + OtherTrait>(item: T) -> bool {
    item.some_function() && item.other_function()
}

fn main() {
    some_func(SomeStruct {});
    some_func(OtherStruct {});
}

// On fait usage de Traits Multiples: La notation <T: SomeTrait + OtherTrait> spécifie que T doit implémenter les traits SomeTrait et OtherTrait.
// On garantit que l'objet passé à some_func aura à la fois les méthodes some_function et other_function disponibles, permettant ainsi d'appeler ces méthodes sans problème.
// On a mit de la flexibilité : Cette approche rend la fonction some_func flexible et capable d'accepter n'importe quel type qui satisfait aux deux contraintes de trait, facilitant ainsi la réutilisation et l'amélioration de la modularité du code.
// On maintient les Appels de Fonction: Les appels à some_func dans main restent inchangés, car SomeStruct et OtherStruct implémentent déjà les deux traits requis, et la fonction peut donc accepter une instance de ces structures sans modification supplémentaire.