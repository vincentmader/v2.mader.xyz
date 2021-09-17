import NavGrid from "./navgrid.js";

const INDEX_SECTIONS = [
  "gravitational n-body dynamics",
  "oscillators",
  "emergent behavior & cellular automata",
  "statistical physics & thermodynamics",
  "electro-magnetism",
  "fluid dynamics",
  // "chronos",
  "various",
];
const INDEX_SUBSECTION = {
  "gravitational n-body dynamics": [
    { id: "3body_moon" },
    { id: "3body_fig8" },
    { id: "nbody_flowers" },
    // { id: "nbody_asteroids" },
  ],
  oscillators: [
    { id: "single_pendulum" },
    { id: "double_pendulum" },
    { id: "lissajous" },
  ],
  "statistical physics & thermodynamics": [
    { id: "gas_in_a_box" },
    { id: "brownian_motion" },
    { id: "ising" },
  ],
  "emergent behavior & cellular automata": [
    { id: "boids" },
    { id: "ants" },
    { id: "game_of_life" },
  ],
  "electro-magnetism": [{ id: "lorentz" }, { id: "charge_interaction" }],
  "fluid dynamics": [{ id: "incompressible_fluid" }, { id: "diffusion" }],
  chronos: [{ id: "correlation_finder" }, { id: "stats" }],
  various: [{ id: "mc_pi_darts" }, { id: "rock_paper_scissors" }],
};

const template = document.createElement("template");
template.innerHTML = `
    <div class="maderxyz-index"'> 
        <div class="navgrid" id="navgrid"> </div>
    </div>

`;
// <maderxyz-index-navgrid class="navgrid"> </div>
// <script src="static/frontend-js/navgrid.js"></script>

class Index extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: "open" });
    this.shadowRoot.appendChild(template.content.cloneNode(true));

    for (let section_title of INDEX_SECTIONS) {
      let foo = '<p class="navgrid_section_title">';
      foo += section_title;
      foo += "</p>";

      foo += `
            <div class="navgrid_section">
                aaaaa
            </div>
        `;

      this.shadowRoot.getElementById("navgrid").innerHTML += foo;

      // <div id="section_navgrid">
      //   <NavGrid
      //     updateViewID={this.updateViewID}
      //     subsections={this.subsections[title]}
      //   ></NavGrid>
      // </div>
    }

    // var index = document.getElementById("maderxyz-index");
  }
}

window.customElements.define("maderxyz-index", Index);
