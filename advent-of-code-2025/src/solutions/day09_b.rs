use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

fn check_intersect((h1, h2): &(Point, Point), (v1, v2): &(Point, Point)) -> bool {
    let y = h1.y;
    let x = v1.x;

    let x_range = (h1.x.min(h2.x), h1.x.max(h2.x));
    let y_range = (v1.y.min(v2.y), v1.y.max(v2.y));

    (x_range.0 <= x && x <= x_range.1) && (y_range.0 <= y && y <= y_range.1)
}

fn is_inside(
    point: &Point,
    vertical_segments: &Vec<(Point, Point)>,
    horizontal_segments: &Vec<(Point, Point)>,
) -> bool {
    let mut num_right = 0;

    for segment in horizontal_segments {
        if check_intersect(segment, &(point.clone(), point.clone())) {
            return true;
        }
    }

    for segment in vertical_segments {
        if check_intersect(&(point.clone(), point.clone()), segment) {
            return true;
        }

        let horizontal_point = Point {
            y: point.y,
            x: i64::MAX,
        };

        if check_intersect(&(point.clone(), horizontal_point), &segment) {
            num_right += 1;
        }
    }

    return num_right % 2 != 0;
}

fn _is_inside(point: &Point, vertical_segments: &Vec<(Point, Point)>) -> bool {
    let num_right = vertical_segments
        .iter()
        .filter(|segment| {
            let horizontal_point = Point {
                y: point.y,
                x: i64::MAX,
            };

            return check_intersect(&(point.clone(), horizontal_point), &segment);
        })
        .count();

    num_right % 2 != 0
}

fn _is_on_segment(
    point: &Point,
    vertical_segments: &Vec<(Point, Point)>,
    horizontal_segments: &Vec<(Point, Point)>,
) -> bool {
    for segment in vertical_segments {
        if check_intersect(&(point.clone(), point.clone()), segment) {
            return true;
        }
    }

    for segment in horizontal_segments {
        if check_intersect(segment, &(point.clone(), point.clone())) {
            return true;
        }
    }

    return false;
}

pub fn run() {
    let input = fs::read_to_string("src/inputs/day09_a.txt").expect("Failed to read file");
    let mut coords: Vec<Point> = vec![];
    let mut vertical_segments: Vec<(Point, Point)> = vec![];
    let mut horizontal_segments: Vec<(Point, Point)> = vec![];

    for line in input.lines() {
        let (a, b) = line.split_once(',').unwrap();
        let x: i64 = a.parse().unwrap();
        let y: i64 = b.parse().unwrap();

        coords.push(Point { x, y });
    }

    for i in 0..coords.len() {
        let prev = if i == 0 {
            coords[coords.len() - 1].clone()
        } else {
            coords[i - 1].clone()
        };

        let curr = coords[i].clone();

        if prev.x == curr.x {
            vertical_segments.push((prev, curr));
        } else if prev.y == curr.y {
            horizontal_segments.push((prev, curr));
        }
    }

    let mut max_string: String = String::new();
    let mut max = 0;

    let mut cache: HashMap<Point, bool> = HashMap::new();

    let mut rectangles: Vec<(Point, Point, u64)> = vec![];

    for a in &coords {
        for b in &coords {
            let width = a.x.abs_diff(b.x) + 1;
            let height = a.y.abs_diff(b.y) + 1;
            let area = width * height;

            rectangles.push((a.clone(), b.clone(), area));
        }
    }

    rectangles.sort_by(|a, b| b.2.cmp(&a.2));

    println!("Num rectangles: {}", rectangles.len());

    'outer: for (i, (a, b, area)) in rectangles.iter().enumerate() {
        println!("{}/{}: {}", i, rectangles.len(), area);
        let label = format!("{:?}, {:?}", a, b);

        let x_min = a.x.min(b.x);
        let x_max = a.x.max(b.x);
        let y_min = a.y.min(b.y);
        let y_max = a.y.max(b.y);

        for x in x_min..=x_max {
            for y in y_min..=y_max {
                let is_inside_cached = cache.get(&Point { x, y }).unwrap_or(&false);

                if !is_inside_cached {
                    if !coords.contains(&Point { x, y })
                        && !is_inside(&Point { x, y }, &vertical_segments, &horizontal_segments)
                    {
                        cache.insert(Point { x, y }, false);
                        continue 'outer;
                    } else {
                        cache.insert(Point { x, y }, true);
                    }
                }
            }
        }

        max = *area as i64;
        max_string = label;

        break 'outer;
    }

    println!("{max}");
    println!("{max_string}");
}
