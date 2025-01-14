// Gestion d'erreur ! 

fn failing_function() -> Result<u32, String> 
{
    Err(String::from("Im' failing"))
}

fn propagating_function() -> Result<u32, String>
{
    // Il y a un opérateur ? sur les results
    let v= failing_function()?;
    Ok(v)
}

fn main(){
    match failing_function(){
        Ok(v) => println!("OK"),
        Err(e) => println!("OK")
    }

    let vv = match failing_function(){
        Ok(v) => v,
        Err(e) => return Err(ErrorType::from(e))
    };


}