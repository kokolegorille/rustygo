#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn next(&self) -> Self {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        match self {
            Color::Black => String::from("X"),
            Color::White => String::from("O"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Point(pub usize, pub usize);

impl Point {
    pub fn neighbors(&self) -> Vec<Point> {
        let mut vec = Vec::new();

        let Point(x, y) = *self;

        if x > 0 { vec.push(Point(x - 1, y)); }
        if y > 0 { vec.push(Point(x, y - 1)); }
        vec.push(Point(x + 1, y));
        vec.push(Point(x, y + 1));
        vec
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GoString {
    color: Color,
    stones: Vec<Point>,
    liberties: Vec<Point>,
}

impl GoString {
    pub fn new(color: Color, stones: Vec<Point>, liberties: Vec<Point>) -> Self {
        Self { color, stones, liberties }
    }

    pub fn num_liberties(&self) -> usize {
        self.liberties.len()
    }

    fn without_liberty(&mut self, point: Point) {
        if let Some(pos) = self.liberties.iter().position(|p| *p == point) {
            self.liberties.remove(pos);
        }
    }

    fn with_liberty(&mut self, point: Point) {
        if let None = self.liberties.iter().position(|p| *p == point) {
            self.liberties.push(point);
        }
    }
}

#[derive(Debug)]
pub struct Board {
    size: usize,
    grid: Vec<Option<GoString>>,
}

impl Board {
    pub fn new(size: usize) -> Self {
        Self { size, grid: vec![None; size.pow(2) ]}
    }

    pub fn place_stone(&mut self, color: Color, point: Point) -> Result<(), &'static str> {
        if ! self.is_on_grid(point) { return Err("Not on grid"); }

        match self.grid[self.point_to_index(&point)] {
            Some(_) => Err("Illegal move"),
            None => {
                let mut adjacent_same_color: Vec<GoString> = vec![];
                let mut adjacent_opposite_color: Vec<GoString> = vec![];
                let mut stones: Vec<Point> = vec![point];
                let mut liberties: Vec<Point> = vec![];

                // println!("Color: {:?}", color);
                // println!("Point: {:?}", point);

                for neighbor in self.neighbors(&point) {
                    match self.get_go_string(neighbor) {
                        Some(go_string) => {
                            if go_string.color == color {
                                if ! adjacent_same_color.contains(&go_string) {
                                    adjacent_same_color.push(go_string);
                                }
                            } else {
                                if ! adjacent_opposite_color.contains(&go_string) {
                                    adjacent_opposite_color.push(go_string);
                                }
                            }
                        },
                        None => liberties.push(neighbor),
                    }
                }

                // println!("Liberties: {:?}", liberties);

                for same_color_string in adjacent_same_color {
                    stones.extend_from_slice(&same_color_string.stones);
                    liberties.extend_from_slice(&same_color_string.liberties);
                    liberties.retain(|el| ! stones.contains(el))
                }

                // println!("Stones: {:?}", stones);
                // println!("Liberties: {:?}", liberties);

                let num_liberties = liberties.len();
                let kill = adjacent_opposite_color
                    .iter()
                    .any(|gs| gs.num_liberties() == 1);

                let go_string = GoString::new(color, stones, liberties);

                // Check if liberties = 0 and killed = [] !

                if num_liberties > 0 || kill {
                    self.replace_string(&go_string);

                    for mut other_color_string in adjacent_opposite_color {
                        other_color_string.without_liberty(point);
                        if other_color_string.num_liberties() > 0 {
                            self.replace_string(&other_color_string);
                        } else {
                            self.remove_string(&other_color_string);
                        }
                    }

                    Ok(())
                } else {
                    Err("Illegal move")
                }

            }
        }
    }

    fn replace_string(&mut self, go_string: &GoString) {
        for point in &go_string.stones {
            let index = self.point_to_index(point);
            self.grid[index] = Some(go_string.clone());
        }
    }

    fn remove_string(&mut self, go_string: &GoString) {
        for point in &go_string.stones {
            for neighbor in point.neighbors() {
                match self.get_go_string(neighbor) {
                    Some(mut gs) => {
                        // println!("GS {:?} {:?}", gs, go_string);

                        // Compare on stones or color! because liberties can be different.
                        if gs.color != go_string.color {
                            gs.with_liberty(*point);
                            self.replace_string(&gs);
                        }
                    },
                    None => continue,
                }
            }

            let index = self.point_to_index(point);
            self.grid[index] = None;
        }
    }

    fn neighbors(&self, point: &Point) -> Vec<Point> {
        point.neighbors()
            .into_iter()
            .filter(|p| self.is_on_grid(*p))
            .collect()
    }

    fn is_on_grid(&self, point: Point) -> bool {
        let Point(x, y) = point;
        x < self.size && y < self.size
    }

    fn get_go_string(&self, point: Point) -> Option<GoString> {
        if ! self.is_on_grid(point) { return None; }

        match &self.grid[self.point_to_index(&point)] {
            Some(go_string) => Some(go_string.clone()),
            None => None,
        }
    }

    fn point_to_index(&self, point: &Point) -> usize {
        let Point(x, y) = point;
        x * self.size + y
    }
}

impl ToString for Board {
    fn to_string(&self) -> String {
        let mut string = String::new();
        for x in 0..self.size {
            for y in 0..self.size {
                match self.get_go_string(Point(x, y)) {
                    Some(gs) => string.push_str(&gs.color.to_string()),
                    None => string.push('.'),
                }
            }
            string.push('\n');
        }
        string
    }
}