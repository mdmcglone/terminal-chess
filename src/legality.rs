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
