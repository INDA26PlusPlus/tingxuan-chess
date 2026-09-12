use std::vec;

#[derive(Debug,PartialEq,Eq)]
enum PieceType { // difference chess pieces
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Debug,PartialEq,Eq)]
enum Color { // white or black pieces
    White,
    Black,
}

#[derive(Debug,PartialEq,Eq)]
struct Piece { 
    piece_type: PieceType,
    color: Color,
}

// new chess board 
fn new_board() {
    // board is 1d array, initialize with the chess piece else None in squares
    // Some() so Option know it is not None, but Some
    let mut chess_board: [Option<Piece>; 64] = [
    Some(Piece { piece_type: PieceType::Rook, color: Color::Black }), Some(Piece { piece_type: PieceType::Knight, color: Color::Black }), Some(Piece { piece_type: PieceType::Bishop, color: Color::Black }), Some(Piece { piece_type: PieceType::Queen, color: Color::Black }), Some(Piece { piece_type: PieceType::King, color: Color::Black }), Some(Piece { piece_type: PieceType::Bishop, color: Color::Black }), Some(Piece { piece_type: PieceType::Knight, color: Color::Black }), Some(Piece { piece_type: PieceType::Rook, color: Color::Black }), 
    Some(Piece { piece_type: PieceType::Pawn, color: Color::Black }), Some(Piece { piece_type: PieceType::Pawn, color: Color::Black }), Some(Piece { piece_type: PieceType::Pawn, color: Color::Black }), Some(Piece { piece_type: PieceType::Pawn, color: Color::Black }), Some(Piece { piece_type: PieceType::Pawn, color: Color::Black }), Some(Piece { piece_type: PieceType::Pawn, color: Color::Black }), Some(Piece { piece_type: PieceType::Pawn, color: Color::Black }), Some(Piece { piece_type: PieceType::Pawn, color: Color::Black }), 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    Some(Piece { piece_type: PieceType::Pawn, color: Color::White }), Some(Piece { piece_type: PieceType::Pawn, color: Color::White }), Some(Piece { piece_type: PieceType::Pawn, color: Color::White }), Some(Piece { piece_type: PieceType::Pawn, color: Color::White }), Some(Piece { piece_type: PieceType::Pawn, color: Color::White }), Some(Piece { piece_type: PieceType::Pawn, color: Color::White }), Some(Piece { piece_type: PieceType::Pawn, color: Color::White }), Some(Piece { piece_type: PieceType::Pawn, color: Color::White }),
    Some(Piece { piece_type: PieceType::Rook, color: Color::White }), Some(Piece { piece_type: PieceType::Knight, color: Color::White }), Some(Piece { piece_type: PieceType::Bishop, color: Color::White }), Some(Piece { piece_type: PieceType::Queen, color: Color::White }), Some(Piece { piece_type: PieceType::King, color: Color::White }), Some(Piece { piece_type: PieceType::Bishop, color: Color::White }), Some(Piece { piece_type: PieceType::Knight, color: Color::White }), Some(Piece { piece_type: PieceType::Rook, color: Color::White }), 
    ];

    //K king, Q queen, R rook, B bishop, N knight, P pawn
    println!("{:?}", chess_board);
    println!("{:?}", chess_board[0].as_ref().unwrap().piece_type); //as a reference, so Rust don't think I want to move it out, unwrap() takes it out of Option()
}

// get what chess piece is at a position, return tuple with what piece type and the color
fn get_piece_at(board: &[Option<Piece>;64], position: usize) -> Option<(&PieceType,&Color)> {
    if board[position].is_none() {
        return None;
    };
    let piecee = &board[position].as_ref().unwrap().piece_type;
    let colorr = &board[position].as_ref().unwrap().color;
    return Some((piecee,colorr));
}

// check if the square is blocked by a white or black piece
fn is_blocked(board: &[Option<Piece>;64], position: usize) -> Option<Color>{
    match get_piece_at(&board, position) {
        Some((_,Color::White)) => return Some(Color::White),
        Some((_,Color::Black)) => return Some(Color::Black),
        None => return None
    }
}

// return what the legal moves for a piece is
// unsigned integer, dynamic, array indices is usize
fn legal_moves(board: &[Option<Piece>;64], position: usize) -> Vec<usize> {
    let mut moves = vec![];
    //if get_piece_at gives none then just return empty vector
    //else piece and color are what we got from the function
    let (piece, color) = match get_piece_at(board, position) {
        Some(tup) => tup,
        None => return moves
    };

    match color { // divide up to if its a black or white piece
        Color::Black => { 
            match piece { // for each piece it has different possible moves
                PieceType::Rook => {
                    // go left (right)
                    if position%8!=7 {
                        for i in position+1..=63 {
                            if i%8==7 {
                                if let Some((_,Color::Black))=get_piece_at(board, i) {
                                    break;
                                }
                                else {
                                    moves.push(i);
                                    break;
                                }
                            }
                            match get_piece_at(board, i) {
                                None => moves.push(i),
                                Some((_,Color::White)) => {moves.push(i);break},
                                Some((_,Color::Black)) => {break}
                            }
                        }
                    }
                    // go right (left)
                    if position%8!=0 {
                        for i in (0..=position-1).rev() {
                            if i%8==0 {
                                if let Some((_,Color::Black))=get_piece_at(board, i) {
                                    break;
                                }
                                else {
                                    moves.push(i);
                                    break;
                                }
                            }
                            match get_piece_at(board, i) {
                                None => moves.push(i),
                                Some((_,Color::White)) => {moves.push(i);break},
                                Some((_,Color::Black)) => {break}
                            }
                        }
                    }
                    // go fram (ner)
                    if position<56 {
                        for i in (position+8..=63).step_by(8) {
                            if i>55 {
                                if let Some((_,Color::Black))=get_piece_at(board, i) {
                                    break;
                                }
                                else {
                                    moves.push(i);
                                    break;
                                }
                            }
                            match get_piece_at(board, i) {
                            None => moves.push(i),
                            Some((_,Color::White)) => {moves.push(i);break},
                            Some((_,Color::Black)) => {break}
                            }
                        }
                    }
                    // go bak (upp)
                    if position<8 {
                        for i in (position+8..=63).step_by(8) {
                            if i>55 {
                                if let Some((_,Color::Black))=get_piece_at(board, i) {
                                    break;
                                }
                                else {
                                    moves.push(i);
                                    break;
                                }
                            }
                            match get_piece_at(board, i) {
                                None => moves.push(i),
                                Some((_,Color::White)) => {moves.push(i);break},
                                Some((_,Color::Black)) => {break}
                            }
                        }
                    }
                },
                PieceType::Knight => { // eight possible moves DDL,DDR,DLL,DRR,ULL,URR,UUL,UUR (Left,Right,Down,Up)
                    // as long as it will not go outside the board and the piece there is not black it can go there
                    // DDL
                    if position>15 && position%8!=7 && !matches! (get_piece_at(board, position-15),Some((_,Color::Black))) {                      
                        moves.push(position-15)   
                    }
                    // DDR
                    if position>16 && position%8!=0 && !matches! (get_piece_at(board, position-17),Some((_,Color::Black))) {
                        moves.push(position-17)
                    }
                    // DLL
                    if position>7 && position%8!=6 && position%8!=7 && !matches! (get_piece_at(board, position-6),Some((_,Color::Black))) {
                        moves.push(position-6)
                    }
                    // DRR
                    if position>9 && position%8!=0 && position%8!=1 && !matches! (get_piece_at(board, position-10),Some((_,Color::Black))) {
                        moves.push(position-10)
                    }

                    // UUL
                    if position<47 && position%8!=7 && !matches! (get_piece_at(board, position+17),Some((_,Color::Black))) {
                        moves.push(position+17)
                    }
                    // UUR
                    if position<48 && position%8!=0 && !matches! (get_piece_at(board, position+15),Some((_,Color::Black))) {
                        moves.push(position+15)
                    }
                    // ULL
                    if position<54 && position%8!=6 && position%8!=7 && !matches! (get_piece_at(board, position+10),Some((_,Color::Black))) {
                        moves.push(position+10)
                    }
                    // URR
                    if position<56 && position%8!=0 && position%8!=1 && !matches! (get_piece_at(board, position+6),Some((_,Color::Black))) {
                        moves.push(position+6)
                    }
                },
                PieceType::Bishop => {},
                PieceType::Queen => {},
                PieceType::King => {
                    // go bak
                    if position>=8 {
                        match get_piece_at(board, position-8) {
                            // if there is no piece or a white piece then it can go there
                            None | Some((_,Color::White)) => moves.push(position-8),
                            _ => {} // everything else, so basically when color is black
                        }
                    }
                    // go fram
                    if position<=55 {
                        match get_piece_at(board, position+8) {
                            None | Some((_,Color::White)) => moves.push(position+8),
                            _ => {} 
                        }
                    }
                    // go right(left)
                    if position%8!=0 {
                        match get_piece_at(board, position-1) {
                            None | Some((_,Color::White)) => moves.push(position-1),
                            _ => {} 
                        }
                    }
                    // go left(right)
                    if position%8!=7 {
                        match get_piece_at(board, position+1) {
                            None | Some((_,Color::White)) => moves.push(position+1),
                            _ => {} 
                        }
                    }

                    // bak vänster
                    if position>7&&position%8!=7 {
                        match get_piece_at(board, position-7) {
                            None | Some((_,Color::White)) => moves.push(position-7),
                            _ => {} 
                        }
                    }
                    // bak höger
                    if position>7&&position%8!=0 {
                        match get_piece_at(board, position-9) {
                            None | Some((_,Color::White)) => moves.push(position-9),
                            _ => {} 
                        }
                    }
                    // fram vänster
                    if position<56&&position%8!=7 {
                        match get_piece_at(board, position+9) {
                            None | Some((_,Color::White)) => moves.push(position+9),
                            _ => {} 
                        }
                    }
                    // fram höger
                    if position<56&&position%8!=0 {
                        match get_piece_at(board, position+7) {
                            None | Some((_,Color::White)) => moves.push(position+7),
                            _ => {} 
                        }
                    }
                },
                PieceType::Pawn => {
                    if (position<=49) && get_piece_at(board, position+8).is_none() {
                        moves.push(position+8)
                    }
                    if (position>=8&&position<=15) && (get_piece_at(board, position+8).is_none()) {
                        moves.push(position+8*2)
                    }
                    if position<55&&position%8!=7 {
                        // less code than match
                        // if get_piece_at returns a Some where the color is white
                        if let Some((_,Color::White)) = get_piece_at(board, position+9) {
                            moves.push(position+9)
                        }    
                    }
                    if position<=55&&position%8!=0 {
                        if let Some((_,Color::White)) = get_piece_at(board, position+7) {
                            moves.push(position+7)
                        }
                    }
                }
            }
        }
        Color::White => {
            match piece {
                PieceType::Rook => {},
                PieceType::Knight => {},
                PieceType::Bishop => {},
                PieceType::Queen => {},
                PieceType::King => {},
                PieceType::Pawn => {}

            }
        }
    
    }
    return moves;
}

// move a piece from a square to another
fn move_piece(from: usize, to:usize) {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test(){

        let mut chess_board: [Option<Piece>; 64] = [
        Some(Piece { piece_type: PieceType::Rook, color: Color::Black }), Some(Piece { piece_type: PieceType::Knight, color: Color::Black }), Some(Piece { piece_type: PieceType::Bishop, color: Color::Black }), Some(Piece { piece_type: PieceType::Queen, color: Color::Black }), Some(Piece { piece_type: PieceType::King, color: Color::Black }), Some(Piece { piece_type: PieceType::Bishop, color: Color::Black }), Some(Piece { piece_type: PieceType::Knight, color: Color::Black }), Some(Piece { piece_type: PieceType::Rook, color: Color::Black }), 
        Some(Piece { piece_type: PieceType::Pawn, color: Color::Black }), Some(Piece { piece_type: PieceType::Pawn, color: Color::Black }), Some(Piece { piece_type: PieceType::Pawn, color: Color::Black }), Some(Piece { piece_type: PieceType::Pawn, color: Color::Black }), Some(Piece { piece_type: PieceType::Pawn, color: Color::Black }), Some(Piece { piece_type: PieceType::Pawn, color: Color::Black }), Some(Piece { piece_type: PieceType::Pawn, color: Color::Black }), Some(Piece { piece_type: PieceType::Pawn, color: Color::Black }), 
        None, None, None, None, None, None, None, None, 
        None, None, None, None, None, None, None, None, 
        None, None, None, None, None, None, None, None, 
        None, None, None, None, None, None, None, None, 
        Some(Piece { piece_type: PieceType::Pawn, color: Color::White }), Some(Piece { piece_type: PieceType::Pawn, color: Color::White }), Some(Piece { piece_type: PieceType::Pawn, color: Color::White }), Some(Piece { piece_type: PieceType::Pawn, color: Color::White }), Some(Piece { piece_type: PieceType::Pawn, color: Color::White }), Some(Piece { piece_type: PieceType::Pawn, color: Color::White }), Some(Piece { piece_type: PieceType::Pawn, color: Color::White }), Some(Piece { piece_type: PieceType::Pawn, color: Color::White }),
        Some(Piece { piece_type: PieceType::Rook, color: Color::White }), Some(Piece { piece_type: PieceType::Knight, color: Color::White }), Some(Piece { piece_type: PieceType::Bishop, color: Color::White }), Some(Piece { piece_type: PieceType::Queen, color: Color::White }), Some(Piece { piece_type: PieceType::King, color: Color::White }), Some(Piece { piece_type: PieceType::Bishop, color: Color::White }), Some(Piece { piece_type: PieceType::Knight, color: Color::White }), Some(Piece { piece_type: PieceType::Rook, color: Color::White }), 
        ];
        let res = get_piece_at(&chess_board, 0);
        print!("{:?}",res);
        let movee = legal_moves(&chess_board,0);
        print!("{:?}",movee)
    }
}