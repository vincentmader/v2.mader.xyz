export function draw_point(
  ctx,
  x,
  y,
  radius,
  edge_color = "white",
  fill_color = "white"
) {
  ctx.strokeStyle = edge_color;
  ctx.fillStyle = fill_color;
  ctx.beginPath();
  ctx.arc(x, y, radius, 0, 2 * Math.PI);
  ctx.stroke();
  ctx.fill();
}

export function draw_line(ctx, x_1, y_1, x_2, y_2, color) {
  ctx.strokeStyle = color;

  ctx.beginPath();
  ctx.moveTo(x_1, y_1);
  ctx.lineTo(x_2, y_2);
  ctx.stroke();
}
