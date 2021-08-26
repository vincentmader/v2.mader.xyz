import React from "react";
import { Navbar, Container, Nav, NavDropdown } from "react-bootstrap";


class Header extends React.Component {
  constructor(props) {
    super(props);
    this.updateViewID = props.updateViewID;
  }

  render() {
    return (
      <div id="header">
        <button
          id="header_home"
          onClick={() => this.updateViewID("index")}
        >
          mader.xyz
        </button>
      </div>
    );

    //   <Navbar
    //     id="navbar"
    //     collapseOnSelect
    //     expand="lg"
    //     bg="#000000"
    //     variant="light"
    //   >
    //     <Container>
    //       <Navbar.Brand href="#home">mader.xyz</Navbar.Brand>
    //     </Container>
    //   </Navbar>

    // <Navbar.Toggle aria-controls="responsive-navbar-nav" />
    // <Navbar.Collapse id="responsive-navbar-nav">
    //   <Nav className="me-auto">
    //     <Nav.Link href="#features">Simulations</Nav.Link>
    //     <Nav.Link href="#pricing">Insights</Nav.Link>
    //     <Nav.Link href="#pricing">About/Contact</Nav.Link>
    //   </Nav>
    // </Navbar.Collapse>
  }
}
export default Header;
