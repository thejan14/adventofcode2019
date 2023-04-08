use std::fs;
use std::path::Path;
use std::time::Instant;

#[derive(Debug, PartialEq)]
enum Orientation {
  Horizontal,
  Vertical,
}

#[derive(Debug)]
struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
  fn cross_product(&self, other: &Vector) -> i32 {
    (self.x * other.y) - (self.y * other.x)
  }

  fn add(&self, other: &Vector) -> Vector {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }

  fn sub(&self, other: &Vector) -> Vector {
    Self {
      x: self.x - other.x,
      y: self.y - other.y,
    }
  }
}

#[derive(Debug)]
struct Line {
    q: Vector,
    s: Vector,
    o: Orientation,
    steps: i32,
}

impl Line {
    fn end(&self) -> Vector {
      self.q.add(&self.s)
    }

    fn length(&self) -> i32 {
        match self.o {
            Orientation::Horizontal => self.s.x.abs(),
            Orientation::Vertical => self.s.y.abs(),
        }
    }
}

fn main() {
    let data = fs::read_to_string(Path::new("day03/input.txt")).unwrap();
    let now = Instant::now();
    
    let lines_data = data
        .split("\n")
        .map(|l| {
            let path_instructions = l.split(",").collect::<Vec<&str>>();
            path_instructions[1..].iter().fold(
                vec![get_line(&0, &0, 0, path_instructions.first().unwrap())],
                |mut acc, instruction| {
                    let prev_line = acc.last().unwrap();
                    let Vector { x, y } = prev_line.end();
                    let line = get_line(&x, &y, &prev_line.steps + prev_line.length(), instruction);
                    acc.push(line);
                    acc
                })
        })
        .collect::<Vec<Vec<Line>>>();

    let cable1 = &lines_data[0];
    let cable2_h = &lines_data[1].iter().filter(|l|  l.o == Orientation::Horizontal).collect::<Vec<&Line>>();
    let cable2_v = &lines_data[1].iter().filter(|l|  l.o == Orientation::Vertical).collect::<Vec<&Line>>();

    let mut min_dist = i32::MAX;
    for line1 in cable1 {
        let cable2 = match line1.o {
          Orientation::Horizontal => cable2_v,
          Orientation::Vertical => cable2_h,
        };

        for line2 in cable2 {
            let intersection = get_intersection(line1, line2);
            if let Some(point) = intersection {
              if point.x != 0 || point.y != 0 {
                let distance = line1.steps
                  + line2.steps
                  + get_distance(&line1.q, &point)
                  + get_distance(&line2.q, &point);
                  
                if distance < min_dist {
                  min_dist = distance;
                }
              }
            }
        }
    }

    let result = min_dist;

    let elapsed = now.elapsed();
    println!("{result:?} {elapsed:.2?}");
}

fn get_line(from_x: &i32, from_y: &i32, steps: i32, path_instruction: &str) -> Line {
    let direction = path_instruction.chars().nth(0).unwrap();
    let distance = path_instruction[1..].parse::<i32>().unwrap();
    let q = Vector { x: *from_x, y: *from_y };
    match direction {
        'R' => Line { q, s: Vector { x: distance, y: 0 }, steps, o: Orientation::Horizontal },
        'U' => Line { q, s: Vector { x: 0, y: distance }, steps, o: Orientation::Vertical },
        'L' => Line { q, s: Vector { x: -distance, y: 0 }, steps, o: Orientation::Horizontal },
        _ /* 'D' */  => Line { q, s: Vector { x: 0, y: -distance }, steps, o: Orientation::Vertical }, 
    }
}

// https://stackoverflow.com/a/565282
fn get_intersection(line1: &Line, line2: &Line) -> Option<Vector> {
    let Line { q, s, o: _o, steps: _steps } = line1;
    let Line { q: p, s: r, o: _o, steps: _steps } = line2;
    let x = (r).cross_product(s);
    if x == 0 {
      None // colinear or parallel
    } else {
      let t = q.sub(p).cross_product(s) as f64 / x as f64;
      let u = q.sub(p).cross_product(r) as f64 / x as f64;
      if x != 0 && 0f64 <= t && t <= 1f64 && 0f64 <= u && u <= 1f64 {
        Some(Vector { x: p.x + (t * r.x as f64) as i32, y: p.y + (t * r.y as f64) as i32 })
      } else {
        None // not parallel but no intersection
      }
    }
}

fn get_distance(point1: &Vector, point2: &Vector) -> i32 {
    let Vector { x: x1, y: y1} = point1;
    let Vector { x: x2, y: y2} = point2;
    (x1 - x2).abs() + (y1 - y2).abs()
}