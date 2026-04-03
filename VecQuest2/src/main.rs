fn main() {
    let vecteur: Vec<i32> = vec![1, 2, 3, 4, 5];

    let number2 = &vecteur[2];
    let number5 = vecteur.get(10);

    println!("{}", number2);
    // println!("{:?}", number5);

    match number5 {
        Some(x) => println!("{}", x),
        None => println!("Aucune valeur trouvée a cette index."),
    }
}
