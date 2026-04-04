const COLLECTION: [i32; 4] = [12, 23, 73, 92];

fn analyze(slice: &[i32], seuil: i32) -> i32 {
    let mut count = 0;

    for num in slice.iter() {
        if *num >= seuil {
            count += 1;
            println!("Le seuil est en train d'être dépassé: {:?} pour {:?}", num, seuil);
        }
    }
    count
}

fn resume(slice: &[i32], seuil: i32, count: i32) {
    println!("Collection: {:?} - Seuil max: {:?} - Nombres au dessus du seuil: {:?}", slice, seuil, count);
}

fn main() {
    let seuil = 50;
    let count = analyze(&COLLECTION, seuil);
    resume(&COLLECTION, seuil, count);
}