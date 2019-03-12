mod turtle_board;

use turtle_board::TurtleBoard;

pub fn run() {
    println!("Hello from lib!");
    let mut board = TurtleBoard::new_lazy();
    board.add_horizontal_line(-2..7, 3);
    board.add_vertical_line(-1, -5..12);
    println!("The board is:\n{}", board);
}