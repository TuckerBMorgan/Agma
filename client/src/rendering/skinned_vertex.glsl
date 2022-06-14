// Input vertex data, different for all executions of this shader.
layout(location = 0) in vec3 vertexPosition_modelspace;
layout(location = 1) in uvec4 joints;
layout(location = 2) in vec4 weights;

// Values that stay constant for the whole mesh.
layout(std140) uniform vertex {
    mat4 MVP;
    mat4 joints_matrix[256];
};

void main(){	
	// Output position of the vertex, in clip space : MVP * position

    mat4 skinMatrix = weights.x * joints_matrix[int(joints.x)] + 
                      weights.y * joints_matrix[int(joints.y)] + 
                      weights.z * joints_matrix[int(joints.z)] + 
                      weights.w * joints_matrix[int(joints.w)];

	gl_Position =  MVP * skinMatrix * vec4(vertexPosition_modelspace, 1);
}