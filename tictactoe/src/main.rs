// Example d'une fonction générique 
fn take_and_give_back<T>(value: T) -> T {
    value
}

// Example d'une fonction générique avec une contrainte... 
fn generic_print<T: std::fmt::Display>(value: T)
// une contrainte sur un type générique 
{
    println!("{}", value);
}

fn generic<F,G,H>(v1: F, v2: G) -> H
where F: std::ops::Add<G, Output = H> + std::fmt::Display,  // F doit s'afficher et avoir une opération d'addition entre F et G retournan H comme type
      G: std::fmt::Display // G doit s'afficher 
{
    println!("{} + {}",v1,v2);
    v1 + v2
}

fn main(){
    let x= 2;
    let y = 3;
    println!("{}", generic(x,y))
}