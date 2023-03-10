pub fn on_board_and_diff(rank: &i8, file: &i8, nrank: &i8, nfile: &i8) -> bool {
    if nfile < &1 || nfile > &8 || nrank < &1 || nrank > &8 {
        false
    } else if rank == nrank && file == nfile {  
        false
    } else {
        true
    }
}