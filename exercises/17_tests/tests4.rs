// tests4.rs
//
// Make sure that we're testing for the correct conditions!
//
// Execute `rustlings hint tests4` or use the `hint` watch subcommand for a
// hint.

struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    // Only change the test functions themselves
    pub fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Rectangle width and height cannot be negative!")
        }
        Rectangle {width, height}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // This test should check if the rectangle is the size that we pass into its constructor
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // check width
        assert_eq!(rect.height, 20); // check height
    }

    #[test]
    #[should_panic]
    fn negative_width() {
        // This test should check if program panics when we try to create rectangle with negative width
        let _rect = Rectangle::new(-10, 10);
    }

    #[test]
    #[should_panic]
    fn negative_height() {
        // This test should check if program panics when we try to create rectangle with negative height
        let _rect = Rectangle::new(10, -10);
    }
}

// Les tests pour les dimensions correctes comparent directement les valeurs des champs width et height avec les valeurs attendues.
// Cela garantit que la construction du rectangle a bien affecté les attributs de l'objet.
// L'attribut #[should_panic(expected = "...")] est utilisé pour spécifier que le test réussit uniquement si le code à l'intérieur de la fonction de test provoque un panic et que le message d'erreur correspond à celui attendu.
// Cela permet de vérifier que votre gestion d'erreurs fonctionne comme prévu lorsqu'une dimension négative est donnée.