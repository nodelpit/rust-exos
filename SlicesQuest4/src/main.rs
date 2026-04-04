const MEASUREMENTS: [i32; 10] = [32, 25, 13, 96, 56, 90, 76, 67, 71, 49];

fn find_values(slice: &[i32], target: i32) {
    let value = slice.iter().position(|&x| x == target);

    match value {
        Some(v) => println!("Position de la target({}): {}", target, v),
        _ => println!("Aucune valeur trouvée"),
    }
}

fn above_threshold(slice: &[i32], seuil: i32) {
    // for num in slice.iter().filter(|&&n| n >= seuil) {
    //     println!("{}", num);
    // }

    let filtered: Vec<_> = slice.iter().filter(|&&n| n >= seuil).collect();
    println!("Nombres >= à {} : {:?}", seuil, filtered);
}

fn main() {
    find_values(&MEASUREMENTS, 90);
    above_threshold(&MEASUREMENTS, 60);
}