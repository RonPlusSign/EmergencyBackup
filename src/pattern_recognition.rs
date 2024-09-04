use std::collections::VecDeque;
use std::fmt::Display;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use guessture::{find_matching_template_with_defaults, Path2D, Template};
use mouse_position::mouse_position::Mouse;
use serde::{Deserialize, Serialize};

const POINTS_PER_FIGURE: usize = 200;   // Maximum number of points to store
const SHAPE_SIZE: f32 = 100.0; // Size of the shapes (each side/radius)

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub enum Shape { Circle, Square, Triangle, Tick, Cross }
impl Shape {
    pub fn get_templates_for_shape(shape: Shape) -> Vec<Template> {
        match shape {
            Shape::Circle => vec![circle_template(false), circle_template(true)],
            Shape::Square => vec![square_template(false), square_template(true)],
            Shape::Triangle => vec![triangle_template(false), triangle_template(true)],
            Shape::Cross => vec![cancel_template(false), cancel_template(true)],
            Shape::Tick => vec![confirm_template()],
        }
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Shape::Circle => "Circle".to_string(),
            Shape::Square => "Square".to_string(),
            Shape::Triangle => "Triangle".to_string(),
            Shape::Tick => "Tick".to_string(),
            Shape::Cross => "Cross".to_string(),
        };
        write!(f, "{}", str)
    }
}

/// Detect the shape of the given list of points
/// #### Arguments
/// * `points`: A list of points representing the mouse positions
/// #### returns
///     Shape of the detected shape
pub fn detect_shape(points: &VecDeque<Mouse>, templates: &Vec<Template>, threshold: f32) -> Option<Shape> {
    // Create a path from the list of points
    let mut path = Path2D::default();
    for point in points {
        match *point {
            Mouse::Position { x, y } => { path.push(x as f32, y as f32); }
            Mouse::Error => { return None; }
        }
    }

    // Compare the path with the templates
    let result = find_matching_template_with_defaults(&templates, &path);
    if result.is_err() { return None; }

    // If the similarity is above the threshold, return the detected shape
    let (template, similarity) = result.unwrap();
    if similarity > threshold {
        match template.name.as_str() {
            "Circle" => { Some(Shape::Circle) }
            "Square" => { Some(Shape::Square) }
            "Triangle" => { Some(Shape::Triangle) }
            "Confirm" => { Some(Shape::Tick) }
            "Cancel" => { Some(Shape::Cross) }
            _ => { None }
        }
    } else { None }
}

/// Draw a circle template.
/// The circle is drawn by drawing the circumference of the circle
///
/// #### Arguments
/// * `invert_direction`: If true, draw the circle in the opposite direction (counter-clockwise)
pub fn circle_template(invert_direction: bool) -> Template {
    let mut circle_points = Path2D::default();
    let range: Vec<usize> = if invert_direction { (0..POINTS_PER_FIGURE).rev().collect() } else { (0..POINTS_PER_FIGURE).collect() };
    for i in range {
        let angle = 2.0 * std::f64::consts::PI * (i as f64) / (POINTS_PER_FIGURE as f64);
        let x = SHAPE_SIZE as f64 * angle.cos();
        let y = SHAPE_SIZE as f64 * angle.sin();
        circle_points.push(x as f32, y as f32);
    }
    Template::new("Circle".to_string(), &circle_points).unwrap()
}

/// Draw a square template.
///
/// The square is drawn by drawing the 4 sides of the square, each with POINTS_PER_FIGURE/4 points.
/// Each side is evenly distanced in the range [-SHAPE_SIZE/2, SHAPE_SIZE/2]
///
/// #### Arguments
/// * `invert_direction`: If true, draw the square in the opposite direction (anti-clockwise)
pub fn square_template(invert_direction: bool) -> Template {
    let mut square_points = Path2D::default();
    let range: Vec<usize> = if invert_direction { (0..POINTS_PER_FIGURE).collect() } else { (0..POINTS_PER_FIGURE).rev().collect() };
    for i in range {
        // Draw the 4 sides of the square, each with POINTS_PER_FIGURE / 4 points, evenly distanced in the range [-SHAPE_SIZE/2, SHAPE_SIZE/2]
        let side = i / (POINTS_PER_FIGURE / 4);
        let points_per_side = POINTS_PER_FIGURE / 4;
        let x = match side {
            0 => -SHAPE_SIZE / 2.0 + (i % points_per_side) as f32 / points_per_side as f32 * SHAPE_SIZE,
            1 => SHAPE_SIZE / 2.0,
            2 => SHAPE_SIZE / 2.0 - (i % points_per_side) as f32 / points_per_side as f32 * SHAPE_SIZE,
            3 => -SHAPE_SIZE / 2.0,
            _ => 0.0
        };

        let y = match side {
            0 => -SHAPE_SIZE / 2.0,
            1 => -SHAPE_SIZE / 2.0 + (i % points_per_side) as f32 / points_per_side as f32 * SHAPE_SIZE,
            2 => SHAPE_SIZE / 2.0,
            3 => SHAPE_SIZE / 2.0 - (i % points_per_side) as f32 / points_per_side as f32 * SHAPE_SIZE,
            _ => 0.0
        };

        square_points.push(x, y);
    }
    Template::new("Square".to_string(), &square_points).unwrap()
}

/// Draw a triangle template.
///
/// The triangle equilateral and is drawn by drawing the 3 sides of the triangle, each with POINTS_PER_FIGURE/3 points.
///
/// #### Arguments
/// * `invert_direction`: If true, draw the triangle in the opposite direction (anti-clockwise)
pub fn triangle_template(invert_direction: bool) -> Template {
    let mut triangle_points = Path2D::default();

    // Calculate the vertices of the triangle
    let height = (SHAPE_SIZE * (3.0_f32).sqrt()) / 2.0;
    let vertices = [
        (0.0, height / 2.0),
        (-SHAPE_SIZE / 2.0, -height / 2.0),
        (SHAPE_SIZE / 2.0, -height / 2.0),
    ];

    // Number of points per side
    let points_per_side = POINTS_PER_FIGURE / 3;

    // Interpolate points between vertices
    for i in 0..points_per_side {
        let t = i as f32 / points_per_side as f32;
        let (x1, y1) = vertices[0];
        let (x2, y2) = vertices[1];
        let x = x1 + t * (x2 - x1);
        let y = y1 + t * (y2 - y1);
        triangle_points.push(x, y);
    }

    for i in 0..points_per_side {
        let t = i as f32 / points_per_side as f32;
        let (x1, y1) = vertices[1];
        let (x2, y2) = vertices[2];
        let x = x1 + t * (x2 - x1);
        let y = y1 + t * (y2 - y1);
        triangle_points.push(x, y);
    }

    for i in 0..points_per_side {
        let t = i as f32 / points_per_side as f32;
        let (x1, y1) = vertices[2];
        let (x2, y2) = vertices[0];
        let x = x1 + t * (x2 - x1);
        let y = y1 + t * (y2 - y1);
        triangle_points.push(x, y);
    }

    if invert_direction {
        let mut triangle_points_new = Path2D::default();
        triangle_points.points().iter().rev().for_each(|(x, y)| triangle_points_new.push(*x, *y));
        triangle_points = triangle_points_new;
    }
    Template::new("Triangle".to_string(), &triangle_points).unwrap()
}

/// Shape representing a "tick"/"checkmark", used as confirmation.
/// Drawn using a small descending line, followed by a larger ascending line.
pub fn confirm_template() -> Template {
    let mut confirm_points = Path2D::default();
    let step = SHAPE_SIZE / POINTS_PER_FIGURE as f32;
    let mut x = 0.0;
    let mut y = 0.0;

    for i in 0..POINTS_PER_FIGURE {
        x += step;
        if i <= POINTS_PER_FIGURE / 3 { // Descending line for 1/3 of points
            y -= 1.5 * step;
        } else { // Ascending line for 2/3 of points
            y += 1.0 * step;
        };

        confirm_points.push(x, y);
    }

    Template::new("Confirm".to_string(), &confirm_points).unwrap()
}

/// Template representing a "cross"/"x", used as cancellation.
/// Drawn using two lines crossing each other, with another line joining the ends (⋊)
/// (because it's how the mouse path is drawn)
/// #### Arguments
/// * `invert_direction`: If true, draw the cross in the opposite direction
pub fn cancel_template(invert_direction: bool) -> Template {
    let mut cancel_points = Path2D::default();
    let step = SHAPE_SIZE / POINTS_PER_FIGURE as f32;
    let mut x = 0.0;
    let mut y = 0.0;

    for i in 0..POINTS_PER_FIGURE {
        if i <= POINTS_PER_FIGURE / 3 { // First line, from bottom-left to top-right
            x += step;
            y += step;
        } else if i <= POINTS_PER_FIGURE * 2 / 3 { // Second line, vertical (descending)
            y -= step;
        } else { // Third line, from bottom-right to top-left
            x -= step;
            y += step;
        }

        cancel_points.push(x, y);
    }

    if invert_direction {
        let mut cancel_points_new = Path2D::default();
        cancel_points.points().iter().rev().for_each(|(x, y)| cancel_points_new.push(*x, *y));
        cancel_points = cancel_points_new;
    }
    Template::new("Cancel".to_string(), &cancel_points).unwrap()
}

/// Check if all the points are similar (within a threshold). If the points are similar, the mouse is not moving enough.
///
/// #### Arguments
/// * `points`: A list of points representing the mouse positions
pub fn all_points_near(points: &VecDeque<Mouse>) -> bool {
    let threshold = 600;  // Minimum distance for the mouse to move in pixels
    let first_point = points.front().unwrap();
    let x0 = match *first_point {
        Mouse::Position { x, y: _y } => x,
        _ => 0,
    };
    let y0 = match *first_point {
        Mouse::Position { x: _x, y } => y,
        _ => 0,
    };
    for point in points.iter() {
        match *point {
            Mouse::Position { x, y } => {
                if (x - x0).abs() > threshold || (y - y0).abs() > threshold {
                    return false;
                }
            }
            _ => {}
        }
    }
    true
}


pub fn wait_for_symbol(templates: &Vec<Template>, stop_condition: Arc<Mutex<bool>>) -> Option<Shape> {
    let mut points = VecDeque::new(); // Circular buffer: a point is added at the end and removed from the front
    let mouse_sampling_time_ms = 10; // Time between each sampling of the mouse position
    let guess_threshold = 0.9;  // Threshold for the guessture algorithm. If the similarity is above this threshold, the shape is detected

    loop {
        // Exit the loop if the stop condition is verified
        let lock = stop_condition.lock();
        if lock.is_ok() && *lock.unwrap() { return None; }

        if points.len() == POINTS_PER_FIGURE { points.pop_front(); } // Buffer is full, remove the oldest point

        let position = Mouse::get_mouse_position();
        match position {
            Mouse::Position { x, y } => { points.push_back(Mouse::Position { x, y }); }
            Mouse::Error => { return None; }  // Exit the loop if an error occurs
        }

        // The buffer is full and the mouse is moving (not all points are similar)
        if points.len() == POINTS_PER_FIGURE && !all_points_near(&points) {
            let shape = detect_shape(&points, &templates, guess_threshold);
            if shape.is_some() { return shape; }
        }

        sleep(Duration::from_millis(mouse_sampling_time_ms));
    }
}