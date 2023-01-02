use regex::Regex;

const _INPUT_EXAMPLE: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
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
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
const _INPUT_PART1: &str = "Sensor at x=193758, y=2220950: closest beacon is at x=652350, y=2000000
Sensor at x=3395706, y=3633894: closest beacon is at x=3404471, y=3632467
Sensor at x=3896022, y=3773818: closest beacon is at x=3404471, y=3632467
Sensor at x=1442554, y=1608100: closest beacon is at x=652350, y=2000000
Sensor at x=803094, y=813675: closest beacon is at x=571163, y=397470
Sensor at x=3491072, y=3408908: closest beacon is at x=3404471, y=3632467
Sensor at x=1405010, y=486446: closest beacon is at x=571163, y=397470
Sensor at x=3369963, y=3641076: closest beacon is at x=3404471, y=3632467
Sensor at x=3778742, y=2914974: closest beacon is at x=4229371, y=3237483
Sensor at x=1024246, y=3626229: closest beacon is at x=2645627, y=3363491
Sensor at x=3937091, y=2143160: closest beacon is at x=4229371, y=3237483
Sensor at x=2546325, y=2012887: closest beacon is at x=2645627, y=3363491
Sensor at x=3505386, y=3962087: closest beacon is at x=3404471, y=3632467
Sensor at x=819467, y=239010: closest beacon is at x=571163, y=397470
Sensor at x=2650614, y=595151: closest beacon is at x=3367919, y=-1258
Sensor at x=3502942, y=6438: closest beacon is at x=3367919, y=-1258
Sensor at x=3924022, y=634379: closest beacon is at x=3367919, y=-1258
Sensor at x=2935977, y=2838245: closest beacon is at x=2645627, y=3363491
Sensor at x=1897626, y=7510: closest beacon is at x=3367919, y=-1258
Sensor at x=151527, y=640680: closest beacon is at x=571163, y=397470
Sensor at x=433246, y=1337298: closest beacon is at x=652350, y=2000000
Sensor at x=2852855, y=3976978: closest beacon is at x=3282750, y=3686146
Sensor at x=3328398, y=3645875: closest beacon is at x=3282750, y=3686146
Sensor at x=3138934, y=3439134: closest beacon is at x=3282750, y=3686146
Sensor at x=178, y=2765639: closest beacon is at x=652350, y=2000000
Sensor at x=3386231, y=3635056: closest beacon is at x=3404471, y=3632467
Sensor at x=3328074, y=1273456: closest beacon is at x=3367919, y=-1258
Sensor at x=268657, y=162438: closest beacon is at x=571163, y=397470";

fn manhattan(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn parse_input(input: &str) -> (Vec<(i32, i32, i32)>, Vec<(i32, i32)>) {
    let re_line = Regex::new(
        r"Sensor at x=([0-9-]+), y=([0-9-]+): closest beacon is at x=([0-9-]+), y=([0-9-]+)",
    )
    .unwrap();

    let mut sensors = Vec::new();
    let mut beacons = Vec::new();
    for line in input.split("\n") {
        let caps = re_line.captures(line).unwrap();
        let sensor_x = caps.get(1).unwrap().as_str().parse().unwrap();
        let sensor_y = caps.get(2).unwrap().as_str().parse().unwrap();
        let beacon_x = caps.get(3).unwrap().as_str().parse().unwrap();
        let beacon_y = caps.get(4).unwrap().as_str().parse().unwrap();
        sensors.push((
            sensor_x,
            sensor_y,
            manhattan(sensor_x, sensor_y, beacon_x, beacon_y),
        ));
        beacons.push((beacon_x, beacon_y));
    }

    (sensors, beacons)
}

fn calculate_impossible_beacons_at(
    sensors: &Vec<(i32, i32, i32)>,
    beacons: &Vec<(i32, i32)>,
    y: i32,
) -> i32 {
    let min_x = sensors.iter().map(|(x, _, d)| x - d).min().unwrap();
    let max_x = sensors.iter().map(|(x, _, d)| x + d).max().unwrap();

    let mut count = 0;
    for x in min_x..max_x {
        if beacons.contains(&(x, y)) {
            continue;
        }
        for sensor in sensors {
            if manhattan(x, y, sensor.0, sensor.1) <= sensor.2 {
                count += 1;
                break;
            }
        }
    }

    count
}

fn part1() {
    let (sensors, beacons) = parse_input(_INPUT_PART1);
    println!(
        "impossible beacons: {}",
        calculate_impossible_beacons_at(&sensors, &beacons, 2000000)
    );
}

fn find_distress_beacon(
    sensors: &Vec<(i32, i32, i32)>,
    beacons: &Vec<(i32, i32)>,
    max_coord: i32,
) -> Option<(i32, i32)> {
    for y in 0..max_coord {
        let mut ranges = sensors
            .iter()
            .map(|(sx, sy, d)| (sx - (d - (sy - y).abs()), sx + (d - (sy - y).abs())))
            .filter(|(from, to)| to > from) // filter far sensors
            .collect::<Vec<(i32, i32)>>();
        ranges.sort_unstable();
        let mut x = 0;
        while x <= max_coord {
            for (from, to) in ranges.iter() {
                if *from <= x && x <= *to {
                    x = to + 1;
                }
            }
            if beacons.contains(&(x, y)) {
                x += 1;
                continue;
            }
            if x <= max_coord {
                return Some((x, y));
            }
        }

        if y % 100000 == 0 {
            println!("y={}", y);
        }
    }
    None
}

fn part2() {
    let max_coord = 4000000;
    let (sensors, beacons) = parse_input(_INPUT_PART1);
    let pos = find_distress_beacon(&sensors, &beacons, max_coord).unwrap();
    println!(
        "distress beacon: {:?}, frequency = {}",
        pos,
        pos.0 as i64 * 4000000 + pos.1 as i64
    );
}

fn main() {
    println!("part1");
    part1();
    println!("part2");
    part2();
}
