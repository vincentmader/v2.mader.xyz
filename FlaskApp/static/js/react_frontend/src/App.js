import React, { Component } from "react";
import Index from "./components/index/Index";

export default class App extends Component {
  constructor(props) {
    super(props);
    this.state = {
      currentView: this.props["currentView"],
      onMobile: window.innerWidth <= 500,
    };
  }
  render() {
    if (this.state.currentView === "index") {
      return <Index />;
    }
  }
}
