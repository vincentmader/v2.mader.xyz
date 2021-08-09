import React from "react";

import Section from "./Section";

const sections = [
  "gravitational n-body dynamics",
  "harmonical oscillations",
  "statistical physics & thermodynamics",
  "emergent behavior & cellular automata",
  "electro-magnetism",
  "fluid dynamics",
];
const subsections = {
  "gravitational n-body dynamics": [
    { title: "ayooo" },
    { title: "ay2oo" },
    { title: "ayo-0003oo" },
    { title: "ayo-0020oo" },
    { title: "ayo-00009o" },
    { title: "ayo-10200o" },
    { title: "ayo-0008oo" },
  ],
  "harmonical oscillations": [{ title: "ayooo" }],
  "statistical physics & thermodynamics": [{ title: "ayooo" }],
  "emergent behavior & cellular automata": [{ title: "ayooo" }],
  "electro-magnetism": [{ title: "ayooo" }],
  "fluid dynamics": [{ title: "ayooo" }],
};

const style = {
  content: {
    // margin: "0px",
    // padding: "0px",
    // backgroundColor: "#222222",
    // borderRadius: "20px",
    border: "1px solid blue",
    fontFamily: "Arial, Helvetica, sans-serif",
  },
  p: {
    color: "white",
    // border: "1px solid red",
    textAlign: "center",
  },
};

class Content extends React.Component {
  constructor(props) {
    super(props);
  }
  render() {
    return (
      <div className="content" style={style.content}>
        {sections.map((title, idx) => {
          return (
            <div key={idx}>
              <p style={style.p}>{title}</p>
              <Section subsections={subsections[title]} />
            </div>
          );
        })}
      </div>
    );
  }
}
export default Content;
