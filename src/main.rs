fn main() {
    println!("Hello, world!");
    let (answer, remainder)  =long_division_algorithm(0, 7);
    println!("the div  is {}, the remainder is {}", answer, remainder);
}
fn get_first_n_digits(num: i64, n: usize) -> i64 {
    let num_str = num.abs().to_string();
    let end = n.min(num_str.len()); 
    let first_n_digits = &num_str[..end];
    first_n_digits.parse::<i64>().unwrap()
}
fn append_number(base: i64, to_append: i64) -> i64 {
    let some_value: Option<i64> = Some(to_append);

    match some_value {
        Some(value) => {
            let base_str = base.to_string();
            let to_append_str = value.to_string();
            let result_str = format!("{}{}", base_str, to_append_str);
            result_str.parse::<i64>().unwrap()
        },
        None => base, 
    }
}
fn get_digit_at_position(num: i64, position: usize) -> Option<i64> {
    let num_str = num.abs().to_string();
    if position > 0 && position <= num_str.len() {
        let digit_char = num_str.chars().nth(position - 1).unwrap();
    Some(digit_char.to_digit(10).unwrap() as i64)
   
    } else {
        None
    }
}
fn number_length(num: i64) -> usize {
    num.abs().to_string().len()
}
fn long_division_algorithm(dividend:i64, divisor:i64)->(i64, i64){
    if divisor == 0 && dividend != 0 {
     panic!("attempt to divide by zero")
    }
    let mut negation = false;
  
    if dividend < 1 || divisor < 1{
    negation = true;
    }
    let mut  len = 0;
    let mut first_iteration = true; 
    let mut answer = 0;
    let mut digit = 0;
    let dividend = dividend.abs();
    let divisor = divisor.abs();
    let dividend_len: usize =    number_length(dividend); 
    let divisor_len: usize =    number_length(divisor); 
    println!("divif {}", dividend);

    if dividend % divisor != 0 && dividend_len > divisor_len && dividend !=0 {
        
    loop{
    len +=  1;
    if first_iteration {
            loop {
                len += 1;
                
                if dividend >= divisor {
                    digit = get_first_n_digits(dividend, len);
                    break;
                }
            }
            first_iteration = false;
        }
    len += 1; 
    
    let answer_to_append = digit / divisor; 
    let mut remainder = digit % divisor;
    answer = append_number(answer, answer_to_append);

    if let Some(dividend_to_append) = get_digit_at_position( dividend,len) {
    len -=1;
    let  new_dividend  = append_number(remainder, dividend_to_append);  
    digit = new_dividend;
}else {
    remainder = digit % divisor;
    if negation{ break(-1 * answer, remainder);} else{ break (answer, remainder);};
}
    }
}else if dividend== 0 && divisor == 0{
    (0, 0)
}
else{
   answer =  dividend/divisor;
    if negation{ (-1 * answer, 0)} else{  (answer, 0)} 
}
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negative_zero_mod() {
       let (answer, remainder)  =long_division_algorithm(-200, 4);
        assert_eq!(answer, -50);
        assert_eq!(remainder, 0);
    }
    #[test]
    fn positive_zero_mod() {
       let (answer, remainder)  =long_division_algorithm(200, 4);
        assert_eq!(answer,50);
        assert_eq!(remainder,0);
    }
    #[test]
    fn positive_nonzero_mod() {
       let (answer, remainder)  =long_division_algorithm(200, 3);
        assert_eq!(answer,66);
        assert_eq!(remainder,2);
    }
    #[test]
    fn negative_nonzero_mod() {
       let (answer, remainder)  =long_division_algorithm(-200, 3);
        assert_eq!(answer,-66);
        assert_eq!(remainder,2);
    }
    #[test]
    fn recurring_mod() {
       let (answer, remainder)  =long_division_algorithm(100, 3);
        assert_eq!(answer,33);
        assert_eq!(remainder, 1);
    }
    #[test]
    fn moon_math1() {
       let (answer, remainder)  =long_division_algorithm(27, 5);
        assert_eq!(answer,5);
        assert_eq!(remainder, 2);
    }
    #[test]
    fn moon_math2() {
       let (answer, remainder)  =long_division_algorithm(27, -5);
        assert_eq!(answer,-5);
        assert_eq!(remainder, 2);
    }
    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn moon_math3() {
     long_division_algorithm(127, 0);
    }
    #[test]
    fn moon_math4() {
       let (answer, remainder)  =long_division_algorithm(-1687, 11);
        assert_eq!(answer,-153);
        assert_eq!(remainder, 4);
    }
    #[test]
    fn moon_math5() {
       let (answer, remainder)  =long_division_algorithm(0, 7);
        assert_eq!(answer,0);
        assert_eq!(remainder, 0);
    }
}