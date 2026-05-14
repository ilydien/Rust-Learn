fn main() {
    let myname: &str = "dien"; //tipe data string 
    let myage: i32 = 19; // tipe data integer (i8 s/d i128 - bisa juga depends on architecture (isize)) 
    let myblodtype:  char =  'A'; // tipe data character
    let mywallet: f64 = 1.500; // tipe data float
    print!("hi my name is {},\nim {} yo,\nmy blod type is {},\nand my wallet is {}", myname, myage, myblodtype, mywallet);
    
}
