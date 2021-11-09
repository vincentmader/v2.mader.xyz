import { draw_line, draw_point } from "../../utils/drawing_utils.js";

function xy_to_canvas_coords(x, y, W, H, zoom_level) {
  // define coordinate origin on screen
  const o_x = W / 2;
  const o_y = H / 2;
  return [x / zoom_level + o_x, -y / zoom_level + o_y];
}

export function draw_tails(
  ctx,
  system_states,
  frame_idx,
  tail_length,
  W,
  H,
  zoom_level
) {
  var current_system_state, previous_system_state;
  var coords_p, coords_c, x_p, y_p, x_c, y_c;
  var color, alpha;
  var nr_of_bodies = system_states[0].length / 6;
  for (const jdx of Array(tail_length).keys()) {
    // get current & previous system state
    current_system_state = system_states[Math.max(0, frame_idx - jdx)];
    previous_system_state = system_states[Math.max(0, frame_idx - jdx - 1)];
    for (let idx = 0; idx < nr_of_bodies; idx++) {
      // get information about body in previous & current timestep
      x_p = previous_system_state[6 * idx + 2];
      y_p = previous_system_state[6 * idx + 3];
      x_c = current_system_state[6 * idx + 2];
      y_c = current_system_state[6 * idx + 3];
      // convert to canvas coords (code units -> px)
      coords_p = xy_to_canvas_coords(x_p, y_p, W, H, zoom_level);
      coords_c = xy_to_canvas_coords(x_c, y_c, W, H, zoom_level);
      x_p = coords_p[0];
      y_p = coords_p[1];
      x_c = coords_c[0];
      y_c = coords_c[1];
      // calculate alpha value
      alpha = 1 - jdx / tail_length;
      color = "rgba(255, 255, 255, " + String(alpha) + ")";
      // color = "white";
      // draw line between previous & current position
      draw_line(ctx, x_p, y_p, x_c, y_c, color);
    }
  }
}

export function draw_bodies(ctx, system_state, nr_of_bodies, W, H, zoom_level) {
  // define coordinate origin on screen
  const o_x = W / 2;
  const o_y = H / 2;

  var m, R, x, y, u, v;
  var coords;
  for (let idx = 0; idx < nr_of_bodies; idx++) {
    // get information about body
    m = system_state[6 * idx];
    R = system_state[6 * idx + 1];
    x = system_state[6 * idx + 2];
    y = system_state[6 * idx + 3];
    u = system_state[6 * idx + 4];
    v = system_state[6 * idx + 5];
    // convert to canvas coords (code units -> px)
    coords = xy_to_canvas_coords(x, y, W, H, zoom_level);
    x = coords[0];
    y = coords[1];
    // draw body
    draw_point(ctx, x, y, 6); // TODO: make radius variable
  }
}

export function draw_velocities(
  ctx,
  system_state,
  nr_of_bodies,
  W,
  H,
  zoom_level
) {
  // define coordinate origin on screen
  const o_x = W / 2;
  const o_y = H / 2;

  var m, R, x, y, u, v;
  var x1, y1, x2, y2;
  var coords;
  const velocity_scale_factor = 40;
  for (let idx = 0; idx < nr_of_bodies; idx++) {
    // get information about body
    m = system_state[6 * idx];
    R = system_state[6 * idx + 1];
    x = system_state[6 * idx + 2];
    y = system_state[6 * idx + 3];
    u = system_state[6 * idx + 4];
    v = system_state[6 * idx + 5];
    // convert to canvas coords (code units -> px)
    coords = xy_to_canvas_coords(x, y, W, H, zoom_level);
    x1 = coords[0];
    y1 = coords[1];
    x2 = x1 + velocity_scale_factor * u;
    y2 = y1 - velocity_scale_factor * v;
    // draw body
    draw_line(ctx, x1, y1, x2, y2, "red");
  }
}
