use web_sys::{WebGlProgram, WebGlRenderingContext};
use nalgebra::{Isometry3, Vector3};
use nalgebra_glm as glm;
use js_sys::WebAssembly;
use wasm_bindgen::JsCast;

use crate::render::texture_unit::*;
use crate::render::lti::*;
use std::rc::Rc;

pub fn init_textures(gl: Rc<WebGlRenderingContext>) {    


    load_texture_image(
        Rc::clone(&gl),
        "/assets/button.png",
        TextureUnit::Button,
    );

    load_texture_image(
        Rc::clone(&gl),
        "/assets/button_pressed.png",
        TextureUnit::ButtonPressed,
    );

    load_texture_image(
        Rc::clone(&gl),
        "/assets/grey.png",
        TextureUnit::Toggle,
    );
    
    load_texture_image(
        Rc::clone(&gl),
        "/assets/blue.png",
        TextureUnit::ToggleActive,
    );

    load_texture_image(
        Rc::clone(&gl),
        "/assets/bg.png",
        TextureUnit::ToggelBackground,
    );
}

pub fn render(context : &WebGlRenderingContext, program: &WebGlProgram, rect_width: f32, rect_height: f32, x: f32, y: f32, texture: TextureUnit) {
    let canvas_width = 1280.0;
    let canvas_height = 703.0;

    let rect_height = rect_height / 2.0;
    let rect_width = rect_width / 2.0;
    // All of the positions of our quad in local space
    let vertices: [f32; 24] = [ -rect_width, rect_height, 0.0, 1.0,
                                rect_width,  -rect_height, 1.0, 0.0,
                                -rect_width, -rect_height, 0.0, 0.0,
                                -rect_width, rect_height, 0.0, 1.0,
                                rect_width, rect_height, 1.0, 1.0,
                                rect_width,  -rect_height, 1.0, 0.0,];


    let vertex_data_attrib = context.get_attrib_location(&program, "vertexData");
    context.enable_vertex_attrib_array(vertex_data_attrib as u32);
    

    let model_uni = context.get_uniform_location(&program, "model");
    let model = Isometry3::new(Vector3::new(x+(rect_width), y+(rect_height), 1.0), nalgebra::zero()); //move to 1,1,1
    let mut model_array = [0.; 16];
    model_array.copy_from_slice(model.to_homogeneous().as_slice());
    context.uniform_matrix4fv_with_f32_array(model_uni.as_ref(), false, &mut model_array);


    let perspective_uni = context.get_uniform_location(&program, "perspective");

    // builds the view "box" Note that the z-axis is turned 180 degrees compared to the vertex shader. Don't know why just yet. 
    let ortho_matrix = glm::ortho(0.0, canvas_width, canvas_height, 0.0, -2.0, 2.0); 

    context.uniform_matrix4fv_with_f32_array(perspective_uni.as_ref(), false, &mut ortho_matrix.as_slice());

    let view_uni = context.get_uniform_location(&program, "view");
    let view = Isometry3::new(Vector3::new(1.0, 1.0, 1.0), nalgebra::zero());
    let mut view_array = [0.; 16];
    view_array.copy_from_slice(view.to_homogeneous().as_slice());
    context.uniform_matrix4fv_with_f32_array(view_uni.as_ref(), false, &mut identity().as_slice());

    buffer_f32_data(&context, &vertices[..], vertex_data_attrib as u32, 4);
    context.enable(WebGlRenderingContext::BLEND);

    context.blend_func(WebGlRenderingContext::SRC_ALPHA, WebGlRenderingContext::ONE_MINUS_SRC_ALPHA);

    let mesh_texture_uni = context.get_uniform_location(&program, "texture");
    context.uniform1i(mesh_texture_uni.as_ref(), texture.texture_unit() as i32);

    context.draw_arrays(
        WebGlRenderingContext::TRIANGLES,
        0,
        6,
    );

}

fn identity() -> glm::TMat4<f32> {
    glm::mat4(  1.0,0.0,0.0,0.0,
                0.0,1.0,0.0,0.0,
                0.0,0.0,1.0,0.0,
                0.0,0.0,0.0,1.0,)
}


 fn buffer_f32_data(gl: &WebGlRenderingContext, data: &[f32], attrib: u32, size: i32) {
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();

        let data_location = data.as_ptr() as u32 / 4;

        let data_array = js_sys::Float32Array::new(&memory_buffer)
            .subarray(data_location, data_location + data.len() as u32);

        let buffer = gl.create_buffer().unwrap();

        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
        gl.buffer_data_with_array_buffer_view(WebGlRenderingContext::ARRAY_BUFFER, &data_array, WebGlRenderingContext::STATIC_DRAW);
        gl.vertex_attrib_pointer_with_i32(attrib, size, WebGlRenderingContext::FLOAT, false, 0, 0);
    }
    
