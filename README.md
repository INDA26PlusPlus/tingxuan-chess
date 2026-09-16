Welcomee to my rust chess library

let mut game = Chess::new()
game.next_turn()
game.current_turn()

pub fn new_board() -> [Option<Piece>;64]
pub fn get_piece_at(&board: &[Option<Piece>;64], position: usize) -> Option<(PieceType,Color)>
pub fn king_in_check(board: [Option<Piece>;64], my_color: Color, opp_color: Color) -> bool
pub fn legal_moves(board: [Option<Piece>;64], position: usize) -> Vec<usize>
pub fn move_piece(from:usize, to:usize, board:&mut[Option<Piece>;64]) -> &mut[Option<Piece>;64]