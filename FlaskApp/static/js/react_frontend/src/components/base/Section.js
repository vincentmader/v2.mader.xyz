import React from "react";

import Thumbnail from "../index/Thumbnail";

const style = {
  section: {
    backgroundColor: "#222222",
    borderRadius: "20px",

    marginLeft: "20px",
    marginRight: "20px",
    marginTop: "20px",
    marginBottom: "20px",

    // paddingLeft: "10px",
    // paddingRight: "10px",
    // paddingTop: "10px", // TODO: Kati fragen
    // paddingBottom: "10px",

    display: "grid",
    gridTemplateColumns: "repeat(3, 1fr)",
    // textAlignVertical: "center",
    // textAlign: "center",
  },
  p: {
    border: "1px solid green",
  },
};

class Section extends React.Component {
  constructor(props) {
    super(props);
    this.title = props.title;
    this.subsections = props.subsections;
    // console.log(props);
  }
  render() {
    // <div key={idx}>
    //   <p style={style.p}>{obj.title}</p>
    // </div>
    return (
      <div style={style.section}>
        {this.subsections.map((obj, idx) => {
          return <Thumbnail key={idx} />;
        })}
      </div>
    );
  }
}
export default Section;
