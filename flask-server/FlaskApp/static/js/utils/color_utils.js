export function get_random_color() {
  let r = Math.floor(255 * Math.random());
  let g = Math.floor(255 * Math.random());
  let b = Math.floor(255 * Math.random());

  return "rgba(" + r + ", " + g + ", " + b + ", 1)";
}
