import React from "react";
import ReactDOM from "react-dom";
import App from "./App";

import "../css/base.css";

import("../pkg/index.js").catch(console.error);

ReactDOM.render(
  <App currentViewID="index" />,
  document.getElementById("react-root")
);
