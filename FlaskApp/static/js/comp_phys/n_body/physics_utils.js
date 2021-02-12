export function kepler_velocity(r, M = 1, G = 1) {
  return Math.sqrt((G * M) / r);
}
