#[derive(Debug,PartialEq,Eq,Clone,Copy)]
pub enum PieceType { // difference chess pieces
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Debug,PartialEq,Eq,Clone,Copy)]
pub enum Color { // white or black pieces
    White,
    Black,
}

#[derive(Debug,PartialEq,Eq,Clone,Copy)]
pub struct Piece { 
    piece_type: PieceType,
    color: Color,
}

#[derive(Debug,PartialEq,Eq,Clone,Copy)]
pub struct Chess {
    board: [Option<Piece>;64],
    turn: Color,
    // if castling has happened
    r_white_castle: bool,
    l_white_castle: bool,
    r_black_castle: bool,
    l_black_castle: bool,
    en_passant: Option<usize>,
}

impl Chess {
    // function to start new game
    pub fn new() -> Chess {
        Chess {
            board: new_board(),
            turn: Color::White,
            // if castling is possible
            r_white_castle: true,
            l_white_castle: true,
            r_black_castle: true,
            l_black_castle: true,
            en_passant: None,
        }
    }

    pub fn next_turn(&mut self) {
        if self.turn == Color::Black {
            self.turn = Color::White
        }
        else {
            self.turn = Color::Black
        }
    }

    pub fn current_turn(&self) -> Color {
        self.turn
    }

}

// new chess board 
pub fn new_board() -> [Option<Piece>;64] {
    // &board is 1d array, initialize with the chess piece else None in squares
    // Some() so Option know it is not None, but Some
    let mut board: [Option<Piece>; 64] = [
    Some(Piece { piece_type: PieceType::Rook, color: Color::Black }), Some(Piece { piece_type: PieceType::Knight, color: Color::Black }), Some(Piece { piece_type: PieceType::Bishop, color: Color::Black }), Some(Piece { piece_type: PieceType::Queen, color: Color::Black }), Some(Piece { piece_type: PieceType::King, color: Color::Black }), Some(Piece { piece_type: PieceType::Bishop, color: Color::Black }), Some(Piece { piece_type: PieceType::Knight, color: Color::Black }), Some(Piece { piece_type: PieceType::Rook, color: Color::Black }), 
    Some(Piece { piece_type: PieceType::Pawn, color: Color::Black }), Some(Piece { piece_type: PieceType::Pawn, color: Color::Black }), Some(Piece { piece_type: PieceType::Pawn, color: Color::Black }), Some(Piece { piece_type: PieceType::Pawn, color: Color::Black }), Some(Piece { piece_type: PieceType::Pawn, color: Color::Black }), Some(Piece { piece_type: PieceType::Pawn, color: Color::Black }), Some(Piece { piece_type: PieceType::Pawn, color: Color::Black }), Some(Piece { piece_type: PieceType::Pawn, color: Color::Black }), 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    Some(Piece { piece_type: PieceType::Pawn, color: Color::White }), Some(Piece { piece_type: PieceType::Pawn, color: Color::White }), Some(Piece { piece_type: PieceType::Pawn, color: Color::White }), Some(Piece { piece_type: PieceType::Pawn, color: Color::White }), Some(Piece { piece_type: PieceType::Pawn, color: Color::White }), Some(Piece { piece_type: PieceType::Pawn, color: Color::White }), Some(Piece { piece_type: PieceType::Pawn, color: Color::White }), Some(Piece { piece_type: PieceType::Pawn, color: Color::White }),
    Some(Piece { piece_type: PieceType::Rook, color: Color::White }), Some(Piece { piece_type: PieceType::Knight, color: Color::White }), Some(Piece { piece_type: PieceType::Bishop, color: Color::White }), Some(Piece { piece_type: PieceType::Queen, color: Color::White }), Some(Piece { piece_type: PieceType::King, color: Color::White }), Some(Piece { piece_type: PieceType::Bishop, color: Color::White }), Some(Piece { piece_type: PieceType::Knight, color: Color::White }), Some(Piece { piece_type: PieceType::Rook, color: Color::White }), 
    ];

    return board
}

// get what chess piece is at a position, return tuple with what piece type and the color
pub fn get_piece_at(board: &[Option<Piece>;64], position: usize) -> Option<(PieceType,Color)> {
    if board[position].is_none() {
        return None;
    };
    let piecee = board[position].as_ref().unwrap().piece_type;
    let colorr = board[position].as_ref().unwrap().color;
    return Some((piecee,colorr));
}

// check if king is in check, by looking all possible squares where a piece would take it
pub fn king_in_check(board: &[Option<Piece>;64], my_color: Color, opp_color: Color) -> Option<bool> {

    let mut position = 100;
    // find the king
    for i in 0..64 {
        if get_piece_at(&&board, i) == Some((PieceType::King,my_color)) {
            position = i;
            break
        }
    }
    if position==100 {return None} // if these is no king, for some reason
    let scol = position%8;
    let srow = (position-(position%8))/8;

    // region: rook? (partially queen)
    if position%8!=7 { // right
        for i in position+1..(srow+1)*8 {
            match get_piece_at(&&board, i) {
                None => {},
                Some((PieceType::Rook,c)) if c==opp_color => return Some(true),
                Some((PieceType::Queen,c)) if c==opp_color => return Some(true),
                _ => break
            }
        }
    }
    if position%8!=0 { // left
        for i in (srow*8..=position-1).rev() {
            match get_piece_at(&&board, i) {
                None => {},
                Some((PieceType::Rook,c)) if c==opp_color => return Some(true),
                Some((PieceType::Queen,c)) if c==opp_color => return Some(true),
                _ => break
            }
        }
    }
    if position<56 { // down
        for i in (position+8..64).step_by(8) {
            match get_piece_at(&&board, i) {
                None => {},
                Some((PieceType::Rook,c)) if c==opp_color => return Some(true),
                Some((PieceType::Queen,c)) if c==opp_color => return Some(true),
                _ => break
            }
        }
    }
    if position>7 { // up
        for i in (scol..=position-8).rev().step_by(8) {
            match get_piece_at(&&board, i) {
                None => {},
                Some((PieceType::Rook,c)) if c==opp_color => return Some(true),
                Some((PieceType::Queen,c)) if c==opp_color => return Some(true),
                _ => break
            }
        }
    }
    // endregion

    // region: knight?
    if position>16 && position%8!=0 && get_piece_at(&&board, position-17) == Some((PieceType::Knight,opp_color)) {
        return Some(true)
    }
    if position>14 && position%8!=7 && get_piece_at(&&board, position-15) == Some((PieceType::Knight,opp_color)) {
        return Some(true)
    }  
    if position>9 && position%8!=0 && position%8!=1 && get_piece_at(&&board, position-10) == Some((PieceType::Knight,opp_color)) {
        return Some(true)
    }  
    if position>5 && position%8!=7 && position%8!=6 && get_piece_at(&&board, position-6) == Some((PieceType::Knight,opp_color)) {
        return Some(true)
    }  
    if position<56 && position%8!=1 && position%8!=0 && get_piece_at(&&board, position+6) == Some((PieceType::Knight,opp_color)) {
        return Some(true)
    }  
    if position<54 && position%8!=6 && position%8!=7 && get_piece_at(&&board, position+10) == Some((PieceType::Knight,opp_color)) {
        return Some(true)
    }
    if position<48 && position%8!=0 && get_piece_at(&&board, position+15) == Some((PieceType::Knight,opp_color)) {
        return Some(true)
    }
    if position<47 && position%8!=7 && get_piece_at(&&board, position+17) == Some((PieceType::Knight,opp_color)) {
        return Some(true)
    }
    // endregion 
    
    // region: bishop? (partially queen)
    if position<55 && position%8!=7 { //RD
        for i in (position+9..64).step_by(9) {
            let col = i%8;
            let row = (i-(i%8))/8;
            if (col as isize - scol as isize).abs() != (row as isize - srow as isize).abs() {break}
            match get_piece_at(&&board, i) {
                Some((PieceType::Bishop,c)) if c==opp_color => return Some(true),
                Some((PieceType::Queen,c)) if c==opp_color => return Some(true),
                None => {},
                _ => break,
            }
        }
    }
    if position<56 && position%8!=0 { //LD
        for i in (position+7..64).step_by(7) {
            let col = i%8;
            let row = (i-(i%8))/8;
            if (col as isize -scol as isize).abs() != (row as isize - srow as isize).abs() {break}
            match get_piece_at(&&board, i) {
                Some((PieceType::Bishop,c)) if c==opp_color => return Some(true),
                Some((PieceType::Queen,c)) if c==opp_color => return Some(true),
                None => {},
                _ => break,
            }
        }
    }
    if position>7 && position%8!=7 { //RU
        for j in (7..50).step_by(7) {
            if j>position {break}
            let i = position-j;
            let col = i%8;
            let row = (i-(i%8))/8;
            if (col as isize-scol as isize).abs() != (row as isize-srow as isize).abs() {break}
            match get_piece_at(&&board, i) {
                Some((PieceType::Bishop,c)) if c==opp_color => return Some(true),
                Some((PieceType::Queen,c)) if c==opp_color => return Some(true),
                None => {},
                _ => break,
            }
        }
    }
    if position>8 && position%8!=0 { //LU
        for j in (9..64).step_by(9) {
            if j>position {break}
            let i = position-j;
            let col = i%8;
            let row = (i-(i%8))/8;
            if (col as isize-scol as isize).abs() != (row as isize-srow as isize).abs() {break}
            match get_piece_at(&&board, i) {
                Some((PieceType::Bishop,c)) if c==opp_color => return Some(true),
                Some((PieceType::Queen,c)) if c==opp_color => return Some(true),
                None => {},
                _ => break,
            }
        }
    }
    // endregion

    // region: pawn?
    if my_color == Color::Black && position<56 {
        if position%8!=0 && get_piece_at(&board, position+7) == Some((PieceType::Pawn,Color::White)) {
            return Some(true)
        }
        if position%8!=7 && get_piece_at(&board, position+9) == Some((PieceType::Pawn,Color::White)) {
            return Some(true)
        }
    }
    if my_color == Color::White && position>7 {
        if position%8!=7 && get_piece_at(&&board, position-7) == Some((PieceType::Pawn,Color::Black)) {
            return Some(true)
        }
        if position%8!=0 && get_piece_at(&board, position-9) == Some((PieceType::Pawn,Color::Black)) {
            return Some(true)
        }
    }
    // endregion
    
    // region: king?
    if position%8!=7 {
        if get_piece_at(&&board, position+1)==Some((PieceType::King,opp_color)) {return Some(true)}
        if srow!=0 && get_piece_at(&board, position-7)==Some((PieceType::King,opp_color)) {return Some(true)}
        if srow!=7 && get_piece_at(&&board, position+9)==Some((PieceType::King,opp_color)) {return Some(true)}
    }
    
    if position%8!=0 {
        if get_piece_at(&&board, position-1)==Some((PieceType::King,opp_color)) {return Some(true)}
        if srow!=0 && get_piece_at(&board, position-9)==Some((PieceType::King,opp_color)) {return Some(true)}
        if srow!=7 && get_piece_at(&&board, position+7)==Some((PieceType::King,opp_color)) {return Some(true)}
    }

    if srow!=0 && get_piece_at(&board, position-8)==Some((PieceType::King,opp_color)) {return Some(true)}
    if srow!=7 && get_piece_at(&&board, position+8)==Some((PieceType::King,opp_color)) {return Some(true)}
    // endregion

    return Some(false)
}

pub fn is_move_legal(board: &mut[Option<Piece>;64], from:usize, to:usize, my_color: Color, opp_color: Color) -> bool {
    // temporary copy of board, so we can try if a move is legal
    // is there a piece on the from-square
    if board[from].is_none() {return false}
    let square1 = board[from];
    let square2 = board[to];
    board[to]=board[from];
    board[from]=None;
    // after moved piece, if king in check then illegal move
    if king_in_check(&board, my_color, opp_color)==Some(true) {
        board[from]=square1; board[to]=square2;
        return false
    }
    else {
        board[from]=square1; board[to]=square2;
        return true
    }
}

// return what the legal moves for a piece is
// unsigned integer, dynamic, array indices is usize
pub fn legal_moves(mut board: [Option<Piece>;64], position: usize) -> Vec<usize> {
    let mut moves = vec![];
    //if get_piece_at gives none then just return empty vector
    //else piece and color are what we got from the function
    let (piece, color) = match get_piece_at(&board, position) {
        Some(tup) => tup,
        None => return moves
    };
    // current position
    let scol = position%8;
    let srow = (position-(position%8))/8;

    match color { // divide up to if its a black or white piece
        Color::Black => { 
            match piece { // for each piece it has different possible moves
                PieceType::Rook => {
                    if position%8!=7 { // go left (right)
                        for i in position+1..(srow+1)*8 {
                            match get_piece_at(&board, i) {
                                None => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true {moves.push(i)},
                                Some((_,Color::White)) => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true {moves.push(i);break},
                                Some((_,Color::Black)) => {break}
                            }
                            
                        }
                    }
                    // go right (left)
                    if position%8!=0 {
                        for i in (srow*8..=position-1).rev() {                       
                            match get_piece_at(&board, i) {
                                None => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true {moves.push(i)},
                                Some((_,Color::White)) => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true {moves.push(i);break},
                                Some((_,Color::Black)) => {break}
                            }                           
                        }
                    }
                    // go fram (ner)
                    if position<56 {
                        for i in (position+8..64).step_by(8) {                           
                            match get_piece_at(&board, i) {
                                None => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i)},
                                Some((_,Color::White)) => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i);break},
                                Some((_,Color::Black)) => {break}
                            }
                        }
                    }
                    // go bak (upp)
                    if position>7 {
                        for i in (scol..=position-8).rev().step_by(8) {                         
                            match get_piece_at(&board, i) {
                                None => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i)},
                                Some((_,Color::White)) => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i);break},
                                Some((_,Color::Black)) => {break}
                            }
                        }
                    }
                },
                PieceType::Knight => { 
                    // eight possible moves DDL,DDR,DLL,DRR,ULL,URR,UUL,UUR (Left,Right,Down,Up)
                    // as long as it will not go outside the &board and the piece there is not black it can go there
                    // DDL
                    
                    if position>15 && position%8!=7 && !matches! (get_piece_at(&board, position-15),Some((_,Color::Black))) {                      
                        if is_move_legal(&mut board, position,position-15,Color::Black,Color::White)==true {
                            moves.push(position-15)   
                        }
                    }
                    // DDR
                    if position>16 && position%8!=0 && !matches! (get_piece_at(&board, position-17),Some((_,Color::Black))) {
                        if is_move_legal(&mut board, position,position-17,Color::Black,Color::White)==true {
                            moves.push(position-17)
                        }
                    }
                    // DLL
                    if position>7 && position%8!=6 && position%8!=7 && !matches! (get_piece_at(&board, position-6),Some((_,Color::Black))) {
                        if is_move_legal(&mut board, position,position-6,Color::Black,Color::White)==true {
                            moves.push(position-6)
                        }
                    }
                    // DRR
                    if position>9 && position%8!=0 && position%8!=1 && !matches! (get_piece_at(&board, position-10),Some((_,Color::Black))) {
                        if is_move_legal(&mut board, position,position-10,Color::Black,Color::White)==true {
                            moves.push(position-10)
                        }
                    }

                    // UUL
                    if position<47 && position%8!=7 && !matches! (get_piece_at(&board, position+17),Some((_,Color::Black))) {
                        if is_move_legal(&mut board, position,position+17,Color::Black,Color::White)==true {
                            moves.push(position+17)
                        }
                    }
                    // UUR
                    if position<48 && position%8!=0 && !matches! (get_piece_at(&board, position+15),Some((_,Color::Black))) {
                        if is_move_legal(&mut board, position,position+15,Color::Black,Color::White)==true {
                            moves.push(position+15)
                        }
                    }
                    // ULL
                    if position<54 && position%8!=6 && position%8!=7 && !matches! (get_piece_at(&board, position+10),Some((_,Color::Black))) {
                        if is_move_legal(&mut board, position,position+10,Color::Black,Color::White)==true {
                            moves.push(position+10)
                        }
                    }
                    // URR
                    if position<56 && position%8!=0 && position%8!=1 && !matches! (get_piece_at(&board, position+6),Some((_,Color::Black))) {
                        if is_move_legal(&mut board, position,position+6,Color::Black,Color::White)==true {
                            moves.push(position+6)
                        }
                    }
                },
                PieceType::Bishop => {
                    // go LU
                    if position<55 && position%8!=7 {
                        for i in (position+9..64).step_by(9) {
                            let col = i%8;
                            let row = (i-(i%8))/8;
                            // usize cant be negative
                            if (col as isize - scol as isize).abs() != (row as isize - srow as isize).abs() {break}
                            match get_piece_at(&board, i) {
                                Some((_,Color::White)) => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i);break},
                                Some((_,Color::Black)) => break,
                                None => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i)}
                            }
                        }
                    }
                    // go RU
                    if position<56 && position%8!=0 {
                        for i in (position+7..64).step_by(7) {                          
                            let col = i%8;
                            let row = (i-(i%8))/8;
                            if (col as isize -scol as isize).abs() != (row as isize - srow as isize).abs() {break}
                            match get_piece_at(&board, i) {
                                Some((_,Color::White)) => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i);break},
                                Some((_,Color::Black)) => break,
                                None => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i)}
                            }
                        }
                    }
                    // go LD
                    if position>7 && position%8!=7 {
                        for j in (7..50).step_by(7) {
                            if j>position {break}
                            let i = position-j;
                            let col = i%8;
                            let row = (i-(i%8))/8;
                            if (col as isize-scol as isize).abs() != (row as isize-srow as isize).abs() {break}
                            match get_piece_at(&board, i) {
                                Some((_,Color::White)) => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i);break},
                                Some((_,Color::Black)) => break,
                                None => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i)}
                            }                           
                        }
                    }
                    // go RD
                    if position>8 && position%8!=0 {
                        for j in (9..64).step_by(9) {
                            if j>position {break}
                            let i = position-j;
                            let col = i%8;
                            let row = (i-(i%8))/8;
                            if (col as isize-scol as isize).abs() != (row as isize-srow as isize).abs() {break}
                            match get_piece_at(&board, i) {
                                Some((_,Color::White)) => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i);break},
                                Some((_,Color::Black)) => break,
                                None => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i)}
                            }
                        }
                    }
                },
                PieceType::Queen => {
                    // queen is basically rook and bishop

                    // rook
                    if position%8!=7 { // go left (right)
                        for i in position+1..(srow+1)*8 {
                            match get_piece_at(&board, i) {
                                None => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true {moves.push(i)},
                                Some((_,Color::White)) => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true {moves.push(i);break},
                                Some((_,Color::Black)) => {break}
                            }
                            
                        }
                    }
                    // go right (left)
                    if position%8!=0 {
                        for i in (srow*8..=position-1).rev() {                       
                            match get_piece_at(&board, i) {
                                None => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true {moves.push(i)},
                                Some((_,Color::White)) => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true {moves.push(i);break},
                                Some((_,Color::Black)) => {break}
                            }                           
                        }
                    }
                    // go fram (ner)
                    if position<56 {
                        for i in (position+8..64).step_by(8) {                           
                            match get_piece_at(&board, i) {
                                None => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i)},
                                Some((_,Color::White)) => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i);break},
                                Some((_,Color::Black)) => {break}
                            }
                        }
                    }
                    // go bak (upp)
                    if position>7 {
                        for i in (scol..=position-8).rev().step_by(8) {                         
                            match get_piece_at(&board, i) {
                                None => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i)},
                                Some((_,Color::White)) => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i);break},
                                Some((_,Color::Black)) => {break}
                            }
                        }
                    }
                    
                    // bishop
                    // go LU
                    if position<55 && position%8!=7 {
                        for i in (position+9..64).step_by(9) {
                            let col = i%8;
                            let row = (i-(i%8))/8;
                            // usize cant be negative
                            if (col as isize - scol as isize).abs() != (row as isize - srow as isize).abs() {break}
                            match get_piece_at(&board, i) {
                                Some((_,Color::White)) => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i);break},
                                Some((_,Color::Black)) => break,
                                None => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i)}
                            }
                        }
                    }
                    // go RU
                    if position<56 && position%8!=0 {
                        for i in (position+7..64).step_by(7) {                          
                            let col = i%8;
                            let row = (i-(i%8))/8;
                            if (col as isize -scol as isize).abs() != (row as isize - srow as isize).abs() {break}
                            match get_piece_at(&board, i) {
                                Some((_,Color::White)) => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i);break},
                                Some((_,Color::Black)) => break,
                                None => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i)}
                            }
                        }
                    }
                    // go LD
                    if position>7 && position%8!=7 {
                        for j in (7..50).step_by(7) {
                            if j>position {break}
                            let i = position-j;
                            let col = i%8;
                            let row = (i-(i%8))/8;
                            if (col as isize-scol as isize).abs() != (row as isize-srow as isize).abs() {break}
                            match get_piece_at(&board, i) {
                                Some((_,Color::White)) => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i);break},
                                Some((_,Color::Black)) => break,
                                None => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i)}
                            }                           
                        }
                    }
                    // go RD
                    if position>8 && position%8!=0 {
                        for j in (9..64).step_by(9) {
                            if j>position {break}
                            let i = position-j;
                            let col = i%8;
                            let row = (i-(i%8))/8;
                            if (col as isize-scol as isize).abs() != (row as isize-srow as isize).abs() {break}
                            match get_piece_at(&board, i) {
                                Some((_,Color::White)) => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i);break},
                                Some((_,Color::Black)) => break,
                                None => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i)}
                            }
                        }
                    }
                },
                PieceType::King => {
                    // go bak
                    if position>=8 {
                        if is_move_legal(&mut board, position,position-8,Color::Black,Color::White)==true {
                            match get_piece_at(&board, position-8) {
                                // if there is no piece or a white piece then it can go there
                                None | Some((_,Color::White)) => moves.push(position-8),
                                _ => {} // everything else, so basically when color is black
                            }
                        }
                    }
                    // go fram
                    if position<=55 {
                        if is_move_legal(&mut board, position,position+8,Color::Black,Color::White)==true {
                            match get_piece_at(&board, position+8) {
                                None | Some((_,Color::White)) => moves.push(position+8),
                                _ => {} 
                            }
                        }
                    }
                    // go right(left)
                    if position%8!=0 {
                        if is_move_legal(&mut board, position,position-1,Color::Black,Color::White)==true {
                            match get_piece_at(&board, position-1) {
                                None | Some((_,Color::White)) => moves.push(position-1),
                                _ => {} 
                            }
                        }
                    }
                    // go left(right)
                    if position%8!=7 {
                        if is_move_legal(&mut board, position,position+1,Color::Black,Color::White)==true {
                            match get_piece_at(&board, position+1) {
                                None | Some((_,Color::White)) => moves.push(position+1),
                                _ => {} 
                            }
                        }
                    }

                    // bak vänster
                    if position>7&&position%8!=7 {
                        if is_move_legal(&mut board, position,position-7,Color::Black,Color::White)==true {
                            match get_piece_at(&board, position-7) {
                                None | Some((_,Color::White)) => moves.push(position-7),
                                _ => {} 
                            }
                        }
                    }
                    // bak höger
                    if position>7&&position%8!=0 {
                        if is_move_legal(&mut board, position,position-9,Color::Black,Color::White)==true {
                            match get_piece_at(&board, position-9) {
                                None | Some((_,Color::White)) => moves.push(position-9),
                                _ => {} 
                            }
                        }
                    }
                    // fram vänster
                    if position<56&&position%8!=7 {
                        if is_move_legal(&mut board, position,position+9,Color::Black,Color::White)==true {
                            match get_piece_at(&board, position+9) {
                                None | Some((_,Color::White)) => moves.push(position+9),
                                _ => {} 
                            }
                        }
                    }
                    // fram höger
                    if position<56&&position%8!=0 {
                        if is_move_legal(&mut board, position,position+7,Color::Black,Color::White)==true {
                            match get_piece_at(&board, position+7) {
                                None | Some((_,Color::White)) => moves.push(position+7),
                                _ => {} 
                            }
                        }
                    }
                },
                PieceType::Pawn => {
                    // two step
                    if (position>7&&position<16) && get_piece_at(&board, position+8).is_none() && get_piece_at(&board, position+16).is_none() {
                        if is_move_legal(&mut board, position,position+16,Color::Black,Color::White)==true {
                            moves.push(position+16)
                        }
                    }
                    // one step
                    if position<56 && get_piece_at(&board, position+8).is_none() {
                        if is_move_legal(&mut board, position,position+8,Color::Black,Color::White)==true {
                            moves.push(position+8)
                        }
                    }

                    // take another piece, left
                    if position<56 && position%8!=7 && matches!(get_piece_at(&board, position+9),Some((_,Color::White))) {
                        if is_move_legal(&mut board, position,position+9,Color::Black,Color::White)==true {
                            moves.push(position+9)
                        }
                    }
                    // take another piece, right
                    if position<56 && position%8!=0 && matches!(get_piece_at(&board, position+7),Some((_,Color::White))) {
                        if is_move_legal(&mut board, position,position+7,Color::Black,Color::White)==true {
                            moves.push(position+7)
                        }
                    }
                }
            }
        }
        Color::White => {
            match piece { // for each piece it has different possible moves
                PieceType::Rook => {
                    // go right
                    if position%8!=7 {
                        for i in position+1..(srow+1)*8 {
                            match get_piece_at(&board, i) {
                                None => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i)},
                                Some((_,Color::Black)) => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i);break},
                                Some((_,Color::White)) => {break}
                            }
                        }
                    }
                    // go left
                    if position%8!=0 {
                        for i in (srow*8..=position-1).rev() {
                            match get_piece_at(&board, i) {
                                None => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i)},
                                Some((_,Color::Black)) => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i);break},
                                Some((_,Color::White)) => {break}
                            }
                        }
                    }
                    // go ner
                    if position<56 {
                        for i in (position+8..64).step_by(8) {
                            match get_piece_at(&board, i) {
                                None => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i)},
                                Some((_,Color::Black)) => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i);break},
                                Some((_,Color::White)) => {break}
                            }
                        }
                    }
                    // go upp
                    if position>7 {
                        for i in (scol..=position-8).rev().step_by(8) {
                            match get_piece_at(&board, i) {
                                None => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i)},
                                Some((_,Color::Black)) => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i);break},
                                Some((_,Color::White)) => {break}
                            }
                        }
                    }
                },
                PieceType::Knight => { 
                    // eight possible moves DDL,DDR,DLL,DRR,ULL,URR,UUL,UUR (Left,Right,Down,Up)
                    // as long as it will not go outside the &board and the piece there is not white it can go there
                    // UUR
                    if position>15 && position%8!=7 && !matches! (get_piece_at(&board, position-15),Some((_,Color::White))) {                      
                        if is_move_legal(&mut board, position,position-15,Color::White,Color::Black)==true {
                            moves.push(position-15)   
                        }
                    }
                    // UUL
                    if position>16 && position%8!=0 && !matches! (get_piece_at(&board, position-17),Some((_,Color::White))) {
                        if is_move_legal(&mut board, position,position-17,Color::White,Color::Black)==true {   
                            moves.push(position-17)
                        }
                    }
                    // URR
                    if position>7 && position%8!=6 && position%8!=7 && !matches! (get_piece_at(&board, position-6),Some((_,Color::White))) {
                        if is_move_legal(&mut board, position,position-6,Color::White,Color::Black)==true {
                            moves.push(position-6)
                        }
                    }
                    // ULL
                    if position>9 && position%8!=0 && position%8!=1 && !matches! (get_piece_at(&board, position-10),Some((_,Color::White))) {
                        if is_move_legal(&mut board, position,position-10,Color::White,Color::Black)==true {
                            moves.push(position-10)
                        }
                    }

                    // DDR
                    if position<47 && position%8!=7 && !matches! (get_piece_at(&board, position+17),Some((_,Color::White))) {
                        if is_move_legal(&mut board, position,position+17,Color::White,Color::Black)==true {
                            moves.push(position+17)
                        }
                    }
                    // DDL
                    if position<48 && position%8!=0 && !matches! (get_piece_at(&board, position+15),Some((_,Color::White))) {
                        if is_move_legal(&mut board, position,position+15,Color::White,Color::Black)==true {
                            moves.push(position+15)
                        }
                    }
                    // DRR
                    if position<54 && position%8!=6 && position%8!=7 && !matches! (get_piece_at(&board, position+10),Some((_,Color::White))) {
                        if is_move_legal(&mut board, position,position+10,Color::White,Color::Black)==true {    
                            moves.push(position+10)
                        }
                    }
                    // DLL
                    if position<56 && position%8!=0 && position%8!=1 && !matches! (get_piece_at(&board, position+6),Some((_,Color::White))) {
                        if is_move_legal(&mut board, position,position+6,Color::White,Color::Black)==true {
                            moves.push(position+6)
                        }
                    }
                },
                PieceType::Bishop => {
                    // go RD
                    if position<55 && position%8!=7 {
                        for i in (position+9..64).step_by(9) {
                            let col = i%8;
                            let row = (i-(i%8))/8;
                            // usize cant be negative
                            if (col as isize - scol as isize).abs() != (row as isize - srow as isize).abs() {break}
                            match get_piece_at(&board, i) {
                                Some((_,Color::Black)) => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i);break},
                                Some((_,Color::White)) => break,
                                None => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i)}
                            }
                        }
                    }
                    // go LD
                    if position<56 && position%8!=0 {
                        for i in (position+7..64).step_by(7) {
                            let col = i%8;
                            let row = (i-(i%8))/8;
                            if (col as isize -scol as isize).abs() != (row as isize - srow as isize).abs() {break}
                            match get_piece_at(&board, i) {
                                Some((_,Color::Black)) => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i);break},
                                Some((_,Color::White)) => break,
                                None => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i)}
                            }
                        }
                    }
                    // go RU
                    if position>7 && position%8!=7 {
                        for j in (7..50).step_by(7) {
                            if j>position {break}
                            let i = position-j;
                            let col = i%8;
                            let row = (i-(i%8))/8;
                            if (col as isize-scol as isize).abs() != (row as isize-srow as isize).abs() {break}
                            match get_piece_at(&board, i) {
                                Some((_,Color::Black)) => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i);break},
                                Some((_,Color::White)) => break,
                                None => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i)}
                            }
                        }
                    }
                    // go LU
                    if position>8 && position%8!=0 {
                        for j in (9..64).step_by(9) {
                            if j>position {break}
                            let i = position-j;
                            let col = i%8;
                            let row = (i-(i%8))/8;
                            if (col as isize-scol as isize).abs() != (row as isize-srow as isize).abs() {break}
                            match get_piece_at(&board, i) {
                                Some((_,Color::Black)) => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i);break},
                                Some((_,Color::White)) => break,
                                None => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i)}
                            }
                        }
                    }
                },
                PieceType::Queen => {
                    // queen is basically rook and bishop

                    // rook
                    // go right
                    if position%8!=7 {
                        for i in position+1..(srow+1)*8 {
                            match get_piece_at(&board, i) {
                                None => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i)},
                                Some((_,Color::Black)) => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i);break},
                                Some((_,Color::White)) => {break}
                            }
                        }
                    }
                    // go left
                    if position%8!=0 {
                        for i in (srow*8..=position-1).rev() {
                            match get_piece_at(&board, i) {
                                None => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i)},
                                Some((_,Color::Black)) => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i);break},
                                Some((_,Color::White)) => {break}
                            }
                        }
                    }
                    // go ner
                    if position<56 {
                        for i in (position+8..64).step_by(8) {
                            match get_piece_at(&board, i) {
                                None => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i)},
                                Some((_,Color::Black)) => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i);break},
                                Some((_,Color::White)) => {break}
                            }
                        }
                    }
                    // go upp
                    if position>7 {
                        for i in (scol..=position-8).rev().step_by(8) {
                            match get_piece_at(&board, i) {
                                None => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i)},
                                Some((_,Color::Black)) => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i);break},
                                Some((_,Color::White)) => {break}
                            }
                        }
                    }

                    // bishop
                    // go RD
                    if position<55 && position%8!=7 {
                        for i in (position+9..64).step_by(9) {
                            let col = i%8;
                            let row = (i-(i%8))/8;
                            // usize cant be negative
                            if (col as isize - scol as isize).abs() != (row as isize - srow as isize).abs() {break}
                            match get_piece_at(&board, i) {
                                Some((_,Color::Black)) => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i);break},
                                Some((_,Color::White)) => break,
                                None => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i)}
                            }
                        }
                    }
                    // go LD
                    if position<56 && position%8!=0 {
                        for i in (position+7..64).step_by(7) {
                            let col = i%8;
                            let row = (i-(i%8))/8;
                            if (col as isize -scol as isize).abs() != (row as isize - srow as isize).abs() {break}
                            match get_piece_at(&board, i) {
                                Some((_,Color::Black)) => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i);break},
                                Some((_,Color::White)) => break,
                                None => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i)}
                            }
                        }
                    }
                    // go RU
                    if position>7 && position%8!=7 {
                        for j in (7..50).step_by(7) {
                            if j>position {break}
                            let i = position-j;
                            let col = i%8;
                            let row = (i-(i%8))/8;
                            if (col as isize-scol as isize).abs() != (row as isize-srow as isize).abs() {break}
                            match get_piece_at(&board, i) {
                                Some((_,Color::Black)) => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i);break},
                                Some((_,Color::White)) => break,
                                None => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i)}
                            }
                        }
                    }
                    // go LU
                    if position>8 && position%8!=0 {
                        for j in (9..64).step_by(9) {
                            if j>position {break}
                            let i = position-j;
                            let col = i%8;
                            let row = (i-(i%8))/8;
                            if (col as isize-scol as isize).abs() != (row as isize-srow as isize).abs() {break}
                            match get_piece_at(&board, i) {
                                Some((_,Color::Black)) => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i);break},
                                Some((_,Color::White)) => break,
                                None => if is_move_legal(&mut board, position,i,Color::Black,Color::White)==true{moves.push(i)}
                            }
                        }
                    }
                },
                PieceType::King => {
                    // go fram
                    if position>=8 {
                        if is_move_legal(&mut board, position,position-8,Color::White,Color::Black)==true {
                            match get_piece_at(&board, position-8) {
                                // if there is no piece or a black piece then it can go there
                                None | Some((_,Color::Black)) => moves.push(position-8),
                                _ => {} // everything else, so basically when color is white
                            }
                        }
                    }
                    // go bak
                    if position<=55 {
                        if is_move_legal(&mut board, position,position+8,Color::White,Color::Black)==true {
                            match get_piece_at(&board, position+8) {
                                None | Some((_,Color::Black)) => moves.push(position+8),
                                _ => {} 
                            }
                        }
                    }
                    // go left
                    if position%8!=0 {
                        if is_move_legal(&mut board, position,position-1,Color::White,Color::Black)==true {
                            match get_piece_at(&board, position-1) {
                                None | Some((_,Color::Black)) => moves.push(position-1),
                                _ => {} 
                            }
                        }
                    }
                    // go right
                    if position%8!=7 {
                        if is_move_legal(&mut board, position,position+1,Color::White,Color::Black)==true {
                            match get_piece_at(&board, position+1) {
                                None | Some((_,Color::Black)) => moves.push(position+1),
                                _ => {} 
                            }
                        }
                    }

                    // bak höger
                    if position>7&&position%8!=7 {
                        if is_move_legal(&mut board, position,position-7,Color::White,Color::Black)==true {
                            match get_piece_at(&board, position-7) {
                                None | Some((_,Color::Black)) => moves.push(position-7),
                                _ => {} 
                            }
                        }
                    }
                    // bak vänster
                    if position>7&&position%8!=0 {
                        if is_move_legal(&mut board, position,position-9,Color::White,Color::Black)==true {
                            match get_piece_at(&board, position-9) {
                                None | Some((_,Color::Black)) => moves.push(position-9),
                                _ => {} 
                            }
                        }
                    }
                    // fram höger
                    if position<56&&position%8!=7 {
                        if is_move_legal(&mut board, position,position+9,Color::White,Color::Black)==true {
                            match get_piece_at(&board, position+9) {
                                None | Some((_,Color::Black)) => moves.push(position+9),
                                _ => {} 
                            }
                        }
                    }
                    // fram vänster
                    if position<56&&position%8!=0 {
                        if is_move_legal(&mut board, position,position+7,Color::White,Color::Black)==true {
                            match get_piece_at(&board, position+7) {
                                None | Some((_,Color::Black)) => moves.push(position+7),
                                _ => {} 
                            }
                        }
                    }
                },
                PieceType::Pawn => {
                    // two step
                    if (position>47&&position<56) && get_piece_at(&board, position-8).is_none() && get_piece_at(&board, position-16).is_none() {
                        if is_move_legal(&mut board, position,position-16,Color::White,Color::Black)==true {
                            moves.push(position-16)
                        }
                    }
                    // one step
                    if position>7 && get_piece_at(&board, position-8).is_none() {
                        if is_move_legal(&mut board, position,position-8,Color::White,Color::Black)==true {  
                            moves.push(position-8)
                        }
                    }
                    // take another piece, left
                    if position>7 && position%8!=0 && matches!(get_piece_at(&board, position-9),Some((_,Color::Black))) {
                        if is_move_legal(&mut board, position,position-9,Color::White,Color::Black)==true {
                            moves.push(position-9)
                        }
                    }
                    // take another piece, right
                    if position>7 && position%8!=7 && matches!(get_piece_at(&board, position-7),Some((_,Color::Black))) {
                        if is_move_legal(&mut board, position,position-7,Color::White,Color::Black)==true {
                            moves.push(position-7)
                        }
                    }
                }
            }
        }
    
    }
    return moves;
}

// move a piece from a square to another
pub fn move_piece(from:usize, to:usize, board:&mut[Option<Piece>;64]) {
    board[to] = board[from];
    board[from] = None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test(){
        let mut chess_board: [Option<Piece>; 64] = [
        Some(Piece { piece_type: PieceType::Rook, color: Color::Black }), Some(Piece { piece_type: PieceType::Knight, color: Color::Black }), Some(Piece { piece_type: PieceType::Bishop, color: Color::Black }), Some(Piece { piece_type: PieceType::Queen, color: Color::Black }), Some(Piece { piece_type: PieceType::King, color: Color::Black }), Some(Piece { piece_type: PieceType::Bishop, color: Color::Black }), Some(Piece { piece_type: PieceType::Knight, color: Color::Black }), Some(Piece { piece_type: PieceType::Rook, color: Color::Black }), 
        Some(Piece { piece_type: PieceType::Pawn, color: Color::Black }), Some(Piece { piece_type: PieceType::Pawn, color: Color::Black }), Some(Piece { piece_type: PieceType::Pawn, color: Color::Black }), Some(Piece { piece_type: PieceType::Pawn, color: Color::Black }), Some(Piece { piece_type: PieceType::Pawn, color: Color::Black }), Some(Piece { piece_type: PieceType::Pawn, color: Color::Black }), Some(Piece { piece_type: PieceType::Pawn, color: Color::Black }), Some(Piece { piece_type: PieceType::Pawn, color: Color::Black }), 
        Some(Piece { piece_type: PieceType::Knight, color: Color::Black }), None, None, None, None, None, None, None, 
        None, None, None, None, None, None, None, None, 
        None, None, None, None, None, None, None, None, 
        None, None, None, None, None, None, None, None, 
        Some(Piece { piece_type: PieceType::Pawn, color: Color::White }), Some(Piece { piece_type: PieceType::Pawn, color: Color::White }), Some(Piece { piece_type: PieceType::Pawn, color: Color::White }), Some(Piece { piece_type: PieceType::Pawn, color: Color::White }), Some(Piece { piece_type: PieceType::Pawn, color: Color::White }), Some(Piece { piece_type: PieceType::Pawn, color: Color::White }), Some(Piece { piece_type: PieceType::Pawn, color: Color::White }), Some(Piece { piece_type: PieceType::Pawn, color: Color::White }),
        Some(Piece { piece_type: PieceType::Rook, color: Color::White }), Some(Piece { piece_type: PieceType::Knight, color: Color::White }), Some(Piece { piece_type: PieceType::Bishop, color: Color::White }), Some(Piece { piece_type: PieceType::Queen, color: Color::White }), Some(Piece { piece_type: PieceType::King, color: Color::White }), Some(Piece { piece_type: PieceType::Bishop, color: Color::White }), Some(Piece { piece_type: PieceType::Knight, color: Color::White }), Some(Piece { piece_type: PieceType::Rook, color: Color::White }), 
        ];
        let res = get_piece_at(&chess_board, 15);
        print!("{:?}",res);
        let movee = legal_moves(chess_board,15);
        print!("{:?}",movee)
    }
}