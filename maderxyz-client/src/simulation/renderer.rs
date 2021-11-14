use crate::utils;


pub struct Renderer {
    category: String,
    page_id: String,
}
impl Renderer {
    pub fn new(category: &str, page_id: &str) -> Self {
        Renderer {
            category: String::from(category), 
            page_id: String::from(page_id), 
        }
    }
    pub fn init(&mut self) {
        // TODO: setup charts, button event listeners, ...
        match self.page_id {
            _ => ()
        }
    }
    pub fn display(&mut self, state: &Vec<f64>) {

        let n = match self.page_id.as_str() {
            "charge-interaction" => 6,
            _ => 5
        };
        let nr_of_bodies: usize = state.len() / n;
        // utils::dom::console_log(&format!("{}", nr_of_bodies));

        match self.page_id.as_str() {
            // "3body-fig8" | "3body-moon" | 
            // "nbody-flowers" | "nbody-asteroids"
                // => display_nbody(&self.page_id, state),
                // => display_bodies(&self.page_id, state, nr_of_bodies),
            // "charge-interaction" 
                // => display_charge_interaction(&self.page_id, state),
                // => display_bodies(&self.page_id, state, nr_of_bodies),
            "ising" 
                => display_cellauto(&self.page_id, state),
            // "mc-pi" => display_graph(&self.page_id, state),
            _ => display_bodies(
                &self.page_id, 
                state, 
                nr_of_bodies
            ),
        }
    }
}

fn display_bodies(
    page_id: &str, 
    state: &Vec<f64>,
    nr_of_bodies: usize, 
) {

    let canvas_centered = match page_id {
        "charge-interaction" => false,
        _ => true
    };
    let mut canvas = utils::dom::Canvas::new(
        "canvas_main", // TODO: page_id
        canvas_centered, 
        false // TODO (log scale)
    ); 
    let n = match page_id {
        "charge-interaction" => 6,
        _ => 5
    };

    // drawing

    canvas.clear();
    let mut color = "#666666"; // TODO: useless

    // display bodies
    for id in 0..nr_of_bodies {

        // set color depending on charge
        if n == 6 { // TODO: might fail on non-charge-int
            let q = state[n*id+5];
            if q < 0. {
                color = "rgba(0, 0, 255, 1)";  // blue
            } else {
                color = "rgba(255, 0, 0, 1)";  // red
            }
        }

        canvas.set_fill_style(&color);
        canvas.set_stroke_style(&color);
        // draw
        let x = state[n*id+1];
        let y = state[n*id+2];
        let r = 0.015; // TODO
        canvas.draw_circle((x, y), r, true);
    }

}
fn display_particles() {

}
fn display_fields() {
    // let grid_size = 35;  // nr. of cells per row
    // for y in 0..grid_size {
    //     for x in 0..grid_size {
    //         let mut Fx = 0.;
    //         let mut Fy = 0.;

    //         let grid_size = grid_size as f64;
    //         let x = x as f64 / grid_size;
    //         let y = y as f64 / grid_size;

    //         let body = [0., 1., x, y, 0., 0.];

    //         for id in 0..nr_of_bodies {

    //             let X = new_state[n*id+1];
    //             let Y = new_state[n*id+2];
    //             let Q = new_state[n*id+5];

    //             // let other = new_state.get(n*id..n*(id+1)).unwrap();
    //             // let other = [
    //             //     new_state[n*id],
    //             //     new_state[n*id+1],
    //             //     new_state[n*id+2],
    //             //     new_state[n*id+3],
    //             //     new_state[n*id+4],
    //             //     new_state[n*id+5],
    //             // ];

    //             let dist = ((X-x).powf(2.) + (Y-y).powf(2.)).sqrt();
    //             let force = -K * Q / (dist.powf(2.) + eps.powf(2.));

    //             let force_x = force * (X-x)/dist;
    //             let force_y = force * (Y-y)/dist;

    //             // let force = -K * Q / dist.powf(2.);
    //             // let force = utils::physics::force_coulomb(
    //             //     &body, 
    //             //     &other
    //             // );
    //             // Fx += force.0;
    //             // Fy += force.1;
    //             Fx += force_x;
    //             Fy += force_y;
    //         }
    //         new_state.extend_from_slice(&[Fx, Fy]);
    //     }
    // }
}


fn display_charge_interaction(page_id: &str, state: &Vec<f64>) {
    let mut canvas = utils::dom::Canvas::new(
        "canvas_main", 
        false,
        false // TODO
    ); // TODO: page_id

    let nr_of_bodies = 7; // TODO
    let n = 6; // TODO (?)

    canvas.clear();
    let mut color = "#666666"; // TODO: useless

    // display field
    let grid_size = 35; // TODO
    let F_max = 100.; // TODO: make changeable via slider
    // let mut F_max = 1.;
    // for y in 0..grid_size {
    //     for x in 0..grid_size {
    //         let Fx = state[start_idx+2*y*grid_size+2*x];
    //         let Fy = state[start_idx+2*y*grid_size+2*x+1];
    //         let F = (Fx.powf(2.) + Fy.powf(2.)).sqrt();
    //         if F > F_max { F_max = F }
    //     }
    // }

    // let start_idx = nr_of_bodies * n;
    // for y in 0..grid_size {
    //     for x in 0..grid_size {
    //         // get field strength (TODO: calc here?)
    //         let mut Fx = state[start_idx+2*y*grid_size+2*x];
    //         let mut Fy = state[start_idx+2*y*grid_size+2*x+1];
    //         let F = (Fx.powf(2.) + Fy.powf(2.)).sqrt();
    //         // set color from field strength (relative to max)
    //         // let r = (F / F_max * 255.) as u8;
    //         // let (b, g) = (r, r);
    //         // let color = format!("rgb({}, {}, {})", r, g, b);
    //         // let color = format!("rgba(255, 255, 255, {})", F/F_max);
    //         let alpha = (F/F_max*255.) as u8;
    //         let color = format!("rgb({}, {}, {})", alpha, alpha, alpha);
    //         canvas.set_stroke_style(&color);
    //         // draw field line
    //         let grid_size = grid_size as f64;
    //         let x = x as f64 / grid_size;
    //         let y = y as f64 / grid_size;
    //         Fx *= 0.6 / grid_size / F; // normalize
    //         Fy *= 0.6 / grid_size / F; // normalize
    //         canvas.draw_line((x, y), (x+Fx, y+Fy));
    //         // draw "arrow" tip
    //         let r = 0.004;
    //         canvas.set_fill_style(&color);
    //         canvas.draw_circle((x+Fx, y+Fy), r, true);
    //     }
    // }

    // display bodies
    let r = 0.015; // TODO
    for id in 0..nr_of_bodies {
        // set color depending on charge
        let q = state[n*id+5];
        if q < 0. {
            color = "rgba(0, 0, 255, 1)";  // blue
        } else {
            color = "rgba(255, 0, 0, 1)";  // red
        }
        canvas.set_fill_style(&color);
        canvas.set_stroke_style(&color);
        // draw
        let x = state[n*id+1];
        let y = state[n*id+2];
        canvas.draw_circle((x, y), r, true);
    }
}


fn display_nbody(page_id: &str, state: &Vec<f64>) {
    let mut canvas = utils::dom::Canvas::new(
        "canvas_main", 
        true,
        true // TODO
    ); // TODO: page_id

    // drawing setup
    canvas.set_fill_style("white");
    canvas.set_stroke_style("white");
    let mut colors: Vec<&str> = Vec::from([]);
    match page_id {
        "3body-moon" => {
            colors = Vec::from([ "white", "blue", "red" ]);
        }, "3body-fig8" => {
            colors = Vec::from([ "red", "blue", "white" ]);
        },
        _ => {}
    }

    // clear canvas
    canvas.clear();

    // rotation
    let delta_phi = match page_id {
        // "3body-fig8" => 0.235 * 2.*3.14159, 
        _ => 0.
    };
    // radius
    let radius = match page_id {
        "3body-fig8" => 0.1,
        _ => 0.005
    };

    let n = 5;
    let N = state.len() / n;
    for id in 0..N {
        // get point mass info
        let m = state[n*id];
        let mut x = state[n*id+1];
        let mut y = state[n*id+2];
            // let u = state[n*id + 3];
            // let v = state[n*id + 4];
        // apply rotation
        // if delta_phi != 0. {
        //     let pol_r = (x.powf(2.) + y.powf(2.)).sqrt();
        //     let phi = y.atan2(x) + delta_phi;
        //     x = pol_r*phi.cos();
        //     y = pol_r*phi.sin();
        // }
        // apply colors
        if colors.len() > 0 {
            canvas.set_fill_style(colors[id]);
            canvas.set_stroke_style(colors[id]);
        }
        // configure drawing radius
        let r = radius;
        // draw
        canvas.draw_circle((x, y), r, true);
    }
}

fn display_cellauto(page_id: &str, state: &Vec<f64>) {
    let mut canvas = utils::dom::Canvas::new(
        "canvas_main", 
        false,
        false
    ); // TODO: page_id
    // clear canvas
    canvas.clear();

    // draw on canvas
    // match page_id {
    //     "ising" => ising_draw(state),
    //     _ => ()
    // }

    let N = (state.len() as f64).sqrt() as usize; // TODO: only works for square

    canvas.set_fill_style("white");
    for i in 0..N {
        for j in 0..N {
            let N = N as f64;
            let x = i as f64 / N;
            let y = j as f64 / N;
            let z = 0.8; // TODO move elsewhere
            let center = (
                x + (1.-z) / (2.*N), 
                y + (1.-z) / (2.*N)
            );
            let width = 1. / N * z;
            let height = 1. / N * z;
            if state[i*N as usize+j] == 1. {
                canvas.fill_rect(center, width, height);
                // console_log(&format!("{}, {}", i, j))
            }
        }
    }
}


// pub fn ising_draw(state: &Vec<f64>) {}

// fn display_graph(page_id: u32, state: &Vec<f64>) {
//     let mut canvas = Canvas::new("canvas_main", false); // TODO: page_id
//     canvas.fill_rect((10., 10.), 20.,30.);
// }

