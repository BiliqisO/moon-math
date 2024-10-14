 use mathematics::extended_euclidean_algorithm;
fn main() {
    let (gcd, s, t) =extended_euclidean_algorithm(100,25);
    println!("gcd {}, s {}, t {} ", gcd, s, t);
      
}
