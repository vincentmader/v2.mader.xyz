export function get_L2_norm(vec) {
  var sum = 0;
  for (const i of vec) {
    sum += i ** 2;
  }
  return Math.sqrt(sum);
}

export class Point {
  constructor(x, y) {
    this.x = x;
    this.y = y;
  }
}

export class Rectangle {
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

export class Vector2D {
  constructor(x, y, type = "cart") {
    if (type == "cart") {
      this.x = x;
      this.y = y;
    } else if (type == "pol") {
      let r = x;
      let phi = y;
      this.x = r * Math.cos(phi);
      this.y = r * Math.sin(phi);
    }
  }
  norm_l2() {
    return Math.sqrt(this.x ** 2 + this.y ** 2);
  }
  add(vec) {
    return new Vector2D(this.x + vec.x, this.y + vec.y);
  }
  sub(vec) {
    return new Vector2D(this.x - vec.x, this.y - vec.y);
  }
  mult(lambda) {
    return new Vector2D(lambda * this.x, lambda * this.y);
  }
  dot(vec) {
    return this.x * vec.x + this.y * vec.y;
  }
  angle() {
    return Math.atan2(this.y, this.x);
  }
  angle_diff(vec) {
    return Math.acos(this.dot(vec) / this.norm_l2(this), this.norm_l2(vec));
  }
  rotate(angle) {
    // rotate anti-clockwise
    let x = this.x * Math.cos(angle) - this.y * Math.sin(angle);
    let y = this.x * Math.sin(angle) + this.y * Math.cos(angle);
    return new Vector2D(x, y);
  }
  polar_coords() {
    let r = Math.sqrt(x ** 2 + y ** 2);
    let phi = Math.atan2(this.y, this.x);
  }
}
