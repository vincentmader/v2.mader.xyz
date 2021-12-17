
pub use mxyz_numerics::state::State;
pub use mxyz_numerics::state::field::Field;
pub use mxyz_numerics::state::object::ObjectFamily;

pub use utils::dom::canvas::Canvas;


pub struct Renderer {

    page_id: String,
    frame_idx: usize,
    canvases: Vec<Canvas>,

}
impl Renderer {

    pub fn new(page_id: &str) -> Self {

        let page_id = String::from(page_id);
        let frame_idx = 0;
        let canvases = Vec::new();

        Renderer {
            page_id,
            frame_idx,
            canvases,
        }

    }

    pub fn init(&mut self) {

        // TODO generalize canvas initialization
        let canvas = Canvas::new("canvas_main");
        self.canvases.push(canvas);

        // TODO initialize user inputs
        self.init_buttons();
        self.init_sliders();

    }

    pub fn reset(&mut self) {
        self.frame_idx = 0;
    }

    pub fn display(&mut self, states: &Vec<State>) {

        // get main canvas
        let canvas_id = 0;

        let canvas = &mut self.canvases[canvas_id];
        canvas.clear();

        let current_state = &states[self.frame_idx];

        for object_family in current_state.object_families.iter() {
                // utils::dom::console::log(&format!("{}", self.frame_idx));
            self.display_objects(object_family, canvas_id);
        }

        for field in current_state.fields.iter() {
            self.display_field(field, canvas_id);
        }

        self.frame_idx += 1;
        
    }

    pub fn display_objects(
        &mut self, 
        object_family: &ObjectFamily,
        canvas_id: usize,
    ) {

        let canvas = &mut self.canvases[canvas_id];

        let objects = &object_family.objects;
        let object_length = object_family.object_length;
        let nr_of_objects = object_family.nr_of_objects;

        for obj_idx in 0..nr_of_objects {

            let m = objects[obj_idx*object_length];
            let x = objects[obj_idx*object_length+1];
            let y = objects[obj_idx*object_length+2];
            let u = objects[obj_idx*object_length+3];
            let v = objects[obj_idx*object_length+4];

            canvas.set_stroke_style("purple");
            canvas.set_fill_style("purple");
            canvas.draw_circle(
                (x, y),
                0.01,
                true,
                // center: (f64, f64), radius: f64, fill: bool
            )

        }

    }

    pub fn display_field(
        &mut self, 
        field: &Field, 
        canvas_id: usize
    ) {

    }

    pub fn init_buttons(&mut self) {

    }

    pub fn init_sliders(&mut self) {

    }

}

