fn main() {
    println!("Hello, world!");
    let (answer, remainder)  =long_division_algorithm(-200, 4);
    println!("the div  is {}, the remainder is {}", answer, remainder);
}
fn get_first_n_digits(num: i64, n: usize) -> i64 {
    let num_str = num.abs().to_string();
    let end = n.min(num_str.len()); 
    let first_n_digits = &num_str[..end];
    first_n_digits.parse::<i64>().unwrap()
}
fn append_number(base: i64, to_append: i64) -> i64 {
    if to_append == 0{
        return 0;
    }
    let base_str = base.to_string();
    let to_append_str = to_append.to_string();
    let result_str = format!("{}{}", base_str, to_append_str);
    result_str.parse::<i64>().unwrap()
}
fn get_digit_at_position(num: i64, position: usize) -> i64 {
    let num_str = num.abs().to_string();
    if position > 0 && position <= num_str.len() {
        let digit_char = num_str.chars().nth(position - 1).unwrap();
    Some(digit_char.to_digit(10).unwrap() as i64).unwrap()
   
    } else {
        0
    }
}
fn number_length(num: i64) -> usize {
    num.abs().to_string().len()
}
fn long_division_algorithm(dividend:i64, divisor:i64)->(i64, i64){
    let mut neg_dividend = false;
    if dividend < 1{
    neg_dividend = true;
    }
    let mut  len =0;
    let mut first_iteration = true;
    let mut answer = 0;
    let mut final_remainder = 0;
    let mut digit = 0;
    let dividend = dividend.abs();
    let dividend_len: usize =    number_length(dividend); 
    let divisor_len: usize =    number_length(divisor); 

    if dividend % divisor != 0 && dividend_len > divisor_len  {
        
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
    println!("dividend length is: {}", len);
    let answer_to_append = digit / divisor; 
    println!(" digit2  is: {}", digit);
    let mut remainder = digit % divisor;
    answer = append_number(answer, answer_to_append);
    let dividend_to_append = get_digit_at_position( dividend,len);
    len -=1;

    if digit > divisor {
    let mut new_dividend  = append_number(remainder, dividend_to_append);  
    final_remainder = remainder; 
    digit = new_dividend;
    new_dividend = 0;
    println!("answer  is: {}", answer);
    remainder = 0;
}else {
    remainder = digit % divisor;
}
    if digit < divisor {  
    if neg_dividend{ break(-1 * answer, final_remainder);} else{ break (answer, final_remainder);};
    }
    }
}else{
   answer =  dividend/divisor;
    println!("answer  is: {}", answer);
      if neg_dividend{ (-1 * answer, 0)} else{  (answer, 0)} 
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
        assert_eq!(answer,6);
        assert_eq!(remainder,2);
    }
    #[test]
    fn nnegative_nonzero_mod() {
       let (answer, remainder)  =long_division_algorithm(-200, 3);
        assert_eq!(answer,-6);
        assert_eq!(remainder,2);
    }
}