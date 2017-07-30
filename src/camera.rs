#[derive(Debug)]
pub struct Camera {
    // Position in the world
    x: f64,
    y: f64,

    // Boundaries of the viewport on the world
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,

    // Amounts that the camera should move
    horiz_scroll: f64,
    vert_scroll: f64,

    // Bounds related to an environment - can also be the size of the world
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,

    // Viewport Information
    viewport_width: u32,
    viewport_height: u32,
}

impl Camera {
    pub fn new(viewport_height: u32, viewport_width: u32, max_x: f64, max_y: f64) -> Camera {
        let cam = Camera {
            x: viewport_width as f64 / 2.0,
            y: viewport_height as f64 / 2.0,

            left: 0.0,
            right: viewport_width as f64,
            top: 0.0,
            bottom: viewport_height as f64,

            horiz_scroll: 10.0,
            vert_scroll: 10.0,

            min_x: 0.0,
            max_x,
            min_y: 0.0,
            max_y,

            viewport_width,
            viewport_height
        };

        println!("Camera instantiated: {:#?}", cam);
        cam
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn get_left(&self) -> f64 {
        self.left
    }

    pub fn get_right(&self) -> f64 {
        self.right
    }

    pub fn get_top(&self) -> f64 {
        self.top
    }

    pub fn get_bottom(&self) -> f64 {
        self.bottom
    }

    fn position_updated(&mut self) {
        self.left = self.x - self.viewport_width as f64 / 2.0;
        self.right = self.x + self.viewport_width as f64 / 2.0;
        self.top = self.y - self.viewport_height as f64 / 2.0;
        self.bottom = self.y + self.viewport_height as f64 / 2.0;
    }

    pub fn move_left(&mut self) {
        if (self.x - self.viewport_width as f64 / 2.0 - self.horiz_scroll) >= 0.0 {
            self.x -= self.horiz_scroll;
        } else {
            self.x = self.viewport_width as f64 / 2.0;
        }

        self.position_updated();

        println!("Camera::move_left() called - {:#?}", self);
    }

    pub fn move_right(&mut self) {
        if (self.x + self.viewport_width as f64 / 2.0 + self.horiz_scroll) <= (self.max_x) {
            self.x += self.horiz_scroll;
        } else {
            self.x = self.max_x - self.viewport_width as f64 / 2.0;
        }

        self.position_updated();

        println!("Camera::move_right() called - {:#?}", self);
    }

    pub fn move_up(&mut self) {
        if (self.y - self.viewport_height as f64 / 2.0 - self.vert_scroll) >= 0.0 {
            self.y -= self.vert_scroll;
        } else {
            self.y = self.viewport_height as f64 / 2.0;
        }

        self.position_updated();

        println!("Camera::move_up() called - {:#?}", self);
    }

    pub fn move_down(&mut self) {
        if (self.y + self.viewport_height as f64 / 2.0 + self.vert_scroll) <= self.max_y {
            self.y += self.vert_scroll;
        } else {
            self.y = self.max_y - self.viewport_height as f64 / 2.0;
        }

        self.position_updated();

        println!("Camera::move_down() called - {:#?}", self);
    }

    // fn update_simulated_dimensions(&mut self) {
    //     // Update the dimesions that this camera should see the world at given the current Z position
    //     let z_pct: f64 = (self.z - self.min_z) / (self.max_z - self.min_z);
    //     // self.ground_width = z_pct * self.viewport_width as f64;
    //     // self.ground_height = z_pct * self.viewport_height as f64;
    // }
    //
    // pub fn zoom_in(&mut self) {
    //     // Move the camera in along the Z axix, towards the world
    //     if (self.z - self.depth_scroll) >= self.min_z {
    //         self.z -= self.depth_scroll;
    //     } else {
    //         self.z = self.min_z;
    //     }
    //
    //     self.update_simulated_dimensions();
    //
    //     println!("Camera::zoom_in() called - {:#?}", self);
    // }
    //
    // pub fn zoom_out(&mut self) {
    //     // Move the camera out along the Z axis, away from the world
    //     if (self.z + self.depth_scroll) <= self.max_z {
    //         self.z += self.depth_scroll;
    //     } else {
    //         self.z = self.max_z;
    //     }
    //
    //     self.update_simulated_dimensions();
    //
    //     println!("Camera::zoom_out() called - {:#?}", self);
    // }
}
