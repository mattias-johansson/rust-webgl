use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader, WebGlUniformLocation};
use nalgebra::{Isometry3, Perspective3, Point3, Vector3};
use std::f32::consts::PI;
use std::rc::Rc;
use self::render::texture_unit::*;
use crate::load_texture_img::load_texture_image;
use js_sys::WebAssembly;
use nalgebra_glm as glm;
use std::cell::RefCell;

mod load_texture_img;
mod render;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    let context = Rc::new(context);
    
    load_texture_image(
        Rc::clone(&context),
        "/assets/button.png",
        TextureUnit::Button,
    );

    load_texture_image(
        Rc::clone(&context),
        "/assets/grey.png",
        TextureUnit::Toggle,
    );
    
    load_texture_image(
        Rc::clone(&context),
        "/assets/blue.png",
        TextureUnit::ToggleActive,
    );

    load_texture_image(
        Rc::clone(&context),
        "/assets/bg.png",
        TextureUnit::ToggelBackground,
    );

    let vert_shader = compile_shader(
        &context,
        WebGlRenderingContext::VERTEX_SHADER,
      r#"
        attribute vec4 vertexData;
        varying vec2 texCoords;
        
        uniform mat4 model;
            uniform mat4 view;
        uniform mat4 perspective;

        void main() {
            gl_Position = perspective * view * model * vec4(vertexData.xy, 0.0, 1.0);
            texCoords = vertexData.zw;
        }
    "#,
    )?;

    let frag_shader = compile_shader(
        &context,
        WebGlRenderingContext::FRAGMENT_SHADER,
    r#"
        precision mediump float;
        varying vec2 texCoords;
        uniform sampler2D texture;

        void main() {
            gl_FragColor = texture2D( texture, texCoords ); 
            gl_FragColor.rgb *= gl_FragColor.a;
        }
        "#,
    )?;
    let program = link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));


    let canvas_width = 1280.0;
    let canvas_height = 703.0;

    let pxw = 2.0 / canvas_width;
    let pxh = 2.0 / canvas_height;

    let rect_width = 100.0;
    let rect_height = 50.0;

    let buffer = context.create_buffer().ok_or("failed to create buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        context.clear_color(1.0, 1.0, 1.0, 1.0);
        context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);

        render(&context, &program, 107.0, 36.0, TextureUnit::ToggelBackground);
        render(&context, &program, 30.0, 30.0, TextureUnit::Toggle);
        render(&context, &program, 30.0, 30.0, TextureUnit::ToggleActive);

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}

pub fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

pub fn get_uniform_location(
    gl: &WebGlRenderingContext,
    uniform_name: &str,
    program: &WebGlProgram
) -> Option<WebGlUniformLocation> {
            gl.get_uniform_location(&program, uniform_name)
}

pub fn buffer_f32_data(gl: &WebGlRenderingContext, data: &[f32], attrib: u32, size: i32) {
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
    
pub fn make_coord() -> Vec<f32> {
                            
        let left_x = -0.7;
        let top_y = 0.7;
        let right_x = 0.7;
        let bottom_y = -0.7;

        // All of the positions of our quad in screen space
        let positions = [
            left_x, top_y, // Top Left
            right_x, bottom_y, // Bottom Right
            left_x, bottom_y, // Bottom Left
            left_x, top_y, // Top Left
            right_x, top_y, // Top Right
            right_x, bottom_y, // Bottom Right
        ];

        let texture_coords = [
            0., 1., // Top left
            1., 0., // Bottom Right
            0., 0., // Bottom Left
            0., 1., // Top Left
            1., 1., // Top Right
            1., 0., // Bottom Right
        ];

        let mut vertices = vec![];

        for i in 0..positions.len() {
            // Skip odd indices
            if i % 2 == 1 {
                continue;
            }

            vertices.push(positions[i]);
            vertices.push(positions[i + 1]);
            vertices.push(texture_coords[i]);
            vertices.push(texture_coords[i + 1]);
        }

        vertices
}

pub fn render(context : &WebGlRenderingContext, program: &WebGlProgram, rect_width: f32, rect_height: f32, texture: TextureUnit) {
    let canvas_width = 1280.0;
    let canvas_height = 703.0;

    
    //let vertices: Vec<f32> = make_coord();

    let pxw = 2.0 / canvas_width;
    let pxh = 2.0 / canvas_height;

    // All of the positions of our quad in local space
    let vertices: [f32; 24] = [ -rect_width / 2.0 * pxw, rect_height / 2.0 * pxh , 0.0, 1.0,
                                rect_width / 2.0 * pxw,  -rect_height / 2.0 * pxh , 1.0, 0.0,
                                -rect_width / 2.0 *  pxw, -rect_height / 2.0 * pxh , 0.0, 0.0,
                                -rect_width / 2.0 * pxw, rect_height / 2.0 * pxh , 0.0, 1.0,
                                rect_width / 2.0 * pxw , rect_height / 2.0 * pxh , 1.0, 1.0,
                                rect_width / 2.0 * pxw,  -rect_height / 2.0 * pxh , 1.0, 0.0,];

    let vertex_data_attrib = context.get_attrib_location(&program, "vertexData");
    context.enable_vertex_attrib_array(vertex_data_attrib as u32);
    

    let model_uni = context.get_uniform_location(&program, "model");
    let model = Isometry3::new(Vector3::new(1.0, 1.0, 1.0), nalgebra::zero());
    let mut model_array = [0.; 16];
    model_array.copy_from_slice(model.to_homogeneous().as_slice());
/*    console::log_1(&"model_array".into());
    for value in model_array.iter() {
        console::log_1(&value.to_string().into());
    }
*/    context.uniform_matrix4fv_with_f32_array(model_uni.as_ref(), false, &mut identity().as_slice());
//    console::log_1(&"perspective_array1".into());
    let perspective_uni = context.get_uniform_location(&program, "perspective");
//    console::log_1(&"perspective_array2".into());
    let ortho_matrix = glm::ortho(0.0, canvas_width, 0.0, canvas_height, 0.1, 100.0);
//    let mut perspective_array = [0.; 16];

//    console::log_1(&"perspective_array3".into());
//    perspective_array.copy_from_slice(ortho_matrix.to_homogeneous().as_slice()); //error
//    console::log_1(&"perspective_array4".into());
//    for value in perspective_array.iter() {
//        console::log_1(&value.to_string().into());
//    }
    context.uniform_matrix4fv_with_f32_array(perspective_uni.as_ref(), false, &mut identity().as_slice());

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

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn identity() -> glm::TMat4<f32> {
    glm::mat4(  1.0,0.0,0.0,0.0,
                0.0,1.0,0.0,0.0,
                0.0,0.0,1.0,0.0,
                0.0,0.0,0.0,1.0,)
}