import React from "react";
import ReactDOM from "react-dom";
import App from "./App";

import "../css/base.css";

// import("../pkg/index.js").catch(console.error);

ReactDOM.render(
  <App currentViewID="single_pendulum" />,
  document.getElementById("react-root")
);
