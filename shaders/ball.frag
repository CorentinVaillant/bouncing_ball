#version 330

uniform vec2 position;
uniform float radius;
uniform vec3 color;
uniform float z;
uniform uvec2 resolution;

uniform float canva_z;
uniform vec2 canva_pos;
uniform vec2 canva_size;

in vec4 gl_FragCoord;

out vec4 fragColor;


void main() {
  if (length(position-gl_FragCoord.xy) <=(radius)){
    fragColor = vec4(position.x/resolution.x,position.y/resolution.y,0.,1.);
    gl_FragDepth = z;
  }
  
  else{
    gl_FragDepth = 0.;
  }


}