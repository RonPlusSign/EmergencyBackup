from math import floor, ceil

import numpy as np
from PIL import Image, ImageDraw


# Function to create a blank transparent image
def create_blank_image(size):
    return Image.new("RGBA", size, (255, 255, 255, 0))


# Function to create each frame of the circle being drawn
def draw_circle_frame(image, radius, angle, stroke_width, color):
    draw = ImageDraw.Draw(image)
    cx, cy = image.size[0] // 2, image.size[1] // 2  # Center of the image

    # Calculate start and end points for the arc
    start_angle = -90  # Start from the top
    end_angle = -90 + angle

    # Draw the arc
    draw.arc([cx - radius, cy - radius, cx + radius, cy + radius], start=start_angle, end=end_angle, fill=color, width=stroke_width)

    # Adjust the radius to account for the stroke width (so that the circles align with the stroke edge)
    adjusted_radius = radius - stroke_width / 2

    # Compute the position of the start and end points of the arc for round edges
    start_x = cx + adjusted_radius * np.cos(np.radians(start_angle))
    start_y = cy + adjusted_radius * np.sin(np.radians(start_angle))
    end_x = cx + adjusted_radius * np.cos(np.radians(end_angle)) + 1
    end_y = cy + adjusted_radius * np.sin(np.radians(end_angle)) + 1

    # Draw circles at the start and end points for rounded edges
    draw.ellipse([start_x - stroke_width // 2 + 1, start_y - stroke_width // 2 + 1, start_x + stroke_width // 2 - 1, start_y + stroke_width // 2 - 1], fill=color)
    draw.ellipse([end_x - stroke_width // 2 + 1, end_y - stroke_width // 2 + 1, end_x + stroke_width // 2 - 1, end_y + stroke_width // 2 - 1], fill=color)

    return image


def draw_circle(image_size, radius, stroke_width, color, total_frames):
    frames = []  # List to hold frames

    # Generate each frame by increasing the arc angle
    for i in range(total_frames + 1):
        angle = (i / total_frames) * 360  # Angle for the current frame

        image = create_blank_image(image_size)  # Create a new blank image
        frame = draw_circle_frame(image, radius, angle, stroke_width, color)  # Draw the arc
        frames.append(frame)

    # Add extra frames at the end to hold the final drawing
    last_frame = frames[-1]
    for _ in range(extra_hold_frames):
        frames.append(last_frame)

    # Save the frames as a GIF
    frames[0].save("circle.gif", save_all=True, append_images=frames[1:], duration=50, loop=0, transparency=0)
    print("GIF saved as circle.gif")


# Function to create each frame of the square being drawn
def draw_square_frame(image, side_length, angle, stroke_width, color):
    draw = ImageDraw.Draw(image)
    cx, cy = image.size[0] // 2, image.size[1] // 2  # Center of the image

    # Coordinates of the square's corners (starting from top-left and going clockwise)
    half_side = side_length // 2
    points = [
        (cx - half_side, cy - half_side),  # Top-left corner
        (cx + half_side, cy - half_side),  # Top-right corner
        (cx + half_side, cy + half_side),  # Bottom-right corner
        (cx - half_side, cy + half_side)  # Bottom-left corner
    ]

    # Draw lines of the square step by step according to the angle
    total_angle = 360
    total_perimeter = 4 * side_length
    perimeter_covered = (angle / total_angle) * total_perimeter

    current_perimeter = 0
    for i in range(4):  # Loop through each side of the square
        p1 = points[i]
        p2 = points[(i + 1) % 4]
        segment_length = np.linalg.norm(np.array(p2) - np.array(p1))  # Length of current side

        if current_perimeter + segment_length >= perimeter_covered:
            # If we are at the side where the drawing should stop
            progress_ratio = (perimeter_covered - current_perimeter) / segment_length
            p2_progress = (
                p1[0] + progress_ratio * (p2[0] - p1[0]),
                p1[1] + progress_ratio * (p2[1] - p1[1])
            )
            draw.line([p1, p2_progress], fill=color, width=stroke_width)

            # Draw circle at the end point for rounded edge
            draw.ellipse([p2_progress[0] - stroke_width // 2 + 1, p2_progress[1] - stroke_width // 2 + 1, p2_progress[0] + stroke_width // 2 - 1, p2_progress[1] + stroke_width // 2 - 1], fill=color)
            break
        else:
            # Draw the whole side
            draw.line([p1, p2], fill=color, width=stroke_width)
            current_perimeter += segment_length

    # Draw circle at the start point for rounded edge
    draw.ellipse([points[0][0] - stroke_width // 2 + 1, points[0][1] - stroke_width // 2 + 1, points[0][0] + stroke_width // 2 - 1, points[0][1] + stroke_width // 2 - 1], fill=color)
    return image


def draw_square(image_size, side_length, stroke_width, color, total_frames):
    frames = []  # List to hold frames

    # Generate each frame by increasing the angle
    for i in range(total_frames + 1):
        angle = (i / total_frames) * 360  # Angle for the current frame
        image = create_blank_image(image_size)  # Create a new blank image
        frame = draw_square_frame(image, side_length, angle, stroke_width, color)  # Draw the square
        frames.append(frame)

    # Add extra frames at the end to hold the final drawing
    last_frame = frames[-1]
    for _ in range(extra_hold_frames):
        frames.append(last_frame)

    # Save the frames as a GIF
    frames[0].save("square.gif", save_all=True, append_images=frames[1:], duration=50, loop=0, transparency=0)
    print("GIF saved as square.gif")


def draw_triangle_frame(image, side_length, angle, stroke_width, color):
    draw = ImageDraw.Draw(image)
    cx, cy = image.size[0] // 2, image.size[1] // 2  # Center of the image

    # Calculate the height of the equilateral triangle
    height = np.sqrt(3) / 2 * side_length

    # Coordinates of the triangle's vertices (starting from topmost and going clockwise)
    half_side = side_length // 2
    points = [
        (cx, cy - height // 2),  # Topmost corner
        (cx + half_side, cy + height // 2),  # Bottom-right corner
        (cx - half_side, cy + height // 2)  # Bottom-left corner
    ]

    # Draw lines of the triangle step by step according to the angle
    total_angle = 360
    total_perimeter = 3 * side_length
    perimeter_covered = (angle / total_angle) * total_perimeter

    current_perimeter = 0
    for i in range(3):  # Loop through each side of the triangle
        p1 = points[i]
        p2 = points[(i + 1) % 3]
        segment_length = np.linalg.norm(np.array(p2) - np.array(p1))  # Length of current side

        if current_perimeter + segment_length >= perimeter_covered:
            # If we are at the side where the drawing should stop
            progress_ratio = (perimeter_covered - current_perimeter) / segment_length
            p2_progress = (
                p1[0] + progress_ratio * (p2[0] - p1[0]),
                p1[1] + progress_ratio * (p2[1] - p1[1])
            )
            draw.line([p1, p2_progress], fill=color, width=stroke_width)

            # Draw circle at the end point for rounded edge
            draw.ellipse([p2_progress[0] - stroke_width // 2 + 1, p2_progress[1] - stroke_width // 2 + 1, p2_progress[0] + stroke_width // 2 - 1, p2_progress[1] + stroke_width // 2 - 1], fill=color)
            break
        else:
            # Draw the whole side
            draw.line([p1, p2], fill=color, width=stroke_width)
            current_perimeter += segment_length

    # Draw circle at the start point for rounded edge
    draw.ellipse([points[0][0] - stroke_width // 2 + 1, points[0][1] - stroke_width // 2 + 1, points[0][0] + stroke_width // 2 - 1, points[0][1] + stroke_width // 2 - 1], fill=color)
    return image


def draw_triangle(image_size, side_length, stroke_width, color, total_frames):
    frames = []  # List to hold frames

    # Generate each frame by increasing the angle
    for i in range(total_frames + 1):
        angle = (i / total_frames) * 360  # Angle for the current frame
        image = create_blank_image(image_size)  # Create a new blank image
        frame = draw_triangle_frame(image, side_length, angle, stroke_width, color)  # Draw the triangle
        frames.append(frame)

    # Add extra frames at the end to hold the final drawing
    last_frame = frames[-1]
    for _ in range(extra_hold_frames):
        frames.append(last_frame)

    # Save the frames as a GIF
    frames[0].save("triangle.gif", save_all=True, append_images=frames[1:], duration=50, loop=0, transparency=0)
    print("GIF saved as triangle.gif")


def draw_cancel_frame(image, width, height, angle, stroke_width, color):
    # Function to create each frame of the ⋊ being drawn
    draw = ImageDraw.Draw(image)
    cx, cy = image.size[0] // 2, image.size[1] // 2  # Center of the image

    # Coordinates of the ⋊'s key points (starting from top-left)
    points = [
        (ceil(cx - width // 2), ceil(cy - height // 2)),  # Top-left point
        (ceil(cx + width // 2), ceil(cy + height // 2)),  # Bottom-right point
        (ceil(cx + width // 2), ceil(cy - height // 2)),  # Top-right point
        (ceil(cx - width // 2), ceil(cy + height // 2))  # Bottom-left point
    ]

    # Define line segments: top-left -> bottom-right, bottom-right -> top-right, top-right -> bottom-left
    segments = [
        (points[0], points[1]),  # Top-left to bottom-right
        (points[1], points[2]),  # Bottom-right to top-right
        (points[2], points[3])  # Top-right to bottom-left
    ]

    # Calculate the total perimeter of the symbol (3 lines)
    segment_lengths = [np.linalg.norm(np.array(p2) - np.array(p1)) for p1, p2 in segments]
    total_perimeter = sum(segment_lengths)
    perimeter_covered = (angle / 360) * total_perimeter

    current_perimeter = 0
    for i, (p1, p2) in enumerate(segments):
        segment_length = segment_lengths[i]

        if current_perimeter + segment_length > perimeter_covered:
            # If we are at the segment where the drawing should stop
            progress_ratio = (perimeter_covered - current_perimeter) / segment_length
            p2_progress = (floor(p1[0] + progress_ratio * (p2[0] - p1[0])), floor(p1[1] + progress_ratio * (p2[1] - p1[1])))
            draw.line([p1, p2_progress], fill=color, width=stroke_width, joint="curve")

            # Draw circle at the end point for rounded edge
            draw.ellipse([p2_progress[0] - stroke_width // 2 + 1, p2_progress[1] - stroke_width // 2 + 1, p2_progress[0] + stroke_width // 2 - 1, p2_progress[1] + stroke_width // 2 - 1], fill=color)
            break

        else:
            # Draw the whole segment
            draw.line([p1, p2], fill=color, width=stroke_width, joint="curve")
            current_perimeter += segment_length

            # Draw circle at the end point for rounded edge
            draw.ellipse([p2[0] - stroke_width // 2 + 1, p2[1] - stroke_width // 2 + 1, p2[0] + stroke_width // 2 - 1, p2[1] + stroke_width // 2 - 1], fill=color)

    # Draw circle at the start point for rounded edge
    draw.ellipse([points[0][0] - stroke_width // 2, points[0][1] - stroke_width // 2, points[0][0] + stroke_width // 2, points[0][1] + stroke_width // 2], fill=color)

    return image


def draw_cancel(image_size, width, height, stroke_width, color, total_frames):
    frames = []  # List to hold frames

    # Generate each frame by increasing the angle
    for i in range(total_frames + 1):
        angle = (i / total_frames) * 360  # Angle for the current frame
        image = create_blank_image(image_size)  # Create a new blank image
        frame = draw_cancel_frame(image, width, height, angle, stroke_width, color)  # Draw the ⋊
        frames.append(frame)

    # Add extra frames at the end to hold the final drawing
    last_frame = frames[-1]
    for _ in range(extra_hold_frames):
        frames.append(last_frame)

    # Save the frames as a GIF
    frames[0].save("cancel.gif", save_all=True, append_images=frames[1:], duration=50, loop=0, transparency=0)
    print("GIF saved as cancel.gif")


if __name__ == "__main__":
    # Parameters
    image_size = (400, 400)  # Size of the image (width, height)
    radius = 200  # Radius of the circle
    stroke_width = 30  # Stroke width of the circle
    confirm_color = (36, 181, 92, 255)  # Green color in RGBA
    cancel_color = (247, 25, 25, 255)  # Red color in RGBA
    total_frames = 60  # Total number of frames in the GIF
    side_length = 350  # Side length of the square
    extra_hold_frames = 20  # Number of extra frames to hold the final drawing

    draw_circle(image_size, radius, stroke_width, confirm_color, total_frames)
    draw_square(image_size, side_length, stroke_width, confirm_color, total_frames)
    draw_triangle(image_size, side_length, stroke_width, confirm_color, total_frames)
    draw_cancel(image_size, side_length, side_length, stroke_width, cancel_color, total_frames)
