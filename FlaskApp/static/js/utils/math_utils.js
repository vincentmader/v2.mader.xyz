export function get_L2_norm(vec) {
  var sum = 0;
  for (const i of vec) {
    sum += i ** 2;
  }
  return Math.sqrt(sum);
}
