fn main() {
    println!("Hello, world!"); 
    let ans =persedens_hash_function(5);

      println!("ans {:?}", ans);
      
}
fn persedens_hash_function( mod_num:u32 ) -> Option<Vec<u32>>  {
let mut generators :Vec<u32>= vec![];
let mut generator: u32 = 1;
let mut exponent = 0;

while  generator < mod_num {
  generator += 1;
  while  exponent < mod_num{
    exponent+=1;
   println!("mod_num {:?}", mod_num);
   println!("exponent {:?}", exponent);
  println!("generator {:?}", generator);
  let result = u32::pow(generator, exponent) % mod_num;
  // if  {
  //   return None;
  // }
  if result  == 1  {     
    generators.push(generator);
   println!("generators {:?}", generators);
} else if result  == 1 && exponent < mod_num{
  return None;
};
  }

 


   
}
   return Some(generators);
}










// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_multiplication() {
//         let modular_arithmetic=remainder_class_algorithm(1, 5, String::from("*"), 5);
//         assert_eq!(modular_arithmetic,0);
  
//     }
//     #[test]
//     fn test_addition() {
//         let modular_arithmetic=remainder_class_algorithm(1, 5, String::from("+"), 5);
//         assert_eq!(modular_arithmetic, 1);
  
//     }
//     #[test]
//     #[should_panic(expected = "Invalid operator. Please enter + or *.")]
//     fn test_invalid_operator() {
//     remainder_class_algorithm(1, 5, String::from(""), 5);
//     }
// }