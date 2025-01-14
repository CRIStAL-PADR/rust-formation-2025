fn main()
{
    // felix has the ownership of the string
    let felix = String::from("A cat");

    // tom has the ownership of the string, felix doesn't have anymore
    let tom = felix;

    println!("Tom {}!\n", tom);

    // The following line is not working. 
    //println!("Felix {}!\n", felix);
}