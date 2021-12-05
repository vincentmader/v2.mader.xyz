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
  for (let i = 0; i < buttons.length; i++) {
    let button = buttons[i];
    let button_id = button.id;
    button.innerHTML = button_id;
    button.addEventListener("click", () => {
      simulation.handle_button_event(button_id);
    });
  }
  // setup event listeners for sliders
  var inputs = document.getElementsByTagName("input");
  for (let i = 0; i < inputs.length; i++) {
    let slider = inputs[i];
    if (slider.getAttribute("type") != "range") continue;
    let slider_id = slider.id;
    slider.innerHTML = slider_id;
    slider.addEventListener("change", () => {
      simulation.handle_slider_event(slider_id);
    });
  }

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
