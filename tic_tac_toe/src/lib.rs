pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    if diagonals('X', table) || horizontal('X', table) || vertical('X', table) {
        return "player X won".to_string()
    }
    if diagonals('O', table) || horizontal('O', table) || vertical('O', table) {
        return "player O won".to_string()
    }
    "tie".to_string()
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    let mut count_main = 0;
    let mut count_anti = 0;

    for i in 0..3 {
        if table[i][i] == player {
            count_main += 1;
        }

        if table[i][2-i] == player {
            count_anti += 1;
        }
    }

    count_main == 3 || count_anti == 3
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    for row in table {
    let mut countplayer: i32 = 0;
    for &cell in row.iter() {
        if cell == player {
            countplayer += 1;
        }
        if countplayer == 3 {
            return true
        }
    }
}
 false
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
let mut countplayer: i32 = 0;
let mut from: i32 = 0;
let mut to: i32 = 1;
for _ in table {
    for slc2 in table {
    for hor in from..to {
        if player == slc2[hor as usize] {
            countplayer += 1;
        }
    }
    if countplayer == 3 {
        return true
    }
}

if to == table.len().try_into().unwrap()  {
    break
}
countplayer = 0;

from += 1;
to += 1;
}
return false 
}