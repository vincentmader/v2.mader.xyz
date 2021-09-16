import React from "react";


class Section extends React.Component {
  constructor(props) {
    super(props);
    this.title = props.title;
  }

  render() {
    // <div key={idx}>
    //   <p style={style.p}>{obj.title}</p>
    // </div>
    // <div style={styles.section}>
    return (
      <div>
        <p>{this.props.title}</p>
        <div className="section"></div>
      </div>
    );
  }
}
export default Section;
