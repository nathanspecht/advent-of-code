use plotters::prelude::*;
use std::fs;

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
    let mut horizontal_segments: Vec<(Point, Point)> = vec![];

    for line in input.lines() {
        let (a, b) = line.split_once(',').unwrap();
        let x: i64 = a.parse().unwrap();
        let y: i64 = b.parse().unwrap();

        coords.push(Point { x, y });
    }

    let points: Vec<(i32, i32)> = coords
        .iter()
        .map(|point| (point.x as i32, point.y as i32))
        .collect();

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
    let mut max_corners: Option<[Point; 4]> = None;

    // let mut cache: HashMap<Point, bool> = HashMap::new();

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

        let corners = [
            a.clone(),
            b.clone(),
            Point { x: a.x, y: b.y },
            Point { x: b.x, y: a.y },
        ];

        for corner in &corners {
            if !is_inside(&corner, &vertical_segments, &horizontal_segments) {
                continue 'outer;
            }
        }

        let center = get_center(&corners);

        if !is_inside(&center, &vertical_segments, &horizontal_segments) {
            continue 'outer;
        }

        /*
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
        */

        max = *area as i64;
        max_string = label;
        max_corners = Some(corners);

        break 'outer;
    }

    let draw_corners: Vec<(i32, i32)> = max_corners
        .unwrap()
        .iter()
        .map(|p| (p.x as i32, p.y as i32))
        .collect();

    let draw_corners: [(i32, i32); 4] = [
        draw_corners[0],
        draw_corners[1],
        draw_corners[2],
        draw_corners[3],
    ];

    let _ = draw_shape(&points, &draw_corners, "src/inputs/day09.png");

    println!("{max}");
    println!("{max_string}");
}

fn draw_shape(
    shape: &[(i32, i32)],
    rect: &[(i32, i32); 4],
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    assert!(shape.len() >= 2, "Shape needs at least two points");

    // Close both shapes
    let mut shape_points = shape.to_vec();
    shape_points.push(shape[0]);

    let mut rect_points = rect.to_vec();
    rect_points.push(rect[0]);

    // Compute bounds from BOTH shapes
    let all_points = shape_points.iter().chain(rect_points.iter());

    let (min_x, max_x) = all_points
        .clone()
        .map(|(x, _)| *x)
        .fold((i32::MAX, i32::MIN), |(min, max), x| {
            (min.min(x), max.max(x))
        });
    let (min_y, max_y) = all_points
        .map(|(_, y)| *y)
        .fold((i32::MAX, i32::MIN), |(min, max), y| {
            (min.min(y), max.max(y))
        });

    // Create drawing area
    let root = BitMapBackend::new(output_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("Shape + Rectangle", ("sans-serif", 30))
        .build_cartesian_2d(min_x..max_x, min_y..max_y)?;

    chart.configure_mesh().draw()?;

    // Draw main shape
    chart.draw_series(LineSeries::new(shape_points.clone(), &BLUE))?;
    chart.draw_series(
        shape_points
            .iter()
            .map(|p| Circle::new(*p, 1, BLUE.filled())),
    )?;

    // Draw rectangle
    chart.draw_series(LineSeries::new(rect_points.clone(), &GREEN))?;
    chart.draw_series(
        rect_points
            .iter()
            .map(|p| Circle::new(*p, 1, GREEN.filled())),
    )?;

    root.present()?;
    Ok(())
}
