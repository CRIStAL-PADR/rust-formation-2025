fn display(board: [char; 9])
{
    for i in 0..3
    {
        println!(" {}", board[i]);
    }
}

enum Square {
    
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
        todo!();
    }
}

fn main() {
    let board = ['.'; 9]; // un tableau de neuf cases

    display(board);
}
