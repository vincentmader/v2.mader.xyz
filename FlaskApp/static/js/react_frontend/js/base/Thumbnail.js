import React from "react";

// import styles from "../../css/base.css";

class Thumbnail extends React.Component {
  constructor(props) {
    super(props);
    this.id = props.id;
    this.source = "static/media/thumbnails/" + this.id + ".png";
    this.updateViewID = props.updateViewID;
    // "static/js/react_frontend/public/thumbnails/" + this.id + ".png";
  }

  render() {
    // return <div style={styles.thumbnail}></div>;

      // TODO: return link?
    return (
      <button
        onClick={() => {
          this.updateViewID(this.id);
        }}
        className="navgrid_cell"
      >
        <img className="navgrid_thumbnail" src={this.source} alt={this.id} />
      </button>
    );
  }
}
export default Thumbnail;
