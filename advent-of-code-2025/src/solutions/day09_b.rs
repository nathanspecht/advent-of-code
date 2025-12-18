use plotters::prelude::*;
use std::{collections::HashSet, fs};

#[derive(Copy, Clone, Debug)]
enum Axis {
    X,
    Y,
}

struct Range {
    min: i64,
    max: i64,
}

impl Range {
    pub fn from(a: &Point, b: &Point, axis: Axis) -> Range {
        Range {
            min: a[axis].min(b[axis]),
            max: a[axis].max(b[axis]),
        }
    }

    pub fn contains(&self, other: i64) -> bool {
        self.min <= other && other <= self.max
    }

    pub fn overlaps(&self, other: Range) -> bool {
        self.contains(other.min)
            || self.contains(other.max)
            || other.contains(self.min)
            || other.contains(self.max)
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl std::ops::Index<Axis> for Point {
    type Output = i64;

    fn index(&self, axis: Axis) -> &Self::Output {
        match axis {
            Axis::X => &self.x,
            Axis::Y => &self.y,
        }
    }
}

#[derive(Clone)]
struct Segment {
    points: (Point, Point),
}

impl Segment {
    pub fn from(a: Point, b: Point) -> Segment {
        Segment { points: (a, b) }
    }

    fn range(&self, axis: Axis) -> Range {
        Range::from(&self.points.0, &self.points.1, axis)
    }

    pub fn x_range(&self) -> Range {
        self.range(Axis::X)
    }

    pub fn y_range(&self) -> Range {
        self.range(Axis::Y)
    }

    pub fn is_vertical(&self) -> bool {
        self.points.0.x == self.points.1.x
    }

    pub fn is_horizontal(&self) -> bool {
        self.points.0.y == self.points.1.y
    }

    pub fn contains(&self, point: &Point) -> bool {
        if self.is_horizontal() {
            return self.points.0.y == point.y && self.x_range().contains(point.x);
        } else {
            return self.points.0.x == point.x && self.y_range().contains(point.y);
        }
    }

    pub fn check_intersect(&self, other: &Segment) -> bool {
        if self.is_vertical() && other.is_vertical() {
            return self.points.0.x == other.points.0.x && self.y_range().overlaps(other.y_range());
        }

        if self.is_horizontal() && other.is_horizontal() {
            return self.points.0.y == other.points.0.y && self.x_range().overlaps(other.x_range());
        }

        if self.is_horizontal() && other.is_vertical() {
            return other.y_range().contains(self.points.0.y)
                && self.x_range().contains(other.points.0.x);
        }

        if self.is_vertical() && other.is_horizontal() {
            return other.x_range().contains(self.points.0.x)
                && self.y_range().contains(other.points.0.y);
        }

        panic!("Lines must be vertical or horizontal");
    }
}

struct Shape {
    segments: Vec<Segment>,
}

impl Shape {
    pub fn new() -> Shape {
        Shape { segments: vec![] }
    }

    pub fn push(&mut self, segment: Segment) {
        self.segments.push(segment)
    }

    pub fn contains(&self, point: &Point) -> bool {
        let mut segments_touched: Vec<Segment> = vec![];

        for segment in &self.segments {
            if segment.contains(point) {
                return true;
            }

            let right_ray = Segment::from(
                point.clone(),
                Point {
                    y: point.y,
                    x: i64::MAX,
                },
            );

            if segment.check_intersect(&right_ray) {
                segments_touched.push(segment.clone());
            }
        }

        let mut has_merged = true;

        while has_merged {
            has_merged = false;

            'outer: for i in 0..segments_touched.len() {
                for j in (i + 1)..segments_touched.len() {
                    let a = &segments_touched[i];
                    let b = &segments_touched[j];

                    let merged = if a.points.0 == b.points.0 {
                        Some(Segment::from(a.points.1.clone(), b.points.1.clone()))
                    } else if a.points.0 == b.points.1 {
                        Some(Segment::from(a.points.1.clone(), b.points.0.clone()))
                    } else if a.points.1 == b.points.0 {
                        Some(Segment::from(a.points.0.clone(), b.points.1.clone()))
                    } else if a.points.1 == b.points.1 {
                        Some(Segment::from(a.points.0.clone(), b.points.0.clone()))
                    } else {
                        None
                    };

                    if let Some(new_seg) = merged {
                        segments_touched.remove(j);
                        segments_touched.remove(i);

                        segments_touched.push(new_seg);

                        has_merged = true;
                        break 'outer;
                    } else {
                        has_merged = false;
                    }
                }
            }
        }

        println!("Num segments: {}", segments_touched.len());

        return segments_touched.len() % 2 != 0;
    }
}

pub fn run() {
    let input = fs::read_to_string("src/inputs/day09_a.txt").expect("Failed to read file");
    let mut coords: Vec<Point> = vec![];
    let mut shape = Shape::new();

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

        shape.push(Segment::from(prev, curr));
    }

    let mut max_string: String = String::new();
    let mut max = 0;

    let mut rectangles: Vec<(Point, Point, u64)> = vec![];

    for a in &coords {
        for b in &coords {
            // if !(a.x == 75487 && a.y == 91729 && b.x == 12644 && b.y == 18423) {
            //    continue;
            //}

            if a == b {
                continue;
            }

            let width = a.x.abs_diff(b.x) + 1;
            let height = a.y.abs_diff(b.y) + 1;
            let area = width * height;

            rectangles.push((a.clone(), b.clone(), area));
        }
    }

    rectangles.sort_by(|a, b| b.2.cmp(&a.2));

    let mut final_rectangle: Option<Vec<Point>> = None;

    'outer: for (a, b, area) in rectangles {
        let label = format!("{:?}, {:?}", a, b);

        let _rectangle = [
            Segment::from(a.clone(), Point { x: a.x, y: b.y }),
            Segment::from(b.clone(), Point { x: b.x, y: a.y }),
            Segment::from(a.clone(), Point { x: b.x, y: a.y }),
            Segment::from(b.clone(), Point { x: a.x, y: b.y }),
        ];

        let rectangle_points = vec![
            a.clone(),
            Point { x: a.x, y: b.y },
            b.clone(),
            Point { x: b.x, y: a.y },
        ];

        for point in &rectangle_points {
            if !shape.contains(&point) {
                continue 'outer;
            }
        }

        final_rectangle = Some(rectangle_points);
        max = area as i64;
        max_string = label;

        break 'outer;
    }

    println!("{max}");
    println!("{max_string}");

    let draw_segments: Vec<(i32, i32)> = shape
        .segments
        .iter()
        .map(|segment| (segment.points.0.x as i32, segment.points.0.y as i32))
        .collect();

    let draw_rectangle: Vec<(i32, i32)> = final_rectangle
        .unwrap()
        .iter()
        .map(|point| (point.x as i32, point.y as i32))
        .collect();

    println!("{:?}", draw_rectangle);

    let _ = draw_shape(
        &draw_segments,
        &draw_rectangle,
        "src/visualizations/day09_plot.png",
    );
}

fn draw_shape(
    shape: &Vec<(i32, i32)>,
    rect: &Vec<(i32, i32)>,
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
    let root = BitMapBackend::new(output_path, (3840, 2160)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("Day 09", ("sans-serif", 30))
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
