import React from "react";

import Thumbnail from "./Thumbnail";


class NavGrid extends React.Component {
  constructor(props) {
    super(props);
    this.subsections = props.subsections;
    this.updateViewID = props.updateViewID;
  }
  render() {
    return (
      <div id="navgrid">
        {this.subsections.map((subsection, idx) => {
          let id = subsection.id;
          return (
            <Thumbnail updateViewID={this.updateViewID} id={id} key={idx} />
          );
        })}
      </div>
    );
  }
}
export default NavGrid;
