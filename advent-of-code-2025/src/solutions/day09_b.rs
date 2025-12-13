use std::fs;

#[derive(Debug, PartialEq, Clone)]
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

fn is_inside(point: &Point, vertical_segments: &Vec<(Point, Point)>) -> bool {
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

fn is_on_segment(
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
    let input = fs::read_to_string("src/inputs/day09_b.txt").expect("Failed to read file");
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

    for a in &coords {
        'inner: for b in &coords {
            let width = a.x.abs_diff(b.x) + 1;
            let height = a.y.abs_diff(b.y) + 1;
            let area = width * height;

            if area <= max {
                continue 'inner;
            }

            let label = format!("{:?}, {:?}", a, b);

            let x_min = a.x.min(b.x);
            let x_max = a.x.max(b.x);
            let y_min = a.y.min(b.y);
            let y_max = a.y.max(b.y);

            for x in x_min..=x_max {
                for y in y_min..=y_max {
                    if !coords.contains(&Point { x, y })
                        && !is_on_segment(&Point { x, y }, &vertical_segments, &horizontal_segments)
                        && !is_inside(&Point { x, y }, &vertical_segments)
                    {
                        continue 'inner;
                    }
                }
            }

            max = area;
            max_string = label;
        }
    }

    println!("{max}");
    println!("{max_string}");
}
