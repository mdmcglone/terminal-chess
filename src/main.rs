fn main() {
    let blocktop = "-----------";  
    let blockbody = "|           ";    // block widths as square


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
                

    render_board(blocktop, blockbody, map);

    // all_pieces[0].move_piece(2, 3);    

    

}


fn render_board(blocktop: &str, blockbody: &str, map: Vec<Vec<&str>>) {
    for letter in 0..=7 {  // print block number down

        for i in 0..=8 {
            print!("{blocktop}"); // print top row
        }

        print!("\n");
        for layer in 0..= 4 {  // print block height
            for number in 0..=7 {  // print block number across

                let p = render_piece(map[7-letter][number]);

                //insert piece[layer] into the center of blockbody
                let mut blockbody_filled = String::from(blockbody); // copy blockbody
                let len_to_span = blockbody_filled.len() - p[layer].len(); // get length of blockbody to span
                blockbody_filled = String::from(&blockbody_filled[..len_to_span]);  // trim blockbody to size of p[layer]
                blockbody_filled.insert_str(blockbody_filled.len()/2+1, p[layer]); // insert p[layer] into center of blockbody

                // if number == 0, add rank number
                if number == 0 {
                    if layer == 2 {
                        print!("{} {blockbody_filled}", 8 - letter); // print first layer with rank number
                    } else {
                        print!("  {blockbody_filled}"); // print first layer without rank number
                    }
                } else {

                print!("{blockbody_filled}"); // print other layers
                }
            }
            print!("|\n");
        }

    }

    for i in 0..=8 {  // print bottom row
        print!("{blocktop}");
    }
    print!("\n  ");
    for file in ["a","b","c","d","e","f","g","h"] {  // print letters across
        print!("      {file}     ");
    }
    print!("\n");
}

enum Piece {
    Knight(Knight),
    King(King),
    Queen(Queen),
    Bishop(Bishop),
    Rook(Rook),
    Pawn(Pawn),
}

impl Piece {
    fn get_kind(&self) -> &str {
        match self {
            Piece::Knight(p) => { 
                if p.yt == true {
                    "WN"
                } else {
                    "BN"
                }
            },
            Piece::King(p) => {
                if p.yt == true {
                    "WK"
                } else {
                    "BK"
                }
            }
            Piece::Queen(p) => {
                if p.yt == true {
                    "WQ"
                } else {
                    "BQ"
                }
            }
            Piece::Bishop(p) => {
                if p.yt == true {
                    "WB"
                } else {
                    "BB"
                }
            }
            Piece::Rook(p) => {
                if p.yt == true {
                    "WR"
                } else {
                    "BR"
                }
            }
            Piece::Pawn(p) => {
                if p.yt == true {
                    "WP"
                } else {
                    "BP"
                }
            }
        }
    }

    fn get_rank(&self) -> &i8 {
        match self {
            Piece::Knight(p) => &p.rank,
            Piece::King(p) => &p.rank,
            Piece::Queen(p) => &p.rank,
            Piece::Bishop(p) => &p.rank,
            Piece::Rook(p) => &p.rank,
            Piece::Pawn(p) => &p.rank,
        }
    }

    fn get_file(&self) -> &i8 {
        match self {
            Piece::Knight(p) => &p.file,
            Piece::King(p) => &p.file,
            Piece::Queen(p) => &p.file,
            Piece::Bishop(p) => &p.file,
            Piece::Rook(p) => &p.file,
            Piece::Pawn(p) => &p.file,
        }
    }



    fn find_id(&self) -> &String {
        match self {
            Piece::Knight(p) => &p.id,
            Piece::King(p) => &p.id,
            Piece::Queen(p) => &p.id,
            Piece::Bishop(p) => &p.id,
            Piece::Rook(p) => &p.id,
            Piece::Pawn(p) => &p.id,
        }
    }

    fn move_piece(self, rank: i8, file: i8) {
        match self {
            Piece::Knight(mut p) => p.change(rank, file),
            Piece::King(mut p) => p.change(rank, file),
            Piece::Queen(mut p) => p.change(rank, file),
            Piece::Bishop(mut p) => p.change(rank, file),
            Piece::Rook(mut p) => p.change(rank, file),
            Piece::Pawn(mut p) => p.change(rank, file),
        }
    }

}

struct Pawn {
    rank: i8,
    file: i8,
    yt: bool,
    id: String,
}

impl Pawn {
    fn legal(&self, nrank: i8, nfile: i8) -> bool {
        if on_board_and_diff(&self.rank, &self.file, &nrank, &nfile) == false {
            return false;
        }

        if self.yt == true {
            let attack = false;
            if attack == false {
                if self.rank == nrank + 1 {
                    true
                } else {
                    false
                }
            } else {
                if self.rank == nrank + 1 && (self.file == nfile + 1 || self.file == nfile - 1) {
                    true
                } else {
                    false
                }
            }
        } else {
            if self.rank == nrank - 1 {
                true
            } else {
                false
            }
        }
    }

    fn change(&mut self, nrank: i8, nfile: i8) {
        if self.legal(nrank, nfile) == true{
            self.rank = nrank;
            self.file = nfile;
        }
    }

}

struct Rook {
    rank: i8,
    file: i8,
    yt: bool,
    id: String,
}

impl Rook {    
    fn legal(&self, nrank: i8, nfile: i8) -> bool {
        if on_board_and_diff(&self.rank, &self.file, &nrank, &nfile) == false {
            return false;
        }

        if self.rank == nrank || self.file == nfile {
            true
        } else {
            false
        }
    }

    fn change(&mut self, nrank: i8, nfile: i8) {
        if self.legal(nrank, nfile) == true{
            self.rank = nrank;
            self.file = nfile;
        }
    }

}


struct Knight {
    rank: i8,
    file: i8,
    yt: bool,
    id: String,
}

impl Knight {    
    fn legal(&self, nrank: i8, nfile: i8) -> bool {
        if on_board_and_diff(&self.rank, &self.file, &nrank, &nfile) == false {
            return false;
        }

        let pos_c = [self.rank-nrank, self.file-nfile];

        if pos_c == [1,2] || pos_c == [2,1] ||
           pos_c == [-1,2] || pos_c == [-2,1] ||
           pos_c == [-1,-2] || pos_c == [-2,-1] ||
           pos_c == [1,-2] || pos_c == [2,-1] {
            true
           } else {
            false
           }
    }

    fn change(&mut self, nrank: i8, nfile: i8) {
        if self.legal(nrank, nfile) == true{
            self.rank = nrank;
            self.file = nfile;
        }
    }

}


struct Bishop {
    rank: i8,
    file: i8,
    yt: bool,
    id: String,
}

impl Bishop {    
    fn legal(&self, nrank: i8, nfile: i8) -> bool {
        if on_board_and_diff(&self.rank, &self.file, &nrank, &nfile) == false {
            return false;
        }

        for b in 0..=7 {
            if (self.rank + b == nrank && self.file + b == nfile) ||
               (self.rank + b == nrank && self.file - b == nfile) ||
               (self.rank - b == nrank && self.file + b == nfile) ||
               (self.rank - b == nrank && self.file - b == nfile) {
                return true;
            } 
        }

        false
        
    }

    fn change(&mut self, nrank: i8, nfile: i8) {
        if self.legal(nrank, nfile) == true{
            self.rank = nrank;
            self.file = nfile;
        }
    }

}


struct Queen {
    rank: i8,
    file: i8,
    yt: bool,
    id: String,
}

impl Queen {
    fn legal(&self, nrank: i8, nfile: i8) -> bool {
        if on_board_and_diff(&self.rank, &self.file, &nrank, &nfile) == false {
            return false;
        }

        if self.rank == nrank || self.file == nfile {
            return true;
        }

        for b in 0..=7 {
            if (self.rank + b == nrank && self.file + b == nfile) ||
            (self.rank + b == nrank && self.file - b == nfile) ||
            (self.rank - b == nrank && self.file + b == nfile) ||
            (self.rank - b == nrank && self.file - b == nfile) {
                return true;
            } 
        } 

        false
        
    }

    fn change(&mut self, nrank: i8, nfile: i8) {
        if self.legal(nrank, nfile) == true{
            self.rank = nrank;
            self.file = nfile;
        }
    }

}


struct King {
    rank: i8,
    file: i8,
    yt: bool,
    id: String,
}

impl King {    
    fn legal(&self, nrank: i8, nfile: i8) -> bool {
        if on_board_and_diff(&self.rank, &self.file, &nrank, &nfile) == false {
            return false;
        }

        let pos_c = [self.rank-nrank, self.file-nfile];

        if pos_c == [0,1] || pos_c == [1,0] ||
           pos_c == [1,1] || pos_c == [-1,-1] ||
           pos_c == [0,-1] || pos_c == [-1,0] ||
           pos_c == [-1,1] || pos_c == [1,-1] {
            true
           } else {
            false
           }
    }

    fn change(&mut self, nrank: i8, nfile: i8) {
        if self.legal(nrank, nfile) == true{
            self.rank = nrank;
            self.file = nfile;
        }
    }

}



fn on_board_and_diff(rank: &i8, file: &i8, nrank: &i8, nfile: &i8) -> bool {
    if nfile < &1 || nfile > &8 || nrank < &1 || nrank > &8 {
        false
    } else if rank == nrank && file == nfile {  
        false
    } else {
        true
    }
}

fn render_piece(kind: &str) -> [&str; 5] {
    match kind {
        "WP" => return ["   0   ", "  /-\\  ", "  \\-/  ", "  /-\\  ", " /---\\ "],
        "BP" => return ["   0   ", "  / \\  ", "  \\ /  ", "  / \\  ", " /   \\ "],
        "WR" => return ["|| ||| ||","|-------|"," |-----| "," |-----| ","|-------|"],
        "BR" => return ["|| ||| ||","|       |"," |     | "," |     | ","|       |"],
        "WN" => return ["  ______"," /--- o ","/L -----","\\~~_____","     \\--"],
        "BN" => return ["  ______"," /    o ","/L      ","\\~~_____","     \\__"],
        "WB" => return ["    O    ","   /-\\   ","  /---\\  "," /- + -\\ ","~\\-----/~"],
        "BB" => return ["    O    ","   / \\   ","  /   \\  "," /  +  \\ ","~\\     /~"],
        "WQ" => return ["O O O O O","\\-|-|-|-/"," \\-----/ ","  \\---/  "," ======= "],
        "BQ" => return ["O O O O O","\\ | | | /"," \\     / ","  \\   /  "," ======= "],
        "WK" => return ["  __+__  "," /--|--\\ "," \\--|--/ "," /--|--\\ ","========="],
        "BK" => return ["  __+__  "," /  |  \\ "," \\  |  / "," /  |  \\ ","========="],
        _ => return ["","","","",""],
    }
}



// return a list of all pieces in initial positions
fn gather_pieces() -> Vec<Piece> {
    let w_apawn = Piece::Pawn(Pawn{rank:2, file:1, yt:true, id:String::from("w_apawn")});
    let w_bpawn = Piece::Pawn(Pawn{rank:2, file:2, yt:true, id:String::from("w_bpawn")});
    let w_cpawn = Piece::Pawn(Pawn{rank:2, file:3, yt:true, id:String::from("w_cpawn")});
    let w_dpawn = Piece::Pawn(Pawn{rank:2, file:4, yt:true, id:String::from("w_dpawn")});
    let w_epawn = Piece::Pawn(Pawn{rank:2, file:5, yt:true, id:String::from("w_epawn")});
    let w_fpawn = Piece::Pawn(Pawn{rank:2, file:6, yt:true, id:String::from("w_fpawn")});
    let w_gpawn = Piece::Pawn(Pawn{rank:2, file:7, yt:true, id:String::from("w_gpawn")});
    let w_hpawn = Piece::Pawn(Pawn{rank:2, file:8, yt:true, id:String::from("w_hpawn")});
    
    let w_arook = Piece::Rook(Rook{rank:1, file:1, yt:true, id:String::from("w_arook")});
    let w_bknight = Piece::Knight(Knight{rank:1, file:2, yt:true, id:String::from("w_bknight")});
    let w_cbishop = Piece::Bishop(Bishop{rank:1, file:3, yt:true, id:String::from("w_cbishop")});
    let w_queen = Piece::Queen(Queen{rank:1, file:4, yt:true, id:String::from("w_queen")});
    let w_king = Piece::King(King{rank:1, file:5, yt:true, id:String::from("w_king")});
    let w_fbishop = Piece::Bishop(Bishop{rank:1, file:6, yt:true, id:String::from("w_fbishop")});
    let w_gknight = Piece::Knight(Knight{rank:1, file:7, yt:true, id:String::from("w_gknight")});
    let w_hrook = Piece::Rook(Rook{rank:1, file:8, yt:true, id:String::from("w_hrook")});

    let b_apawn = Piece::Pawn(Pawn{rank:7, file:1, yt:false, id:String::from("b_apawn")});
    let b_bpawn = Piece::Pawn(Pawn{rank:7, file:2, yt:false, id:String::from("b_bpawn")});
    let b_cpawn = Piece::Pawn(Pawn{rank:7, file:3, yt:false, id:String::from("b_cpawn")});
    let b_dpawn = Piece::Pawn(Pawn{rank:7, file:4, yt:false, id:String::from("b_dpawn")});
    let b_epawn = Piece::Pawn(Pawn{rank:7, file:5, yt:false, id:String::from("b_epawn")});
    let b_fpawn = Piece::Pawn(Pawn{rank:7, file:6, yt:false, id:String::from("b_fpawn")});
    let b_gpawn = Piece::Pawn(Pawn{rank:7, file:7, yt:false, id:String::from("b_gpawn")});
    let b_hpawn = Piece::Pawn(Pawn{rank:7, file:8, yt:false, id:String::from("b_hpawn")});

    let b_arook = Piece::Rook(Rook{rank:8, file:1, yt:false, id:String::from("b_arook")});
    let b_bknight = Piece::Knight(Knight{rank:8, file:2, yt:false, id:String::from("b_bknight")});
    let b_cbishop = Piece::Bishop(Bishop{rank:8, file:3, yt:false, id:String::from("b_cbishop")});
    let b_queen = Piece::Queen(Queen{rank:8, file:4, yt:false, id:String::from("b_queen")});
    let b_king = Piece::King(King{rank:8, file:5, yt:false, id:String::from("b_king")});
    let b_fbishop = Piece::Bishop(Bishop{rank:8, file:6, yt:false, id:String::from("b_fbishop")});
    let b_gknight = Piece::Knight(Knight{rank:8, file:7, yt:false, id:String::from("b_gknight")});
    let b_hrook = Piece::Rook(Rook{rank:8, file:8, yt:false, id:String::from("b_hrook")});



    let pieces = vec![w_apawn, w_bpawn, w_cpawn, w_dpawn, w_epawn, w_fpawn, w_gpawn, w_hpawn,
                      w_arook, w_bknight, w_cbishop, w_queen, w_king, w_fbishop, w_gknight, w_hrook,
                      b_apawn, b_bpawn, b_cpawn, b_dpawn, b_epawn, b_fpawn, b_gpawn, b_hpawn, 
                      b_arook, b_bknight, b_cbishop, b_queen, b_king, b_fbishop, b_gknight, b_hrook];

    return pieces;
}

fn generate_blockbody(letter: &usize, number: &usize) -> String {
    if letter + number % 2 == 0 {
        return String::from("|           ");
    } else {
        return String::from("|xxxxxxxxxxx");
    }
}

// white pawn
//   0
//  /-\
//  \-/
//  /-\
// /---\

// black pawn
//   0
//  / \
//  \ /
//  / \
// /   \

// white rook
// || ||| ||
// |-------|
//  |-----|
//  |-----|
// |-------|

// black rook
// || ||| ||
// |       |
//  |     |
//  |     |
// |       |

// white knight
//    ______
//   /--- o 
//  /L -----
//  \~~_____
//       \--

// black knight
//    ______
//   /    o 
//  /L      
//  \~~_____
//       \__

// white bishop
//      O
//     /-\
//    /---\
//   /- + -\
//  ~\-----/~

// black bishop
//      O
//     / \
//    /   \
//   /  +  \
//  ~\     /~

// white queen
//O O O O O
//\-|-|-|-/
// \-----/
//  \---/
// =======

// black queen
//O O O O O
//\ | | | /
// \     /
//  \   /
// =======


// white king
//   __+__
//  /--|--\
//  \--|--/
//  /--|--\
// =========


// black king
//   __+__
//  /  |  \
//  \  |  /
//  /  |  \
// =========


// color square white:
// -------------
// |···········|
// |···········|
// |···········|
// |···········|
// -------------

