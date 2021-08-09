import React from "react";

const style = {
  thumbnail: {
    // height: "900%",
    height: "100px",
    // width: "100px",
    flex: 1,
    backgroundColor: "black",
    borderRadius: "30px",
    // margin: "10px",
    // padding: "0px",
    // height: 2 * window.height,
    border: "1px solid green",
    // width: "100%",
    // height: "100%",
  }, // TODO: classes !
};

class Thumbnail extends React.Component {
  constructor(props) {
    super(props);
  }
  render() {
    return <div style={style.thumbnail}></div>;
  }
}
export default Thumbnail;
