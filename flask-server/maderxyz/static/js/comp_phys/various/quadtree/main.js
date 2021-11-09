// QuadTree
// Vincent C. Mader

// VARIABLE DEFINITIONS
// ============================================================================
var canvas, ctx, W, H;
var quadtree;
var range; // for testing

const TAU = 2 * Math.PI;

// CLASSES
// ============================================================================

class Point {
  constructor(x, y) {
    this.x = x;
    this.y = y;
  }
}

class Rectangle {
  constructor(x, y, w, h) {
    this.x = x;
    this.y = y;
    this.w = w;
    this.h = h;
  }
  contains(point) {
    return (
      point.x >= this.x - this.w &&
      point.x < this.x + this.w &&
      point.y >= this.y - this.h &&
      point.y < this.y + this.h
    );
  }
  intersects(range) {
    return !(
      range.x - range.w > this.x + this.w ||
      range.x + range.w < this.x - this.w ||
      range.y - range.h > this.y + this.h ||
      range.y + range.h < this.y - this.h
    );
  }
}

class QuadTree {
  constructor(boundary, n) {
    this.boundary = boundary;
    this.capacity = n;
    this.points = [];
    this.divided = false;
  }
  subdivide() {
    let x = this.boundary.x;
    let y = this.boundary.y;
    let w = this.boundary.w;
    let h = this.boundary.h;
    let ne = new Rectangle(x + w / 2, y - h / 2, w / 2, h / 2);
    let nw = new Rectangle(x - w / 2, y - h / 2, w / 2, h / 2);
    let se = new Rectangle(x + w / 2, y + h / 2, w / 2, h / 2);
    let sw = new Rectangle(x - w / 2, y + h / 2, w / 2, h / 2);
    this.northeast = new QuadTree(ne, this.capacity);
    this.northwest = new QuadTree(nw, this.capacity);
    this.southeast = new QuadTree(se, this.capacity);
    this.southwest = new QuadTree(sw, this.capacity);
    this.divided = true;
    for (let p of this.points) {
      this.northeast.insert(p);
      this.northwest.insert(p);
      this.southeast.insert(p);
      this.southwest.insert(p);
    }
  }
  insert(point) {
    if (!this.boundary.contains(point)) return;
    if (this.points.length < this.capacity) {
      this.points.push(point);
    } else {
      if (!this.divided) {
        this.subdivide();
      }
      this.northeast.insert(point);
      this.northwest.insert(point);
      this.southeast.insert(point);
      this.southwest.insert(point);
    }
  }
  show() {
    ctx.lineWidth = 1;
    ctx.strokeStyle = "gray";
    ctx.beginPath();
    ctx.rect(
      // TODO: why no left/bottom border?
      this.boundary.x - this.boundary.w,
      this.boundary.y - this.boundary.h,
      2 * this.boundary.w,
      2 * this.boundary.h
    );
    ctx.stroke();

    if (this.divided) {
      this.northeast.show();
      this.northwest.show();
      this.southeast.show();
      this.southwest.show();
    }
    for (let p of this.points) {
      ctx.strokeStyle = "red";
      ctx.fillStyle = "red";
      ctx.beginPath();
      ctx.arc(p.x, p.y, 2, 0, TAU);
      ctx.stroke();
      ctx.fill();
    }
  }
  query(range, found) {
    if (!this.boundary.intersects(range)) {
      return;
    } else {
      for (let p of this.points) {
        if (range.contains(p) && !found.includes(p)) {
          found.push(p);
        }
      }
      if (this.divided) {
        this.northwest.query(range, found);
        this.northeast.query(range, found);
        this.southwest.query(range, found);
        this.southeast.query(range, found);
      }
    }
  }
}

// UTILITY FUNCTIONS
// ============================================================================

function getCursorPosition(canvas, event) {
  const rect = canvas.getBoundingClientRect();
  const x = event.clientX - rect.left;
  const y = event.clientY - rect.top;
  return [x, y];
}

// INITIALIZATION
// ============================================================================

const init = () => {
  // setup canvas
  canvas = document.getElementById("canvas");
  W = canvas.getBoundingClientRect().width;
  H = W;
  canvas.width = W;
  canvas.height = W;
  ctx = canvas.getContext("2d");

  let n = 4;
  let boundary = new Rectangle(W / 2, H / 2, W / 2, H / 2);
  quadtree = new QuadTree(boundary, n);

  // add points to tree on click
  document.getElementById("canvas").addEventListener("click", (e) => {
    for (let _ = 0; _ < 1; _++) {
      const pos = getCursorPosition(canvas, e);
      let p = new Point(
        pos[0], //+ ((2 * Math.random() - 1) * W) / 8,
        pos[1] //+ ((2 * Math.random() - 1) * H) / 8
      );
      quadtree.insert(p);
    }
    draw();
  });

  // add points on init
  for (let _ = 0; _ < 0; _++) {
    let p = new Point(Math.random() * W, Math.random() * H);
    quadtree.insert(p);
  }
  // testing
  let x = (0.3 * Math.random() + 0.2) * W;
  let y = (0.3 * Math.random() + 0.2) * H;
  let w = (0.1 * Math.random() + 0.05) * W;
  let h = (0.1 * Math.random() + 0.05) * H;
  range = new Rectangle(x, y, w, h);
};

function draw() {
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  quadtree.show();

  // test
  ctx.strokeStyle = "green";
  ctx.lineWidth = 2;
  ctx.beginPath();
  ctx.rect(range.x - range.w, range.y - range.h, 2 * range.w, 2 * range.h);
  ctx.stroke();
  var points = [];
  quadtree.query(range, points);
  for (let p of points) {
    ctx.strokeStyle = "green";
    ctx.fillStyle = "green";
    ctx.beginPath();
    ctx.arc(p.x, p.y, 2, 0, TAU);
    ctx.stroke();
    ctx.fill();
  }
}

init();
draw();
