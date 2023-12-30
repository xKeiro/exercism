pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.as_bytes()
                .iter()
                .enumerate()
                .map(|(x, &cell)| get_annotation(cell, minefield, x, y))
                .collect()
        })
        .collect()
}

fn get_annotation(cell: u8, minefield: &[&str], x: usize, y: usize) -> char {
    if cell == b'*' {
        '*'
    } else {
        let neighbours = get_neighbours(minefield, x, y);
        match get_neighbour_mine_count(&neighbours) {
            0 => ' ',
            n => char::from_digit(n as u32, 10).unwrap(),
        }
    }
}

fn get_neighbours<'a>(minefield: &'a [&'a str], x: usize, y: usize) -> Vec<&u8> {
    let width = minefield[0].len();
    let height = minefield.len();
    (-1..=1)
        .flat_map(|xi| (-1..=1).map(move |yi| (xi, yi)))
        .filter(|&(xi, yi)| !(xi == 0 && yi == 0))
        .filter_map(|(xi, yi)| {
            let new_x = x as i8 + xi;
            let new_y = y as i8 + yi;
            if new_x < 0 || new_x >= width as i8 || new_y < 0 || new_y >= height as i8 {
                None
            } else {
                Some(&minefield[new_y as usize].as_bytes()[new_x as usize])
            }
        })
        .collect()
}

fn get_neighbour_mine_count(neighbours: &[&u8]) -> usize {
    neighbours.iter().filter(|&&cell| *cell == b'*').count()
}
