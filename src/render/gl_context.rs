use web_sys::{WebGlProgram, WebGlRenderingContext};
use wasm_bindgen::JsCast;

use crate::render::shaders::compile_shader;
use crate::render::shaders::link_program;

use makepad_ttf_parser::*;
use makepad_font::*;
use makepad_trapezoidator::*;
use makepad_geometry::*;
use makepad_internal_iter::*;
use makepad_path::*;

pub fn get_webgl_context() -> WebGlRenderingContext {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    
    let canvas: web_sys::HtmlCanvasElement =
        canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    let ctx_options = js_sys::Object::new();
    js_sys::Reflect::set(&ctx_options, &"stencil".into(), &true.into()).unwrap();
            
    let gl = canvas
        .get_context_with_context_options("webgl", &ctx_options)
        .unwrap()
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()
        .unwrap();

    gl
}

pub fn setup_redering_context(gl: &WebGlRenderingContext, program: &WebGlProgram) { 
    
    gl.use_program(Some(&program));
    
    let buffer = gl.create_buffer().ok_or("failed to create buffer").unwrap();
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
    
}

pub fn create_webgl_program(gl: &WebGlRenderingContext) -> WebGlProgram {
    let vert_shader = compile_shader(
        &gl,
        WebGlRenderingContext::VERTEX_SHADER,
        r#"
        precision mediump float;
        attribute vec4 vertexData;
        varying vec2 texCoords;
        
        uniform float transparency;
        uniform mat4 model;
        uniform mat4 view;
        uniform mat4 perspective;
    
        void main() {
            gl_Position = perspective * view * model * vec4(vertexData.xy, 1.0, 1.0);
            texCoords = vertexData.zw;
        }
    "#,
    )
    .unwrap();
    
    let frag_shader = compile_shader(
        &gl,
        WebGlRenderingContext::FRAGMENT_SHADER,
        r#"
        precision mediump float;
        varying vec2 texCoords;
        uniform float transparency;
        uniform sampler2D texture;
    
        void main() {
            gl_FragColor = texture2D( texture, texCoords ); 
            gl_FragColor.rgb *= gl_FragColor.a;
            gl_FragColor.w = transparency * gl_FragColor.a;
        }
        "#,
    )
    .unwrap();
    
    let program = link_program(&gl, &vert_shader, &frag_shader).unwrap();

    program
}

pub fn create_webgl_program_color(gl: &WebGlRenderingContext) -> WebGlProgram {
    let vert_shader = compile_shader(
        &gl,
        WebGlRenderingContext::VERTEX_SHADER,
        r#"
        precision mediump float;
        attribute vec2 vertexData;

        uniform float transparency;
        uniform mat4 model;
        uniform mat4 view;
        uniform mat4 perspective;
    
        void main() {
            gl_Position = perspective * view * model * vec4(vertexData.xy, 1.0, 1.0);
        }
    "#,
    )
    .unwrap();
    
    let frag_shader = compile_shader(
        &gl,
        WebGlRenderingContext::FRAGMENT_SHADER,
        r#"
        precision mediump float;
        uniform float transparency;
        uniform vec3 color;
    
        void main() {
            gl_FragColor = vec4(color, 1.0);
            gl_FragColor.rgb *= gl_FragColor.a;
            gl_FragColor.w = transparency * gl_FragColor.a;
        }
        "#,
    )
    .unwrap();
    
    let program = link_program(&gl, &vert_shader, &frag_shader).unwrap();

    program
}


pub fn parse_font() -> Vec<f32> {

    let mut points : Vec<f32> = vec![];
    static FONT: &'static [u8] = include_bytes!("../../assets/LiberationMono-Regular.ttf");

    let font : Result<Font> = parse_ttf(FONT);
    let font = font.unwrap();
    let unicode = 'W' as usize;
    let glyph_id = font.char_code_to_glyph_index_map[unicode];

    let glyph = &font.glyphs[glyph_id];
    let outline = &glyph.outline;
    let outline_points = outline.points();

    for outline_point in outline_points {
        if outline_point.is_on_curve {
            let point = outline_point.point;
            points.push(point.x);
            points.push(point.y);
        }    
    }

    let rect = &glyph.bounds;

    let mut trapezoidator = Trapezoidator::new();


    let mut points : Vec<f32> = vec![];

    let trapezoids = {
        let font_scale_pixels = 0.14;
        let mut trapezoids = Vec::new();
        let trapezoidate = trapezoidator.trapezoidate(
            glyph
                .outline
                .commands()
                .map({
                move | command | {
                    command.transform(
                        &AffineTransformation::identity()
                            .translate(Vector::new(-glyph.bounds.p_min.x, -glyph.bounds.p_min.y))
                            .uniform_scale(font_scale_pixels)
                            .translate(Vector::new(0.0, 0.0))
                    )
                }
            }).linearize(0.5),
        );
        trapezoids.extend_from_internal_iter(
                trapezoidate
        );
        trapezoids
    };
//    Y     Y
//    
//    A     B
// X  _______
//    |\    |
//    | \   |
//    |  \  |
//    |   \ |
//    |    \|
// X  -------
//    D      C
     
//   X0
//Y0 |\
//   | \
//   |  \ X1
//   |   \
//   |    | Y1
//   |    | Y2
//   |   /
//   |  /
//   | /
//Y3 |/

    for trapezoid in trapezoids {

        points.push(trapezoid.xs[0]); //A
        points.push(trapezoid.ys[0]);
        points.push(trapezoid.xs[1]); //B
        points.push(trapezoid.ys[1]);
        points.push(trapezoid.xs[0]); //C
        points.push(trapezoid.ys[2]);

        points.push(trapezoid.xs[0]); //D
        points.push(trapezoid.ys[2]);
        points.push(trapezoid.xs[1]); //A
        points.push(trapezoid.ys[1]);
        points.push(trapezoid.xs[1]); //C
        points.push(trapezoid.ys[3]);


    }    
    points
}
 
