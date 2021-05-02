use web_sys::{WebGlProgram, WebGlRenderingContext};
use wasm_bindgen::JsCast;

use crate::render::shaders::compile_shader;
use crate::render::shaders::link_program;

pub fn get_webgl_context() -> WebGlRenderingContext {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    
    let canvas: web_sys::HtmlCanvasElement =
        canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    let ctx_options = js_sys::Object::new();
    js_sys::Reflect::set(&ctx_options, &"stencil".into(), &true.into()).unwrap();
    js_sys::Reflect::set(&ctx_options, &"alpha".into(), &false.into()).unwrap();
            
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

pub fn create_webgl_program_nine(gl: &WebGlRenderingContext) -> WebGlProgram {
    let vert_shader = compile_shader(
        &gl,
        WebGlRenderingContext::VERTEX_SHADER,
        r#"
        precision mediump float;

        attribute   vec4    vertexData;
        attribute   vec3    position;
        attribute   vec2    uv;

        varying vec2 size;
        
        varying vec3    v_position;
        varying vec2    v_uv;

        varying mediump vec2 scale;

        uniform     float   transparency;
        uniform     mat4    model;
        uniform     mat4    view;
        uniform     mat4    perspective;
        uniform     float   xscale;
        uniform     float   yscale;

        void main(){
            vec4 ws_position    = view * model * vec4( position, 1.0 );
            v_position          = position.xyz;
            gl_Position         = ws_position;
        
            v_uv      = uv;
            scale   = vec2( xscale, yscale );           // Scaling
            size    = position.xy * v_scale + v_scale;  // Size of the quad after being scaled.
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
        uniform sampler2D texture;
    
        varying     vec3        v_position;
        varying     vec2        v_uv;
        varying     vec2        v_size;
        varying mediump vec2    v_scale;

        uniform     vec3        u_color;

        const float QUAD_SIZE = 2.0;

        void main() {

            vec2 max_size = scale * QUAD_SIZE;
            vec4 edges = vec4( 0.45, 0.45, 0.45, 0.45, );
            vec4 uv_edges = ( 0.45 / QUAD_SIZE );
            
            float r_edge = max_size.x - edges.y;
            float t_edge = max_size.y - edges.z
            
	        // LEFT EDGE
            if( size.x <= edges.w ) uv.x = uv_edges.w * ( size.x / edges.w );

        	// RIGHT EDGE
	        if( size.x >= r_edge ) uv.x = 1.0 - ( uv_edges.y * ( 1.0 - ( size.x - r_edge ) / edges.y ));

            // CENTER
            if( size.x > edges.w && size.x < r_edge ){
                float cw		= QUAD_SIZE - edges.y - edges.w;	// Center Size Width UNSCALED !!!
                float uv_cw		= 1.0 - uv_edges.y - uv_edges.w;	// Center UV Width
        
                uv.x = uv_edges.w + uv_cw * fract( ( size.x - edges.w ) / cw );
            }
        	// BOTTOM EDGE
            if( size.y <= edges.z )	uv.y = uv_edges.z * ( size.y / edges.z );

            // TOP EDGE
            if( size.y >= t_edge)		uv.y = 1.0 - ( uv_edges.x * ( 1.0 - ( size.y - t_edge ) / edges.x ) );

            // CENTER
            if( v_size.y > edges.z && v_size.y < t_edge ){
                float cw		= QUAD_SIZE - edges.x - edges.z;	// Center Size Width UNSCALED !!!
                float uv_cw		= 1.0 - uv_edges.x - uv_edges.z;	// Center UV Width

                uv.y = uv_edges.z + uv_cw * fract( ( size.y - edges.z ) / cw );
            }

            //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            gl_FragColor = texture2D( texture, uv );

            gl_FragColor.rgb *= gl_FragColor.a;
            gl_FragColor.w = transparency * gl_FragColor.a;
        }
        "#,
    )
    .unwrap();
    
    let program = link_program(&gl, &vert_shader, &frag_shader).unwrap();

    program
}

/*
//##################################################################################
const V_SHADER = `
attribute	vec3	position;
attribute	vec2	uv;

uniform 	mat4	modelViewMatrix;
uniform 	mat4	projectionMatrix;

uniform 	float 	xscale;
uniform 	float 	yscale;

varying 	vec3 	v_position;
varying 	vec2	v_uv;

varying 	vec2 	v_size;
varying  	mediump vec2 v_scale;

void main(){	
	vec4 ws_position 	= modelViewMatrix * vec4( position, 1.0 );
	v_position			= position.xyz;
	gl_Position			= projectionMatrix * ws_position;

	v_uv				= uv;
	v_scale				= vec2( xscale, yscale );			// Scaling
	v_size				= position.xy * v_scale + v_scale;	// Size of the quad after being scaled.
}`;

const F_SHADER = `
//#extension GL_OES_standard_derivatives : enable
precision mediump float;

varying 	vec3 	v_position;
varying 	vec2	v_uv;
varying 	vec2 	v_size;
varying  	mediump vec2 	v_scale;

uniform vec3		u_color;
uniform sampler2D 	map;

const float QUAD_SIZE = 2.0;

void main(){
	//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
	// Following Values should Probably be in a Uniform.
	vec2 max_size	= v_scale * QUAD_SIZE;
	vec4 edges		= vec4( 0.45, 0.45, 0.45, 0.45 );	// TOP(x) - RIGHT(y) - BOTTOM(z) - LEFT(w);
	vec4 uv_edges	= vec4( 0.45 / QUAD_SIZE );			// TOP - RIGHT - BOTTOM - LEFT;

	//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
	vec2 uv			= v_uv;					
	float rEdge		= max_size.x - edges.y;		// Max Width - Right Width, Starting position of Right Edge
	float tEdge		= max_size.y - edges.z;

	//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
	// MAP OUT UV.X

	// Mapping the Edge between 0 to 1, Then multiplying that by the UV Edge value
	// The idea about how to map the Edges is we normalized the current width over the 
	// borders width. With the Border Norm, use that as a scale for max UV.x that defines
	// how much of the texture makes up that border.

	// LEFT EDGE
	if( v_size.x <= edges.w )	uv.x = uv_edges.w * ( v_size.x / edges.w );
	
	// RIGHT EDGE
	if( v_size.x >= rEdge)		uv.x = 1.0 - ( uv_edges.y * ( 1.0 - ( v_size.x - rEdge ) / edges.y ) );

	// CENTER
	if( v_size.x > edges.w && v_size.x < rEdge ){
		float cw		= QUAD_SIZE - edges.y - edges.w;	// Center Size Width UNSCALED !!!
		float uv_cw		= 1.0 - uv_edges.y - uv_edges.w;	// Center UV Width

		uv.x = uv_edges.w + uv_cw * fract( ( v_size.x - edges.w ) / cw );
	}

	//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
	// MAP OUT UV.Y

	// BOTTOM EDGE
	if( v_size.y <= edges.z )	uv.y = uv_edges.z * ( v_size.y / edges.z );

	// TOP EDGE
	if( v_size.y >= tEdge)		uv.y = 1.0 - ( uv_edges.x * ( 1.0 - ( v_size.y - tEdge ) / edges.x ) );

	// CENTER
	if( v_size.y > edges.z && v_size.y < tEdge ){
		float cw		= QUAD_SIZE - edges.x - edges.z;	// Center Size Width UNSCALED !!!
		float uv_cw		= 1.0 - uv_edges.x - uv_edges.z;	// Center UV Width

		uv.y = uv_edges.z + uv_cw * fract( ( v_size.y - edges.z ) / cw );
	}

	//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
	gl_FragColor = texture2D( map, uv );
}`;
*/