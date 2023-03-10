use chess::pieces::gather_pieces;
use chess::pieces::{Piece, Pawn, Knight, Bishop, Rook, Queen, King};
use chess::board::render_board;

fn main() {
    let blocktop = "-----------";  
    // let blockbody = "|           ";    // block widths as square


    let mut all_pieces = gather_pieces();

    let empty = String::from("x");

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

    let file = &all_pieces[0].get_file();

    println!("file: {}", file);



    

}



