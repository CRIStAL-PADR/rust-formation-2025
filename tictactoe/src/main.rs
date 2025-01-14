fn square_to_char(s: &Square) -> char
{
    match s {
        Square::Empty => '.',
        Square::Cross => '+',
        Square::Circle => 'O'
    }
}


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

fn main() {
    let board = Board::new();

    board.display();
}




