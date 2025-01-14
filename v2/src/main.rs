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
    let mut s = String::from("A cat");

    // create a local context
    {
        // two immutable borrow, valid
        let borrow = &s;
        let borrow2 = &s;
        println!("{},{}", borrow, borrow2);
        
        // not possible to do:
        // let mborrow = &mut s; 
    }


    display(&mut s);

    // The following does not work because 
    println!("Tom {}!\n", s);
}