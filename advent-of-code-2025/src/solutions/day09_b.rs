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

fn get_center(points: &[Point; 4]) -> Point {
    let sum_x: i64 = points.iter().map(|p| p.x).sum();
    let sum_y: i64 = points.iter().map(|p| p.y).sum();

    Point {
        x: sum_x / 4,
        y: sum_y / 4,
    }
}

pub fn run() {
    let input = fs::read_to_string("src/inputs/day09_a.txt").expect("Failed to read file");
    let mut coords: Vec<Point> = vec![];
    let mut vertical_segments: Vec<(Point, Point)> = vec![];

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
        }
    }

    let mut max_string: String = String::new();
    let mut max = 0;

    for a in &coords {
        for b in &coords {
            let width = a.x.abs_diff(b.x) + 1;
            let height = a.y.abs_diff(b.y) + 1;
            let area = width * height;

            let corners = [
                a.clone(),
                b.clone(),
                Point { x: a.x, y: b.y },
                Point { x: b.x, y: a.y },
            ];

            let center = get_center(&corners);

            let is_inside = corners
                .iter()
                .chain(std::iter::once(&center))
                .all(|corner| {
                    if coords.contains(corner) {
                        return true;
                    }

                    let num_right = vertical_segments
                        .iter()
                        .filter(|segment| {
                            let horizontal_point = Point {
                                y: corner.y,
                                x: i64::MAX,
                            };

                            check_intersect(&(corner.clone(), horizontal_point), &segment)
                        })
                        .count();

                    num_right % 2 != 0
                });

            if area > max && is_inside {
                max = area;
                max_string = format!("{:?}, {:?}", a, b);
            }
        }
    }

    println!("{max}");
    println!("{max_string}");
}
