import React from "react";

import Header from "../base/Header";
import Content from "../base/Content";

const style = {
  index: {
    backgroundColor: "black",
    color: "white",
    // margin: "0px",
    // padding: "0px",
    // height: 2 * window.height,
    // border: "1px solid red",
  },
};

class Index extends React.Component {
  constructor(props) {
    super(props);
  }
  render() {
    return (
      <div style={style.index}>
        <Header />
        <Content />
      </div>
    );
  }
}
export default Index;
