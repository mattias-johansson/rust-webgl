use crate::animation::animation::*;
use crate::application::context::*;
use crate::events::mouse::Event;
use crate::events::mouse::Message::AnimationEnded;
use js_sys::WebAssembly;
use nalgebra::{Isometry3, Vector3};
use nalgebra_glm as glm;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext};

use std::rc::Rc;

use crate::controls::node::*;

use crate::render::gl_context::*;

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

pub fn travers_tree(cx: &Context, parent: Node, collection: &mut Vec<Node>) {
    //    web_sys::console::log_1(&parent.uuid.to_string().into());
    match cx.node_relations.get(&parent.uuid) {
        Some(children) => {
            for child in children.as_slice() {
                let node = cx.get_node_unmut(*child);
                let mut node = *node.unwrap();
                {
                    node.x = node.x + parent.x;
                    node.y = node.y + parent.y;
                    node.translate_x = node.translate_x + parent.translate_x;
                    node.translate_y = node.translate_y + parent.translate_y;
                }
                collection.push(node);

                travers_tree(&cx, node, collection);
            }
        }
        None => (),
    }
    if parent.clip {
        let mut node = Node::new(Uuid::new_v4(), 0.0, 0.0, 0.0, 0.0); //TODO FIX
        node.end_clip = true;
        collection.push(node);
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
    let mut collection: Vec<Node> = Vec::new();

    travers_tree(cx, *node.unwrap(), &mut collection);
    //TODO, select program based on node type
    for node in collection.as_slice() {
        //TODO I think GL can handle this
        let x = node.x + node.translate_x;
        let y = node.y + node.translate_y;
        //Load texture for node
        if node.texture == None {
            if node.text {
                let points = cx.vertices.get(&node.uuid).unwrap().to_vec();
                render_text(&webgl_context, program_color, points, 1.0, (0.0, 0.0, 0.0));
            } else {
                if node.end_clip {
                    end_stencil(&webgl_context);
                } else { 
                    if node.clip {
                        render_stencil(&webgl_context, program_color, node.width, node.height, x, y);
                    }
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
                }
            } 
        } else {
            match &node.texture {
                Some(texture) => {
                    let texture_slot = cx.textures.load_texture(Rc::clone(&webgl_context), texture);
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
    //    let canvas_width = context.canvas().clientWidth();
    //    let canvas_height = context.canvas().clientHeight();

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
    let vertices: [f32; 12] = [
        -rect_width,
        rect_height,
        rect_width,
        -rect_height,
        -rect_width,
        -rect_height,
        -rect_width,
        rect_height,
        rect_width,
        rect_height,
        rect_width,
        -rect_height,
    ];

    let vertex_data_attrib = context.get_attrib_location(&program, "vertexData");
    context.enable_vertex_attrib_array(vertex_data_attrib as u32);

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

    buffer_f32_data(&context, &vertices[..], vertex_data_attrib as u32, 2);
    context.enable(WebGlRenderingContext::BLEND);

    context.blend_func(
        WebGlRenderingContext::SRC_ALPHA,
        WebGlRenderingContext::ONE_MINUS_SRC_ALPHA,
    );

    let color_data_attrib = context.get_uniform_location(&program, "color");
    context.uniform3f(color_data_attrib.as_ref(), color.0, color.1, color.2);

    context.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 6);
}

fn render_text(
    context: &WebGlRenderingContext,
    program: &WebGlProgram,
    vertices: Vec<f32>,
    opacity: f32,
    color: (f32, f32, f32),
) {
    let rect_width = 50.0;
    let rect_height = 50.0;
    context.use_program(Some(&program));
    let canvas_width = 1280.0;
    let canvas_height = 703.0;

    let vertex_data_attrib = context.get_attrib_location(&program, "vertexData");
    context.enable_vertex_attrib_array(vertex_data_attrib as u32);

    context.blend_func(
        WebGlRenderingContext::SRC_ALPHA,
        WebGlRenderingContext::ONE_MINUS_SRC_ALPHA,
    );

    let transparency_data_attrib = context.get_uniform_location(&program, "transparency");
    context.uniform1f(transparency_data_attrib.as_ref(), opacity);

    let model_uni = context.get_uniform_location(&program, "model");
    let model = Isometry3::new(
        Vector3::new(400.0 + (rect_width), 0.0 + (rect_height), 1.0), //THIS IS CHANGED
        nalgebra::zero(),
    ); //move to 1,1,1
    let mut model_array = [0.; 16];
    model_array.copy_from_slice(model.to_homogeneous().as_slice());
    context.uniform_matrix4fv_with_f32_array(model_uni.as_ref(), false, &mut model_array);

    let perspective_uni = context.get_uniform_location(&program, "perspective");

    // builds the view "box"
    //Note for fonts  the z-axis is NOT turned 180 degrees compared to the vertex shader. Don't know why just yet.
    let ortho_matrix = glm::ortho(0.0, canvas_width, 0.0, canvas_height, -2.0, 2.0);

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

    buffer_f32_data(&context, &vertices[..], vertex_data_attrib as u32, 2);
    context.enable(WebGlRenderingContext::BLEND);

    context.blend_func(
        WebGlRenderingContext::SRC_ALPHA,
        WebGlRenderingContext::ONE_MINUS_SRC_ALPHA,
    );

    let color_data_attrib = context.get_uniform_location(&program, "color");
    context.uniform3f(color_data_attrib.as_ref(), color.0, color.1, color.2);
    let num_vertices = vertices.len() / 2;
    context.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, num_vertices as i32);
}

/**
 * Used to make "clipping"
 */
fn render_stencil(
    context: &WebGlRenderingContext,
    program: &WebGlProgram,
    rect_width: f32,
    rect_height: f32,
    x: f32,
    y: f32,
) {
    context.use_program(Some(&program));

    let vertex_data_attrib = context.get_attrib_location(&program, "vertexData");
    context.enable_vertex_attrib_array(vertex_data_attrib as u32);

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

    //    context.enable(WebGlRenderingContext::DEPTH_TEST);

    context.clear_stencil(0);
    context.clear(WebGlRenderingContext::STENCIL_BUFFER_BIT);

    context.stencil_op(
        WebGlRenderingContext::KEEP,
        WebGlRenderingContext::KEEP,
        WebGlRenderingContext::REPLACE,
    );
    context.stencil_func(WebGlRenderingContext::ALWAYS, 1, 0xff);
    context.stencil_mask(0xff);
    context.color_mask(false, true, false, false);

    context.enable(WebGlRenderingContext::STENCIL_TEST);

    let transparency_data_attrib = context.get_uniform_location(&program, "transparency");
    context.uniform1f(transparency_data_attrib.as_ref(), 0.0);

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

    context.blend_func(WebGlRenderingContext::ZERO, WebGlRenderingContext::ONE);

    let color_data_attrib = context.get_uniform_location(&program, "color");
    context.uniform3f(color_data_attrib.as_ref(), 0.0, 0.0, 0.0);

    context.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 6);
    context.stencil_func(WebGlRenderingContext::EQUAL, 1, 0xff);
    context.stencil_mask(0x00);
    context.color_mask(true, true, true, true);
}

fn end_stencil(context: &WebGlRenderingContext) {
    context.disable(WebGlRenderingContext::STENCIL_TEST);
    //    context.disable(WebGlRenderingContext::DEPTH_TEST);
    context.clear_stencil(0);
    context.clear(WebGlRenderingContext::STENCIL_BUFFER_BIT);
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
