use itertools::Itertools;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Coordinate {
    x: i32,
    y: i32,
}

pub struct Bounds(i32, i32, i32, i32);

impl Coordinate {
    pub fn is_neighboor(&self, other: &Coordinate) -> bool {
        let neighboors = {
            let Some(zone_bounds) = get_bounds(&vec![self.clone()]) else {
                unreachable!()
            };
            Self::gen_zone(&zone_bounds.get_large())
                .into_iter()
                .filter(|c| c != self)
                .collect_vec()
        };
        return neighboors.iter().find(|&c| c == other).is_some();
    }

    pub fn gen_zone(bounds: &Bounds) -> Vec<Coordinate> {
        let &Bounds(min_x, max_x, min_y, max_y) = bounds;
        let capacity = ((max_x - min_x) * (max_y - min_y)).try_into().expect("conversion should fit.");
        let mut zone = Vec::with_capacity(capacity);
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                zone.push(Self { x, y });
            }
        }
        return zone;
    }
}

impl Bounds {
    pub fn get_large(&self) -> Self {
        let Self(min_x, max_x, min_y, max_y) = self;
        Self(min_x - 1, max_x + 1, min_y - 1, max_y + 1)
    }
}

pub fn game_step(cells: Vec<Coordinate>) -> Vec<Coordinate> {
    let _bounds = get_bounds(&cells);
    todo!()
}

pub fn get_bounds(coordinates: &Vec<Coordinate>) -> Option<Bounds> {
    if coordinates.len() == 0 {
        return None;
    }
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (i32::MAX, i32::MIN, i32::MAX, i32::MIN);
    for &Coordinate { x, y } in coordinates {
        min_x = if x < min_x { x } else { min_x };
        max_x = if x > max_x { x } else { max_x };
        min_y = if y < min_x { y } else { min_y };
        max_y = if y > max_y { y } else { max_y };
    }
    return Some(Bounds(min_x, max_x, min_y, max_y));
}

pub fn count_live_neighboors(subject: &Coordinate, cells: Vec<Coordinate>) -> usize {
    return cells.iter().filter(|c| c.is_neighboor(subject)).count();
}

pub fn is_alive(subject: &Coordinate, cells: Vec<Coordinate>) -> bool {
    let alive = cells.iter().find(|&c| c == subject).is_some();
    let nb_neighboors = count_live_neighboors(subject, cells);
    match alive {
        true => match nb_neighboors {
            nb if nb < 2 => false,
            nb if nb > 3 => false,
            _ => true,
        },
        false => nb_neighboors == 3,
    }
}
