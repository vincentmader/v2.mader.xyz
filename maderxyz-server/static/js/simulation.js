import init, { Simulation } from "../../static/pkg/maderxyz_client.js";

let page_id = document.getElementById("simulation.js").getAttribute("page_id");

// TODO: async
(async () => {
  await init();

  // initialize simulation with page id
  let simulation = Simulation.new(page_id);
  simulation.init();

  // setup event listeners for buttons
  var buttons = document.getElementsByTagName("button");
  for (let idx = 0; idx < buttons.length; idx++) {
    let button = buttons[idx];
    let button_id = button.id;
    button.addEventListener("click", () => {
      simulation.handle_button_event(button_id);
    });
  }
  // setup event listeners for sliders
  var inputs = document.getElementsByTagName("input");
  for (let idx = 0; idx < inputs.length; idx++) {
    let slider = inputs[idx];
    if (slider.getAttribute("type") != "range") continue;
    let slider_id = slider.id;
    slider.addEventListener("change", () => {
      simulation.handle_slider_event(slider_id);
    });
  }

  // step loop
  let N = 1;
  const loop = () => {
    for (let _ = 0; _ < N; _++) {
      simulation.step();
    }
    simulation.render();
    requestAnimationFrame(loop);
  };
  loop();
})();
