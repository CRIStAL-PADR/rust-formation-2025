// Closure & lambda
fn main(){
    let x = String::from("Hi");
    let y = String::from("Hi2");

    let hi = || {
        // closure by borrow 
        println!("{}", x)
    };

    let hi2 = move || {
        // string's ownership transfered to the closure (moved) 
        println!("{}", y)
    };
    hi();
    hi2();
}