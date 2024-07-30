use std::cmp::PartialEq;
use std::collections::VecDeque;
use std::thread::sleep;
use std::time::Duration;
use mouse_position::mouse_position::{Mouse};
use guessture;
use guessture::{Path2D, PathCoord, Template, find_matching_template, find_matching_template_with_defaults};

#[derive(Debug, Eq, PartialEq)]
enum Shape { Circle, Square, Triangle, Unknown }

enum Confirm { Yes, No }

/// Detect the shape of the given list of points
/// ### Arguments
/// * `points`: A list of points representing the mouse positions
/// ### returns
///     Shape of the detected shape
fn detect_shape(points: &VecDeque<Mouse>, templates: &Vec<Template>, threshold: f32) -> Shape {
    // Create a path from the list of points
    let mut path = Path2D::default();
    for point in points {
        match point {
            Mouse::Position { x, y } => { path.push((*x as f32).into(), (*y as f32).into()); }
            Mouse::Error => { return Shape::Unknown; }
        }
    }

    // Compare the path with the templates
    let result = find_matching_template_with_defaults(&templates, &path);
    if result.is_err() {
        return Shape::Unknown;
    }
    let (template, similarity) = result.unwrap();

    // If the similarity is above the threshold, return the detected shape
    return if similarity > threshold {
        match template.name.as_str() {
            "Circle" => { Shape::Circle }
            "Square" => { Shape::Square }
            "Triangle" => { Shape::Triangle }
            _ => { Shape::Unknown }
        }
    } else {
        Shape::Unknown
    };
}

fn circle_template(number_of_points: usize, radius: f64) -> Template {
    // Draw a circle with buffer_size points and a specific radius
    let mut circle_points = Path2D::default();
    for i in 0..number_of_points {
        let angle = 2.0 * std::f64::consts::PI * (i as f64) / (number_of_points as f64);
        let x = radius * angle.cos();
        let y = radius * angle.sin();
        circle_points.push((x as f32).into(), (y as f32).into());
    }
    Template::new("Circle".to_string(), &circle_points).unwrap()
}

fn square_template(number_of_points: usize, side_length: f64) -> Template {
    // Draw a square with buffer_size points and a specific side length
    let mut square_points = Path2D::default();
    for i in 0..number_of_points {
        let x = if i < number_of_points / 4 { side_length } else if i < number_of_points / 2 { side_length } else if i < 3 * number_of_points / 4 { -side_length } else { -side_length };
        let y = if i < number_of_points / 4 { -side_length } else if i < number_of_points / 2 { side_length } else if i < 3 * number_of_points / 4 { side_length } else { -side_length };
        square_points.push((x as f32).into(), (y as f32).into());
    }
    Template::new("Square".to_string(), &square_points).unwrap()
}

fn triangle_template(number_of_points: usize, side_length: f64) -> Template {
    // Draw a triangle with buffer_size points and a specific side length
    let mut triangle_points = Path2D::default();
    for i in 0..number_of_points {
        let x = if i < number_of_points / 3 { side_length } else if i < 2 * number_of_points / 3 { -side_length } else { 0.0 };
        let y = if i < number_of_points / 3 { 0.0 } else if i < 2 * number_of_points / 3 { 0.0 } else { side_length };
        triangle_points.push((x as f32).into(), (y as f32).into());
    }
    Template::new("Triangle".to_string(), &triangle_points).unwrap()
}

fn main() {
    let mut points = VecDeque::new(); // Circular buffer: a point is added at the end and removed from the front
    let buffer_size = 100;   // Maximum number of points to store
    let mouse_sampling_time_ms = 10; // Time between each sampling of the mouse position
    let guess_threshold = 0.9;  // Threshold for the guessture algorithm. If the similarity is above this threshold, the shape is detected

    // Create the template shapes for guessture
    let templates = vec![
        circle_template(buffer_size, 100.0),
        square_template(buffer_size, 100.0),
        triangle_template(buffer_size, 100.0),
    ];

    loop {
        if points.len() == buffer_size { points.pop_front(); }

        let position = Mouse::get_mouse_position();
        match position {
            Mouse::Position { x, y } => {
                points.push_back(Mouse::Position { x, y });
            }
            Mouse::Error => {
                println!("Error: Could not get the mouse position");
                break;  // Exit the loop if an error occurs
            }
        }

        if points.len() < buffer_size { continue; }

        let shape = detect_shape(&points, &templates, guess_threshold);
        if shape != Shape::Unknown {
            println!("Detected shape: {:?}", shape);
        }

        sleep(Duration::from_millis(mouse_sampling_time_ms));
    }
}