fn main()
{
    let first_name = "Damien";
    let last_name = "Marchal";

    // Le ! signifie que ce qui précède est une macro. 
    // c'est une opération sur l'AST (et pas sur le texte en entrée) 
    // pour avoir des nombre d'arguments variable il faut passer par une macro
    // autrement lest fonctions ont un nombre fini de paramètres 
    println!("Hello world, {} {}!\n", first_name, last_name);
}