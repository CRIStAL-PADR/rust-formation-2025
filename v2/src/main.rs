// Use the & to indicate a borrow... 
fn display(s: &String) {
    println!("{}", s);
}

fn main()
{
    // felix has the ownership of the string
    let felix = String::from("A cat");

    display(&felix);

    // The following does not work because 
    //println!("Tom {}!\n", felix);
}