pub fn on_board_and_diff(rank: &i8, file: &i8, nrank: &i8, nfile: &i8) -> bool {
    if nfile < &1 || nfile > &8 || nrank < &1 || nrank > &8 {
        false
    } else if rank == nrank && file == nfile {  
        false
    } else {
        true
    }
}

pub fn trace_move(rank: &i8, file: &i8, nrank: &i8, nfile: &i8, map: &Vec<Vec<&str>>) -> bool {
    // handle lateral move 
    if rank == nrank {
        if file < nfile {
            for i in file+1..*file {
                if map[*rank as usize - 1][i as usize - 1] != "x" {
                    return false
                }
            }
        } else {
            for i in nfile+1..*file {
                if map[*rank as usize - 1][i as usize - 1] != "x" {
                    return false
                }
            }
        }
    }

    if file == nfile {
        if rank < nrank {
            for i in rank+1..*nrank {
                if map[i as usize - 1][*file as usize - 1] != "x" {
                    return false
                }
            }
        } else {
            for i in nrank+1..*rank {
                if map[i as usize - 1][*file as usize - 1] != "x" {
                    return false
                }
            }
        }
    }

    // handle diagonal move
    for i in 1..8 {
        if rank + i == *nrank && file + i == *nfile {
            for j in 1..i {
                if map[(rank + j) as usize - 1][(file + j) as usize - 1] != "x" {
                    return false
                }
            }
        }
        if rank + i == *nrank && file - i == *nfile {
            for j in 1..i {
                if map[(rank + j) as usize - 1][(file - j) as usize - 1] != "x" {
                    return false
                }
            }
        }
        if rank - i == *nrank && file + i == *nfile {
            for j in 1..i {
                if map[(rank - j) as usize - 1][(file + j) as usize - 1] != "x" {
                    return false
                }
            }
        }
        if rank - i == *nrank && file - i == *nfile {
            for j in 1..i {
                if map[(rank - j) as usize - 1][(file - j) as usize - 1] != "x" {
                    return false
                }
            }
        }
    }
    
    return true;

}


pub fn check_check(white_turn: &bool, krank: &i8, kfile: &i8, map: &Vec<Vec<&str>>) -> bool {
    if white_turn == &true {
        // check for the first non 'x' piece in horizontal and vertical directions
        for i in 1..8 {
            if map[*krank as usize - 1][i as usize - 1] != "x" {
                if map[*krank as usize - 1][i as usize - 1] == "BR" || map[*krank as usize - 1][i as usize - 1] == "BQ" {
                    return true;
                }
            }
            if map[i as usize - 1][*kfile as usize - 1] != "x" {
                if map[i as usize - 1][*kfile as usize - 1] == "BR" || map[i as usize - 1][*kfile as usize - 1] == "BQ" {
                    return true;
                }
            }
        }

        // check for the first non 'x' piece in diagonal directions
        for i in 1..8 {
            if map[i as usize - 1][i as usize - 1] != "x" {
                if map[i as usize - 1][i as usize - 1] == "BQ" || map[i as usize - 1][i as usize - 1] == "BB" {
                    return true;
                }
            }
            if map[i as usize - 1][8 - i as usize] != "x" {
                if map[i as usize - 1][8 - i as usize] == "BQ" || map[i as usize - 1][8 - i as usize] == "BB" {
                    return true;
                }
            }
            if map[8 - i as usize][i as usize - 1] != "x" {
                if map[8 - i as usize][i as usize - 1] == "BQ" || map[8 - i as usize][i as usize - 1] == "BB" {
                    return true;
                }
            }
            if map[8 - i as usize][8 - i as usize] != "x" {
                if map[8 - i as usize][8 - i as usize] == "BQ" || map[8 - i as usize][8 - i as usize] == "BB" {
                    return true;
                }
            }
        }

        // check for knights
        // if map[*krank as usize - 2][*kfile as usize] == "BN" || map[*krank as usize - 2][*kfile as usize - 2] == "BN" 
        // || map[*krank as usize][*kfile as usize - 2] == "BN" || map[*krank as usize][*kfile as usize] == "BN"  
        // || map[*krank as usize - 2][*kfile as usize - 2] == "BN" || map[*krank as usize - 1][*kfile as usize - 1] == "BN"
        // || map[*krank as usize - 1][*kfile as usize] == "BN" || map[*krank as usize][*kfile as usize - 1] == "BN" {
        //     return true;
        // }

        // check for pawns
        if map[*krank as usize - 1][*kfile as usize] == "BP" || map[*krank as usize - 1][*kfile as usize - 2] == "BP" {
            return true;
        }

    } else { // repeat for black

        // check for horizontal and vertical moves
        for i in 1..8 {
            if map[*krank as usize - 1][i as usize - 1] != "x" {
                if map[*krank as usize - 1][i as usize - 1] == "WR" || map[*krank as usize - 1][i as usize - 1] == "WQ" {
                    return true;
                }
            }
            if map[i as usize - 1][*kfile as usize - 1] != "x" {
                if map[i as usize - 1][*kfile as usize - 1] == "WR" || map[i as usize - 1][*kfile as usize - 1] == "WQ" {
                    return true;
                }
            }
        }

        // check for diagonal moves
        for i in 1..8 {
            if map[i as usize - 1][i as usize - 1] != "x" {
                if map[i as usize - 1][i as usize - 1] == "WQ" || map[i as usize - 1][i as usize - 1] == "WB" {
                    return true;
                }
            }
            if map[i as usize - 1][8 - i as usize] != "x" {
                if map[i as usize - 1][8 - i as usize] == "WQ" || map[i as usize - 1][8 - i as usize] == "WB" {
                    return true;
                }
            }
        }
        // check for knights
        // if map[*krank as usize - 2][*kfile as usize] == "WN" || map[*krank as usize - 2][*kfile as usize - 2] == "WN"
        // || map[*krank as usize][*kfile as usize - 2] == "WN" || map[*krank as usize][*kfile as usize] == "WN"
        // || map[*krank as usize - 2][*kfile as usize - 2] == "WN" || map[*krank as usize - 1][*kfile as usize - 1] == "WN"
        // || map[*krank as usize - 1][*kfile as usize] == "WN" || map[*krank as usize][*kfile as usize - 1] == "WN" {
        //     return true;
        // }

        // check for pawns
        if map[*krank as usize - 1][*kfile as usize] == "WP" || map[*krank as usize - 1][*kfile as usize - 2] == "WP" {
            return true;
        }

    }
    return false;
}


