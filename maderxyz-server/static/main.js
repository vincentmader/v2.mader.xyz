import init, { Simulation } from "./pkg/maderxyz_frontend.js";

let page_id = "3body_fig8"; // TODO: get from tera template (?)

(async () => {
  await init();

  var simulation = Simulation.new("hello world!");
  simulation.init();

  let N = 1;
  const loop = () => {
    for (let i = 0; i < N; i++) {
      simulation.step();
    }
    requestAnimationFrame(loop);
  };
  loop();
})();
