use mathematics::extended_euclidean_algorithm;


/// Computes the pedersen hash of given inputs and a cyclic group.
///
/// # Examples
///
/// ```
/// use pedersen_hash_function::pedersen_hash_function;
/// let x = 1;
/// let y = 2;
/// let cyclic_group = 5;
/// let pedersen_hash = pedersen_hash_function(x ,y, 0, cyclic_group);
///
/// assert_eq!(pedersen_hash,3);
/// 
///  
/// ```
/// ```
/// use pedersen_hash_function::pedersen_hash_function;
/// let x = 3;
/// let y = 7;
/// let z = 11;
/// let cyclic_group = 13;
/// let pedersen_hash = pedersen_hash_function(x ,y, z, cyclic_group);
///
/// assert_eq!(pedersen_hash,8);
/// 
///  
/// ```
pub fn pedersen_hash_function(x:u64, y:u64, z: u64, mod_num:u64) -> u64{
  let generators = natural_numbers_generators(mod_num);
  let mut h = 1;
  let mut input: Vec<u64> = Vec::new();
  input.push(x);
  input.push(y);
  if z != 0{
  input.push(z);
  }
  loop{
  for (index, item) in input.clone().into_iter().enumerate(){ 
    let generator = generators.get(index).unwrap_or(&0);
        let hash = generator.pow(item as u32) ;
        h *= hash ;            
  }
  if generators.len() >= input.len()  {
  break;
  }
  }
  let pedersen_hash =  h % mod_num ;
  pedersen_hash
}
/// Computes the generators of a cyclic group
///
/// # Examples
///
/// ```
/// use pedersen_hash_function::natural_numbers_generators;
/// 
/// let cyclic_group = 5;
/// let generators = natural_numbers_generators(cyclic_group);
///
///  assert_eq!(generators,[2,3]);
/// 
///  
/// ```
pub fn natural_numbers_generators( mod_num:u64 ) -> Vec<u64> {
let mod_num_vec = create_vec_to_n(mod_num-1);
let mut generators:Vec<u64> = Vec::new();
let mut elements :Vec<u64>= vec![];
let mut new_vectors: Vec<Vec<u64>> =  Vec::new(); 
let mut generator: u64 = 1;

while mod_num > generator{
generator+=1;
for item in &mod_num_vec{
if extended_euclidean_algorithm(generator.try_into().unwrap(),mod_num.try_into().unwrap() ).0 ==1{
 let result =   generator.pow((*item).try_into().unwrap())% mod_num;
  elements.push(result);
};
}
  new_vectors.push(elements.clone());
  let mut elements_sorted= elements.clone();
  let mut mod_num_vec_sorted = mod_num_vec.clone();
    elements_sorted.sort();
    mod_num_vec_sorted.sort();
  if mod_num_vec_sorted == elements_sorted{
  generators.push(generator );

  }
elements = [].to_vec();
}
generator = 1;
generators
 
}

fn create_vec_to_n(n: u64) -> Vec<u64> {
    (1..=n).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let peserden_hash=pedersen_hash_function(1 ,2, 0, 5);
        assert_eq!(peserden_hash,3);
  
    }
    #[test]
    fn test2() {
        let peserden_hash=pedersen_hash_function(3 ,7, 11, 13);
        assert_eq!(peserden_hash,8);
  
    }
    #[test]
    fn test3() {
        let generator=natural_numbers_generators(5);
        assert_eq!(generator, [2,3]);
  
    }
}