pub fn tic_tac_toe(table: [[char;3];3]) -> String {
    if horizontal('O', table) || vertical('O', table) || diagonals('O', table) {
        return "player O won".to_string();
    }else if horizontal('X', table) || vertical('X', table) || diagonals('X', table) {
        return "player X won".to_string();
    }else {
        for row in table.iter() {
            for &cell in row.iter() {
                if cell == '#' {
                    return "tie".to_string();
                }
            }
        }
        return "tie".to_string();
    }
}

pub fn horizontal(player: char, table: [[char;3];3]) -> bool {
    for row in table.iter() {
        if row[0] == player && row[1] == player && row[2] == player {
            return true;
        }
    }
    return false;
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    for col in 0..3 {
        if table[0][col] == player && table[1][col] == player && table[2][col] == player {
            return true;
        }
    }
    false
}


pub fn diagonals(player: char, table: [[char;3];3]) -> bool {
    if table[0][0] == player && table[1][1] == player && table[2][2] == player {
        return true;
    }

    if table[0][2] == player && table[1][1] == player && table[2][0] == player {
        return true;
    }
    false
}
