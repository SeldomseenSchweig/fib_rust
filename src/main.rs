use std::io;


fn main() {


println!("Please type in the nth number you want from fib:   ");

 let mut x = String::new();

        io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line.");

        let x: i32 = x.trim().parse().unwrap();


        fib(x);

println!("the nth number is : {}", fib(x));
}



fn fib (n: i32) -> i32 {
    if n <= 0 {
          return 0;
    } else if n== 1{
          return 1;
} else {
    return fib (n-1)  + fib(n-2);
 }
}


    
