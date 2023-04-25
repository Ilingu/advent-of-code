use regex::Regex;

use crate::utils::Pos;

#[derive(Debug)]
pub struct Sensor {
    pos: Pos,
    nearest_beacon: Pos,
    max_distance_range: usize,
}

impl Sensor {
    pub fn new(pos: Pos, nearest_beacon: Pos) -> Self {
        Self {
            max_distance_range: Pos::manhattan_distance(&pos, &nearest_beacon),
            pos,
            nearest_beacon,
        }
    }
}

#[derive(Debug)]
pub struct Cave {
    extremum_x: (isize, isize), // (min_x,max_x)
    extremum_y: (isize, isize), // (min_y,max_y)
    sensors: Vec<Sensor>,
    sensors_max_x_range: (isize, isize), // (min_x,max_x)
    sensors_pos: Vec<String>,
    beacons_pos: Vec<String>,
}

impl Cave {
    pub fn parse(input: &str) -> Self {
        let re = Regex::new(
            r"Sensor at x=(-?\d*\.?\d+), y=(-?\d*\.?\d+): closest beacon is at x=(-?\d*\.?\d+), y=(-?\d*\.?\d+)",
        )
        .unwrap();

        let (mut min_x, mut max_x) = (0, 0);
        let (mut min_y, mut max_y) = (0, 0);

        let mut sensors: Vec<Sensor> = vec![];
        let mut beacons_pos: Vec<String> = vec![];
        let mut sensors_pos: Vec<String> = vec![];

        for (i, sensor_report) in input.lines().enumerate() {
            let coords = re.captures(sensor_report).unwrap();

            let (sx, sy) = (
                coords.get(1).unwrap().as_str().parse::<isize>().unwrap(),
                coords.get(2).unwrap().as_str().parse::<isize>().unwrap(),
            );
            let (bx, by) = (
                coords.get(3).unwrap().as_str().parse::<isize>().unwrap(),
                coords.get(4).unwrap().as_str().parse::<isize>().unwrap(),
            );

            let (min_loc_x, max_loc_x) = (
                vec![sx, bx].iter().cloned().min().unwrap(),
                vec![sx, bx].iter().cloned().max().unwrap(),
            );
            if max_loc_x > max_x || i == 0 {
                max_x = max_loc_x
            }
            if min_loc_x < min_x || i == 0 {
                min_x = min_loc_x
            }

            let (min_loc_y, max_loc_y) = (
                vec![sy, by].iter().cloned().min().unwrap(),
                vec![sy, by].iter().cloned().max().unwrap(),
            );
            if max_loc_y > max_y || i == 0 {
                max_y = max_loc_y
            }
            if min_loc_y < min_y || i == 0 {
                min_y = min_loc_y
            }

            beacons_pos.push(Pos::new(bx, by).as_string());
            sensors_pos.push(Pos::new(sx, sy).as_string());
            sensors.push(Sensor::new(Pos::new(sx, sy), Pos::new(bx, by)))
        }

        let (mut min_x_range_sensors, mut max_x_range_sensors) = (0, 0);
        sensors.iter().for_each(|sensor| {
            let (local_min, local_max) = (
                sensor.pos.x - sensor.max_distance_range as isize,
                sensor.pos.x + sensor.max_distance_range as isize,
            );
            if local_min < min_x_range_sensors || min_x_range_sensors == 0 {
                min_x_range_sensors = local_min
            }
            if local_max > max_x_range_sensors || max_x_range_sensors == 0 {
                max_x_range_sensors = local_max
            }
        });

        return Self {
            extremum_x: (min_x, max_x),
            extremum_y: (min_y, max_y),
            sensors,
            sensors_max_x_range: (min_x_range_sensors, max_x_range_sensors),
            sensors_pos,
            beacons_pos,
        };
    }

    pub fn count_not_available_raw_point(&self, raw_id: isize) -> usize {
        if raw_id < self.extremum_y.0 || raw_id > self.extremum_y.1 {
            panic!("invalid line number, out of cave bound")
        }

        let mut count: usize = 0;
        for x in (self.sensors_max_x_range.0)..=(self.sensors_max_x_range.1) {
            let curr_pos = Pos::new(x, raw_id);
            if self.beacons_pos.contains(&curr_pos.as_string()) {
                continue;
            }

            for sensor in self.sensors.iter() {
                let distance = Pos::manhattan_distance(&sensor.pos, &curr_pos);
                if distance <= sensor.max_distance_range {
                    count += 1;
                    break;
                }
            }
        }

        return count;
    }

    pub fn get_perimeter(sensor_pos: Pos, size: usize) -> Vec<Pos> {
        let step_outside = (size + 1) as isize;
        let mut perimeter_points: Vec<Pos> = vec![];

        // left top
        let mut curr_point = Pos::new(sensor_pos.x, sensor_pos.y - step_outside);
        while curr_point.y != sensor_pos.y {
            curr_point.x += 1;
            curr_point.y += 1;
            perimeter_points.push(curr_point);
        }

        // left bottom
        let mut curr_point = Pos::new(sensor_pos.x, sensor_pos.y + step_outside);
        while curr_point.y != sensor_pos.y {
            curr_point.x += 1;
            curr_point.y -= 1;
            perimeter_points.push(curr_point);
        }

        // right top
        let mut curr_point = Pos::new(sensor_pos.x, sensor_pos.y - step_outside);
        while curr_point.y != sensor_pos.y {
            curr_point.x -= 1;
            curr_point.y += 1;
            perimeter_points.push(curr_point);
        }

        // right bottom
        let mut curr_point = Pos::new(sensor_pos.x, sensor_pos.y + step_outside);
        while curr_point.y != sensor_pos.y {
            curr_point.x -= 1;
            curr_point.y -= 1;
            perimeter_points.push(curr_point);
        }

        return perimeter_points;
    }

    pub fn distress_beacon(&self) -> Pos {
        for sensor in self.sensors.iter() {
            let perimeter = Self::get_perimeter(sensor.pos, sensor.max_distance_range);

            for point in perimeter.iter().filter(|point| {
                point.x <= 4000000 && point.x >= 0 && point.y <= 4000000 && point.y >= 0
            }) {
                let mut is_outside_all_ranges = true;
                for check_sensor in self.sensors.iter() {
                    let distance = Pos::manhattan_distance(&check_sensor.pos, &point);
                    if distance <= check_sensor.max_distance_range {
                        is_outside_all_ranges = false;
                        break;
                    }
                }

                if is_outside_all_ranges {
                    return Pos::from_string(point.as_string());
                }
            }
        }

        panic!("no point found");
    }
}
