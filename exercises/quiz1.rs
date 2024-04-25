// quiz1.rs
//
// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - If Mary buys more than 40 apples, each apple only costs 1 rustbuck!
// Write a function that calculates the price of an order of apples given the
// quantity bought.
//
// No hints this time ;)


// Put your function here!
fn calculate_price_of_apples (quantity: u32) -> u32 {
    if quantity > 40 {
        quantity // Au dessus de 40 ça coute 1 rustbuck
    }   else {
        quantity * 2 // En dessous de 40 ça coute 2 rustbuck
    }
}

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(41);
    let price4 = calculate_price_of_apples(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(41, price3);
    assert_eq!(65, price4);
}

// La fonction fn calculate_price_of_apples inclus une condition
// pour déterminer le prix total, Si le nombre de pommes est supérieur à 40, alors le prix par pomme est réduit à 1 rustbuck. 
// Donc, le coût total est simplement égal à la quantité de pommes (puisque chaque pomme coûte 1 rustbuck).
// Si le nombre de pommes est de 40 ou moins, alors chaque pomme coûte 2 rustbucks. Le coût total est donc la quantité multipliée par 2.