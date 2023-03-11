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
        let mut whose_turn = String::new();
        if turn_count % 2 == 0 {
            println!("\nIt's white's turn. Enter your move: ");
            whose_turn = String::from("w");
        } else {
            println!("\nIt's black's turn. Enter your move: ");
            whose_turn = String::from("b");
        }

        let mut user_input = String::new();

        io::stdin().read_line(&mut user_input).expect("Failed to read line");

        // If user input is of none type, then the input was invalid, continue the loop
        if parse_input(user_input.clone()).is_none() {
            continue;
        }

        println!("You entered: {}", user_input);


        parse_input(user_input);

        turn_count += 1;
    }

}



fn parse_input(input: String) -> Option<(String, i8, i8)> {
    let re = Regex::new(r".*^[a-h][1-8]$|^[RNBQK].{0,2}[a-h][1-8]|^[a-h]x[a-h][1-8]$|^O-O$|^O-O-O$.*").unwrap();

    // strip input of whitespace
    // let input = input.trim().to_string();

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


    // if file < 1 || rank < 1 || rank > 8 || file > 8 || 
    //    (piece_kind != "R" && piece_kind != "N" && piece_kind != "B" && piece_kind != "Q" && piece_kind != "K" && 
    //    piece_kind != "a" && piece_kind != "b" && piece_kind != "c" && piece_kind != "d" && piece_kind != "e" &&
    //    piece_kind != "f" && piece_kind != "g" && piece_kind != "h" ) {
    //     println!("Invalid input. Please try again.");
    //     return None;
    // }

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
