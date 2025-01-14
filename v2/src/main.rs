// Life time annotation, 'a ... then x and y have the smallest lifetime 
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len(){
        x
    }else{  
        y
    }
}

// Show invalid code because of wrong lifetime of variable pointed by 
// borrowers 
fn main()
{
    let s1 = String::from("abcd");
    let s2 = "xyz";

    let result = longest(s1.as_str(), s2);

    // The following does not work because 
    println!("Tom {} !\n", result);
}