// Show invalid code because of wrong lifetime of variable pointed by 
// borrowers 
fn main()
{
    let r;

    {
        let x = 5;
        r = &x; 
    }

    // The following does not work because 
    println!("Tom {} {}!\n", r, x);
}