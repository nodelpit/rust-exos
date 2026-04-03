fn main() {
    let mut scores: Vec<i32> = Vec::new();

    scores.push(7);
    scores.push(19);
    scores.push(13);
    scores.push(12);
    scores.push(16);

    let mut somme = 0;

    for score in &scores {
        somme += score;
    }
    println!("Somme: {}", somme);



    let higher_number_in_vec = scores.iter().max();
    match higher_number_in_vec {
        Some(x) => println!("Le plus grand nombre dans le vecteur est: {}", x),
        None => println!("Le vecteur ne peut pas etre vide !"),
    }
}
