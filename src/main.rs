use chess::pieces::gather_pieces;
use chess::pieces::{Piece, Pawn, Knight, Bishop, Rook, Queen, King};
use chess::board::render_board;
use std::io;
use regex::Regex;

fn main() {
    let blocktop = "-----------";  
    // let blockbody = "|           ";    // block widths as square

    let mut turn_count = 0;

    let mut all_pieces = gather_pieces();

    let empty = String::from("x");

    loop {

        let mut map: Vec<Vec<&str>> = vec![
            vec![&empty, &empty, &empty, &empty, &empty, &empty, &empty, &empty],
            vec![&empty, &empty, &empty, &empty, &empty, &empty, &empty, &empty],
            vec![&empty, &empty, &empty, &empty, &empty, &empty, &empty, &empty],
            vec![&empty, &empty, &empty, &empty, &empty, &empty, &empty, &empty],
            vec![&empty, &empty, &empty, &empty, &empty, &empty, &empty, &empty],
            vec![&empty, &empty, &empty, &empty, &empty, &empty, &empty, &empty],
            vec![&empty, &empty, &empty, &empty, &empty, &empty, &empty, &empty],
            vec![&empty, &empty, &empty, &empty, &empty, &empty, &empty, &empty],

            ];

        //INNER GAME LOOP BEGINS SOMEWHERE IN HERE

        
        // iterate thru all pieces and get their file and rank
        for piece in all_pieces.iter() {

            let ifile = piece.get_rank();
            let irank = piece.get_file();
            
            let file = *ifile as usize;
            let rank = *irank as usize;
            
            map[file-1][rank-1] = piece.get_kind();

        };
                    

        render_board(blocktop, &map);

        // Steps to move a piece: 
        // 1. recieve nrank, nfile, piece from user
        // 2. check match and legality among all pieces
        // 3. If singular, recreate piece with new rank and file. Else, disambiguate.

        //  Let's set up IO first
        let mut whose_turn = true;
        if turn_count % 2 == 0 {
            println!("\nIt's white's turn. Enter your move: ");
            whose_turn = true;
        } else {
            println!("\nIt's black's turn. Enter your move: ");
            whose_turn = false;
        }

        let mut user_input = String::new();

        io::stdin().read_line(&mut user_input).expect("Failed to read line");

        let parsed_input = parse_input(user_input);

        
        // extract move_to tuple from the option
        let (piece_kind, file, rank, promote_to) = match parsed_input {
            Some((piece_kind, file, rank, promote_to)) => (piece_kind, file, rank, promote_to),
            None => continue,
        };

        // handle castling 
        if piece_kind == "R".to_string() && promote_to == "K".to_string() {
            // Kingside castle

            } else if whose_turn == false {

            }

        } else if piece_kind == "R".to_string() && promote_to == "Q".to_string() {
            // Queenside castle
            if whose_turn == true {

            } else if whose_turn == false {

            }

        }


        let mut some_legal_move = 0;
        let mut original_piece = (&0, &0, &String::from("x"));
        let mut illegal_move = false;
        for piece in all_pieces.iter() {
            // check that this move does not overlap with another piece on the same team
            if piece.get_rank() == &rank && piece.get_file() == &file && piece.get_color() == &whose_turn {
                println!("You cannot move to a square occupied by your own piece. Please try again.");
                illegal_move = true;
                break
            }

            // check that this move is legal for some piece
            if piece.get_legal(rank, file, &map) == true && piece.get_color() == &whose_turn && piece.get_id() == &piece_kind {
                some_legal_move += 1;
                original_piece = (piece.get_rank(), piece.get_file(), piece.get_id());
            } 
        }

        if illegal_move == true {
            continue
        }

        if some_legal_move > 1 {
            println!("There are multiple pieces that can move to this square. Please disambiguate.");
            continue
        } else if some_legal_move == 0 {
            println!("This move is not legal.");
            continue
        }

        // remove the original piece from all_pieces
        let mut index = 0;
        for piece in all_pieces.iter() {
            if piece.get_rank() == original_piece.0 && piece.get_file() == original_piece.1 && piece.get_id() == original_piece.2 {
                all_pieces.remove(index);
                break
            }
            index += 1;
        }

        // remove the piece at the destination square from all_pieces (capture)
        index = 0;
        for piece in all_pieces.iter() {
            if piece.get_rank() == &rank && piece.get_file() == &file && piece.get_color() != &whose_turn {
                all_pieces.remove(index);
                break
            }
            index += 1;
        }


        let moved_piece = create_moved_piece(piece_kind, rank, file, whose_turn, promote_to);

        // if moved_piece is None, continue, else, unwrap
        let moved_piece = match moved_piece {
            Some(moved_piece) => moved_piece,
            None => continue,
        };


        // add the moved piece to all_pieces
        all_pieces.push(moved_piece);



        turn_count += 1;
    }

}

fn create_moved_piece(piece_kind: String, rank: i8, file: i8, whose_turn: bool, promote_to: String) -> Option<Piece> {
    if promote_to == "none".to_string() {
        let moved_piece = match &piece_kind[..] {
            "R" => Some(Piece::Rook(Rook{rank: rank, file: file, yt: whose_turn, id: piece_kind, orig: false})),
            "N" => Some(Piece::Knight(Knight{rank: rank, file: file, yt: whose_turn, id: piece_kind, orig: false})),
            "B" => Some(Piece::Bishop(Bishop{rank: rank, file: file, yt: whose_turn, id: piece_kind, orig: false})),
            "Q" => Some(Piece::Queen(Queen{rank: rank, file: file, yt: whose_turn, id: piece_kind, orig: false})),
            "K" => Some(Piece::King(King{rank: rank, file: file, yt: whose_turn, id: piece_kind, orig: false})),
            "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" => Some(Piece::Pawn(Pawn{rank: rank, file: file, yt: whose_turn, id: piece_kind, orig: false})),
            _ => None,
            };
            return moved_piece;
        } else {
            let moved_piece = match &promote_to[..] {
                "R" => Some(Piece::Rook(Rook{rank: rank, file: file, yt: whose_turn, id: promote_to, orig: false})),
                "N" => Some(Piece::Knight(Knight{rank: rank, file: file, yt: whose_turn, id: promote_to, orig: false})),
                "B" => Some(Piece::Bishop(Bishop{rank: rank, file: file, yt: whose_turn, id: promote_to, orig: false})),
                "Q" => Some(Piece::Queen(Queen{rank: rank, file: file, yt: whose_turn, id: promote_to, orig: false})),
                _ => None,
            };
            return moved_piece;
        }
        println!("Invalid move, try again");
        return None;
}


fn parse_input(input: String) -> Option<(String, i8, i8, String)> {

    let recastle = Regex::new(r"(?m)^O-O$").unwrap();
    if recastle.is_match(&input) {
        return Some(("R".to_string(), 0, 0, "K".to_string()))
    } 

    let relongcastle = Regex::new(r"(?m)^O-O-O$").unwrap();
    if recastle.is_match(&input) {
        return Some(("R".to_string(), 0, 0, "Q".to_string()))
    } 


    let repromote = Regex::new(r"(?m)^[a-h]8=[RNBQ]$").unwrap();
    if repromote.is_match(&input) {
        println!("PROMOTION");
        let pawn_file = input.chars().nth(0).unwrap().to_string();
        let rank = input.chars().nth(1).unwrap().to_string();
        let rank = rank.parse::<i8>().unwrap();

        // convert file to i8
        let file = file_to_num(&pawn_file);
        

        // This is sticky! We need to return the piece kind as a string, but we don't know what it is yet.
        // take the last letter of the input as promote_to
        let promote_to = &input[input.len() - 2..input.len() - 1];

        return Some((pawn_file, file, rank, promote_to.to_string()));


    }
    
    let retakesandpromote = Regex::new(r"(?m)^[a-h]x[a-h]8=[RNBQ]$").unwrap();
    if retakesandpromote.is_match(&input) {
        println!("TAKES AND PROMOTION");

        // take the first letter of the input as the pawn name
        let pawn_file = input.chars().nth(0).unwrap().to_string();
        // take the third letter of the input as the rank
        let rank = input.chars().nth(2).unwrap().to_string();
        let rank = rank.parse::<i8>().unwrap();

        // convert file to i8
        let file = file_to_num(&pawn_file);

            // This is sticky! We need to return the piece kind as a string, but we don't know what it is yet.
        let promote_to = &input[input.len() - 2..input.len() - 1];

        return Some((pawn_file, file, rank, promote_to.to_string()));


    }


    let repawn = Regex::new(r"(?m)^[a-h][1-7]$").unwrap();
    if repawn.is_match(&input) {

        // take the first letter of the input as the file
        let pawn_file = input.chars().nth(0).unwrap().to_string();
        // take the second letter of the input as the rank
        let rank = input.chars().nth(1).unwrap().to_string();
        let rank = rank.parse::<i8>().unwrap();

        // convert file to i8
        let file = file_to_num(&pawn_file);

        return Some((pawn_file, file, rank, "none".to_string()));
    }

    // standard moves or takes
    let re = Regex::new(r"(?m)^[RNBQK][a-h][1-8]$|^[a-h]x[a-h][1-8]$|^[RNBQK]x[a-h][1-7]$").unwrap();
    // use the regex string ^[a-zA-Z].*[a-zA-Z][0-9]$ to check if the input is valid
    if !re.is_match(&input) {
        println!("NO MATCH. Please try again.");
        return None;
    }

    // Take the first character of the input as the piece kind
    let piece_kind = input.chars().nth(0).unwrap().to_string();

    // Take the second to last character of the input as the file
    let file_letter = &input[input.len() - 3..input.len() - 2];
    let file = file_to_num(file_letter);

    // Take the final character of the input as the rank
    let rank = &input[input.len() - 2..input.len() - 1];
    let rank = rank.parse::<i8>().unwrap();


    return Some((piece_kind, file, rank, "none".to_string()));
}

fn file_to_num (file: &str) -> i8 {
    let file = file.to_lowercase();
    let file = match &file[..] {
        "a" => 1,
        "b" => 2,
        "c" => 3,
        "d" => 4,
        "e" => 5,
        "f" => 6,
        "g" => 7,
        "h" => 8,
        _ => 0,
    };
    return file;
}



// if whose_turn == true {
//     // check if rook on h1 can move to f1
//     for piece in all_pieces.iter() {
//         if piece.get_rank() == &1 && piece.get_file() == &8 && piece.get_id() == &"R".to_string() && piece.get_color() ==  &true {
//             if piece.get_legal(1, 6, &map) == true {
//                 // remove the rook from all_pieces
//                 let mut index = 0;
//                 for piece in all_pieces.iter() {
//                     if piece.get_rank() == &1 && piece.get_file() == &8 && piece.get_id() == &"R".to_string() && piece.get_color() == &true {
//                         all_pieces.remove(index);
//                         break
//                     }
//                     index += 1;
//                 }
//                 // add the rook to f1
//                 let moved_piece = create_moved_piece("R".to_string(), 1, 6, true, "x".to_string());
//                 let moved_piece = match moved_piece {
//                     Some(moved_piece) => moved_piece,
//                     None => continue,
//                 };
//                 all_pieces.push(moved_piece);

//                 // remove the king from all_pieces
//                 let mut index = 0;
//                 for piece in all_pieces.iter() {
//                     if piece.get_rank() == &1 && piece.get_file() == &5 && piece.get_id() == &"K".to_string() && piece.get_color() == &true {
//                         all_pieces.remove(index);
//                         break
//                     }
//                     index += 1;
//                 }
//                 // add the king to g1
//                 let moved_piece = create_moved_piece("K".to_string(), 1, 7, true, "x".to_string());
//                 let moved_piece = match moved_piece {
//                     Some(moved_piece) => moved_piece,
//                     None => continue,
//                 };
//                 all_pieces.push(moved_piece);

//             }
//         }
//     }