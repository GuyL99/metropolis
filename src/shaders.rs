pub mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: "#version 450
precision highp float;
layout(location = 0) in vec2 position;
layout(location = 1) in vec4 color;
layout(location = 2) in vec2 tex_coords;
layout(location = 0) out vec4 _color;
layout(location = 1) out vec2 _tex_coords;
void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    _color = color;
    _tex_coords = position+ vec2(tex_coords*2);
}"
    }
}
pub mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        src: "#version 450
precision highp float;
layout(location = 0) in vec4 _color;
layout(location = 1) in vec2 _tex_coords;
layout(location = 0) out vec4 f_color;
void main() {
    f_color = _color;
}
"
    }
}
pub mod fsimg {
    vulkano_shaders::shader! {
        ty: "fragment",
        src: "#version 450
precision highp float;
layout(location = 0) in vec4 _color;
layout(location = 1) in vec2 _tex_coords;
layout(location = 0) out vec4 f_color;
layout(set = 0, binding = 0) uniform sampler2D tex;
void main() {
    f_color = texture(tex, _tex_coords);
    //f_color = _color;
}
"
    }
}
pub mod cs_sub {
    vulkano_shaders::shader! {
        ty: "compute",
        src: "#version 450
precision highp float;
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
layout(set = 0, binding = 0) buffer Data {
    int vecer[];
} buf;
layout(set = 0, binding = 1) buffer Data2 {
    int sclr;
} buf2;
void main(){
    int idx = int(gl_GlobalInvocationID.x);
    buf.vecer[idx]-=buf2.sclr;
}"
    }
}
pub mod cs_add {
    vulkano_shaders::shader! {
        ty: "compute",
        src: "#version 450
precision highp float;
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
layout(set = 0, binding = 0) buffer Data {
    int vecer[];
} buf;
layout(set = 0, binding = 1) buffer Data2 {
    int sclr;
} buf2;
void main(){
    int idx = int(gl_GlobalInvocationID.x);
    buf.vecer[idx]+=buf2.sclr;
}"
    }
}
pub mod cs_mult {
    vulkano_shaders::shader! {
        ty: "compute",
        src: "#version 450
precision highp float;
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
layout(set = 0, binding = 0) buffer Data {
    int vecer[];
} buf;
layout(set = 0, binding = 1) buffer Data2 {
    int sclr;
} buf2;
void main(){
    int idx = int(gl_GlobalInvocationID.x);
    buf.vecer[idx]*=buf2.sclr;
}"
    }
}
pub mod cs_div {
    vulkano_shaders::shader! {
        ty: "compute",
        src: "#version 450
precision highp float;
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
layout(set = 0, binding = 0) buffer Data {
    int vecer[];
} buf;
layout(set = 0, binding = 1) buffer Data2 {
    int sclr;
} buf2;
void main(){
    int idx = int(gl_GlobalInvocationID.x);
    buf.vecer[idx]/=buf2.sclr;
}"
    }
}
pub mod cs_float_div {
    vulkano_shaders::shader! {
        ty: "compute",
        src: "#version 450
precision highp float;
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
layout(set = 0, binding = 0) buffer Data {
    float vecer[];
} buf;
layout(set = 0, binding = 1) buffer Data2 {
    float sclr;
} buf2;
void main(){
    int idx = int(gl_GlobalInvocationID.x);
    buf.vecer[idx]/=buf2.sclr;
}"
    }
}
pub mod cs_float_sub {
    vulkano_shaders::shader! {
        ty: "compute",
        src: "#version 450
precision highp float;
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
layout(set = 0, binding = 0) buffer Data {
    float vecer[];
} buf;
layout(set = 0, binding = 1) buffer Data2 {
    float sclr;
} buf2;
void main(){
    int idx = int(gl_GlobalInvocationID.x);
    buf.vecer[idx]-=buf2.sclr;
}"
    }
}
pub mod cs_float_add {
    vulkano_shaders::shader! {
        ty: "compute",
        src: "#version 450
precision highp float;
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
layout(set = 0, binding = 0) buffer Data {
    float vecer[];
} buf;
layout(set = 0, binding = 1) buffer Data2 {
    float sclr;
} buf2;
void main(){
    int idx = int(gl_GlobalInvocationID.x);
    buf.vecer[idx]+=buf2.sclr;
}"
    }
}
pub mod cs_float_mult {
    vulkano_shaders::shader! {
        ty: "compute",
        src: "#version 450
precision highp float;
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
layout(set = 0, binding = 0) buffer Data {
    float vecer[];
} buf;
layout(set = 0, binding = 1) buffer Data2 {
    float sclr;
} buf2;
void main(){
    int idx = int(gl_GlobalInvocationID.x);
    buf.vecer[idx]*=buf2.sclr;
}"
    }
}
pub mod cs_float_add_vec {
    vulkano_shaders::shader! {
        ty: "compute",
        src: "#version 450
precision highp float;
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
layout(set = 0, binding = 0) buffer Data {
    float vecer[];
} buf;
layout(set = 0, binding = 1) buffer Data2 {
    float vecer[];
} buf2;
void main(){
    int idx = int(gl_GlobalInvocationID.x);
    buf.vecer[idx]+=buf2.vecer[idx];
}"
    }
}
pub mod cs_float_sub_vec {
    vulkano_shaders::shader! {
        ty: "compute",
        src: "#version 450
precision highp float;
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
layout(set = 0, binding = 0) buffer Data {
    float vecer[];
} buf;
layout(set = 0, binding = 1) buffer Data2 {
    float vecer[];
} buf2;
void main(){
    int idx = int(gl_GlobalInvocationID.x);
    buf.vecer[idx]-=buf2.vecer[idx];
}"
    }
}
pub mod cs_add_vec {
    vulkano_shaders::shader! {
        ty: "compute",
        src: "#version 450
precision highp float;
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
layout(set = 0, binding = 0) buffer Data {
    int vecer[];
} buf;
layout(set = 0, binding = 1) buffer Data2 {
    int vecer[];
} buf2;
void main(){
    int idx = int(gl_GlobalInvocationID.x);
    buf.vecer[idx]+=buf2.vecer[idx];
}"
    }
}
pub mod cs_sub_vec {
    vulkano_shaders::shader! {
        ty: "compute",
        src: "#version 450
precision highp float;
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
layout(set = 0, binding = 0) buffer Data {
    int vecer[];
} buf;
layout(set = 0, binding = 1) buffer Data2 {
    int vecer[];
} buf2;
void main(){
    int idx = int(gl_GlobalInvocationID.x);
    buf.vecer[idx]-=buf2.vecer[idx];
}"
    }
}
pub mod cs_float_div_vec {
    vulkano_shaders::shader! {
        ty: "compute",
        src: "#version 450
precision highp float;
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
layout(set = 0, binding = 0) buffer Data {
    float vecer[];
} buf;
layout(set = 0, binding = 1) buffer Data2 {
    float vecer[];
} buf2;
void main(){
    int idx = int(gl_GlobalInvocationID.x);
    buf.vecer[idx]/=buf2.vecer[idx];
}"
    }
}
pub mod cs_float_mult_vec {
    vulkano_shaders::shader! {
        ty: "compute",
        src: "#version 450
precision highp float;
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
layout(set = 0, binding = 0) buffer Data {
    float vecer[];
} buf;
layout(set = 0, binding = 1) buffer Data2 {
    float vecer[];
} buf2;
void main(){
    int idx = int(gl_GlobalInvocationID.x);
    buf.vecer[idx]*=buf2.vecer[idx];
}"
    }
}
