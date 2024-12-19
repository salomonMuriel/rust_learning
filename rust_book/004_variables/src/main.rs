fn main() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    {
        let x = x*2;
        println!("The value of x in the inner scope is {x}")
    }

    println!("The value of x is back to {x}");

    
    // This is interesting. Constant is evaluated at compile time, 
    // so I can declare it anywhere.
    println!("The value of the constant is {SAMPLE}");
    const SAMPLE: u32 = 25;


    let spaces = "       ";
    let spaces = spaces.len();
    println!("There are {spaces} spaces");
}
