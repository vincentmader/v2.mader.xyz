import React from "react";

import Section from "./Section";
// import Calendar from "../chronos/calendar/Calendar";

import "../css/base.css";

class Content extends React.Component {
  constructor(props) {
    super(props);
    this.sections = props.sections;
    this.subsections = props.subsections;
  }
  render() {
    // <Calendar user="Vincent" month="2021-08" MonthStr="2021-08" />
    // <div style={styles.content}>
    return (
      <div className="content">
        {this.sections.map((title, idx) => {
          return (
            <div key={idx}>
              <p>{title}</p>
              <Section subsections={this.subsections[title]} />
            </div>
          );
        })}
      </div>
    );
  }
}
export default Content;
