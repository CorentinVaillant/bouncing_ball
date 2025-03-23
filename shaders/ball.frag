#version 330

uniform vec2 position;
uniform float radius;
uniform vec3 color;
uniform float z;
uniform uvec2 resolution;

in vec4 gl_FragCoord;

out vec4 fragColor;


void main() {
  if (length(position-gl_FragCoord.xy) <=(radius)){
    fragColor = vec4(color,1.);
    gl_FragDepth = z;
  }else{
    gl_FragDepth = 0.;
  }

  // fragColor = vec4(gl_FragCoord.xyz,1.);

}