#version 150

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

  vec2 inv_position = vec2(position.x,resolution.y - position.y);

  if (length(inv_position-gl_FragCoord.xy) <=(radius-1.)){
    fragColor = vec4(color.xyz,1.);
    gl_FragDepth = z;
  
  }else if (length(inv_position-gl_FragCoord.xy) <=(radius)){
    fragColor = vec4(0.,0.,0.,1.);
    gl_FragDepth = z;
  
  }else{
    fragColor = vec4(gl_FragCoord.x/resolution.x,1.0- gl_FragCoord.y/resolution.y,0.5,1.0);
    gl_FragDepth = 0.;
  }

    // fragColor = vec4(vec3(gl_FragCoord.z), 1.0);

}