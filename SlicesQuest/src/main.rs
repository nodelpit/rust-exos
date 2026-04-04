const COLLECTION: [i32; 8] = [21, 122, 891, 19, 9, 6, 90, 29];

fn show_len(data: &[i32]) {
    let length = data.len();

    println!("Taille de la collection: {}", length);
}

fn first_and_last(data: &[i32]) {
    let first = data.first();
    let last = data.last();

    if let (Some(f), Some(l)) = (first, last) {
        println!("Le premier et le dernier élément sont {:?} et {:?}", f, l)
    } else { 
        println!("La slice est vide ou une valeur est absente")
    }
}

fn part_of_collection(data: &[i32]) {
    println!("Voici une partie de la collecion: {:?}", data);
}

fn process_text(data: &str) {

    println!("{}", &data[8..16]);
}


fn main() {
    first_and_last(&COLLECTION);
    show_len(&COLLECTION);
    part_of_collection(&COLLECTION[1..6]);
    process_text("critical system error !!");
}
