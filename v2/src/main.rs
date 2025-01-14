// Use the & to indicate a borrow that can be mutated... 
fn display(s: &mut String) {
    println!("{}", s);
}

// The rule of the borrowing... validated at compile time by borrow checker
// - one mutable borrow OR any number of immutable borrow. 
// - the borrow checker insure that the borrowed reference does not survive to its borrower

fn main()
{
    // felix has the ownership of the string, flaggued as mutable... 
    let mut felix = String::from("A cat");

    display(&mut felix);

    // The following does not work because 
    println!("Tom {}!\n", felix);
}