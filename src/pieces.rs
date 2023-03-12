use super::legality::on_board_and_diff;

pub enum Piece {
    Knight(Knight),
    King(King),
    Queen(Queen),
    Bishop(Bishop),
    Rook(Rook),
    Pawn(Pawn),
}

impl Piece {
    pub fn get_kind(&self) -> &str {
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

    pub fn get_rank(&self) -> &i8 {
        match self {
            Piece::Knight(p) => &p.rank,
            Piece::King(p) => &p.rank,
            Piece::Queen(p) => &p.rank,
            Piece::Bishop(p) => &p.rank,
            Piece::Rook(p) => &p.rank,
            Piece::Pawn(p) => &p.rank,
        }
    }

    pub fn get_file(&self) -> &i8 {
        match self {
            Piece::Knight(p) => &p.file,
            Piece::King(p) => &p.file,
            Piece::Queen(p) => &p.file,
            Piece::Bishop(p) => &p.file,
            Piece::Rook(p) => &p.file,
            Piece::Pawn(p) => &p.file,
        }
    }

    pub fn get_color(&self) -> &bool {
        match self {
            Piece::Knight(p) => &p.yt,
            Piece::King(p) => &p.yt,
            Piece::Queen(p) => &p.yt,
            Piece::Bishop(p) => &p.yt,
            Piece::Rook(p) => &p.yt,
            Piece::Pawn(p) => &p.yt,
        }
    }

    pub fn get_legal(&self, nrank: i8, nfile: i8) -> bool {
        match self {
            Piece::Knight(p) => p.legal(nrank, nfile),
            Piece::King(p) => p.legal(nrank, nfile),
            Piece::Queen(p) => p.legal(nrank, nfile),
            Piece::Bishop(p) => p.legal(nrank, nfile),
            Piece::Rook(p) => p.legal(nrank, nfile),
            Piece::Pawn(p) => p.legal(nrank, nfile),
        }
    }



    pub fn get_id(&self) -> &String {
        match self {
            Piece::Knight(p) => &p.id,
            Piece::King(p) => &p.id,
            Piece::Queen(p) => &p.id,
            Piece::Bishop(p) => &p.id,
            Piece::Rook(p) => &p.id,
            Piece::Pawn(p) => &p.id,
        }
    }


    pub fn return_piece(self) -> Piece {
        match self {
            Piece::Knight(p) => Piece::Knight(Knight{rank: p.rank, file: p.file, yt: p.yt, id: p.id}),
            Piece::King(p) => Piece::King(King{rank: p.rank, file: p.file, yt: p.yt, id: p.id}),
            Piece::Queen(p) => Piece::Queen(Queen{rank: p.rank, file: p.file, yt: p.yt, id: p.id}),
            Piece::Bishop(p) => Piece::Bishop(Bishop{rank: p.rank, file: p.file, yt: p.yt, id: p.id}),
            Piece::Rook(p) => Piece::Rook(Rook{rank: p.rank, file: p.file, yt: p.yt, id: p.id}),
            Piece::Pawn(p) => Piece::Pawn(Pawn{rank: p.rank, file: p.file, yt: p.yt, id: p.id}),
        }
    }

}

pub struct Pawn {
    pub rank: i8,
    pub file: i8,
    pub yt: bool,
    pub id: String,
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

pub struct Rook {
    pub rank: i8,
    pub file: i8,
    pub yt: bool,
    pub id: String,
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


pub struct Knight {
    pub rank: i8,
    pub file: i8,
    pub yt: bool,
    pub id: String,
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


pub struct Bishop {
    pub rank: i8,
    pub file: i8,
    pub yt: bool,
    pub id: String,
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


pub struct Queen {
    pub rank: i8,   
    pub file: i8,
    pub yt: bool,
    pub id: String,
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


pub struct King {
    pub rank: i8,
    pub file: i8,
    pub yt: bool,
    pub id: String,
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





// return a list of all pieces in initial positions
pub fn gather_pieces() -> Vec<Piece> {
    let w_apawn = Piece::Pawn(Pawn{rank:2, file:1, yt:true, id:String::from("a")});
    let w_bpawn = Piece::Pawn(Pawn{rank:2, file:2, yt:true, id:String::from("b")});
    let w_cpawn = Piece::Pawn(Pawn{rank:2, file:3, yt:true, id:String::from("c")});
    let w_dpawn = Piece::Pawn(Pawn{rank:2, file:4, yt:true, id:String::from("d")});
    let w_epawn = Piece::Pawn(Pawn{rank:2, file:5, yt:true, id:String::from("e")});
    let w_fpawn = Piece::Pawn(Pawn{rank:2, file:6, yt:true, id:String::from("f")});
    let w_gpawn = Piece::Pawn(Pawn{rank:2, file:7, yt:true, id:String::from("g")});
    let w_hpawn = Piece::Pawn(Pawn{rank:2, file:8, yt:true, id:String::from("h")});
    
    let w_arook = Piece::Rook(Rook{rank:1, file:1, yt:true, id:String::from("R")});
    let w_bknight = Piece::Knight(Knight{rank:1, file:2, yt:true, id:String::from("N")});
    let w_cbishop = Piece::Bishop(Bishop{rank:1, file:3, yt:true, id:String::from("B")});
    let w_queen = Piece::Queen(Queen{rank:1, file:4, yt:true, id:String::from("Q")});
    let w_king = Piece::King(King{rank:1, file:5, yt:true, id:String::from("K")});
    let w_fbishop = Piece::Bishop(Bishop{rank:1, file:6, yt:true, id:String::from("B")});
    let w_gknight = Piece::Knight(Knight{rank:1, file:7, yt:true, id:String::from("N")});
    let w_hrook = Piece::Rook(Rook{rank:1, file:8, yt:true, id:String::from("R")});

    let b_apawn = Piece::Pawn(Pawn{rank:7, file:1, yt:false, id:String::from("a")});
    let b_bpawn = Piece::Pawn(Pawn{rank:7, file:2, yt:false, id:String::from("b")});
    let b_cpawn = Piece::Pawn(Pawn{rank:7, file:3, yt:false, id:String::from("c")});
    let b_dpawn = Piece::Pawn(Pawn{rank:7, file:4, yt:false, id:String::from("d")});
    let b_epawn = Piece::Pawn(Pawn{rank:7, file:5, yt:false, id:String::from("e")});
    let b_fpawn = Piece::Pawn(Pawn{rank:7, file:6, yt:false, id:String::from("f")});
    let b_gpawn = Piece::Pawn(Pawn{rank:7, file:7, yt:false, id:String::from("g")});
    let b_hpawn = Piece::Pawn(Pawn{rank:7, file:8, yt:false, id:String::from("h")});

    let b_arook = Piece::Rook(Rook{rank:8, file:1, yt:false, id:String::from("R")});
    let b_bknight = Piece::Knight(Knight{rank:8, file:2, yt:false, id:String::from("N")});
    let b_cbishop = Piece::Bishop(Bishop{rank:8, file:3, yt:false, id:String::from("B")});
    let b_queen = Piece::Queen(Queen{rank:8, file:4, yt:false, id:String::from("Q")});
    let b_king = Piece::King(King{rank:8, file:5, yt:false, id:String::from("K")});
    let b_fbishop = Piece::Bishop(Bishop{rank:8, file:6, yt:false, id:String::from("B")});
    let b_gknight = Piece::Knight(Knight{rank:8, file:7, yt:false, id:String::from("N")});
    let b_hrook = Piece::Rook(Rook{rank:8, file:8, yt:false, id:String::from("R")});



    let pieces = vec![w_apawn, w_bpawn, w_cpawn, w_dpawn, w_epawn, w_fpawn, w_gpawn, w_hpawn,
                      w_arook, w_bknight, w_cbishop, w_queen, w_king, w_fbishop, w_gknight, w_hrook,
                      b_apawn, b_bpawn, b_cpawn, b_dpawn, b_epawn, b_fpawn, b_gpawn, b_hpawn, 
                      b_arook, b_bknight, b_cbishop, b_queen, b_king, b_fbishop, b_gknight, b_hrook];

    return pieces;
}


