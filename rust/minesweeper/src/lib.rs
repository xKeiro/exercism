static NEIGHBOUR_CELL_OFFSETS: &'static [(i8, i8)] = &[
    (-1, -1),   (0, -1),    (1, -1),
    (-1, 0),                (1, 0),
    (-1, 1),    (0, 1),     (1, 1),
];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len();
    if height == 0 {
        return minefield.iter().map(|&s| s.to_string()).collect();
    }
    let width = minefield[0].len();
    (0..height)
        .map(|y| {
            (0..width)
                .map(|x| {
                    let cell = minefield[y].as_bytes()[x];
                    if cell == b'*' {
                        '*'
                    } else {
                        match NEIGHBOUR_CELL_OFFSETS
                            .iter()
                            .map(|(ox, oy)| (x as i8 + ox, y as i8 + oy))
                            .filter(|&(x, y)| {
                                x >= 0 && x < width as i8 && y >= 0 && y < height as i8
                            })
                            .filter(|&(x, y)| {
                                let cell = minefield[y as usize].as_bytes()[x as usize];
                                cell == b'*'
                            })
                            .count()
                        {
                            0 => ' ',
                            n => char::from_digit(n as u32, 10).unwrap(),
                        }
                    }
                })
                .collect()
        })
        .collect()
}
