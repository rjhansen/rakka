mod map;

use map::get_point;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use super::*;

    #[test]
    fn it_works() {
        if let Some(p1) = get_point("0101") {
            assert_eq!(p1.hex, "0101");
            println!("found hex 0101 ok");
            if let Some(p2) = p1.project2d(PI/2.0, 3.0) {
                assert_eq!(p2.hex, "0401");
                println!("reached p2 ok");
                if let Some(p3) = p2.project2d(PI, 3.0) {
                    assert_eq!(p3.hex, "0404");
                    println!("reached p3 ok")
                } else {
                    panic!("couldn't move to p3")
                }
            } else {
                panic!("couldn't move to p2");
            }
        } else {
            panic!("couldn't find hex 0101");
        };
    }
}
