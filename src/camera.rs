use winit::keyboard::KeyCode;

pub struct Camera {
    pub location: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect_ratio: f32,
    pub fov: f32,
    pub z_near: f32,
    pub z_far: f32,
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::from_cols(
    cgmath::Vector4::new(1.0, 0.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 1.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 1.0),
);

impl Camera {
    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let move_matrix = cgmath::Matrix4::look_at_rh(self.location, self.target, self.up);
        let project_matrix = cgmath::perspective(
            cgmath::Deg(self.fov),
            self.aspect_ratio,
            self.z_near,
            self.z_far,
        );
        OPENGL_TO_WGPU_MATRIX * project_matrix * move_matrix
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}

pub struct CameraController {
    pub speed: f32,
    pub is_forward_pressed: bool,
    pub is_backward_pressed: bool,
    pub is_left_pressed: bool,
    pub is_right_pressed: bool,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }

    pub fn handle_key(&mut self, code: KeyCode, is_pressed: bool) -> bool {
        match code {
            KeyCode::KeyW | KeyCode::ArrowUp => {
                self.is_forward_pressed = is_pressed;
                true
            }
            KeyCode::KeyA | KeyCode::ArrowLeft => {
                self.is_left_pressed = is_pressed;
                true
            }
            KeyCode::KeyS | KeyCode::ArrowDown => {
                self.is_backward_pressed = is_pressed;
                true
            }
            KeyCode::KeyD | KeyCode::ArrowRight => {
                self.is_right_pressed = is_pressed;
                true
            }
            _ => false,
        }
    }

    pub fn update_camera(&self, camera: &mut Camera) {
        use cgmath::InnerSpace;
        let forward = camera.target - camera.location;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        // Prevents glitching when the camera gets too close to the center of the scene.
        if self.is_forward_pressed && forward_mag > self.speed {
            camera.location += forward_norm * self.speed;
        }
        if self.is_backward_pressed {
            camera.location -= forward_norm * self.speed;
        }

        let right = forward_norm.cross(camera.up);

        // Redo radius calc in case the forward/backward is pressed.
        let forward = camera.target - camera.location;
        let forward_mag = forward.magnitude();

        if self.is_right_pressed {
            // Rescale the distance between the target and the location so
            // that it doesn't change. The location, therefore, still
            // lies on the circle made by the target and location.
            camera.location =
                camera.target - (forward + right * self.speed).normalize() * forward_mag;
        }
        if self.is_left_pressed {
            camera.location =
                camera.target - (forward - right * self.speed).normalize() * forward_mag;
        }
    }
}
