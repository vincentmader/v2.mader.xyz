// hierarchical tree for spatial division into nested squares
export class QuadTree {
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
    // create rectangle objects representing children
    let ne = new Rectangle(x + w / 2, y - h / 2, w / 2, h / 2);
    let nw = new Rectangle(x - w / 2, y - h / 2, w / 2, h / 2);
    let se = new Rectangle(x + w / 2, y + h / 2, w / 2, h / 2);
    let sw = new Rectangle(x - w / 2, y + h / 2, w / 2, h / 2);
    // create quad tree objects
    this.northeast = new QuadTree(ne, this.capacity);
    this.northwest = new QuadTree(nw, this.capacity);
    this.southeast = new QuadTree(se, this.capacity);
    this.southwest = new QuadTree(sw, this.capacity);
    this.divided = true;
    // insert points into children
    for (let p of this.points) {
      this.northeast.insert(p);
      this.northwest.insert(p);
      this.southeast.insert(p);
      this.southwest.insert(p);
    }
  }
  // insert point into tree
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
  // draw tree grid
  show() {
    ctx.lineWidth = 1;
    ctx.strokeStyle = "gray";
    ctx.beginPath();
    let ctx_coords = get_ctx_coords([
      this.boundary.x - this.boundary.w,
      this.boundary.y - this.boundary.h,
    ]);
    let ctx_w = get_ctx_radius(2 * this.boundary.w);
    let ctx_h = get_ctx_radius(2 * this.boundary.h);
    ctx.rect(ctx_coords[0], ctx_coords[1], ctx_w, ctx_h);
    ctx.stroke();
    if (this.divided) {
      this.northeast.show();
      this.northwest.show();
      this.southeast.show();
      this.southwest.show();
    }
    // draw points in tree
    // for (let p of this.points) {
    //   ctx.strokeStyle = "white";
    //   ctx.fillStyle = "white";
    //   ctx.beginPath();
    //   ctx.arc(p.x, p.y, 1, 0, TAU);
    //   ctx.stroke();
    //   ctx.fill();
    // }
  }
  // search for points in range, save to input arg list
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
