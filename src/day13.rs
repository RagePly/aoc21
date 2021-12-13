// Conditional Compilation

pub const WAS_COMPILED: bool = cfg!(feature = "day13");

#[cfg(not(feature = "day13"))]
pub fn part1(_source: String) -> String {
    super::comp_features::fetch_default(13, 1)
}

#[cfg(not(feature = "day13"))]
pub fn part2(_source: String) -> String {
    super::comp_features::fetch_default(13, 2)
}

// v START SOLUTION v
#[cfg(feature = "day13")]
type Points = std::collections::HashSet<(usize, usize)>;

#[cfg(feature = "day13")]
pub fn part1(source: String) -> usize {
    let input:Vec<_> = source.splitn(2, "\r\n\r\n").collect();
    let fold: (bool, usize) = input[1]
    .splitn(2, "\r\n")
    .map(|line| {
        line
        .split_ascii_whitespace()
        .skip(2)
        .map(|split| {
            let mut terms = split.split("=");
            (terms.next().unwrap() == "x", terms.next().unwrap().parse().unwrap())
        })
        .next()
        .unwrap()
    })
    .next()
    .unwrap();
    
    let points: Points = input[0]
    .split("\r\n")
    .map(|line| {
        let coords: Vec<_> = line.split(",").map(|coord| coord.parse().unwrap()).collect();
        if fold.0 && coords[0] > fold.1 {
            (2*fold.1 - coords[0], coords[1])
        } else if !fold.0 && coords[1] > fold.1 {
            (coords[0], 2*fold.1 - coords[1])
        } else {
            (coords[0], coords[1])
        }
    })
    .collect();
    points.len()
}

#[cfg(feature = "day13")]
pub fn part2(source: String) -> String {
    let input:Vec<_> = source.splitn(2, "\r\n\r\n").collect();
    let mut points: Points = input[0]
    .split("\r\n")
    .map(|line| {
        let coords: Vec<_> = line.split(",").map(|coord| coord.parse().unwrap()).collect();
        (coords[0], coords[1])
    })
    .collect();

    let mut width = 0;
    let mut height = 0;
    
    input[1]
    .split("\r\n")
    .for_each(|line| {
        let fold: (bool, usize) = line
        .split_ascii_whitespace()
        .skip(2)
        .map(|split| {
            let mut terms = split.split("=");
            (terms.next().unwrap() == "x", terms.next().unwrap().parse().unwrap())
        })
        .next()
        .unwrap();

        if fold.0 {
            width = fold.1;
        } else {
            height = fold.1;
        }

        points = points
        .iter()
        .map(|coords|
        if fold.0 && coords.0 > fold.1 {
            (2*fold.1 - coords.0, coords.1)
        } else if !fold.0 && coords.1 > fold.1 {
            (coords.0, 2*fold.1 - coords.1)
        } else {
            (coords.0, coords.1)
        })
        .collect()
    });

    let mut map: Vec<Vec<char>> = Vec::new();
    map.resize(height, {
        let mut row: Vec<char> = Vec::new(); 
        row.resize(width, ' '); 
        row
    });

    points
    .into_iter()
    .for_each(|(x, y)| map[y][x] = '#');

    map
    .into_iter()
    .map(|row| {
        String::from("\n") + row.iter().map(|c| format!("{} ", c)).collect::<String>().as_str()
    }).collect::<String>()
}
