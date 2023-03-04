fn main() {
    let blocktop = "------------";  
    let blockbody = "|            ";    // block widths as square

    render_board(blocktop, blockbody);

    let w = King{
        rank: 0,
        file: 0,
        yt: true,
    };

    let b = King{
        rank: 0,
        file: 0,
        yt: false,
    };
    
    for i in 0..5 {
        println!("{}", w.render()[i])
    }

    for i in 0..5 {
        println!("{}", b.render()[i])
    }
 

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
                        print!("{} {blockbody}", letter, blockbody=blockbody);
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


struct Pawn {
    rank: i8,
    file: i8,
    yt: bool,
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
