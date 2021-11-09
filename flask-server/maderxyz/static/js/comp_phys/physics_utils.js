export function kepler_velocity(r, M = 1, G = 1) {
  return Math.sqrt((G * M) / r);
}

export function get_boltzmann_probability(dE, T, k_B = 1) {
  const beta = 1 / (k_B * T);
  return Math.exp(-beta * dE);
}

export function apply_periodic_bounds(idx, N) {
  if (idx >= N) {
    idx -= N;
  } else if (idx < 0) {
    idx += N;
  }
  return idx;
}
