pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return vec![];
    }

    let rows = minefield.len();
    let cols = minefield.get(0).unwrap().len();
    let mut result = create_intermediate_output(minefield);

    (0..rows).for_each(|i| {
        (0..cols).for_each(|j| {
            if result[i][j] == '*' {
                update_surrounding_blocks(&mut result, i as i32, j as i32);
            }
        })
    });

    result
        .into_iter()
        .map(|v| v.into_iter().collect::<String>())
        .collect()
}

fn update_surrounding_blocks(minefield: &mut Vec<Vec<char>>, i: i32, j: i32) {
    let rows = minefield.len() as i32;
    let cols = minefield[0].len() as i32;
    (-1i32..2).for_each(|x| {
        (-1i32..2).for_each(|y| {
            let r = i + x;
            let c = j + y;
            if r < rows && r >= 0 && c < cols && c >= 0 && minefield[r as usize][c as usize] != '*'
            {
                if minefield[r as usize][c as usize] == ' ' {
                    minefield[r as usize][c as usize] = '1';
                } else {
                    minefield[r as usize][c as usize] =
                        (minefield[r as usize][c as usize] as u8 + 1) as char;
                }
            }
        })
    });
}

fn create_intermediate_output(minefield: &[&str]) -> Vec<Vec<char>> {
    minefield
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}
