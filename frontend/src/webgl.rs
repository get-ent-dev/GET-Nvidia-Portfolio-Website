// frontend/src/webgl.rs

use web_sys::{
    WebGlRenderingContext as GL, // Alias for convenience
    HtmlCanvasElement,
    WebGlBuffer,
    WebGlProgram,
    WebGlShader,
    WebGlUniformLocation,
};
use gloo_utils::window; // Used for performance.now()
use glam::{Mat4, Vec3}; // Kept Mat4, Vec3 for typical WebGL use
use anyhow::{Result, anyhow};
use log::info;
use wasm_bindgen::JsCast; // Added: For dyn_into
use js_sys; // Added: For Float32Array, Uint16Array

// --- Vertex Shader Source ---
const VERTEX_SHADER_SOURCE: &str = r#"
    attribute vec4 a_position;
    uniform mat4 u_matrix;

    void main() {
        gl_Position = u_matrix * a_position;
    }
"#;

// --- Fragment Shader Source ---
const FRAGMENT_SHADER_SOURCE: &str = r#"
    precision mediump float;
    uniform vec4 u_color;

    void main() {
        gl_FragColor = u_color;
    }
"#;

pub struct WebGlRenderer {
    gl: GL,
    pub vertex_buffer: Option<WebGlBuffer>,
    pub index_buffer: Option<WebGlBuffer>,
    pub program: Option<WebGlProgram>,
    pub color_uniform_location: Option<WebGlUniformLocation>,
    pub matrix_uniform_location: Option<WebGlUniformLocation>,
    pub position_attribute_location: i32, // Attribute location for position
}

impl WebGlRenderer {
    pub fn new(canvas: HtmlCanvasElement) -> Result<Self> {
        info!("Attempting to get WebGL context.");

        let gl: GL = canvas
            .get_context("webgl") // Returns Result<Option<Object>>
            .map_err(|e| anyhow!("Failed to get WebGL context: {:?}", e))? // Convert JsValue to anyhow::Error
            .ok_or_else(|| anyhow!("Failed to get WebGL context from canvas (returned None)."))? // Handle None case
            .dyn_into::<GL>() // Cast to WebGlRenderingContext, now JsCast is in scope
            .map_err(|e| anyhow!("Failed to cast context to WebGlRenderingContext: {:?}", e))?; // Error handling for cast

        info!("WebGL context obtained.");

        // --- Compile Shaders ---
        let vertex_shader = WebGlRenderer::compile_shader(&gl, GL::VERTEX_SHADER, VERTEX_SHADER_SOURCE)?;
        let fragment_shader = WebGlRenderer::compile_shader(&gl, GL::FRAGMENT_SHADER, FRAGMENT_SHADER_SOURCE)?;

        // --- Link Program ---
        let program = WebGlRenderer::link_program(&gl, &vertex_shader, &fragment_shader)?;

        // --- Get Attribute and Uniform Locations ---
        let position_attribute_location = gl.get_attrib_location(&program, "a_position");
        if position_attribute_location == -1 {
            return Err(anyhow!("Failed to get attribute location for 'a_position'"));
        }
        let color_uniform_location = gl.get_uniform_location(&program, "u_color");
        let matrix_uniform_location = gl.get_uniform_location(&program, "u_matrix");

        // --- Create Buffers (Placeholders for now) ---
        let vertex_buffer = gl.create_buffer(); // Option<WebGlBuffer>
        gl.bind_buffer(GL::ARRAY_BUFFER, vertex_buffer.as_ref()); // Bind the buffer

        // Example data for vertex buffer (a simple triangle)
        #[rustfmt::skip]
        let vertices: [f32; 9] = [
            0.0, 0.5, 0.0,  // Top
            -0.5, -0.5, 0.0, // Bottom-left
            0.5, -0.5, 0.0,  // Bottom-right
        ];
        // SAFETY: `vertices.as_ptr()` points to valid memory.
        // `std::mem::size_of_val(&vertices)` is the correct size.
        // `GL::STATIC_DRAW` is a valid enum value.
        unsafe {
            let vert_array = js_sys::Float32Array::view(&vertices); // Use js_sys here
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);
        }
        info!("Vertex buffer created.");

        let index_buffer = gl.create_buffer(); // Option<WebGlBuffer>
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, index_buffer.as_ref()); // Bind the index buffer

        // Example data for index buffer
        #[rustfmt::skip]
        let indices: [u16; 3] = [
            0, 1, 2, // Triangle
        ];
        // SAFETY: `indices.as_ptr()` points to valid memory.
        // `std::mem::size_of_val(&indices)` is the correct size.
        // `GL::STATIC_DRAW` is a valid enum value.
        unsafe {
            let index_array = js_sys::Uint16Array::view(&indices); // Use js_sys here
            gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &index_array, GL::STATIC_DRAW);
        }
        info!("Index buffer created.");

        Ok(Self {
            gl,
            vertex_buffer,
            index_buffer,
            program: Some(program), // Program is valid
            color_uniform_location,
            matrix_uniform_location,
            position_attribute_location,
        })
    }

    fn compile_shader(gl: &GL, shader_type: u32, source: &str) -> Result<WebGlShader> {
        let shader = gl.create_shader(shader_type)
            .ok_or_else(|| anyhow!("Failed to create shader"))?;
        gl.shader_source(&shader, source);
        gl.compile_shader(&shader);

        if !gl.get_shader_parameter(&shader, GL::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false) {
            let log = gl.get_shader_info_log(&shader)
                .unwrap_or_else(|| "Unknown shader compile error".to_string());
            gl.delete_shader(Some(&shader)); // Clean up shader if compilation failed
            return Err(anyhow!("Shader compilation failed: {}", log));
        }
        Ok(shader)
    }

    fn link_program(gl: &GL, vertex_shader: &WebGlShader, fragment_shader: &WebGlShader) -> Result<WebGlProgram> {
        let program = gl.create_program()
            .ok_or_else(|| anyhow!("Failed to create program"))?;

        gl.attach_shader(&program, vertex_shader);
        gl.attach_shader(&program, fragment_shader);
        gl.link_program(&program);

        if !gl.get_program_parameter(&program, GL::LINK_STATUS)
            .as_bool()
            .unwrap_or(false) {
            let log = gl.get_program_info_log(&program)
                .unwrap_or_else(|| "Unknown program link error".to_string());
            gl.delete_program(Some(&program)); // Clean up program if linking failed
            return Err(anyhow!("Program linking failed: {}", log));
        }
        Ok(program)
    }

    // This method will be called in the animation loop
    pub fn render_frame(&self) {
        let gl = &self.gl;
        let program = self.program.as_ref().expect("WebGL program not set.");
        let color_loc = self.color_uniform_location.as_ref().expect("Color uniform location not set.");
        let matrix_loc = self.matrix_uniform_location.as_ref().expect("Matrix uniform location not set.");

        gl.viewport(0, 0, gl.drawing_buffer_width(), gl.drawing_buffer_height());
        gl.clear_color(0.0, 0.0, 0.0, 1.0); // Black background
        gl.clear(GL::COLOR_BUFFER_BIT);

        gl.use_program(Some(program));

        // Set up the perspective matrix
        let aspect_ratio = gl.drawing_buffer_width() as f32 / gl.drawing_buffer_height() as f32;
        let projection_matrix = Mat4::perspective_rh(
            (45.0f32).to_radians(), // Field of view
            aspect_ratio,          // Aspect ratio
            0.1,                   // Near clip plane
            100.0,                 // Far clip plane
        );

        // Simple camera setup (looking at origin from (0,0,3))
        let view_matrix = Mat4::look_at_rh(
            Vec3::new(0.0, 0.0, 3.0), // Camera position
            Vec3::new(0.0, 0.0, 0.0), // Look at origin
            Vec3::new(0.0, 1.0, 0.0), // Up direction
        );

        // Model matrix (e.g., for rotation based on time)
        let current_time = window().performance().unwrap().now(); // Get current time
        let rotation_speed = 0.0005; // Adjust as needed
        let angle = (current_time as f32) * rotation_speed;
        let model_matrix = Mat4::from_rotation_y(angle); // Rotate around Y axis

        let mvp_matrix = projection_matrix * view_matrix * model_matrix;

        // Pass the MVP matrix to the shader (wrap in Some())
        gl.uniform_matrix4fv_with_f32_array(Some(matrix_loc), false, mvp_matrix.as_ref());

        // Set color (e.g., red triangle) (wrap in Some())
        gl.uniform4fv_with_f32_array(Some(color_loc), &[1.0, 0.0, 0.0, 1.0]);

        // Bind vertex buffer
        gl.bind_buffer(GL::ARRAY_BUFFER, self.vertex_buffer.as_ref());
        gl.vertex_attrib_pointer_with_i32(self.position_attribute_location as u32, 3, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(self.position_attribute_location as u32);

        // Bind index buffer
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, self.index_buffer.as_ref());

        // Draw the triangle using indices
        gl.draw_elements_with_i32(GL::TRIANGLES, 3, GL::UNSIGNED_SHORT, 0);
    }
}
