fn main() {
    println!("Hello, world!"); 
    let ans =remainder_class_algorithm(2,6, String::from("*") ,5);

      println!("gcd {:?}", ans);
      
}
enum Action {
  Addition,
  Multiplication,
  None
}
fn remainder_class_algorithm(a:i64,b:i64, operator:String, mod_num:i64 ) -> i64  {
let mut final_ans = 0;
let mut  inter_action:Action = Action::None;

  match operator.as_str() {
            "+" => {let operation =  Action::Addition;
            inter_action = operation},
            "*" => {let operation= Action::Multiplication;
            inter_action = operation},
            _ => {
                panic!("Invalid operator. Please enter + or *.");
            }
          }
match inter_action {
    Action::Addition=>{let ans = a + b; 
    final_ans = ans}
    Action::Multiplication => { let ans = a* b;
    final_ans = ans},
    Action::None =>{
      println!("Invalid operator. Please enter + or *.");
    }

}
final_ans = final_ans % mod_num;
println!("{}", final_ans);
final_ans




}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplication() {
        let modular_arithmetic=remainder_class_algorithm(1, 5, String::from("*"), 5);
        assert_eq!(modular_arithmetic,0);
  
    }
    #[test]
    fn test_addition() {
        let modular_arithmetic=remainder_class_algorithm(1, 5, String::from("+"), 5);
        assert_eq!(modular_arithmetic, 1);
  
    }
    #[test]
    #[should_panic(expected = "Invalid operator. Please enter + or *.")]
    fn test_invalid_operator() {
    remainder_class_algorithm(1, 5, String::from(""), 5);
    }
}