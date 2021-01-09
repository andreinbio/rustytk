use super::{Point, Vector};

/// Tests if a point is Left|On|Right of an infinite line.
///
/// >0 for P2 left of the line through P0 and P1
//  =0 for P2  on the line
//  <0 for P2  right of the line
fn is_left(p0: &Point, p1: &Point, p2: &Point) -> i32 {
    (p1.x as i32 - p0.x as i32) * (p2.y as i32 - p0.y as i32) - (p2.x as i32 - p0.x as i32) * (p1.y as i32 - p0.y as i32)
}

/// Winding number test for a point in a polygon
/// =0 only when the point is outside
pub fn wn_pnpoly(point: &Point, vectors: &Vec<Vector>) -> u32 {
    let mut winding_number: u32 = 0;

    // loop through all edges of the polygon
    for vector in vectors.iter() { // edge from i] to  V[i+1]
        if vector.start.y <= point.y {
            if vector.end.y > point.y {
                if is_left(&vector.start, &vector.end, point) > 0 {
                    winding_number += 1; // have  a valid up intersect
                }
            }
        } else {
            if vector.end.y <= point.y {
                if is_left(&vector.start, &vector.end, point) < 0 {
                    winding_number -= 1; // have  a valid down intersect
                }
            }
        }
    }

    winding_number
}
