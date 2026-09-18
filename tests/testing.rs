use chessy::*;

// unit tests inside src, integration tests in tests
// private vs public functions

#[cfg(test)]
mod tests {
    use super::*;
    fn empty_game() -> Chess {
        let game = Chess {
            board: [None; 64],
            turn: Color::White,
            opposite: Color::Black,
            r_white_castle: true,
            l_white_castle: true,
            r_black_castle: true,
            l_black_castle: true,
            en_passant: None,
        };
        game
    }
    fn sorted(mut v: Vec<usize>) -> Vec<usize> {
        v.sort_unstable();
        v
    }
    #[test]
    fn rook_movements() {
        let mut game = empty_game();

        // rook on empty board
        game.board[0]=Some(Piece { piece_type: PieceType::Rook, color: Color::White });
        assert_eq!(sorted(game.legal_moves(0)),sorted(vec![1,2,3,4,5,6,7,8,16,24,32,40,48,56]));

        // rook on empty board with a blocking white piece and black piece
        game.board[5]=Some(Piece { piece_type: PieceType::Pawn, color: Color::White });
        game.board[40]=Some(Piece { piece_type: PieceType::Pawn, color: Color::Black });
        assert_eq!(sorted(game.legal_moves(0)),sorted(vec![1,2,3,4,8,16,24,32,40]));
    }

    #[test]
    fn bishop_movements() {
        let mut game = empty_game();

        // bishop on empty board
        game.board[36]=Some(Piece { piece_type: PieceType::Bishop, color: Color::White });
        assert_eq!(sorted(game.legal_moves(36)),sorted(vec![45,54,63,43,50,57,29,22,15,27,18,9,0]));

        // bishop on empty board with a blocking white piece and black piece
        game.board[18]=Some(Piece { piece_type: PieceType::Pawn, color: Color::White });
        game.board[50]=Some(Piece { piece_type: PieceType::Pawn, color: Color::Black });
        assert_eq!(sorted(game.legal_moves(36)),sorted(vec![45,54,63,43,50,29,22,15,27]));
    }
    
    #[test]
    fn queen_movements() {
        let mut game = empty_game();

        // queen on empty board
        game.board[34]=Some(Piece { piece_type: PieceType::Queen, color: Color::White });
        assert_eq!(sorted(game.legal_moves(34)),sorted(vec![32,33,35,36,37,38,39,42,50,58,26,18,10,2,43,52,61,25,16,27,20,13,6,41,48]));

        // queen on empty board with a blocking white piece and black piece
        game.board[10]=Some(Piece { piece_type: PieceType::Pawn, color: Color::White });
        game.board[13]=Some(Piece { piece_type: PieceType::Pawn, color: Color::Black });
        assert_eq!(sorted(game.legal_moves(34)),sorted(vec![32,33,35,36,37,38,39,26,18,42,50,58,25,16,27,20,13,41,48,43,52,61]));
    }

    #[test]
    fn knight_movements() {
        let mut game = empty_game();

        // knight on empty board
        game.board[14]=Some(Piece { piece_type: PieceType::Knight, color: Color::White });
        assert_eq!(sorted(game.legal_moves(14)),sorted(vec![4,20,29,31]));

        // kight on empty board with a blocking white piece and black piece
        game.board[4]=Some(Piece { piece_type: PieceType::Pawn, color: Color::White });
        game.board[20]=Some(Piece { piece_type: PieceType::Pawn, color: Color::Black });
        assert_eq!(sorted(game.legal_moves(14)),sorted(vec![20,29,31]));
    }

    #[test]
    fn king_movements() {
        let mut game = empty_game();

        // king on empty board
        game.board[44]=Some(Piece { piece_type: PieceType::King, color: Color::White });
        assert_eq!(sorted(game.legal_moves(44)),sorted(vec![36,37,45,53,52,51,43,35]));

        // king not self check
        game.board[36]=Some(Piece { piece_type: PieceType::Pawn, color: Color::White });
        game.board[28]=Some(Piece { piece_type: PieceType::Pawn, color: Color::Black });
        assert_eq!(sorted(game.legal_moves(44)),sorted(vec![45,53,52,51,43]));
    }
   
    #[test]
    fn castling() {
        let mut game = empty_game();

        // empty board
        game.board[63] = Some(Piece { piece_type: PieceType::Rook, color: Color::White });
        game.board[60] = Some(Piece { piece_type: PieceType::King, color: Color::White });
        assert_eq!(sorted(game.legal_moves(60)),sorted(vec![58,59,51,52,53,61,62]));
        let res = game.move_piece(60, 62, None);
        assert_eq!(game.r_white_castle,false);

        // piece between
        let mut game = empty_game();
        game.board[63] = Some(Piece { piece_type: PieceType::Rook, color: Color::White });
        game.board[60] = Some(Piece { piece_type: PieceType::King, color: Color::White });
        game.board[61] = Some(Piece { piece_type: PieceType::Queen, color: Color::White });
        assert_eq!(sorted(game.legal_moves(60)),sorted(vec![58,59,51,52,53]));

        // rook is captured
        game.next_turn();
        game.board[39] = Some(Piece { piece_type: PieceType::Rook, color: Color::Black });
        let res = game.move_piece(39, 63, None);
        println!("{:?}",res);
        assert_eq!(game.r_white_castle,false);

        // king moves through attacked square
        game.board = [None; 64];
        game.board[63] = Some(Piece { piece_type: PieceType::Rook, color: Color::White });
        game.board[60] = Some(Piece { piece_type: PieceType::King, color: Color::White });
        game.board[54] = Some(Piece { piece_type: PieceType::Pawn, color: Color::Black });
        //assert_eq!(sorted(game.legal_moves(60)),sorted(vec![59,51,52,53]));
        // game.print();
    }
    
    #[test]
    fn en_passant() {
        let mut game = empty_game();

        // most normal en passant
        game.board[50]=Some(Piece { piece_type: PieceType::Pawn, color: Color::White });
        game.board[35]=Some(Piece { piece_type: PieceType::Pawn, color: Color::Black });
        game.move_piece(50,34,None);
        assert_eq!(sorted(game.legal_moves(35)),sorted(vec![43,42]));
        game.move_piece(35, 42, None);
        assert_eq!(game.board[34],None);

        // has to be after one two-step move not two steps
        game = empty_game();
        game.board[50]=Some(Piece { piece_type: PieceType::Pawn, color: Color::White });
        game.board[35]=Some(Piece { piece_type: PieceType::Pawn, color: Color::Black });
        game.move_piece(50,42,None);
        game.next_turn();
        game.move_piece(42,34,None);
        assert_eq!(sorted(game.legal_moves(35)),sorted(vec![43]));

        // works for black too
        game = empty_game();
        game.board[27]=Some(Piece { piece_type: PieceType::Pawn, color: Color::White });
        game.board[10]=Some(Piece { piece_type: PieceType::Pawn, color: Color::Black });
        game.next_turn();
        game.move_piece(10,26,None);
        game.move_piece(27, 18, None);
        assert_eq!(game.board[26],None);
    }
    
    // En passant:

// Rätt bricka tas bort (raden bakom to).
    // println!("{:?}",game.legal_moves(14));


}
