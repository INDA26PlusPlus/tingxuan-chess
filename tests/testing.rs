use chessy::Chess; 
use chessy::Piece; 
use chessy::PieceType; 
use chessy::Color; 

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

    #[test]
    fn rook_movements() {
        let mut game = empty_game();

        // rook on empty board
        game.board[0]=Some(Piece { piece_type: PieceType::Rook, color: Color::White });
        assert_eq!(game.legal_moves(0).sort_unstable(),[1,2,3,4,5,6,7,8,16,24,32,40,48,56].sort_unstable());

        // rook on empty board with a blocking white piece and black piece
        game.board[5]=Some(Piece { piece_type: PieceType::Pawn, color: Color::White });
        game.board[40]=Some(Piece { piece_type: PieceType::Pawn, color: Color::Black });
        assert_eq!(game.legal_moves(0).sort_unstable(),[1,2,3,4,8,16,24,32,40].sort_unstable());
    }

    #[test]
    fn bishop_movements() {
        let mut game = empty_game();

        // bishop on empty board
        game.board[36]=Some(Piece { piece_type: PieceType::Bishop, color: Color::White });
        assert_eq!(game.legal_moves(36).sort_unstable(),[45,54,63,43,50,57,29,22,15,27,18,9,0].sort_unstable());

        // bishop on empty board with a blocking white piece and black piece
        game.board[18]=Some(Piece { piece_type: PieceType::Pawn, color: Color::White });
        game.board[50]=Some(Piece { piece_type: PieceType::Pawn, color: Color::Black });
        assert_eq!(game.legal_moves(36).sort_unstable(),[45,54,63,43,50,29,22,15,27].sort_unstable());
    }
    
    #[test]
    fn queen_movements() {
        let mut game = empty_game();

        // queen on empty board
        game.board[34]=Some(Piece { piece_type: PieceType::Queen, color: Color::White });
        assert_eq!(game.legal_moves(34).sort_unstable(),[32,33,35,36,37,38,39,42,50,58,26,18,10,2,43,52,61,25,16,27,20,13,6,41,48].sort_unstable());

        // queen on empty board with a blocking white piece and black piece
        game.board[10]=Some(Piece { piece_type: PieceType::Pawn, color: Color::White });
        game.board[13]=Some(Piece { piece_type: PieceType::Pawn, color: Color::Black });
        assert_eq!(game.legal_moves(34).sort_unstable(),[32,33,35,36,37,38,39,26,18,42,50,58,25,16,27,20,13,41,48,43,52,61].sort_unstable());
    }

    #[test]
    fn knight_movements() {
        let mut game = empty_game();

        // knight on empty board
        game.board[14]=Some(Piece { piece_type: PieceType::Knight, color: Color::White });
        assert_eq!(game.legal_moves(14).sort_unstable(),[4,20,29,31].sort_unstable());

        // kight on empty board with a blocking white piece and black piece
        game.board[4]=Some(Piece { piece_type: PieceType::Pawn, color: Color::White });
        game.board[20]=Some(Piece { piece_type: PieceType::Pawn, color: Color::Black });
        assert_eq!(game.legal_moves(14).sort_unstable(),[20,29,31].sort_unstable());
    }

    #[test]
    fn king_movements() {
        let mut game = empty_game();

        // king on empty board
        game.board[44]=Some(Piece { piece_type: PieceType::Knight, color: Color::White });
        assert_eq!(game.legal_moves(44).sort_unstable(),[36,37,45,53,52,51,43,35].sort_unstable());

        // king not self check
        game.board[36]=Some(Piece { piece_type: PieceType::Pawn, color: Color::White });
        game.board[28]=Some(Piece { piece_type: PieceType::Pawn, color: Color::Black });
        assert_eq!(game.legal_moves(14).sort_unstable(),[45,53,52,51,43].sort_unstable());
    }
   
    #[test]
    fn castling() {
        let mut game = empty_game();

        // empty board
        game.board[63] = Some(Piece { piece_type: PieceType::Rook, color: Color::White });
        game.board[60] = Some(Piece { piece_type: PieceType::King, color: Color::White });
        assert_eq!(game.legal_moves(60).sort_unstable(),[59,51,52,53,61,62].sort_unstable());
        let res = game.move_piece(60, 62, None);
        assert_eq!(game.r_white_castle,false);

        // piece between
        game.board = [None; 64];
        game.board[63] = Some(Piece { piece_type: PieceType::Rook, color: Color::White });
        game.board[60] = Some(Piece { piece_type: PieceType::King, color: Color::White });
        game.board[61] = Some(Piece { piece_type: PieceType::Queen, color: Color::White });
        assert_eq!(game.legal_moves(60).sort_unstable(),[59,51,52,53].sort_unstable());

        // rook is captured
        game.board[39] = Some(Piece { piece_type: PieceType::Rook, color: Color::Black });
        let res = game.move_piece(39, 63, None);
        println!("{:?}",res);
        assert_eq!(game.r_white_castle,false);

        // king moves through attacked square
        game.board = [None; 64];
        game.board[63] = Some(Piece { piece_type: PieceType::Rook, color: Color::White });
        game.board[60] = Some(Piece { piece_type: PieceType::King, color: Color::White });
        game.board[54] = Some(Piece { piece_type: PieceType::Pawn, color: Color::Black });
        assert_eq!(game.legal_moves(60).sort_unstable(),[59,51,52,53].sort_unstable());
        // game.print();
    }
    
    #[test]
    fn en_passant() {
        let game = Chess::new();
        game.print();
    }
   
    // println!("{:?}",game.legal_moves(14));


}
