import React from "react";

import Header from "../../base/Header";

// import("../../../pkg/index.js").catch(console.error);
// import("../../../pkg/index").catch(console.error);
// import { main_js } from "../../../pkg/index.js";
// import "../../../pkg/index.js";

// import { main_js } from "../../../pkg/index.js";

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
const DT = 0.1;

class Canvas extends React.Component {
  constructor(props) {
    super(props);
    this.state = { time: 0 };
    this.canvasRef = React.createRef();
  }

  clear_canvas(ctx, width, height) {
    ctx.beginPath();
    ctx.clearRect(0, 0, width, height);
  }

  componentDidUpdate() {
    const canvas = this.canvasRef.current;
    const ctx = canvas.getContext("2d");
    const width = canvas.width;
    const height = width;
    canvas.height = height;
    ctx.save();
    // ctx.translate(width / 2, height / 2);
    // ctx.rotate((angle * Math.PI) / 180);
    const W = 250;
    const H = W;
    const L = (0.8 * W) / 2;
    const w = 1;
    const r = 15;
    this.clear_canvas(ctx, width, height);

    // console.log(this.state.time);
    // console.log(this.state.time);
    console.log(this.state.time);
    ctx.beginPath();
    ctx.fillStyle = "black";
    ctx.fillRect(0, 0, width, height);
    ctx.strokeStyle = "white";
    ctx.fillStyle = "white";
    let time = this.state.time;
    let x = W / 2 + L * Math.cos(w * time);
    let y = H / 2 + L * Math.sin(w * time);
    // pendulum mass
    ctx.arc(x, y, r, 0, 360);
    ctx.stroke();
    ctx.fill();
    // rod
    ctx.beginPath();
    ctx.moveTo(W / 2, H / 2);
    ctx.lineTo(x, y);
    ctx.stroke();

    ctx.restore();
  }

  render() {
    return <canvas className="canvas" ref={this.canvasRef} />;
  }
}

class Animation extends React.Component {
  constructor(props) {
    super(props);
    this.state = { time: 0, frame_idx: 0 };
    this.updateAnimationState = this.updateAnimationState.bind(this);
  }
  updateAnimationState() {
    this.setState({ time: this.state.time + DT });
    this.rAF = requestAnimationFrame(this.updateAnimationState);
  }
  componentDidMount() {
    this.rAF = requestAnimationFrame(this.updateAnimationState);
  }
  componentWillUnmount() {
    cancelAnimationFrame(this.rAF);
  }
  render() {
    return (
      <div class="canvas_holder">
        <Canvas time={this.state.time} />
      </div>
    );
  }
}

export default class SinglePendulum extends React.Component {
  constructor(props) {
    super(props);
    this.updateViewID = props.updateViewID;
    this.canvasRef = React.createRef();
    // this.setState(state)
    // main_js();
  }

  render() {
    return (
      <div>
        <Header updateViewID={this.updateViewID} />
        <div id="content">
          <div className="section_title">single pendulum</div>
          <div className="section">
            <Animation />
          </div>
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
