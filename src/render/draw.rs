use uuid::Uuid;
use crate::animation::animation::*;
use crate::application::context::*;
use crate::events::mouse::Event;
use crate::events::mouse::Message::AnimationEnded;
use js_sys::WebAssembly;
use nalgebra::{Isometry3, Vector3};
use nalgebra_glm as glm;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext};

use std::rc::Rc;

use crate::render::lti::*;
use crate::render::texture_unit::*;

use crate::controls::node::*;


pub fn update_animations(dt: f32, cx: &mut Context) {
    for i in 0..cx.animations.len() {
        let animation: &mut Animation = cx.animations.get_mut(i).unwrap();
        if animation.state == AnimationState::Started {
            animation.set_start_time(dt);
            animation.state = AnimationState::Playing;
        } else if animation.state == AnimationState::Playing {
            if !animation.running(dt) {
                animation.state = AnimationState::Ending
            }
        } else if animation.state == AnimationState::Ending {
            animation.state = AnimationState::Ended;
            cx.events
                .push(Event::Message(AnimationEnded(animation.uuid)));
        }
    }
}

pub fn update_target_attributes(dt: f32, cx: &mut Context) {
    for i in 0..cx.animations.len() {
        let animation: Animation = *cx.animations.get(i).unwrap();
        if animation.state == AnimationState::Playing {
            update_target_attribute(dt, cx, &animation)
        } else if animation.state == AnimationState::Ending {
            update_target_attribute(dt, cx, &animation)
        }
    }
}

fn update_target_attribute(dt: f32, cx: &mut Context, animation: &Animation) {
    web_sys::console::log_1(&"update_animation:".into());
    let target_node = cx.get_node(animation.target_node);
    if target_node.is_some() {
        let value = animation.get_animated_value(dt);
        match animation.target_attribute {
            Attribute::X => {
                target_node.unwrap().translate_x = value;
            }
            Attribute::Y => {
                target_node.unwrap().translate_y = value;
            }
            Attribute::OPACITY => target_node.unwrap().opacity = value,
        }
    }
}

pub fn travers_tree(cx: &Context, parent: Node) {
    match cx.node_relations.get(&parent.uuid) {
        Some(children) => {
            for child in children.as_slice() {
                let node = cx.get_node_unmut(*child);
                travers_tree(&cx, *node.unwrap());
            }
        }, None => ()
    }
}

pub fn draw_scene(
    cx: &mut Context,
    webgl_context: Rc<WebGlRenderingContext>,
    program: &WebGlProgram,
    program_color: &WebGlProgram,
) {
    let uuid = cx.root.unwrap();
    let node = cx.get_node_unmut(uuid);
    travers_tree(cx, *node.unwrap());
    //TODO, select program based on node type
    for node in cx.nodes.as_slice() {
        //TODO I think GL can handle this
        let x = node.x + node.translate_x;
        let y = node.y + node.translate_y;
        //Load texture for node
        if node.texture == None {
            render_bg(
                &webgl_context,
                program_color,
                node.width,
                node.height,
                x,
                y,
                node.opacity,
                node.color,
            );
        } else {
            match &node.texture {
                Some(texture) => {
                    let texture_slot = cx
                        .textures
                        .load_texture(Rc::clone(&webgl_context), texture);
                    render(
                        &webgl_context,
                        program,
                        node.width,
                        node.height,
                        x,
                        y,
                        node.opacity,
                        texture_slot,
                    );
                }
                None => render_bg(
                    &webgl_context,
                    program_color,
                    node.width,
                    node.height,
                    x,
                    y,
                    node.opacity,
                    node.color,
                ),
            };
        }
    }
}

fn render(
    context: &WebGlRenderingContext,
    program: &WebGlProgram,
    rect_width: f32,
    rect_height: f32,
    x: f32,
    y: f32,
    opacity: f32,
    texture: i32,
) {
    context.use_program(Some(&program));

    let canvas_width = 1280.0;
    let canvas_height = 703.0;

    let rect_height = rect_height / 2.0;
    let rect_width = rect_width / 2.0;
    // All of the positions of our quad in local space
    let vertices: [f32; 24] = [
        -rect_width,
        rect_height,
        0.0,
        1.0,
        rect_width,
        -rect_height,
        1.0,
        0.0,
        -rect_width,
        -rect_height,
        0.0,
        0.0,
        -rect_width,
        rect_height,
        0.0,
        1.0,
        rect_width,
        rect_height,
        1.0,
        1.0,
        rect_width,
        -rect_height,
        1.0,
        0.0,
    ];

    let vertex_data_attrib = context.get_attrib_location(&program, "vertexData");
    context.enable_vertex_attrib_array(vertex_data_attrib as u32);
    context.disable(WebGlRenderingContext::DEPTH_TEST);
    context.blend_func(
        WebGlRenderingContext::SRC_ALPHA,
        WebGlRenderingContext::ONE_MINUS_SRC_ALPHA,
    );

    let transparency_data_attrib = context.get_uniform_location(&program, "transparency");
    context.uniform1f(transparency_data_attrib.as_ref(), opacity);

    let model_uni = context.get_uniform_location(&program, "model");
    let model = Isometry3::new(
        Vector3::new(x + (rect_width), y + (rect_height), 1.0),
        nalgebra::zero(),
    ); //move to 1,1,1
    let mut model_array = [0.; 16];
    model_array.copy_from_slice(model.to_homogeneous().as_slice());
    context.uniform_matrix4fv_with_f32_array(model_uni.as_ref(), false, &mut model_array);

    let perspective_uni = context.get_uniform_location(&program, "perspective");

    // builds the view "box" Note that the z-axis is turned 180 degrees compared to the vertex shader. Don't know why just yet.
    let ortho_matrix = glm::ortho(0.0, canvas_width, canvas_height, 0.0, -2.0, 2.0);

    context.uniform_matrix4fv_with_f32_array(
        perspective_uni.as_ref(),
        false,
        &mut ortho_matrix.as_slice(),
    );

    let view_uni = context.get_uniform_location(&program, "view");
    let view = Isometry3::new(Vector3::new(1.0, 1.0, 1.0), nalgebra::zero());
    let mut view_array = [0.; 16];
    view_array.copy_from_slice(view.to_homogeneous().as_slice());
    context.uniform_matrix4fv_with_f32_array(view_uni.as_ref(), false, &mut identity().as_slice());

    buffer_f32_data(&context, &vertices[..], vertex_data_attrib as u32, 4);
    context.enable(WebGlRenderingContext::BLEND);

    context.blend_func(
        WebGlRenderingContext::SRC_ALPHA,
        WebGlRenderingContext::ONE_MINUS_SRC_ALPHA,
    );

    let mesh_texture_uni = context.get_uniform_location(&program, "texture");
    context.uniform1i(mesh_texture_uni.as_ref(), texture as i32);

    context.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 6);
}

fn render_bg(
    context: &WebGlRenderingContext,
    program: &WebGlProgram,
    rect_width: f32,
    rect_height: f32,
    x: f32,
    y: f32,
    opacity: f32,
    color: (f32, f32, f32),
) {
    context.use_program(Some(&program));
    let canvas_width = 1280.0;
    let canvas_height = 703.0;

    let rect_height = rect_height / 2.0;
    let rect_width = rect_width / 2.0;
    // All of the positions of our quad in local space
    let vertices: [f32; 24] = [
        -rect_width,
        rect_height,
        0.0,
        1.0,
        rect_width,
        -rect_height,
        1.0,
        0.0,
        -rect_width,
        -rect_height,
        0.0,
        0.0,
        -rect_width,
        rect_height,
        0.0,
        1.0,
        rect_width,
        rect_height,
        1.0,
        1.0,
        rect_width,
        -rect_height,
        1.0,
        0.0,
    ];

    let vertex_data_attrib = context.get_attrib_location(&program, "vertexData");
    context.enable_vertex_attrib_array(vertex_data_attrib as u32);
    context.disable(WebGlRenderingContext::DEPTH_TEST);
    context.blend_func(
        WebGlRenderingContext::SRC_ALPHA,
        WebGlRenderingContext::ONE_MINUS_SRC_ALPHA,
    );

    let transparency_data_attrib = context.get_uniform_location(&program, "transparency");
    context.uniform1f(transparency_data_attrib.as_ref(), opacity);

    let model_uni = context.get_uniform_location(&program, "model");
    let model = Isometry3::new(
        Vector3::new(x + (rect_width), y + (rect_height), 1.0),
        nalgebra::zero(),
    ); //move to 1,1,1
    let mut model_array = [0.; 16];
    model_array.copy_from_slice(model.to_homogeneous().as_slice());
    context.uniform_matrix4fv_with_f32_array(model_uni.as_ref(), false, &mut model_array);

    let perspective_uni = context.get_uniform_location(&program, "perspective");

    // builds the view "box" Note that the z-axis is turned 180 degrees compared to the vertex shader. Don't know why just yet.
    let ortho_matrix = glm::ortho(0.0, canvas_width, canvas_height, 0.0, -2.0, 2.0);

    context.uniform_matrix4fv_with_f32_array(
        perspective_uni.as_ref(),
        false,
        &mut ortho_matrix.as_slice(),
    );

    let view_uni = context.get_uniform_location(&program, "view");
    let view = Isometry3::new(Vector3::new(1.0, 1.0, 1.0), nalgebra::zero());
    let mut view_array = [0.; 16];
    view_array.copy_from_slice(view.to_homogeneous().as_slice());
    context.uniform_matrix4fv_with_f32_array(view_uni.as_ref(), false, &mut identity().as_slice());

    buffer_f32_data(&context, &vertices[..], vertex_data_attrib as u32, 4);
    context.enable(WebGlRenderingContext::BLEND);

    context.blend_func(
        WebGlRenderingContext::SRC_ALPHA,
        WebGlRenderingContext::ONE_MINUS_SRC_ALPHA,
    );

    let color_data_attrib = context.get_uniform_location(&program, "color");
    context.uniform3f(color_data_attrib.as_ref(), color.0, color.1, color.2);

    context.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 6);
}

fn identity() -> glm::TMat4<f32> {
    glm::mat4(
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    )
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
    gl.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &data_array,
        WebGlRenderingContext::STATIC_DRAW,
    );
    gl.vertex_attrib_pointer_with_i32(attrib, size, WebGlRenderingContext::FLOAT, false, 0, 0);
}
