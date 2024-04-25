// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let data = "Rust is great!".to_string();

    let last_char = get_char(&data);  
    println!("Last character: {}", last_char);

    string_uppercase(data);  
}


fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}


fn string_uppercase(mut data: String) {
    data = data.to_uppercase();
    println!("{}", data);
}

// get_char : La fonction prend maintenant une référence (&String) au lieu de prendre la propriété (String).
// Cela permet à la chaîne de rester dans main après que get_char soit appelée.
// string_uppercase : La fonction prend maintenant la propriété de la chaîne (en acceptant String sans le &).
// L'intérieur de la fonction convertit la chaîne en majuscules et l'imprime. La chaîne originale est consommée,
// ce qui signifie qu'elle ne peut plus être utilisée dans main après cet appel.