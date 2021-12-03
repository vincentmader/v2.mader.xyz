// import wasm script from static
import init, { Simulation } from "../../static/pkg/maderxyz_client.js";
// get page id
let page_id = document.getElementById("simulation.js").getAttribute("page_id");

// TODO: async
(async () => {
  await init();

  // initialize simulation with page id
  let simulation = Simulation.new(page_id);
  simulation.init();

  // step loop
  let N = 1;
  const loop = () => {
    for (let i = 0; i < N; i++) {
      simulation.step();
    }
    requestAnimationFrame(loop);
  };
  loop();
})();
