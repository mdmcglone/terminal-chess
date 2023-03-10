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



    pub fn find_id(&self) -> &String {
        match self {
            Piece::Knight(p) => &p.id,
            Piece::King(p) => &p.id,
            Piece::Queen(p) => &p.id,
            Piece::Bishop(p) => &p.id,
            Piece::Rook(p) => &p.id,
            Piece::Pawn(p) => &p.id,
        }
    }

    pub fn move_piece(self, rank: i8, file: i8) {
        match self {
            Piece::Knight(mut p) => p.change(rank, file),
            Piece::King(mut p) => p.change(rank, file),
            Piece::Queen(mut p) => p.change(rank, file),
            Piece::Bishop(mut p) => p.change(rank, file),
            Piece::Rook(mut p) => p.change(rank, file),
            Piece::Pawn(mut p) => p.change(rank, file),
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

pub struct Rook {
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


pub struct Knight {
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


pub struct Bishop {
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


pub struct Queen {
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


pub struct King {
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





// return a list of all pieces in initial positions
pub fn gather_pieces() -> Vec<Piece> {
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


