use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader, WebGlUniformLocation};

struct ImageQuad {
    program : WebGlProgram,
}

impl ImageQuad {
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
            gl_Position = perspective * view * model * vec4(vertexData.xy, 1.0, 1.0);
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
    self.program = link_program(&context, &vert_shader, &frag_shader)?;
}