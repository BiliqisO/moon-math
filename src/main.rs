
pub mod lib;

use pedersen_hash_function::{pedersen_hash_function, natural_numbers_generators};



fn main() {
  natural_numbers_generators(13);
  let pedersen_hash = pedersen_hash_function(1 ,2, 0, 5);
  println!("Pedersen_hash {:?}", pedersen_hash);
}