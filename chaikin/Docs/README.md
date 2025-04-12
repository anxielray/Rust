# Chaikin's Algorithm Animation

This project implements Chaikin's algorithm as a step-by-step animation.

- You can explore the instructions for the project in the [Quiz](INSTRUCTIONS.md)

- The program allows a user to;

  - Allows the user to draw 1 or more points on a canvas.
  - Allows the user to press `Enter` to start the animation.
  - Allows the user to close the canvas using the `Escape` key.
- It animates each step taken to obtain the final result of a drawn curve using Chaikin's algorithm. The animation plays for 7 steps and then restarts.

## Functionality

- The canvas receives input from the mouse. The user can use the left button to place control points for Chaikin's algorithm.
- The created control points are used to draw a curve using Chaikin's algorithm by the instantiation by hitting the `Enter` key.
- Pressing Enter starts the animation if control points are drawn on the canvas. It cycles through the steps until the 7th step of Chaikin's algorithm, then restarts in a loop.
- If Enter is pressed before any points have been drawn, nothing happens. It is still possible to draw points.

### Algorithm Instances

- If the canvas only has one control point, the program only presents that point and does not cycle through the steps.
- If the canvas has only two control points, the program draws a straight line.
- If instead the canvas has more than two control points, the program draws a curve using Chaikin's algorithm.
- Pressing Escape quits the window.

## Bonus

- It is possible to clear the screen, so that the user can select new control points.
- It is possible to drag the control points in real time.

## Controls

To use the application, follow these steps:

1. Draw points on the canvas using the left mouse button.
2. Press ` Enter ` to open an animation-window and start the animation.
3. Press ` Escape ` to quit the application window.
4. Press ` r ` to reset/clear the canvas and draw new points.
5. Hold down the Control key and drag the mouse to move a point in real time.

## Installation

Provide code and explanations for how to install your project.

```bash
git clone https://github.com/anxielray/Rust.git
cd Rust/chaikin
cargo run
```
