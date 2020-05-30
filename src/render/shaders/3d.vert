#version 450

layout(location = 0) in vec4 in_position;
layout(location = 1) in vec4 in_normal;

layout(location = 0) out VertexData {
    vec3 normal;
    float depth;
} o;

layout(set = 0, binding = 0)
#include "uniform.glsl"

void main() {
    gl_Position = u.view_proj * u.ir_rotation * in_position;
    o.normal = in_normal.xyz;
    o.depth = in_position.z;
}
