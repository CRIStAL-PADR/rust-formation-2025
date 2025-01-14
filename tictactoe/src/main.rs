fn take_and_give_back<T>(value: T) -> T {
    value
}

fn main(){
    let i = 3;
    let j = take_and_give_back(i);
    let s = String::from("Hi");
    let s2 = take_and_give_back(s);
    println!("{} {}",j, s2);
}