export function draw_point(ctx, x, y, radius) {
  ctx.strokeStyle = "white";
  ctx.fillStyle = "white";
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
