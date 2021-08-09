import React from "react";

const style = {
  header: {
    // height: "2em",
    // margin: "5%",
    // backgroundColor: "#222222",
    // borderRadius: "20px",
    // padding: "5%",
    border: "1px solid blue",
    textAlign: "right",
  },
  p: {
    fontSize: "22px",
    height: "1em",
    // verticalAlign: "middle",
    // margin: "auto",
    margin: "10px",
    // border: "1px solid green",
  },
};

class Header extends React.Component {
  constructor(props) {
    super(props);
  }
  render() {
    return (
      <div className="header" style={style.header}>
        <p style={style.p}>mader.xyz</p>
      </div>
    );
  }
}
export default Header;
