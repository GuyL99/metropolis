pub mod vs {
        vulkano_shaders::shader!{
            ty: "vertex",
            src: "#version 310 es
precision highp float;
layout(location = 0) in vec2 position;
layout(location = 1) in vec4 color;
layout(location = 0) out vec4 _color;
void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    _color = color;
}"
        }
    }

pub mod fs {
        vulkano_shaders::shader!{
            ty: "fragment",
            src: "#version 310 es
precision highp float;
layout(location = 0) in vec4 _color;
layout(location = 0) out vec4 f_color;
void main() {
    f_color = _color;
}
"
        }
    }
