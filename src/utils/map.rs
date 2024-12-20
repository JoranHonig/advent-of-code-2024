use std::fmt;


#[macro_export]
macro_rules! define_nodes {

    ($($symbol:expr => $variant:ident),* $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        enum Node {
            $(
                $variant,
            )*
        }

        impl TryFrom<char> for Node {
            type Error = ();

            fn try_from(value: char) -> Result<Self, Self::Error> {
                match value {
                    $(
                        $symbol => Ok(Node::$variant),
                    )*
                    _ => {
                        eprintln!("Unknown symbol: {}", value);
                        Err(())
                    },
                }
            }
        }

        impl std::fmt::Display for Node {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Node::$variant => write!(f, "{}", $symbol),
                    )*
                }
            }
        }


    };
}

// pub(crate) use define_nodes;

const NEIGHBOR_OFFSETS: [(i32, i32); 4] = [
    (-1, 0),
    (0, -1),
    (0, 1),
    (1, 0),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn turn_one(&self, reverse: bool) -> Direction {
        if !reverse {
            match self {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            }
        } else {
            match self {
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
            }
        }
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

pub trait Map<T> where T: Clone + PartialEq + Eq {
    fn dimensions(&self) -> (usize, usize);


    fn get(&self, position: &Position) -> Option<&T>;
    fn get_mut(&mut self, position: &Position) -> Option<&mut T>;

    fn position_iter(&self) -> Box<dyn Iterator<Item=Position>> {
        let (max_x, max_y) = self.dimensions();
        Box::new((0..max_x).flat_map(move |x| (0..max_y).map(move |y| Position { x: x as u32, y: y as u32 })))
    }

    fn neighbours(&self, position: &Position) -> Vec<Position> {
        let Position { x, y } = position;

        let (width, height) = self.dimensions();
        NEIGHBOR_OFFSETS
            .iter()
            .map(|(dx, dy)| (*x as i32 + dx, *y as i32 + dy))
            .filter(|(x, y)| *x >= 0 && *y >= 0 && width as i32 > *x && height as i32 > * y)
            .map(|(x, y)| Position { x: x as u32, y: y as u32 })
            .collect()
    }


    fn neighbour_in_direction(&self, position: Position, direction: Direction) -> Option<Position> {
        let Position { x, y } = position;

        match direction {
            Direction::Up => {
                if y == 0 {
                    None
                } else {
                    Some(Position { x, y: y - 1 })
                }
            }
            Direction::Down => {
                if y == self.dimensions().1 as u32 - 1 {
                    None
                } else {
                    Some(Position { x, y: y + 1 })
                }
            }
            Direction::Left => {
                if x == 0 {
                    None
                } else {
                    Some(Position { x: x - 1, y })
                }
            }
            Direction::Right => {
                if x == self.dimensions().0 as u32 - 1 {
                    None
                } else {
                    Some(Position { x: x + 1, y })
                }
            }
        }
    }

    fn swap(&mut self, a: &Position, b: &Position) {
        let temp = self.get(a).unwrap().clone();
        *self.get_mut(a).unwrap() = self.get(b).unwrap().clone();
        *self.get_mut(b).unwrap() = temp;
    }

}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BoxedMap<T> {
    pub data: Vec<Vec<Box<T>>>,
    bounds: (usize, usize),
}

impl<T> TryFrom<Vec<Vec<T>>> for BoxedMap<T> {
    type Error = ();

    fn try_from(data: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        let bounds = (data.get(0).map(|row| row.len()).unwrap_or(0), data.len());
        let data = data.into_iter().map(|row| row.into_iter().map(|item| Box::new(item)).collect()).collect();
        Ok(BoxedMap { data, bounds })
    }
}

impl<T: Clone + PartialEq + Eq + fmt::Display> Map<T> for BoxedMap<T> {
    fn dimensions(&self) -> (usize, usize) {
        self.bounds
    }

    fn get(&self, position: &Position) -> Option<&T> {
        let Position { x, y } = position;
        self.data.get(*y as usize).and_then(|row| row.get(*x as usize)).map(|item| &**item)
    }

    fn get_mut(&mut self, position: &Position) -> Option<&mut T> {
        let Position { x, y } = position;
        self.data.get_mut(*y as usize).and_then(|row| row.get_mut(*x as usize)).map(|item| &mut **item)
    }

}

impl<T: fmt::Display> fmt::Display for BoxedMap<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.data.iter().map(|row|
            {
                row.iter().map(|n| write!(f, "{}", n)).collect::<Result<Vec<_>, _>>()?;
                writeln!(f)?;
                Ok(())
            }
        ).collect::<Result<Vec<_>, _>>()?;
        Ok(())
    }
}
