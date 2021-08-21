import React, { Component } from "react";
// import Index from "./components/index/Index";
import Index from "./base/Index";

import "./css/base.css";

export default class App extends Component {
  constructor(props) {
    super(props);
    this.state = {
      currentViewID: this.props["currentViewID"],
      onMobile: window.innerWidth <= 640,
    };
  }

  updateViewID(newViewID) {
    console.log("new view: ", newViewID);
    this.setState({ currentViewID: newViewID });
  }

  render() {
    if (this.state.currentViewID === "index") {
      return <Index updateViewID={this.updateViewID} />;
    } else {
      return "aaaaaaaa";
    }
  }
}
