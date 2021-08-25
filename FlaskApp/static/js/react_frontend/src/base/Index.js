import React from "react";

import Header from "./Header";
import NavGrid from "./NavGrid";

import "../css/base.css";
// import "../css/bootstrap.min.css";

const sections = [
  "gravitational n-body dynamics",
  "oscillations",
  "statistical physics & thermodynamics",
  "emergent behavior & cellular automata",
  "electro-magnetism",
  "fluid dynamics",
  // "chronos",
];
const subsections = {
  "gravitational n-body dynamics": [
    { id: "3body_moon" },
    { id: "3body_fig8" },
    { id: "nbody_flowers" },
    { id: "nbody_asteroids" },
  ],
  oscillations: [
    { id: "single_pendulum" },
    { id: "double_pendulum" },
    { id: "lissajous" },
  ],
  "statistical physics & thermodynamics": [
    { id: "gas_in_a_box" },
    { id: "brownian_motion" },
    { id: "ising" },
    { id: "mc_pi_darts" },
  ],
  "emergent behavior & cellular automata": [
    { id: "boids" },
    { id: "ants" },
    { id: "game_of_life" },
    { id: "rock_paper_scissors" },
  ],
  "electro-magnetism": [{ id: "lorentz" }, { id: "charge_interaction" }],
  "fluid dynamics": [{ id: "incompressible_fluid" }, { id: "diffusion" }],
  chronos: [{ id: "correlation_finder" }, { id: "stats" }],
};

class Index extends React.Component {
  constructor(props) {
    super(props);
    this.sections = sections;
    this.subsections = subsections;
    this.updateViewID = props.updateViewID;
  }
  render() {
    return (
      <div className="index">
        <Header updateViewID={this.updateViewID} />
        <div id="content">
          {this.sections.map((title, idx) => {
            return (
              <div key={idx}>
                <p className="navgrid_section_title">{title}</p>
                <div id="navgrid_section">
                  <NavGrid
                    updateViewID={this.updateViewID}
                    subsections={this.subsections[title]}
                  ></NavGrid>
                </div>
              </div>
            );
          })}
        </div>
      </div>
    );
  }
}
export default Index;
