fn longest(x: &str, y: &str) -> &str{
    if x.len() > y.len(){
        x
    }  
    y
}

// Show invalid code because of wrong lifetime of variable pointed by 
// borrowers 
fn main()
{
    let s1 = String::from("abcd");
    let s2 = "xyz";

    let result = longest(strng1.as_str(), string2);

    // The following does not work because 
    println!("Tom {} !\n", result);
}