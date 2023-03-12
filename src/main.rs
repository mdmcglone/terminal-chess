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
        for piece in all_pieces.iter_mut() {

            let ifile = piece.get_rank();
            let irank = piece.get_file();
            
            let file = *ifile as usize;
            let rank = *irank as usize;
            
            map[file-1][rank-1] = piece.get_kind();

        };
                    

        render_board(blocktop, map);

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

        println!("You entered: {}", user_input);

        let parsed_input = parse_input(user_input);




        
        // extract move_to tuple from the option
        let (piece_kind, file, rank) = match parsed_input {
            Some((piece_kind, file, rank)) => (piece_kind, file, rank),
            None => continue,
        };

        let mut some_legal_move = 0;
        let mut original_piece = (&0, &0, &String::from("x"));
        for piece in all_pieces.iter() {
            // check that this move does not overlap with another piece on the same team
            if piece.get_rank() == &rank && piece.get_file() == &file && piece.get_color() == &whose_turn {
                println!("You cannot move to a square occupied by your own piece. Please try again.");
                continue
            }

            // check that this move is legal for some piece
            if piece.get_legal(rank, file) == true && piece.get_color() == &whose_turn && piece.get_id() == &piece_kind {
                some_legal_move += 1;
                println!("this move is legal for the {} at {} {}", piece.get_kind(), piece.get_rank(), piece.get_file());
                original_piece = (piece.get_rank(), piece.get_file(), piece.get_id());
            } 
        }

        if some_legal_move == 1 {
            println!("Moving {} to {} {}", piece_kind, file, rank);
        } else if some_legal_move > 1 {
            println!("There are multiple pieces that can move to this square. Please disambiguate.");
            continue
        } else {
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

        index = 0;
        for piece in all_pieces.iter() {
            if piece.get_rank() == &rank && piece.get_file() == &file && piece.get_color() != &whose_turn {
                all_pieces.remove(index);
                break
            }
            index += 1;
        }

        let moved_piece = match &piece_kind[..] {
            "R" => Piece::Rook(Rook{rank: rank, file: file, yt: whose_turn, id: piece_kind}),
            "N" => Piece::Knight(Knight{rank: rank, file: file, yt: whose_turn, id: piece_kind}),
            "B" => Piece::Bishop(Bishop{rank: rank, file: file, yt: whose_turn, id: piece_kind}),
            "Q" => Piece::Queen(Queen{rank: rank, file: file, yt: whose_turn, id: piece_kind}),
            "K" => Piece::King(King{rank: rank, file: file, yt: whose_turn, id: piece_kind}),
            "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" => Piece::Pawn(Pawn{rank: rank, file: file, yt: whose_turn, id: piece_kind}),
            _ => continue,
        };

        // add the moved piece to all_pieces
        all_pieces.push(moved_piece);



        turn_count += 1;
    }

}




fn parse_input(input: String) -> Option<(String, i8, i8)> {

    let recastle = Regex::new(r"(?m)^O-O$|^O-O-O$").unwrap();
    if recastle.is_match(&input) {
        println!("CASTLING");
    } 

    let re = Regex::new(r"(?m)^[a-h][1-8]$|^[RNBQK].{0,2}[a-h][1-8]|^[a-h].{0,2}[a-h][1-8]$").unwrap();
    // use the regex string ^[a-zA-Z].*[a-zA-Z][0-9]$ to check if the input is valid
    if !re.is_match(&input) {
        println!("NO MATCH. Please try again.");
        return None;
    }

    // handle castling
    if input == "O-O" {
        return Some((String::from("O-O"), 0, 0));
    } else if input == "O-O-O" {
        return Some((String::from("O-O-O"), 0, 0));
    }

    // Take the first character of the input as the piece kind
    let piece_kind = input.chars().nth(0).unwrap().to_string();

    // Take the second to last character of the input as the file
    let file_letter = &input[input.len() - 3..input.len() - 2];
    let file = file_to_num(file_letter);

    // Take the final character of the input as the rank
    let rank = &input[input.len() - 2..input.len() - 1];
    let rank = rank.parse::<i8>().unwrap();


    return Some((piece_kind, file, rank));
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
