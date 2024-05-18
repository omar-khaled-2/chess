
use std::{borrow::BorrowMut, io::{Error, ErrorKind}};
use colored::*;

enum Color{
    White,
    Black
}

enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Piece {
    fn as_char(&self) -> char {
        match self {
            Piece::Pawn => 'P',
            Piece::Knight => 'N',
            Piece::Bishop => 'B',
            Piece::Rook => 'R',
            Piece::Queen => 'Q',
            Piece::King => 'K',
        }
    }

    fn as_string(&self) -> String {
        match self {
            Piece::Pawn => "Pawn".to_string(),
            Piece::Knight => "Knight".to_string(),
            Piece::Bishop => "Bishop".to_string(),
            Piece::Rook => "Rook".to_string(),
            Piece::Queen => "Queen".to_string(),
            Piece::King => "King".to_string(),
        }

    }
}




struct MovingPiece {
    position:u8,
    piece:Piece,
    color:Color
}

impl MovingPiece {
    fn set_position(&mut self, position:u8){
        self.position = position
    }

    fn promate(&mut self, piece:Piece) {
        self.piece = piece
    }
}

fn get_row(position:u8) -> u8 {
    position / 8
}

fn get_column(position:u8) -> u8 {
    position % 8
}






fn generate_pieces() -> Vec<MovingPiece> {
    let mut list = Vec::new();

    // Adding black pawns
    for i in 0..8 {
        let piece = MovingPiece {
            position: i + 8,
            color: Color::Black,
            piece: Piece::Pawn,
        };
        list.push(piece);
    }

    // Adding black pieces
    list.push(MovingPiece {
        position: 0,
        color: Color::Black,
        piece: Piece::Rook,
    });

    list.push(MovingPiece {
        position: 1,
        color: Color::Black,
        piece: Piece::Knight,
    });

    list.push(MovingPiece {
        position: 2,
        color: Color::Black,
        piece: Piece::Bishop,
    });

    list.push(MovingPiece {
        position: 3,
        color: Color::Black,
        piece: Piece::Queen,
    });

    list.push(MovingPiece {
        position: 4,
        color: Color::Black,
        piece: Piece::King,
    });

    list.push(MovingPiece {
        position: 5,
        color: Color::Black,
        piece: Piece::Bishop,
    });

    list.push(MovingPiece {
        position: 6,
        color: Color::Black,
        piece: Piece::Knight,
    });

    list.push(MovingPiece {
        position: 7,
        color: Color::Black,
        piece: Piece::Rook,
    });

    // Adding white pawns
    for i in 0..8 {
        let piece = MovingPiece {
            position: i + 8 * 6,
            color: Color::White,
            piece: Piece::Pawn,
        };
        list.push(piece);
    }

    // Adding white pieces
    list.push(MovingPiece {
        position: 56,
        color: Color::White,
        piece: Piece::Rook,
    });

    list.push(MovingPiece {
        position: 57,
        color: Color::White,
        piece: Piece::Knight,
    });

    list.push(MovingPiece {
        position: 58,
        color: Color::White,
        piece: Piece::Bishop,
    });

    list.push(MovingPiece {
        position: 59,
        color: Color::White,
        piece: Piece::Queen,
    });

    list.push(MovingPiece {
        position: 60,
        color: Color::White,
        piece: Piece::King,
    });

    list.push(MovingPiece {
        position: 61,
        color: Color::White,
        piece: Piece::Bishop,
    });

    list.push(MovingPiece {
        position: 62,
        color: Color::White,
        piece: Piece::Knight,
    });

    list.push(MovingPiece {
        position: 63,
        color: Color::White,
        piece: Piece::Rook,
    });

    list
}



enum MoveType {
    Normal,
    Capture,
    EnPassant,
    Castling,
    Promotion,
}

struct Move {
    from:u8,
    to:u8,
    move_type:MoveType
}


fn get_mut_piece(pieces:&mut Vec<MovingPiece>, position:u8) -> Option<&mut MovingPiece> {
    for piece in pieces {
        if piece.position == position {
            return Some(piece);
        }
    }
    None
}





fn remove_piece(pieces:&mut Vec<MovingPiece>, position:u8) {

    for (index, piece) in pieces.iter().enumerate() {
        if piece.position == position {
            pieces.remove(index);
            break;
        }
    }

}

impl Move {
    
    fn new(from:u8,to:u8, move_type:MoveType) -> Move {
        Move { from,to, move_type }
    }

    fn make_move(&self, pieces:&mut Vec<MovingPiece>) {
        let piece = get_mut_piece(pieces, self.from).unwrap();


        match self.move_type {
            MoveType::Normal => {
                piece.set_position(self.to);
            },
            MoveType::Capture => {
                piece.set_position(self.to);
                drop(piece);
                
                remove_piece(pieces, self.to);
               
            },
            MoveType::EnPassant => {
                

            },
            MoveType::Castling => {
    
            },
            MoveType::Promotion => {
                
            }
        }
        
    }
}

fn can_be_promoted(piece:&MovingPiece) -> bool {
    if !matches!(piece.piece, Piece::Pawn) {
        return false;
    }
        

    let row = get_row(piece.position);
    match piece.color {
        Color::White => row == 0,
        Color::Black => row == 7
    }
  
}


fn is_piece_exits(pieces:&Vec<MovingPiece>, position:u8) -> bool {
    for piece in pieces {
        if piece.position == position {
            return true;
        }
    }
    false
}



fn get_moves(pieces:&mut Vec<MovingPiece>, position:u8) -> Vec<Move> {
    let mut moves = Vec::new();
    let mut piece = get_piece(pieces, position).unwrap();


    match piece.piece {
        Piece::Pawn => {
            let row = get_row(position);

            match piece.color {
                Color::Black => {

                    
                    if  row < 7 && !is_piece_exits(pieces, position + 8) {
                        moves.push(Move::new(position, position + 8, MoveType::Normal));
                    }

                    if  row == 1 && !is_piece_exits(pieces, position + 16) {
                        moves.push(Move::new(position, position + 16, MoveType::Normal));
                    }

                    if  row < 7 && get_column(position) < 7 && !is_piece_exits(pieces, position + 9) {
                        moves.push(Move::new(position, position + 9, MoveType::Capture));
                    }

                    if  row < 7 && get_column(position) > 0 && !is_piece_exits(pieces, position + 7) {
                        moves.push(Move::new(position, position + 7, MoveType::Capture));
                    }

                }
                Color::White => {
                    if row > 0 && !is_piece_exits(pieces, position - 8) {
                        moves.push(Move::new(position, position - 8, MoveType::Normal));
                    }

                    if row == 6 && !is_piece_exits(pieces, position - 16) {
                        moves.push(Move::new(position, position - 16, MoveType::Normal));
                    }

                    if row > 0 && get_column(position) < 7 && is_piece_exits(pieces, position - 7) {
                        moves.push(Move::new(position, position - 7, MoveType::Capture));
                    }

                    if row > 0 && get_column(position) > 0 && is_piece_exits(pieces, position - 9) {
                        moves.push(Move::new(position, position - 9, MoveType::Capture));
                    }
                }
            }
        },
        Piece::Knight => {
            let row = get_row(position);
            let column = get_column(position);
            if row < 6 && column < 7 {
                if !is_piece_exits(pieces, position + 17) {
                    moves.push(Move::new(position, position + 17, MoveType::Normal));
                }
            }
            if row < 6 && column > 0 {
                if !is_piece_exits(pieces, position + 15) {
                    moves.push(Move::new(position, position + 15, MoveType::Normal));
                }
            }

            if row > 1 && column < 7 {
                if !is_piece_exits(pieces, position - 15) {
                    moves.push(Move::new(position, position - 15, MoveType::Normal));
                }
            }
            if row > 1 && column > 0 {
                if !is_piece_exits(pieces, position - 17) {
                    moves.push(Move::new(position, position - 17, MoveType::Normal));
                }
            }


            if column < 6 && row < 7 {
                if !is_piece_exits(pieces, position + 10) {
                    moves.push(Move::new(position, position + 10, MoveType::Normal));
                }
            }
            if column < 6 && row > 0 {
                if !is_piece_exits(pieces, position - 6) {
                    moves.push(Move::new(position, position - 6, MoveType::Normal));
                }
            }


            if column > 1 && row < 7 {
                if !is_piece_exits(pieces, position + 6) {
                    moves.push(Move::new(position, position + 6, MoveType::Normal));
                }
            }
            if column > 1 && row > 0 {
                if !is_piece_exits(pieces, position - 10) {
                    moves.push(Move::new(position, position - 10, MoveType::Normal));
                }
            }

        },
        Piece::Bishop => {
            // TODO
        },
        Piece::Rook => {
            let mut p = position;

            while get_row(p - 8) >= 0 {
                if !is_piece_exits(pieces, p - 8) {
                    moves.push(Move::new(position, p - 8, MoveType::Normal));
                }else{
                    let captured_piece = get_piece(pieces, p - 8).unwrap();
                    if false{
                        moves.push(Move::new(position, p - 8, MoveType::Capture));
                    }
                    break;
                }

                p = p - 8;

            };

            p = position;

            while get_row(p + 8) <= 7 {
                if !is_piece_exits(pieces, p + 8) {
                    moves.push(Move::new(position, p + 8, MoveType::Normal));
                }else{
                    let captured_piece = get_piece(pieces, p + 8).unwrap();
                    if false {
                        moves.push(Move::new(position, p + 8, MoveType::Capture));
                    }
                    break;
                }
                p = p + 8;
            }; 

            p = position;

            while get_column(p - 1) >= 0 {
                if !is_piece_exits(pieces, p - 1) {
                    moves.push(Move::new(position, p - 1, MoveType::Normal));
                }else{
                    let captured_piece = get_piece(pieces, p - 1).unwrap();
                    if false {
                        moves.push(Move::new(position, p - 1, MoveType::Capture));
                    }
                    break;
                }
                p = p - 1;
            };


            p = position;

            while get_column(p + 1) <= 7 {
                if !is_piece_exits(pieces, p + 1) {
                    moves.push(Move::new(position, p + 1, MoveType::Normal));
                }else{
                    let captured_piece = get_piece(pieces, p + 1).unwrap();
                    if !matches!(&piece.color,captured_piece){
                        moves.push(Move::new(position, p + 1, MoveType::Capture));
                    }
                    break;
                }
                p = p + 1;
            };


            

            
        },
        Piece::Queen => {
            // TODO
        },
        Piece::King => {
            // TODO
        },
    }


    moves



}

fn is_pinned(pieces:&Vec<MovingPiece>, position:u8) -> bool {
    false
}




fn get_king_position(pieces:&Vec<MovingPiece>, color:Color) -> u8 {
    for piece in pieces {
        if matches!(&piece.color,color) && matches!(piece.piece,Piece::King) {
            return piece.position;
        }
    }
    return 0;
}


fn is_checkmate(pieces:&Vec<MovingPiece>,color:Color) -> bool {

    let king_position = get_king_position(pieces, color);
    
    for piece in pieces  {
        if matches!(&piece.color,color)  {
            continue;
        }

        match piece.piece {
            Piece::Rook => {
                let rook_row = get_row(piece.position);
                let rook_column = get_column(piece.position);
                let king_row = get_row(king_position);
                let king_column = get_column(king_position);
                if king_row != rook_row && king_column != rook_column {
                    return false;
                }
                
                for piece in pieces {
                    
                    let piece_row = get_row(piece.position);
                    let piece_column = get_column(piece.position);

                    if 
                        rook_row == piece_row &&
                        rook_row == king_row &&
                        ((rook_column < king_column && piece_column > rook_column) || (rook_column > king_column && piece_column < rook_column)) {
                            return  false;
                        }

                    
                }

                return true;

            },
            Piece::Knight => {
                
                
            },
            Piece::Bishop => {
                
            },
            Piece::Queen => {
                
            },
            Piece::King => {
                continue;
            },
            Piece::Pawn => {
                
            },

        }
    }

    false
}










fn display_board(pieces:&Vec<MovingPiece>) {
    const INIT: Option<&MovingPiece> = None;
    let mut board: [Option<&MovingPiece>; 64] = [INIT; 64];

    for piece in pieces {
        let position = piece.position;

        board[position as usize] = Some(piece);

    }

    print!(" \t");
    for c in 'A'..='H' {
        print!("{}\t", c);
    }

    print!("\n\n");

    for i in 0..8 {
        print!("{}\t", 8 - i);
        for j in 0..8 {
            let index = 8 * i + j;
            let moving_piece = board[index];

            match moving_piece {
                Some(moving_piece) => {
                    let p:ColoredString = match moving_piece.color {
                        Color::White => moving_piece.piece.as_char().to_string().white(),
                        Color::Black => moving_piece.piece.as_char().to_string().black(),
                    };

                
             
                    print!("{}\t",p);
                }
                None => print!(" \t")
            }
        }
        print!("\n\n");
    }

    
}


fn notation_to_position(notation:&String) -> Result<u8, Error> {

   
    if notation.len() != 2 {
        return Err(Error::new(ErrorKind::InvalidInput, "notation must be two characters"));
    }

    if notation.chars().nth(1).unwrap().is_numeric() == false {
        return Err(Error::new(ErrorKind::InvalidInput, "second character must be a number"));
    }

    if notation.chars().nth(0).unwrap().is_alphabetic() == false {
        return Err(Error::new(ErrorKind::InvalidInput, "first character must be a letter"));
    }

    let letter = notation.chars().nth(0).unwrap();
    let mut number = notation.chars().nth(1).unwrap().to_digit(10).unwrap() as u8;
    if number < 1 || number > 8 {
        return Err(Error::new(ErrorKind::InvalidInput, "number must be between 1 and 8"));
    }

    if letter.is_uppercase() == false {
        return Err(Error::new(ErrorKind::InvalidInput, "letter must be uppercase"));
    }

    let letter_index = letter as u8 - 'A' as u8;
    if letter_index > 7 {
        return Err(Error::new(ErrorKind::InvalidInput, "letter must be between A and H"));
    }

    let position: u8 = 8 * (8 - number) + letter_index;

    return Ok(position);

}

fn position_to_notation(position:u8) -> String {
    let letter = (position % 8) + 'A' as u8;
    let number = 8 - (position / 8);
    format!("{}{}", letter as char, number)
}

fn get_piece(pieces:&Vec<MovingPiece>, position:u8) -> Option<&MovingPiece> {
    for piece in pieces {
        if piece.position == position {
            return Some(&piece);
        }
    }
    None
}




fn main() {
    let mut pieces = generate_pieces();
    let promated_pieces = [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight];
    let is_game_over = false;
    while !is_game_over  {
        // clear_console();
        display_board(&pieces);
        let mut input = String::new();
        loop {
            println!("Enter notation: ");
            std::io::stdin().read_line(&mut input);
            let notation = input.trim().to_string();
            input.clear();
            match notation_to_position(&notation) {
                Ok(position) => {
                    let piece = get_piece(&mut pieces, position);
                    if piece.is_some() {
                        let moves = get_moves(&mut pieces, position);
                        if moves.is_empty() {
                            println!("No moves available");
                        }else{
                            for i in 0..moves.len() {
                                println!("{}. {}", i + 1, position_to_notation(moves[i].to));
                            }
                          
                            std::io::stdin().read_line(&mut input);
                            let selected_move_index:usize = input.trim().parse::<usize>().unwrap() - 1;
                            input.clear();
                            let selected_move = &moves[selected_move_index];

                            selected_move.make_move(&mut pieces);

                            let moved_piece = get_mut_piece(&mut pieces, selected_move.to).unwrap();

                            if can_be_promoted(moved_piece) { 
                                println!("promote to: ");
                                for(i, piece) in promated_pieces.iter().enumerate() {
                                    println!("{}. {}", i + 1, piece.as_string());
                                }

                                std::io::stdin().read_line(&mut input);
                                let selected_promotion_index:usize = input.trim().parse::<usize>().unwrap() - 1;
                                input.clear();
                                let selected_promotion = &promated_pieces[selected_promotion_index];

                                let selected_promotion = Piece::Queen;
                                moved_piece.promate(selected_promotion);
                                
                            }

                            


                            // moves[selected_move_index].make_move(&mut pieces);


                        }
                  



                        break;
                    }else{
                        println!("No piece at that position");
                    }


                    
                }
                Err(error) => {
                    println!("{}", error);
                    
                }
            }

        }
    }

    println!("Game Over");
}
