fn square_to_char(s: &Square) -> char
{
    match s {
        Square::Empty => '.',
        Square::Cross => '+',
        Square::Circle => 'O'
    }
}

/// Adding a behavior ... 
// std::string ConvertToString(auto& object);
// std::string ConvertToString(Square& object){
//      blabla
// }
// 

// On définit un nouveau traits ici 
trait ConvertToString {
    // Returns a string representation of self
    fn to_string(&self) -> String;
}

impl ConvertToString for Square{
    fn to_string(&self) -> String 
    {
        match self {
            Square::Empty => String::from("."),
            Square::Cross => String::from("X"),
            Square::Circle => String::from("O"),            
        }
    }
}

impl ConvertToString for Board {
    fn to_string(&self) -> String 
    {
        let mut out = String::new();
        for row in 0..3 {

            //self.board[row*3].to_string(),
            //self.board[row*3].to_string(),
            //self.board[row*3].to_string())
            out.push_str(format!("{} {} {}", "one", "two", "three"));    
        }
        out;
    }
}

// Mais il existe des traits pré-existant...
// Debug: format with "{:.}"
// Display: format with "{}"
// PartialEq, Eq: pour définir des types qui sont comparables
// Copy: a small copiable type (copiable bit à bit, continu etc...automatique appelé par copy)
// Clone: explicitly called ... can be implemented by the 


enum Square {
    Empty, Cross, Circle    
}

struct Board {
    // std::array<Square, 9>
    board: [Square; 9] // Un tableau qui contient des éléments de type Square de taille 9
}

// Add implementations...
impl Board {
    fn new() -> Self { // Le type self avec un S majuscule... c'est le type qu'on est en train d'implémenter
        todo!();
    }

    fn display(&self) {
        for row in 0..3
        {
            println!(" {} | {} | {} ", 
                square_to_char(&self.board[row*3+0]), 
                square_to_char(&self.board[row*3+1]), 
                square_to_char(&self.board[row*3+2]));
            println!("--------------")
        }
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Square::Empty => write!(f, "."),
            Square::Cross => write!(f, "X"),
            Square::Circle => write!(f, "O"),
        }
    }
}

fn main() {
    let board = Board::new();

    board.display();
}




