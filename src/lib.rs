/// Computes the greatest common divisor of 2 natural numbers and 2 additional numbers such that gcd(a,b)=s·a+t·b holds.
///
/// # Examples
///
/// ```
/// let a = 27;
/// let b = 5;
/// let  (gcd, s, t) = mathematics::extended_euclidean_algorithm(a, b);
///
///  assert_eq!(gcd,5);
///  assert_eq!(s, 1);
///  assert_eq!(t, -4);
/// ```
pub fn extended_euclidean_algorithm(a:i64,b:i64)->(i64, i64, i64){ if a <= 0 || b <= 0{
        panic!("natural numbers only")
    }
    let mut q =0;
    let mut r1 = a;
    let mut r2=b;
    let mut s = 1;
    let mut t = 0;
    let mut s1 = 0;
    let mut t1 = 1;

    loop{
        q = r1/r2;
        let new_r = r2;
        let new_s = s1;
        let new_t = t1;

        r2 = r1 % r2;

        s1 = s - q * s1;

        s = new_s;
        t1 = t - q * t1;
        t = new_t;
        r1 =  new_r;
        if r2 == 0{
        break  (s*a + t*b, s, t);
        }
   
    }
    


}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moon_math1() {
        let (gcd, s, t)=extended_euclidean_algorithm(27, 5);
        assert_eq!(gcd,1);
        assert_eq!(s, -2);
        assert_eq!(t, 11);
    }
    #[test]
    fn moon_math2() {
      let (gcd, s, t)=extended_euclidean_algorithm(45, 10);
        assert_eq!(gcd,5);
        assert_eq!(s, 1);
        assert_eq!(t, -4);
    }
    #[test]
    fn moon_math3() {
      let (gcd, s, t) =extended_euclidean_algorithm(13, 11);
        assert_eq!(gcd,1);
        assert_eq!(s, -5);
        assert_eq!(t, 6);
    }
    #[test]
    fn moon_math4() {
      let (gcd, s, t)=extended_euclidean_algorithm(13, 12);
        assert_eq!(gcd,1);
        assert_eq!(s, 1);
        assert_eq!(t, -1);
    }
      #[test]
    #[should_panic(expected = "natural numbers only")]
    fn moon_math5() {
    extended_euclidean_algorithm(0, 12);
    }
      #[test]
    #[should_panic(expected = "natural numbers only")]
    fn moon_math6() {
    extended_euclidean_algorithm(12, 0);
    }

       #[test]
    #[should_panic(expected = "natural numbers only")]
    fn moon_math7() {
    extended_euclidean_algorithm(0, 0);
    }
   
}