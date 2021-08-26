import React, { Component } from "react";
// import Index from "./components/index/Index";
import Index from "./base/Index";
// import Boids from "./comp_phys/boids/Boids";

import "../css/base.css";

export default class App extends Component {
  constructor(props) {
    super(props);
    this.state = {
      currentViewID: this.props["currentViewID"],
      onMobile: window.innerWidth <= 640,
    };
    this.updateViewID = (newViewID) => {
      console.log("new view: ", newViewID);
      this.setState({ currentViewID: newViewID });
    };
  }

  render() {
    if (this.state.currentViewID === "index") {
      return <Index updateViewID={this.updateViewID} />;
      // } else if (this.state.currentViewID === "boids") {
      //   return <Boids updateViewID={this.updateViewID} />;
    } else {
      return this.state.currentViewID + " is not yet implemented :(";
    }
  }
}
