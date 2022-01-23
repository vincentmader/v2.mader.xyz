import init, { Simulation } from "../../static/pkg/mxyz_client.js";

let sim_id = document.getElementById("simulation.js").getAttribute("sim_id");

(async () => {
  await init();

  // initialization
  let simulation = Simulation.new(sim_id);
  simulation.init();

  // event listeners for options
  var options = document.getElementsByTagName("select");
  for (let idx = 0; idx < options.length; idx++) {
    let option = options[idx];
    option.addEventListener("change", () => {
      simulation.handle_option_event(option.id);
    });
  }

  // event listeners for buttons
  var buttons = document.getElementsByTagName("button");
  for (let idx = 0; idx < buttons.length; idx++) {
    let button = buttons[idx];
    button.addEventListener("click", () => {
      simulation.handle_button_event(button.id);
    });
  }

  // event listeners for sliders
  var inputs = document.getElementsByTagName("input");
  for (let idx = 0; idx < inputs.length; idx++) {
    let slider = inputs[idx];
    if (slider.getAttribute("type") != "range") continue;
    slider.addEventListener("change", () => {
      simulation.handle_slider_event(slider.id);
    });
  }

  // step loop
  const loop = () => {
    simulation.step();
    simulation.render();
    requestAnimationFrame(loop);
  };
  loop();
})();
