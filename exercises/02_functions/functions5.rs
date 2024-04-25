// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}

// Ici on supprime le point virgule afin que la fonction s'éxécute correctement
// Avec le point virgule cela transformait la ligne en une instruction qui ne renvoie rien
// au lieu de la valeur du calcul.