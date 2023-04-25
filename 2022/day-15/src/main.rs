extern crate regex;

pub mod cave;
pub mod utils;

use std::fs;

use cave::Cave;

fn main() {
    let sensors_reports = fs::read_to_string("./Assets/input.txt").unwrap();
    let cave = Cave::parse(&sensors_reports);
    // println!("{:#?}", cave);

    /*   let p1 = cave.count_not_available_raw_point(2000000);
    println!("{p1}"); */

    let p2 = cave.distress_beacon();
    println!("{}", 4000000 * p2.x + p2.y)
}

/*  Computung the diamond:
    1. compute Manhattan distance d of the 2 points: "the taxicab distance between two points (x1,y1) is |x1-x2|+|y1-y2|"
    2. this will be the semi-width and semi-height of the diamond, thus w=2d+1 and h=2d+1 (+1 for the central sensor point)
    3. a point is inside the lozanse only if his Manhattan distance to the sensor is <= d

    Parsing Regex: /Sensor at x=([0-9]+), y=([0-9]+): closest beacon is at x=([0-9]+), y=([0-9]+)/mi
*/
