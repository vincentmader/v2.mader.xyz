import React from "react";

// import("../../../pkg/index.js").catch(console.error);
// import("../../../pkg/index").catch(console.error);
// import { main_js } from "../../../pkg/index.js";
// import "../../../pkg/index.js";
import { main_js } from "../../../pkg/index.js";

// import { fib } from "wasm_test";
// import * as wasm_test from "wasm_test/wasm_test_bg";
// import { greet } from "wasm_test/wasm_test_bg";
// let wasm_test = require("wasm_test/wasm_test_bg");
// import init, { greet } from "wasm_test";
// import { greet } from "wasm_test";
// import("wasm_test").then((module) => {
// console.log(module);
// });

const FPS_GOAL = 60;

class Canvas extends React.Component {
  constructor(props) {
    super(props);
    this.canvasRef = React.createRef();
  }

  componentDidUpdate() {
    const canvas = this.canvasRef.current;
    const ctx = canvas.getContext("2d");
    const width = canvas.width;
    const height = width;
    canvas.height = height;
    ctx.save();
    ctx.beginPath();
    ctx.clearRect(0, 0, width, height);
    // ctx.translate(width / 2, height / 2);
    // ctx.rotate((angle * Math.PI) / 180);
    ctx.fillStyle = "black";
    ctx.fillRect(0, 0, width, height);
    ctx.restore();
  }

  render() {
    return <canvas className="canvas" ref={this.canvasRef} />;
  }
}

class Animation extends React.Component {
  constructor(props) {
    super(props);
    this.state = {};
    this.updateAnimationState = this.updateAnimationState.bind(this);
  }

  updateAnimationState() {
    this.setState((prevState) => ({}));
    this.rAF = requestAnimationFrame(this.updateAnimationState);
  }

  componentDidMount() {
    this.rAF = requestAnimationFrame(this.updateAnimationState);
  }

  componentWillUnmount() {
    cancelAnimationFrame(this.rAF);
  }

  render() {
    return <Canvas />;
  }
}

export default class Boids extends React.Component {
  constructor(props) {
    super(props);
    this.updateViewID = props.updateViewID;
    this.canvasRef = React.createRef();
    this.frame_idx = 0;
    main_js();
  }

  render() {
    return (
      <div id="content">
        <div className="section_title">boids</div>
        <div className="section">
          <Animation />
        </div>
      </div>
    );
    // <canvas
    //   ref={this.canvasRef}
    //   className="canvas"
    //   id="canvas_boids"
    // ></canvas>
  }
}
