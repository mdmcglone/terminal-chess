pub fn render_board(blocktop: &str, map: &Vec<Vec<&str>>) {
    for letter in 0..=7 {  // print block number down

        for i in 0..=8 {
            print!("{blocktop}"); // print top row
        }

        print!("\n");
        for layer in 0..= 4 {  // print block height
            for number in 0..=7 {  // print block number across

                let p = render_piece(map[7-letter][number]);

                //insert piece[layer] into the center of blockbody
                // let mut blockbody_filled = String::from("|           ");
                // let mut blockbody_filled = String::from("|...........");
                let odd_or_even = letter+number;
                let mut blockbody_filled = match odd_or_even%2 {
                    0 => String::from("|..........."),
                    1 => String::from("|           "),
                    _ => String::from("|..........."),
                };


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



fn render_piece(kind: &str) -> [&str; 5] {
    match kind {
        "WP" => return ["   o   ", "  /=\\  ", "  \\=/  ", "  /=\\  ", " /===\\ "],
        "BP" => return ["   o   ", "  / \\  ", "  \\ /  ", "  / \\  ", " /___\\ "],
        "WR" => return ["|| ||| ||","|=======|"," |=====| "," |=====| ","|=======|"],
        "BR" => return ["|| ||| ||","|       |"," |     | "," |     | ","|_______|"],
        "WN" => return ["   ______","  /=== o "," /L ====="," \\~~_____","      \\=="],
        "BN" => return ["   ______","  /    o "," /L      "," \\~~_____","      \\__"],
        "WB" => return ["    o    ","   /=\\   ","  /===\\  "," /JESUS\\ ","~\\=====/~"],
        "BB" => return ["    o    ","   / \\   ","  /   \\  "," /JESUS\\ ","~\\_____/~"],
        "WQ" => return ["O O O O O","\\=|=|=|=/"," \\=====/ ","  \\===/  "," ======= "],
        "BQ" => return ["O O O O O","\\ | | | /"," \\     / ","  \\   /  "," ======= "],
        "WK" => return [" ___+___ "," \\=====/ "," /==|==\\ ","/===|===\\","========="],
        "BK" => return [" ___+___ "," \\     / "," /  |  \\ ","/   |   \\","========="],
        _ => return ["","","","",""],
    }
}

// white pawn
//   .
//  /=\
//  \=/
//  /=\
// /===\

// black pawn
//   .
//  / \
//  \ /
//  / \
// /   \

// white rook
// || ||| ||
// |=======|
//  |=====|
//  |=====|
// |=======|

// black rook
// || ||| ||
// |       |
//  |     |
//  |     |
// |_______|

// white knight
//    ______
//   /=== o 
//  /L =====
//  \≈≈_____
//       \==

// black knight
//    ______
//   /    o 
//  /L      
//  \~~_____
//       \__

// white bishop
//      o
//     /=\
//    /===\
//   /= + =\
//  ~\=====/~

// black bishop
//      o
//     / \
//    /   \
//   /  +  \
//  ~\     /~

// white queen
//O O O O O
//\=|=|=|=/
// \=====/
//  \===/
// =======

// black queen
//O O O O O
//\ | | | /
// \     /
//  \   /
// =======


// white king
//  ___+___
//  \=====/
//  /==|==\
// /===|===\
// =========


// black king
//  ___+___
//  \     /
//  /  |  \
// /   |   \
// ==========


// color square white:
// -------------
// |···········|
// |···········|
// |···········|
// |···········|
// -------------

// black bishop
//      +
//    /   \
//   |jesus|
//    \   /
//   ~~\ /~~

// white bishop
//      +
//    /   \
//   |jesus|
//    \   /
//   ~~\ /~~

