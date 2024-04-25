// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.


struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}

// En déclarant Wrapper<T>, on permet à cette structure de stocker n'importe quel type T. 
// L'implémentation de new prend également un paramètre de type générique T, rendant cette méthode constructeur applicable à n'importe quel type de données.
// Les tests restent fonctionnels sans modification car Rust peut déduire les types u32 et &str pour les arguments passés à Wrapper::new(). 
// Si nécessaire, les types peuvent être spécifiés explicitement, mais dans ce cas simple, l'inférence de type est suffisante.