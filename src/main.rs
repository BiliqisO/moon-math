fn main() {
    println!("Hello, world!"); 
    let ans =extended_euclidean_algorithm(30,12);

      println!("ans {} ", ans);
}
fn extended_euclidean_algorithm(a:i64,b:i64)->i64{

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
        break  (s*a + t*b);
        }
   
    }
    


}