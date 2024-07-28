use std::collections::VecDeque;
use std::thread::sleep;
use std::time::Duration;
use mouse_position::mouse_position::{Mouse};

#[derive(Debug)]
enum Shape { Circle, Square, Triangle, Unknown }

enum Confirm { Yes, No }

/// Detect the shape of the given list of points
/// ### Arguments
/// * `points`: A list of points representing the mouse positions
/// ### returns
///     Shape of the detected shape
fn detect_shape(points: &VecDeque<Mouse>) -> Shape {
    // TODO: Implement the shape detection algorithm

    // At the moment for simplicity, return a random shape
    let random_number = rand::random::<u8>() % 4;
    match random_number {
        0 => Shape::Circle,
        1 => Shape::Square,
        2 => Shape::Triangle,
        _ => Shape::Unknown
    }
}

fn main() {
    let mut points = VecDeque::new(); // Circular buffer: a point is added at the end and removed from the front
    let buffer_size = 10;   // Maximum number of points to store
    let mouse_sampling_time_ms = 100; // Time between each sampling of the mouse position

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

        print!("Mouse position: ");
        for point in &points {
            match point {
                Mouse::Position { x, y } => { print!("({}, {}) ", x, y); }
                _ => {}
            }
        }
        println!();

        let shape = detect_shape(&points);
        // println!("Detected shape: {:?}", shape);

        sleep(Duration::from_millis(mouse_sampling_time_ms));
    }
}