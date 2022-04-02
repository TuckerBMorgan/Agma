precision mediump float;

in vec2 v_uv;
out vec4 a_color;

uniform sampler2D tex;

void main() {
    a_color = texture(tex, v_uv);
    if (a_color.a <= 0.0) {
        discard;
    }
}