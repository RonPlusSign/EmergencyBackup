use std::collections::VecDeque;

use guessture::{find_matching_template_with_defaults, Path2D, Template};
use mouse_position::mouse_position::Mouse;
use plotters::drawing::IntoDrawingArea;
use plotters::style::Color;
use plotters::style::colors::colormaps::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Shape { Unknown, Circle, Square, Triangle }

pub enum Confirm { Yes, No }

/// Detect the shape of the given list of points
/// #### Arguments
/// * `points`: A list of points representing the mouse positions
/// #### returns
///     Shape of the detected shape
pub fn detect_shape(points: &VecDeque<Mouse>, templates: &Vec<Template>, threshold: f32) -> Shape {
    // Create a path from the list of points
    let mut path = Path2D::default();
    for point in points {
        match *point {
            Mouse::Position { x, y } => { path.push((x as f32).into(), (y as f32).into()); }
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
    } else { Shape::Unknown };
}

/// Draw a circle with buffer_size points and a specific radius.
/// The circle is drawn by drawing the circumference of the circle
///
/// #### Arguments
/// * `number_of_points`: Number of points to draw the circle
/// * `radius`: Radius of the circle
/// * `invert_direction`: If true, draw the circle in the opposite direction (counter-clockwise)
pub fn circle_template(number_of_points: usize, radius: f32, invert_direction: bool) -> Template {
    let mut circle_points = Path2D::default();
    let range: Vec<usize> = if invert_direction { (0..number_of_points).rev().collect() } else { (0..number_of_points).collect() };
    for i in range {
        let angle = 2.0 * std::f64::consts::PI * (i as f64) / (number_of_points as f64);
        let x = radius as f64 * angle.cos();
        let y = radius as f64 * angle.sin();
        circle_points.push((x as f32).into(), (y as f32).into());
    }
    Template::new("Circle".to_string(), &circle_points).unwrap()
}


/// Draw a square with buffer_size points and a specific side length.
///
/// The square is drawn by drawing the 4 sides of the square, each with number_of_points/4 points.
/// Each side is evenly distanced in the range [-side_length/2, side_length/2]
///
/// #### Arguments
/// * `number_of_points`: Number of points to draw the square
/// * `side_length`: Side length of the square
/// * `invert_direction`: If true, draw the square in the opposite direction (anti-clockwise)
pub fn square_template(number_of_points: usize, side_length: f32, invert_direction: bool) -> Template {
    let mut square_points = Path2D::default();
    let range: Vec<usize> = if invert_direction { (0..number_of_points).collect() } else { (0..number_of_points).rev().collect() };
    for i in range {
        // Draw the 4 sides of the square, each with buffer_size / 4 points, evenly distanced in the range [-side_length/2, side_length/2]
        let side = i / (number_of_points / 4);
        let points_per_side = number_of_points / 4;
        let x = match side {
            0 => -side_length / 2.0 + (i % points_per_side) as f32 / points_per_side as f32 * side_length,
            1 => side_length / 2.0,
            2 => side_length / 2.0 - (i % points_per_side) as f32 / points_per_side as f32 * side_length,
            3 => -side_length / 2.0,
            _ => 0.0
        };

        let y = match side {
            0 => -side_length / 2.0,
            1 => -side_length / 2.0 + (i % points_per_side) as f32 / points_per_side as f32 * side_length,
            2 => side_length / 2.0,
            3 => side_length / 2.0 - (i % points_per_side) as f32 / points_per_side as f32 * side_length,
            _ => 0.0
        };

        square_points.push(x.into(), y.into());
    }
    Template::new("Square".to_string(), &square_points).unwrap()
}

/// Draw a triangle with buffer_size points and a specific side length.
///
/// The triangle equilateral and is drawn by drawing the 3 sides of the triangle, each with number_of_points/3 points.
///
/// #### Arguments
/// * `number_of_points`: Number of points to draw the triangle
/// * `side_length`: Side length of the triangle
/// * `invert_direction`: If true, draw the triangle in the opposite direction (anti-clockwise)
pub fn triangle_template(number_of_points: usize, side_length: f32, invert_direction: bool) -> Template {
    let mut triangle_points = Path2D::default();

    // Calculate the vertices of the triangle
    let height = (side_length * (3.0_f32).sqrt()) / 2.0;
    let vertices = [
        (0.0, height / 2.0),
        (-side_length / 2.0, -height / 2.0),
        (side_length / 2.0, -height / 2.0),
    ];

    // Number of points per side
    let points_per_side = number_of_points / 3;

    // Interpolate points between vertices
    for i in 0..points_per_side {
        let t = i as f32 / points_per_side as f32;
        let (x1, y1) = vertices[0];
        let (x2, y2) = vertices[1];
        let x = x1 + t * (x2 - x1);
        let y = y1 + t * (y2 - y1);
        triangle_points.push(x.into(), y.into());
    }

    for i in 0..points_per_side {
        let t = i as f32 / points_per_side as f32;
        let (x1, y1) = vertices[1];
        let (x2, y2) = vertices[2];
        let x = x1 + t * (x2 - x1);
        let y = y1 + t * (y2 - y1);
        triangle_points.push(x.into(), y.into());
    }

    for i in 0..points_per_side {
        let t = i as f32 / points_per_side as f32;
        let (x1, y1) = vertices[2];
        let (x2, y2) = vertices[0];
        let x = x1 + t * (x2 - x1);
        let y = y1 + t * (y2 - y1);
        triangle_points.push(x.into(), y.into());
    }

    if invert_direction {
        let mut triangle_points_new = Path2D::default();
        triangle_points.points().iter().rev().for_each(|(x, y)| triangle_points_new.push(*x, *y));
        triangle_points = triangle_points_new;
    }
    Template::new("Triangle".to_string(), &triangle_points).unwrap()
}

/// Draw a shape using plotters
///
/// #### Arguments
/// * `shape`: Path2D representing the shape
/// * `title`: File name to save the plot
pub fn draw_shape(shape: Path2D, title: String) {
    // Find the bounding box of the shape
    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;
    for (x, y) in shape.points() {
        if x < min_x { min_x = x; }
        if x > max_x { max_x = x; }
        if y < min_y { min_y = y; }
        if y > max_y { max_y = y; }
    }

    let root = plotters::prelude::BitMapBackend::new(&title, (800, 800)).into_drawing_area();
    root.fill(&plotters::prelude::WHITE).unwrap();

    let mut chart = plotters::chart::ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .margin(10)
        .build_cartesian_2d(min_x as f64..max_x as f64, min_y as f64..max_y as f64)
        .unwrap();
    chart.configure_mesh().draw().unwrap();
    let points = shape.points().iter().cloned().collect::<Vec<(f32, f32)>>();
    chart.draw_series(points.iter()
        .enumerate()
        .map(|(index, (x, y))| plotters::prelude::Circle::new((*x as f64, *y as f64), 5, VulcanoHSL.get_color(index as f64 / points.len() as f64).filled())))
        .unwrap();
}

/// Draw multiple shapes in the same plot using plotters
///
/// #### Arguments
/// * `shapes`: List of Path2D representing the shapes
/// * `title`: File name to save the plot
pub fn draw_multiple_shapes(shapes: Vec<Path2D>, title: String) {
    // Find the bounding box of the shape
    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;
    for shape in &shapes {
        for (x, y) in shape.points() {
            if x < min_x { min_x = x; }
            if x > max_x { max_x = x; }
            if y < min_y { min_y = y; }
            if y > max_y { max_y = y; }
        }
    }

    let root = plotters::prelude::BitMapBackend::new(&title, (800, 800)).into_drawing_area();
    root.fill(&plotters::prelude::WHITE).unwrap();

    let mut chart = plotters::chart::ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .margin(10)
        .build_cartesian_2d(min_x as f64..max_x as f64, min_y as f64..max_y as f64)
        .unwrap();
    chart.configure_mesh().draw().unwrap();

    enum ColorPalette { VulcanoHSL(VulcanoHSL), MandelbrotHSL(MandelbrotHSL), ViridisRGB(ViridisRGB), Copper(Copper), Bone(Bone) }
    let palettes: Vec<ColorPalette> = vec![ColorPalette::VulcanoHSL(VulcanoHSL), ColorPalette::MandelbrotHSL(MandelbrotHSL), ColorPalette::ViridisRGB(ViridisRGB), ColorPalette::Copper(Copper), ColorPalette::Bone(Bone)];

    for (shape_index, shape) in shapes.iter().enumerate() {
        let points = shape.points().iter().cloned().collect::<Vec<(f32, f32)>>();
        chart.draw_series(points.iter()
            .enumerate()
            .map(|(index, (x, y))| plotters::prelude::Circle::new((*x as f64, *y as f64), 5, match &palettes[shape_index % palettes.len()] {
                ColorPalette::VulcanoHSL(palette) => palette.get_color(index as f64 / points.len() as f64).filled(),
                ColorPalette::MandelbrotHSL(palette) => palette.get_color(index as f64 / points.len() as f64).filled(),
                ColorPalette::ViridisRGB(palette) => palette.get_color(index as f64 / points.len() as f64).filled(),
                ColorPalette::Copper(palette) => palette.get_color(index as f64 / points.len() as f64).filled(),
                ColorPalette::Bone(palette) => palette.get_color(index as f64 / points.len() as f64).filled(),
            })))
            .unwrap();
    }
}

/// Check if all the points are similar (within a threshold). If the points are similar, the mouse is not moving.
///
/// #### Arguments
/// * `points`: A list of points representing the mouse positions
pub fn all_points_similar(points: &VecDeque<Mouse>) -> bool {
    let threshold = 5;  // Number of pixels
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
    return true;
}


/// Convert a list of points to a scaled&centered Path2D
/// ### Arguments
/// * `points`: A list of points representing the mouse positions
/// * `max_range_for_dimensions`: maximum variation for the dimensions
/// ### returns
/// Path2D containing the scaled points
pub fn points_to_path(points: &VecDeque<Mouse>, max_range_for_dimensions: i32) -> Path2D {
    // The path is scaled to fit within the maximum dimension (range [-max_dimension/2, max_dimension/2], centered at (0, 0))

    let mut path = Path2D::default();
    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;
    let mut mean_x = 0.0;
    let mut mean_y = 0.0;

    // Find the bounding box of the points and calculate the mean
    for point in points {
        match *point {
            Mouse::Position { x, y } => {
                if x < min_x as i32 { min_x = x as f32; }
                if x > max_x as i32 { max_x = x as f32; }
                if y < min_y as i32 { min_y = y as f32; }
                if y > max_y as i32 { max_y = y as f32; }
                mean_x += x as f32;
                mean_y += y as f32;
            }
            Mouse::Error => { return path; }
        }
    }
    mean_x /= points.len() as f32;
    mean_y /= points.len() as f32;

    // Scale the points to fit within the maximum dimension
    let scale = max_range_for_dimensions as f32 / (max_x - min_x).max(max_y - min_y);
    for point in points {
        match *point {
            Mouse::Position { x, y } => {
                let x = (x as f32 - mean_x) * scale;
                let y = (y as f32 - mean_y) * scale;
                path.push(x.into(), y.into());
            }
            Mouse::Error => { return path; }
        }
    }
    path
}