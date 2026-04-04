const COLLECTION: [i32; 14] = [21, 122, 891, 19, 9, 6, 90, 29, 752, 28, 92, 76, 22, 902];


fn cut_slice(data: &[i32], index_cut: usize) {
    if index_cut <= data.len() {
        let (l, r) = data.split_at(index_cut);

        display("left", l);
        display("right", r);
    } else {
        println!("Le slice n'a pas pu etre découpé");
    }
}

fn display(label: &str, data: &[i32]) {
    println!("Content of {}: {:?}", label, data);
    println!("Length of slice: {}", data.len());
    println!("\n");
}

fn main() { 
    cut_slice(&COLLECTION, 7);
}
