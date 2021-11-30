type BinaryGridMap = Vec<Vec<bool>>;
#[allow(dead_code)]
pub struct BinaryGrid {
    map: BinaryGridMap,
    w: usize,
    h: usize
}

impl BinaryGrid {
    pub fn draw(&self) {
        for r in &self.map {
            for c in r {
                print!("{}", if *c { "# " } else { ". " });
            }
            println!();
        }
    }

    pub fn new() -> BinaryGrid {
        BinaryGrid {
            map: BinaryGridMap::new(),
            w: 0,
            h: 0
        }
    }

    pub fn init(width: usize, height: usize, state: bool) -> BinaryGrid {
        let mut map: BinaryGridMap = BinaryGridMap::new();

        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(state); 
            }
            map.push(row);
        }

        BinaryGrid {
            map: map,
            w: width,
            h: height
        }
    }
}