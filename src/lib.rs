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
pub enum Error { 
    IllegalMove,
    OutOfBound,
    IllegalPromotion,
}

#[derive(Debug,PartialEq,Eq,Clone,Copy)]
pub enum GameStatus { 
    Ongoing,
    Checkmate,
    Stalemate,
    Check
}

#[derive(Debug,PartialEq,Eq,Clone,Copy)]
pub struct Piece { 
    pub piece_type: PieceType,
    pub color: Color,
}

#[derive(Debug,PartialEq,Eq,Clone,Copy)]
pub struct Chess {
    pub board: [Option<Piece>;64],
    pub turn: Color,
    pub opposite: Color,
    // if castling has happened
    pub r_white_castle: bool,
    pub l_white_castle: bool,
    pub r_black_castle: bool,
    pub l_black_castle: bool,
    pub en_passant: Option<usize>,
}

impl Chess {
    // function to start new game
    pub fn new() -> Chess {
        Chess {
            board: new_board(),
            turn: Color::White,
            opposite: Color::Black,
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
            self.turn = Color::White;
            self.opposite = Color::Black
        }
        else {
            self.turn = Color::Black;
            self.opposite = Color::White
        }
    }
    
    // move a piece from a square to another, paramters are: from, to, and promotion if relevant
    pub fn move_piece(&mut self, from:usize, to:usize, promotion:Option<PieceType>) -> Result<(), Error>{
        // check if out of bound
        if from>63 || to>63 {
            return Err(Error::OutOfBound)
        }
        
        // check so this is a legal move
        if self.legal_moves(from).contains(&to)  {
            let square = get_piece_at(&self.board, from);

            // check so promotion is legal before checking castling
            if square == Some((PieceType::Pawn,self.turn)) && ((from>=8 && from<=15 && to<=7) || (from>=48 && from<=55 && to >=56 && to <= 63)) {
                match promotion {
                    Some(PieceType::Bishop) | Some(PieceType::Queen) | Some(PieceType::Knight) | Some(PieceType::Rook) => {},
                    _ => {return Err(Error::IllegalPromotion)}
                } 
            }

            // castling
            if square == Some((PieceType::King,self.turn)) {
                // moves the rook, king is moved further down
                if from==60 && to==62 {
                    self.board[61] = self.board[63];
                    self.board[63] = None;
                }
                else if from==60 && to==58 {
                    self.board[59] = self.board[56];
                    self.board[56] = None;
                }
                else if from==4 && to==6 {
                    self.board[5] = self.board[7];
                    self.board[7] = None;
                }
                else if from==4 && to==2 {
                    self.board[3] = self.board[0];
                    self.board[0] = None;
                }

                if self.turn == Color::Black {self.r_black_castle=false;self.l_black_castle=false}
                else {self.r_white_castle=false;self.l_white_castle=false}
            }
            else if square == Some((PieceType::Rook,self.turn)) {
                if from==56 {self.l_white_castle=false}
                else if from==63 {self.r_white_castle=false}
                else if from==7 {self.l_black_castle=false}
                else if from==0 {self.r_black_castle=false}
            }
            // if a rook is captured then castling cannot happen
            if get_piece_at(&self.board, to) == Some((PieceType::Rook,self.opposite)) {
                if self.opposite == Color::White {
                    if to==63 {self.r_white_castle=false}
                    else if to==56 {self.l_white_castle=false}
                }
                else {
                    if to==7 {self.l_black_castle=false}
                    else if to==0 {self.r_black_castle=false}
                }
            }

            // if a pawn moves two steps
            if square == Some((PieceType::Pawn,self.turn)) && (to==from+16 || from>=16 && to==from-16) {
                self.en_passant = Some(to)
            }
            else {
                self.en_passant = None
            }
            
            // en passant
            if square == Some((PieceType::Pawn,self.turn)) && (from>=7 && to==from-7 || from>=9 && to==from-9 || to==from+9 || to==from+7) && get_piece_at(&self.board, to)==None {
                self.board[to]=self.board[from];
                self.board[from]=None;
                self.board[to-8]=None;
                self.next_turn();
                return Ok(());
            }
        
            // promotion
            if square == Some((PieceType::Pawn,self.turn)) && ((from>=8 && from<=15 && to<=7) || (from>=48 && from<=55 && to >=56 && to <= 63)) {
                if promotion == Some(PieceType::Bishop) || promotion == Some(PieceType::Queen) || promotion == Some(PieceType::Knight) || promotion == Some(PieceType::Rook) {
                    // if white promotion
                    if from>=8 && from<=15 && to<=7 {
                        self.board[from] = None;
                        self.board[to] = Some(Piece { piece_type: promotion.unwrap(), color: Color::White });
                    }
                    // if black promotion
                    else if from>=48 && from<=55 && to >=56 && to <= 63 {
                        self.board[from] = None;
                        self.board[to] = Some(Piece { piece_type: promotion.unwrap(), color: Color::Black });
                    }
                    else {
                        return Err(Error::IllegalPromotion);
                    }
                    self.next_turn();
                    return Ok(());
                }
            }
        
            self.board[to] = self.board[from];
            self.board[from] = None;
            self.next_turn();
            return Ok(());

        }
        else {
            return Err(Error::IllegalMove);
        }
    }

    // check legal moves for a piece
    pub fn legal_moves(&self, position: usize) -> Vec<usize> {
        let mut board = self.board;
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
        if color != self.turn {return moves}

        match piece { // for each piece it has different possible moves
            PieceType::Rook | PieceType::Queen => {
                if position%8!=7 { // go right
                    for i in position+1..(srow+1)*8 {
                        match get_piece_at(&board, i) {
                            None => if is_move_legal(&mut board, position,i,self.turn,self.opposite)==true {moves.push(i)},
                            Some((_,color)) => {
                                if color==self.opposite && is_move_legal(&mut board, position,i,self.turn,self.opposite)==true {moves.push(i)};
                                break                           
                            }
                        }
                        
                    }
                }
                if position%8!=0 { // go left
                    for i in (srow*8..=position-1).rev() {                       
                        match get_piece_at(&board, i) {
                            None => if is_move_legal(&mut board, position,i,self.turn,self.opposite)==true {moves.push(i)},
                            Some((_,color)) => {
                                if color==self.opposite && is_move_legal(&mut board, position,i,self.turn,self.opposite)==true {moves.push(i)};
                                break                           
                            }
                        }                           
                    }
                }
                if position<56 { // go down
                    for i in (position+8..64).step_by(8) {                           
                        match get_piece_at(&board, i) {
                            None => if is_move_legal(&mut board, position,i,self.turn,self.opposite)==true {moves.push(i)},
                            Some((_,color)) => {
                                if color==self.opposite && is_move_legal(&mut board, position,i,self.turn,self.opposite)==true {moves.push(i)};
                                break                           
                            }
                        }
                    }
                }
                if position>7 { // go up
                    for i in (scol..=position-8).rev().step_by(8) {                         
                        match get_piece_at(&board, i) {
                            None => if is_move_legal(&mut board, position,i,self.turn,self.opposite)==true {moves.push(i)},
                            Some((_,color)) => {
                                if color==self.opposite && is_move_legal(&mut board, position,i,self.turn,self.opposite)==true {moves.push(i)};
                                break                           
                            }
                        }
                    }
                }

            },
            PieceType::Knight => { 
                // eight possible moves DDL,DDR,DLL,DRR,ULL,URR,UUL,UUR (Left,Right,Down,Up)
                // as long as it will not go outside the board and the piece there is not same color it can go there
                if position>15 && position%8!=7 && !matches! (get_piece_at(&board, position-15),Some((_,color)) if color==self.turn) {                      
                    if is_move_legal(&mut board, position,position-15,self.turn,self.opposite)==true {
                        moves.push(position-15)   
                    } // UUR
                }
                if position>16 && position%8!=0 && !matches! (get_piece_at(&board, position-17),Some((_,color)) if color==self.turn) {
                    if is_move_legal(&mut board, position,position-17,self.turn,self.opposite)==true {   
                        moves.push(position-17)
                    } // UUL
                }
                if position>7 && position%8!=6 && position%8!=7 && !matches! (get_piece_at(&board, position-6),Some((_,color)) if color==self.turn) {
                    if is_move_legal(&mut board, position,position-6,self.turn,self.opposite)==true {
                        moves.push(position-6)
                    } // URR
                }
                if position>9 && position%8!=0 && position%8!=1 && !matches! (get_piece_at(&board, position-10),Some((_,color)) if color==self.turn) {
                    if is_move_legal(&mut board, position,position-10,self.turn,self.opposite)==true {
                        moves.push(position-10)
                    } // ULL
                }
                if position<47 && position%8!=7 && !matches! (get_piece_at(&board, position+17),Some((_,color)) if color==self.turn) {
                    if is_move_legal(&mut board, position,position+17,self.turn,self.opposite)==true {
                        moves.push(position+17)
                    } // DDR
                }
                if position<48 && position%8!=0 && !matches! (get_piece_at(&board, position+15),Some((_,color)) if color==self.turn) {
                    if is_move_legal(&mut board, position,position+15,self.turn,self.opposite)==true {
                        moves.push(position+15)
                    } // DDL
                }
                if position<54 && position%8!=6 && position%8!=7 && !matches! (get_piece_at(&board, position+10),Some((_,color)) if color==self.turn) {
                    if is_move_legal(&mut board, position,position+10,self.turn,self.opposite)==true {    
                        moves.push(position+10)
                    } // DRR
                }
                if position<56 && position%8!=0 && position%8!=1 && !matches! (get_piece_at(&board, position+6),Some((_,color)) if color==self.turn) {
                    if is_move_legal(&mut board, position,position+6,self.turn,self.opposite)==true {
                        moves.push(position+6)
                    } // DLL
                }
            },
            PieceType::King => {
                if position>=8 { // U
                    if is_move_legal(&mut board, position,position-8,self.turn,self.opposite)==true {
                        match get_piece_at(&board, position-8) {
                            // if there is no piece or a black piece then it can go there
                            None => moves.push(position-8),
                            Some((_,color)) => if color==self.opposite {moves.push(position-8)},
                            // when color is white nothing happens
                        }
                    }
                }
                if position<=55 { // D
                    if is_move_legal(&mut board, position,position+8,self.turn,self.opposite)==true {
                        match get_piece_at(&board, position+8) {
                            None => moves.push(position+8), 
                            Some((_,color)) => if color==self.opposite {moves.push(position+8)},
                        }
                    }
                }
                if position%8!=0 { // L
                    if is_move_legal(&mut board, position,position-1,self.turn,self.opposite)==true {
                        match get_piece_at(&board, position-1) {
                            None => moves.push(position-1), 
                            Some((_,color)) => if color==self.opposite {moves.push(position-1)},
                        }
                    }
                }
                if position%8!=7 { // R
                    if is_move_legal(&mut board, position,position+1,self.turn,self.opposite)==true {
                        match get_piece_at(&board, position+1) {
                            None => moves.push(position+1), 
                            Some((_,color)) => if color==self.opposite {moves.push(position+1)},
                        }
                    }
                }

                if position>7&&position%8!=7 { // DR
                    if is_move_legal(&mut board, position,position-7,self.turn,self.opposite)==true {
                        match get_piece_at(&board, position-7) {
                            None => moves.push(position-7), 
                            Some((_,color)) => if color==self.opposite {moves.push(position-7)},
                        }
                    }
                }
                if position>7&&position%8!=0 { // DL
                    if is_move_legal(&mut board, position,position-9,self.turn,self.opposite)==true {
                        match get_piece_at(&board, position-9) {
                            None => moves.push(position-9), 
                            Some((_,color)) => if color==self.opposite {moves.push(position-9)},
                        }
                    }
                }
                if position<56&&position%8!=7 { // UR
                    if is_move_legal(&mut board, position,position+9,self.turn,self.opposite)==true {
                        match get_piece_at(&board, position+9) {
                            None => moves.push(position+9), 
                            Some((_,color)) => if color==self.opposite {moves.push(position+9)},
                        }
                    }
                }
                if position<56&&position%8!=0 { // UL
                    if is_move_legal(&mut board, position,position+7,self.turn,self.opposite)==true {
                        match get_piece_at(&board, position+7) {
                            None => moves.push(position+7), 
                            Some((_,color)) => if color==self.opposite {moves.push(position+7)},
                        }
                    }
                }
            }
            _ => {}
        }
        match piece {
            // queen has both bishop and rook features
            PieceType::Bishop | PieceType::Queen => {
                if position<55 && position%8!=7 { // go RD 
                    for i in (position+9..=position+(7-scol).min(7-srow)*9).step_by(9) {
                        match get_piece_at(&board, i) {
                            Some((_,color)) => {
                                if color==self.opposite && is_move_legal(&mut board, position,i,self.turn,self.opposite)==true{moves.push(i)};
                                break;
                            }
                            None => if is_move_legal(&mut board, position,i,self.turn,self.opposite)==true{moves.push(i)}
                        }
                    }
                }
                if position<56 && position%8!=0 { // go LD
                    for i in (position+7..=position+(scol).min(7-srow)*7).step_by(7) {
                        match get_piece_at(&board, i) {
                            Some((_,color)) => {
                                if color==self.opposite && is_move_legal(&mut board, position,i,self.turn,self.opposite)==true{moves.push(i)};
                                break;
                            }
                            None => if is_move_legal(&mut board, position,i,self.turn,self.opposite)==true{moves.push(i)}
                        }
                    }
                }
                if position>7 && position%8!=7 { // go RU
                    for i in ((position-(7-scol).min(srow)*7..=position-7)).rev().step_by(7) {
                        match get_piece_at(&board, i) {
                            Some((_,color)) => {
                                if color==self.opposite && is_move_legal(&mut board, position,i,self.turn,self.opposite)==true{moves.push(i)};
                                break;
                            }
                            None => if is_move_legal(&mut board, position,i,self.turn,self.opposite)==true{moves.push(i)}
                        }
                    }
                }
                if position>8 && position%8!=0 { // go LU
                    for i in ((position-(scol).min(srow)*9..=position-9)).rev().step_by(9) {
                        match get_piece_at(&board, i) {
                            Some((_,color)) => {
                                if color==self.opposite && is_move_legal(&mut board, position,i,self.turn,self.opposite)==true{moves.push(i)};
                                break;
                            }
                            None => if is_move_legal(&mut board, position,i,self.turn,self.opposite)==true{moves.push(i)}
                        }
                    }
                }
            },
            _ => {}
        }

        match color { // divide up to if its a black or white piece
            Color::Black => { 
                match piece { 
                    PieceType::King => {
                        // castling
                        if self.l_black_castle && get_piece_at(&board, 5)==None && get_piece_at(&board, 6)==None {
                            // cannot be checked before, during, after move
                            if king_in_check(&board, Color::Black,Color::White)==Some(false) && is_move_legal(&mut board, position,5,Color::Black,Color::White) && is_move_legal(&mut board, position,6,Color::Black,Color::White) {
                                moves.push(6)
                            }
                        }
                        if self.r_black_castle && get_piece_at(&board, 3)==None && get_piece_at(&board, 2)==None && get_piece_at(&board, 1)==None {
                            if king_in_check(&board, Color::Black,Color::White)==Some(false) && is_move_legal(&mut board, position,3,Color::Black,Color::White) && is_move_legal(&mut board, position,2,Color::Black,Color::White) {
                                moves.push(2)
                            }
                        }
                    },
                    PieceType::Pawn => {
                        // en passant
                        if srow==4 && self.en_passant != None && (self.en_passant.unwrap()==position-1 || self.en_passant.unwrap()==position+1) {
                            if self.en_passant.unwrap()==position-1 && is_move_legal(&mut board, position,position+7,Color::Black,Color::White) {
                                if get_piece_at(&board, position+7).is_none(){moves.push(position+7)} 
                            }
                            else if self.en_passant.unwrap()==position+1 && is_move_legal(&mut board, position,position+9,Color::Black,Color::White) {
                                if get_piece_at(&board, position+9).is_none(){moves.push(position+9)} 
                            }
                        }

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
                    _ => {}
                }
            }
            Color::White => {
                match piece { // for each piece it has different possible moves
                    PieceType::King => {
                        // castling
                        if self.r_white_castle && get_piece_at(&board, 61)==None && get_piece_at(&board, 62)==None {
                            if king_in_check(&board, Color::White,Color::Black)==Some(false) && is_move_legal(&mut board, position,61,Color::White,Color::Black) && is_move_legal(&mut board, position,62,Color::White,Color::Black) {                            
                                moves.push(62)
                            }
                        }
                        if self.l_white_castle && get_piece_at(&board, 57)==None && get_piece_at(&board, 58)==None && get_piece_at(&board, 59)==None {
                            if king_in_check(&board, Color::White,Color::Black)==Some(false) && is_move_legal(&mut board, position,59,Color::White,Color::Black) && is_move_legal(&mut board, position,58,Color::White,Color::Black) {
                                moves.push(58)
                            }
                        }
                    },
                    PieceType::Pawn => {
                        // en passant
                        if srow==3 && self.en_passant != None && (self.en_passant.unwrap()==position-1 || self.en_passant.unwrap()==position+1) {
                            if self.en_passant.unwrap()==position-1 && is_move_legal(&mut board, position,position-9,Color::White,Color::Black) {
                                if get_piece_at(&board, position-9).is_none(){moves.push(position-9)} 
                            }
                            else if self.en_passant.unwrap()==position+1 && is_move_legal(&mut board, position,position-7,Color::White,Color::Black) {
                                if get_piece_at(&board, position-7).is_none(){moves.push(position-7)} 
                            }
                        }

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
                    _ => {}
                }
            }
        
        }
        moves
    }

    pub fn game_status(&self) -> GameStatus {
        for i in 0..64 {
            if let Some((_,color)) = get_piece_at(&self.board, i) {
                if color==self.turn && !self.legal_moves(i).is_empty() {
                    if king_in_check(&self.board,self.turn,self.opposite)==Some(true){return GameStatus::Check}
                    else {return GameStatus::Ongoing}
                }
            }  
        }
        if king_in_check(&self.board,self.turn,self.opposite)!=Some(true) {
            return GameStatus::Stalemate;
        }
        else {
            return GameStatus::Checkmate
        }
    }

    pub fn print(&self) {
        println!("+-------------------------------+");
        for i in 0..64{
            let mut p = ".";
            if self.board[i]==Some(Piece { piece_type: PieceType::Rook, color: Color::Black }) {p="r"}
            else if self.board[i]==Some(Piece { piece_type: PieceType::Knight, color: Color::Black }) {p="n"}
            else if self.board[i]==Some(Piece { piece_type: PieceType::Bishop, color: Color::Black }) {p="b"}
            else if self.board[i]==Some(Piece { piece_type: PieceType::Queen, color: Color::Black }) {p="q"}
            else if self.board[i]==Some(Piece { piece_type: PieceType::King, color: Color::Black }) {p="k"}
            else if self.board[i]==Some(Piece { piece_type: PieceType::Pawn, color: Color::Black }) {p="p"}
            else if self.board[i]==Some(Piece { piece_type: PieceType::Rook, color: Color::White }) {p="R"}
            else if self.board[i]==Some(Piece { piece_type: PieceType::Knight, color: Color::White }) {p="N"}
            else if self.board[i]==Some(Piece { piece_type: PieceType::Bishop, color: Color::White }) {p="B"}
            else if self.board[i]==Some(Piece { piece_type: PieceType::Queen, color: Color::White }) {p="Q"}
            else if self.board[i]==Some(Piece { piece_type: PieceType::King, color: Color::White }) {p="K"}
            else if self.board[i]==Some(Piece { piece_type: PieceType::Pawn, color: Color::White }) {p="P"}
            
            if i%8==7 {println!("| {p} |")}
            else {print!("| {p} ")}
        }
        println!("+-------------------------------+");
    }
}

// new chess board 
fn new_board() -> [Option<Piece>;64] {
    // &board is 1d array, initialize with the chess piece else None in squares
    // Some() so Option know it is not None, but Some
    let board: [Option<Piece>; 64] = [
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
fn get_piece_at(board: &[Option<Piece>;64], position: usize) -> Option<(PieceType,Color)> {
    if board[position].is_none() {
        return None;
    };
    let piecee = board[position].as_ref().unwrap().piece_type;
    let colorr = board[position].as_ref().unwrap().color;
    return Some((piecee,colorr));
}

// check if king is in check, by looking all possible squares where a piece would take it
fn king_in_check(board: &[Option<Piece>;64], my_color: Color, opp_color: Color) -> Option<bool> {

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

// return what the legal moves for a piece is
fn is_move_legal(board: &mut[Option<Piece>;64], from:usize, to:usize, my_color: Color, opp_color: Color) -> bool {
    // temporary copy of board, so we can try if a move is legal
    // is there a piece on the from-square
    if board[from].is_none() {return false}

    // special case: en passant
    if get_piece_at(board, from) == Some((PieceType::Pawn,Color::White)) && (from>=7 && to==from-7||from>=9 && to==from-9) && get_piece_at(board, to) == None {
        let square1 = board[from];
        let square2 = board[to+8];
        board[to]=board[from];
        board[from]=None;
        board[to+8]=None;
        if king_in_check(&board, my_color, opp_color)==Some(true) {
            board[from]=square1; board[to+8]=square2; board[to]=None;
            return false
        }
        else {
            board[from]=square1; board[to+8]=square2; board[to]=None;
            return true
        }
    }   
    if get_piece_at(board, from) == Some((PieceType::Pawn,Color::Black)) && (from<=56 && to==from+7||from<=54 && to==from+9) && get_piece_at(board, to) == None {
        let square1 = board[from];
        let square2 = board[to-8];
        board[to]=board[from];
        board[from]=None;
        board[to-8]=None;
        if king_in_check(&board, my_color, opp_color)==Some(true) {
            board[from]=square1; board[to-8]=square2; board[to]=None;
            return false
        }
        else {
            board[from]=square1; board[to-8]=square2; board[to]=None;
            return true
        }
    } 


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
// unsigned integer, dynamic, array indices is usize