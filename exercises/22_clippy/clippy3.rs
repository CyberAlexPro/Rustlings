// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
// No hints.

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if let Some(x) = my_option {};

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}

// J'ai changé la condition pour vérifier si l'option contient une valeur avant d'essayer de la déballer, évitant ainsi un panic potentiel.
// J'ai ajouté des virgules entre les éléments du tableau pour corriger les erreurs syntaxiques et assurer que le tableau est correctement défini.
// Au lieu de tenter d'initialiser et de redimensionner un vecteur à zéro dans une seule expression, j'ai utilisé deux étapes : initialisation puis appel à resize() pour vider le vecteur, ce qui rend l'opération explicite et correcte.
// Pour échanger correctement les valeurs de deux variables, j'ai utilisé std::mem::swap() qui échange efficacement les valeurs sans nécessiter une variable temporaire.