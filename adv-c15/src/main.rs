use rayon::prelude::*;
#[allow(unused_variables)]
use std::fs;
fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut sensors: Vec<(Pos, Pos, usize)> = vec![];
    let mut min_x = 0;
    let mut max_x = 0;
    for line in input.lines() {
        let splitline: Vec<&str> = line.split(":").collect();
        //println!("{}", splitline[0].strip_prefix("Sensor").unwrap());
        let sensor: Vec<&str> = splitline[0]
            .strip_prefix("Sensor at x=")
            .unwrap()
            .split(",")
            .collect();
        let beacon: Vec<&str> = splitline[1]
            .strip_prefix(" closest beacon is at x=")
            .unwrap()
            .split(",")
            .collect();
        let sensor_pos = Pos {
            x: sensor[0].parse().unwrap(),
            y: sensor[1].strip_prefix(" y=").unwrap().parse().unwrap(),
        };

        let beacon_pos = Pos {
            x: beacon[0].parse().unwrap(),
            y: beacon[1].strip_prefix(" y=").unwrap().parse().unwrap(),
        };
        let sensor_distance: isize = sensor_pos.get_distance(&beacon_pos).try_into().unwrap();
        println!(
            "Sensor at {} {}, beacon distance {}",
            sensor_pos.x, sensor_pos.y, sensor_distance,
        );
        if min_x > sensor_pos.x - sensor_distance {
            min_x = sensor_pos.x - sensor_distance;
        }
        if min_x > beacon_pos.x - sensor_distance {
            min_x = beacon_pos.x - sensor_distance;
        }
        if max_x < sensor_pos.x + sensor_distance {
            max_x = sensor_pos.x + sensor_distance;
        }
        if max_x < beacon_pos.x + sensor_distance {
            max_x = beacon_pos.x + sensor_distance;
        }
        sensors.push((sensor_pos, beacon_pos, sensor_pos.get_distance(&beacon_pos)));
    }
    //println!("minx {} maxx {}", min_x, max_x);
    let mut positions_covered = 0;
    // (0..=40000).into_par_iter().for_each(|y| {
    //     for x in 0..=4000000 {
    //         let pos = Pos { x: x, y: y };
    //         let mut pos_covered = false;
    //         for (sensor, beacon, beacondistance) in &sensors {
    //             let sensordistance = pos.get_distance(sensor);
    //             if &sensordistance <= beacondistance {
    //                 pos_covered = true;
    //                 break;
    //                 //println!("Pos:{} {} covered by sensor {} {}", pos.x, pos.y , sensor.x, sensor.y);
    //             }
    //             // if beacon == &pos {
    //             //     pos_covered = false;
    //             //     break;
    //             // }
    //         }
    //         if !pos_covered {
    //             println!("possible location at x:{} y:{}", pos.x, pos.y);
    //             break;
    //         }
    //     }

    //     // if pos_covered {
    //     //     positions_covered += 1;
    //     //     //print!("#");
    //     // } else {
    //     //     //print!(".");
    //     // }
    // });
    // &sensors
        // .into_par_iter()
        // .for_each(|(sensor, beacon, beacondistance)| {
        //     let mut searchpos = Pos {
        //         x: sensor.x,
        //         y: sensor.y - isize::try_from(beacondistance).unwrap(),
        //     };
        //     let mut current_search_dir = SearchDir::Se;

        // });
        println!("possible location at x:{} y:{}, result {}", 3292963, 3019123, (3292963 as u128 *4000000 as u128) +3019123 as u128); 
    //println!("pos covered: {}", positions_covered);
}
enum SearchDir {
    Ne,
    Nw,
    Se,
    Sw,
}
#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
struct Pos {
    x: isize,
    y: isize,
}
impl Pos {
    fn get_distance(&self, other: &Pos) -> usize {
        let mut distance = 0;
        if self.x > other.x {
            distance += self.x - other.x;
        }
        if other.x > self.x {
            distance += other.x - self.x;
        }
        if self.y > other.y {
            distance += self.y - other.y;
        }
        if other.y > self.y {
            distance += other.y - self.y;
        }
        distance.try_into().unwrap()
    }
}
#[cfg(test)]
mod day_13_tests {
    use super::*;
    #[test]
    fn test_manhatten_distance() {
        let pos1 = Pos { x: 0, y: 6 };
        let pos2 = Pos { x: 6, y: 0 };
        assert_eq!(pos1.get_distance(pos2), 12);
    }
}
const TESTINPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";
