fn main() {
    let blocktop = "------------";  
    let blockbody = "|            ";    // block widths as square


    let all_pieces = gather_pieces();

    let empty = String::from("x");
       
    let map: Vec<Vec<&str>> = vec![
        vec![&all_pieces[0].match_kind(), &all_pieces[1].match_kind(), &all_pieces[2].match_kind(), &all_pieces[3].match_kind(), &all_pieces[4].match_kind(), &all_pieces[5].match_kind(), &all_pieces[6].match_kind(), &all_pieces[7].match_kind()],
        vec![&all_pieces[8].match_kind(), &all_pieces[9].match_kind(), &all_pieces[10].match_kind(), &all_pieces[11].match_kind(), &all_pieces[12].match_kind(), &all_pieces[13].match_kind(), &all_pieces[14].match_kind(), &all_pieces[15].match_kind()],
        vec![&empty, &empty, &empty, &empty, &empty, &empty, &empty, &empty],
        vec![&empty, &empty, &empty, &empty, &empty, &empty, &empty, &empty],
        vec![&empty, &empty, &empty, &empty, &empty, &empty, &empty, &empty],
        vec![&empty, &empty, &empty, &empty, &empty, &empty, &empty, &empty],
        vec![&all_pieces[16].match_kind(), &all_pieces[17].match_kind(), &all_pieces[18].match_kind(), &all_pieces[19].match_kind(), &all_pieces[20].match_kind(), &all_pieces[21].match_kind(), &all_pieces[22].match_kind(), &all_pieces[23].match_kind()],
        vec![&all_pieces[24].match_kind(), &all_pieces[25].match_kind(), &all_pieces[26].match_kind(), &all_pieces[27].match_kind(), &all_pieces[28].match_kind(), &all_pieces[29].match_kind(), &all_pieces[30].match_kind(), &all_pieces[31].match_kind()],
        ];
           
        
    let find_rook = map[0][0];

    // println!("{}", find_rook);

    // iterate thru all pieces

    render_board(blocktop, blockbody);

    println!("{:?}", map);

    // let w = King{
    //     rank: 0,
    //     file: 0,
    //     yt: true,
    // };

    // let b = King{
    //     rank: 0,
    //     file: 0,
    //     yt: false,
    // };
    
    // for i in 0..5 {
    //     println!("{}", w.render()[i])
    // }

    // for i in 0..5 {
    //     println!("{}", b.render()[i])
    // }
 

}


fn render_board(blocktop: &str, blockbody: &str) {
    for letter in 1..=8 {  // print block number down

        for i in 0..=8 {
            print!("{blocktop}");
        }
        print!("\n");
        for i in 0..= 4 {  // print block height
            for number in 0..=8 {  // print block number across

                // if piece exists, print piece
                // render piece by splitting the piece's signature at \n into a slice which inserts in the center of blockbody
                // else, print blockbody


                // if number == 0, add rank number
                if number == 0 {
                    if i == 2 {
                        print!("{} {blockbody}", 9 - letter, blockbody=blockbody);
                    } else {
                        print!("  {blockbody}");
                    }
                } else {

                print!("{blockbody}"); 
                }
            }
            print!("\n");
        }

    }
    for i in 0..=8 {  // print bottom row
        print!("{blocktop}");
    }
    print!("\n  ");
    for file in ["a","b","c","d","e","f","g","h"] {  // print letters across
        print!("      {file}      ");
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
    fn match_kind(&self) -> &str {
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

    fn find_id(&self) -> &String {
        match self {
            Piece::Knight(k) => &k.id,
            Piece::King(k) => &k.id,
            Piece::Queen(k) => &k.id,
            Piece::Bishop(k) => &k.id,
            Piece::Rook(k) => &k.id,
            Piece::Pawn(k) => &k.id,
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
    fn legal(&self, nrank: i8, nfile: i8, attack: bool) -> bool {
        if on_board_and_diff(&self.rank, &self.file, &nrank, &nfile) == false {
            return false;
        }

        if self.yt == true {
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

    fn change(&mut self, nrank: i8, nfile: i8, attack: bool) {
        if self.legal(nrank, nfile, attack) == true{
            self.rank = nrank;
            self.file = nfile;
        }
    }

    fn render(&self) -> [&str; 5] {
        if self.yt == true {
            return ["   ()", "  /--\\", "  \\--/", "  /--\\", " /----\\"]
        } else {
            return ["   ()", "  /  \\", "  \\  /", "  /  \\", " /    \\"]
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
    fn legal(&self, nrank: i8, nfile: i8, attack: bool) -> bool {
        if on_board_and_diff(&self.rank, &self.file, &nrank, &nfile) == false {
            return false;
        }

        if self.rank == nrank || self.file == nfile {
            true
        } else {
            false
        }
    }

    fn change(&mut self, nrank: i8, nfile: i8, attack: bool) {
        if self.legal(nrank, nfile, attack) == true{
            self.rank = nrank;
            self.file = nfile;
        }
    }

    fn render(&self) -> [&str; 5] {
        if self.yt == true {
            return ["|| || ||","|------|"," |----| "," |----| ","|------|"] 
        } else {
            return ["|| || ||","|      |"," |    | "," |    | ","|      |"]
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
    fn legal(&self, nrank: i8, nfile: i8, attack: bool) -> bool {
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

    fn change(&mut self, nrank: i8, nfile: i8, attack: bool) {
        if self.legal(nrank, nfile, attack) == true{
            self.rank = nrank;
            self.file = nfile;
        }
    }

    fn render(&self) -> [&str; 5] {
        if self.yt == true {  
            return ["  ______"," /--- o ","/L -----","\\~~_____","     \\--"]
        } else { 
            return ["  ______"," /    o ","/L      ","\\~~_____","     \\__"]
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
    fn legal(&self, nrank: i8, nfile: i8, attack: bool) -> bool {
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

    fn change(&mut self, nrank: i8, nfile: i8, attack: bool) {
        if self.legal(nrank, nfile, attack) == true{
            self.rank = nrank;
            self.file = nfile;
        }
    }

    fn render(&self) -> [&str; 5] {
        if self.yt == true {  
            return ["    O    ","   /-\\   ","  /---\\  "," /- + -\\ ","~\\-----/~"]
        } else { 
            return ["    O    ","   / \\   ","  /   \\  "," /  +  \\ ","~\\     /~"];
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
    fn legal(&self, nrank: i8, nfile: i8, attack: bool) -> bool {
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

    fn change(&mut self, nrank: i8, nfile: i8, attack: bool) {
        if self.legal(nrank, nfile, attack) == true{
            self.rank = nrank;
            self.file = nfile;
        }
    }

    fn render(&self) -> [&str; 5] {
        if self.yt == true {  
            return ["O O O O O","\\-|-|-|-/"," \\-----/ ","  \\---/  "," ======= "];
        } else { 
            return ["O O O O O","\\ | | | /"," \\     / ","  \\   /  "," ======= "];
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
    fn legal(&self, nrank: i8, nfile: i8, attack: bool) -> bool {
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

    fn change(&mut self, nrank: i8, nfile: i8, attack: bool) {
        if self.legal(nrank, nfile, attack) == true{
            self.rank = nrank;
            self.file = nfile;
        }
    }

    fn render(&self) -> [&str; 5] {
        if self.yt == true {  
            return ["  __+__  "," /--|--\\ "," \\--|--/ "," /--|--\\ ","========="];
        } else { 
            return ["  __+__  "," /  |  \\ "," \\  |  / "," /  |  \\ ","========="];
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



// return a list of all pieces in initial positions
fn gather_pieces() -> Vec<Piece> {
    let w_apawn = Piece::Pawn(Pawn{rank:2, file:0, yt:true, id:String::from("w_apawn")});
    let w_bpawn = Piece::Pawn(Pawn{rank:2, file:1, yt:true, id:String::from("w_bpawn")});
    let w_cpawn = Piece::Pawn(Pawn{rank:2, file:2, yt:true, id:String::from("w_cpawn")});
    let w_dpawn = Piece::Pawn(Pawn{rank:2, file:3, yt:true, id:String::from("w_dpawn")});
    let w_epawn = Piece::Pawn(Pawn{rank:2, file:4, yt:true, id:String::from("w_epawn")});
    let w_fpawn = Piece::Pawn(Pawn{rank:2, file:5, yt:true, id:String::from("w_fpawn")});
    let w_gpawn = Piece::Pawn(Pawn{rank:2, file:6, yt:true, id:String::from("w_gpawn")});
    let w_hpawn = Piece::Pawn(Pawn{rank:2, file:7, yt:true, id:String::from("w_hpawn")});
    
    let w_arook = Piece::Rook(Rook{rank:1, file:0, yt:true, id:String::from("w_arook")});
    let w_bknight = Piece::Knight(Knight{rank:1, file:1, yt:true, id:String::from("w_bknight")});
    let w_cbishop = Piece::Bishop(Bishop{rank:1, file:2, yt:true, id:String::from("w_cbishop")});
    let w_queen = Piece::Queen(Queen{rank:1, file:3, yt:true, id:String::from("w_queen")});
    let w_king = Piece::King(King{rank:1, file:4, yt:true, id:String::from("w_king")});
    let w_fbishop = Piece::Bishop(Bishop{rank:1, file:5, yt:true, id:String::from("w_fbishop")});
    let w_gknight = Piece::Knight(Knight{rank:1, file:6, yt:true, id:String::from("w_gknight")});
    let w_hrook = Piece::Rook(Rook{rank:1, file:7, yt:true, id:String::from("w_hrook")});

    let b_apawn = Piece::Pawn(Pawn{rank:7, file:0, yt:false, id:String::from("b_apawn")});
    let b_bpawn = Piece::Pawn(Pawn{rank:7, file:1, yt:false, id:String::from("b_bpawn")});
    let b_cpawn = Piece::Pawn(Pawn{rank:7, file:2, yt:false, id:String::from("b_cpawn")});
    let b_dpawn = Piece::Pawn(Pawn{rank:7, file:3, yt:false, id:String::from("b_dpawn")});
    let b_epawn = Piece::Pawn(Pawn{rank:7, file:4, yt:false, id:String::from("b_epawn")});
    let b_fpawn = Piece::Pawn(Pawn{rank:7, file:5, yt:false, id:String::from("b_fpawn")});
    let b_gpawn = Piece::Pawn(Pawn{rank:7, file:6, yt:false, id:String::from("b_gpawn")});
    let b_hpawn = Piece::Pawn(Pawn{rank:7, file:7, yt:false, id:String::from("b_hpawn")});

    let b_arook = Piece::Rook(Rook{rank:8, file:0, yt:false, id:String::from("b_arook")});
    let b_bknight = Piece::Knight(Knight{rank:8, file:1, yt:false, id:String::from("b_bknight")});
    let b_cbishop = Piece::Bishop(Bishop{rank:8, file:2, yt:false, id:String::from("b_cbishop")});
    let b_queen = Piece::Queen(Queen{rank:8, file:3, yt:false, id:String::from("b_queen")});
    let b_king = Piece::King(King{rank:8, file:4, yt:false, id:String::from("b_king")});
    let b_fbishop = Piece::Bishop(Bishop{rank:8, file:5, yt:false, id:String::from("b_fbishop")});
    let b_gknight = Piece::Knight(Knight{rank:8, file:6, yt:false, id:String::from("b_gknight")});
    let b_hrook = Piece::Rook(Rook{rank:8, file:7, yt:false, id:String::from("b_hrook")});



    let pieces = vec![
                      b_arook, b_bknight, b_cbishop, b_queen, b_king, b_fbishop, b_gknight, b_hrook,
                      b_apawn, b_bpawn, b_cpawn, b_dpawn, b_epawn, b_fpawn, b_gpawn, b_hpawn, 
                      w_apawn, w_bpawn, w_cpawn, w_dpawn, w_epawn, w_fpawn, w_gpawn, w_hpawn,
                      w_arook, w_bknight, w_cbishop, w_queen, w_king, w_fbishop, w_gknight, w_hrook, 
                      ];


    return pieces;
}



// white pawn
//   ()
//  /--\
//  \--/
//  /--\
// /----\

// black pawn
//   ()
//  /  \
//  \  /
//  /  \
// /    \

// white rook
// || || ||
// |------|
//  |----|
//  |----|
// |------|

// black rook
// || || ||
// |      |
//  |    |
//  |    |
// |      |

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
