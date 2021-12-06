
use crate::state::Field;


pub fn get_moore_neighborhood(

    field: &Field, 
    position: (usize, usize)

) -> Vec<(usize, usize)> {

    let mut neighborhood: Vec<(usize, usize)> = Vec::new();
    let x = position.0 as i32;
    let y = position.1 as i32;
    for dx in [-1, 0, 1].iter() {
        if (x + dx) < 0 || (x + dx) >= field.dimensions.0 as i32 { continue; }
        for dy in [-1, 0, 1].iter() {
            if *dx == 0 && *dy == 0 { continue }
            if (y + dy) < 0 || (y + dy) >= field.dimensions.1 as i32 { continue; }
            neighborhood.push(((x+dx) as usize, (y+dy) as usize));
        }
    }
    neighborhood
}

pub fn get_neumann_neighborhood(

    field: &Field, 
    position: (usize, usize)

) -> Vec<(usize, usize)> {

    let neighborhood: Vec<(usize, usize)> = Vec::new();
    // TODO
    neighborhood
}

