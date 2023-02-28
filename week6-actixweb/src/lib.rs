/*A library that returns back random fruit */

use rand::Rng;

//create an const array of 10 fruits
pub const FRUITS: [&str; 10] = [
    "Apple",
    "Banana",
    "Orange",
    "Pineapple",
    "Strawberry",
    "Watermelon",
    "Grapes",
    "Mango",
    "Papaya",
    "Kiwi",
];

//create an const array of 10 drink options
pub const DRINKS: [&str; 10] = [
    "Water",
    "Milk",
    "Orange Juice",
    "Sparkling Water",
    "Soda",
    "Iced Americano",
    "Iced Latte",
    "Iced Tea",
    "Smoothie",
    "Tea",
];

//create a function that returns a random fruit
pub fn random_fruit() -> &'static str {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..FRUITS.len());
    FRUITS[random_index]
}

//create a function that returns a random drink
pub fn random_drink() -> &'static str {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..DRINKS.len());
    DRINKS[random_index]
}
