mod pattern_recognition;
mod configuration;

use std::collections::VecDeque;
use std::thread::sleep;
use std::time::Duration;
use mouse_position::mouse_position::{Mouse};
use pattern_recognition::{all_points_similar, draw_shape, draw_multiple_shapes, Shape};

fn main() {
    let mut points = VecDeque::new(); // Circular buffer: a point is added at the end and removed from the front
    let buffer_size = 200;   // Maximum number of points to store
    let mouse_sampling_time_ms = 10; // Time between each sampling of the mouse position
    let guess_threshold = 0.9;  // Threshold for the guessture algorithm. If the similarity is above this threshold, the shape is detected

    // Create the template shapes for guessture
    let templates = vec![
        pattern_recognition::circle_template(buffer_size, 100.0),
        pattern_recognition::square_template(buffer_size, 100.0),
        pattern_recognition::triangle_template(buffer_size, 100.0),
    ];

    // for template in &templates { // Draw the shapes, for debug
    //     draw_shape(template.path.clone(), template.name.clone() + ".png");
    // }

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

        // If the points are all similar, skip the detection (mouse not moving)
        if all_points_similar(&points) { continue; }

        let shape = pattern_recognition::detect_shape(&points, &templates, guess_threshold);
        if shape != Shape::Unknown {
            println!("Detected shape: {:?}", shape);

            // For debug, convert the points to a Path2D and draw the shape comparison
            // let path = pattern_recognition::points_to_path(&points, 250);
            // let template_path = match shape {
            //     Shape::Circle => templates[0].path.clone(),
            //     Shape::Square => templates[1].path.clone(),
            //     Shape::Triangle => templates[2].path.clone(),
            //     Shape::Unknown => path.clone(),
            // };
            // draw_multiple_shapes(vec![path, template_path], "detected_shape.png".to_string());

            points.clear(); // Clear the points buffer, so the shape is not detected again
        }

        sleep(Duration::from_millis(mouse_sampling_time_ms));
    }
}