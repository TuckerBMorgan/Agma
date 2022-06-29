// Input vertex data, different for all executions of this shader.
layout(location = 0) in vec3 vertexPosition_modelspace;
layout(location = 1) in vec2 uv;

// Output data ; will be interpolated for each fragment.
out vec2 v_uv;
// Values that stay constant for the whole mesh.
layout(std140) uniform vertex {
    mat4 MVP;
};

void main(){	
	// Output position of the vertex, in clip space : MVP * position
	v_uv = uv;

	gl_Position =  MVP * vec4(vertexPosition_modelspace,1);
}