use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    let points = (-100..100).map(|i| {
        let x = i as f32;          //subtract 25 to center the sine wave
        let point = pt2(x, x.sin()) * 20.0; //scale sine wave by 20.0
        (point, STEELBLUE)
      });

      for point in points {
        let x = point.0.x;
        let y = point.0.y;
        draw.rect().w(10.0).h(10.0).x_y(x, y).z_degrees(y * 20.0);
      }
    //   draw.polyline()
    //       .weight(3.0)
    //       .points_colored(points);
    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
