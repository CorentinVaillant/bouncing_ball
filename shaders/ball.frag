#version 150

uniform vec2 position;
uniform vec2 velocity;
uniform float radius;
uniform vec3 color;
uniform float z;
uniform uvec2 resolution;
uniform vec2 collision_pos;

uniform float canva_z;
uniform vec2 canva_pos;
uniform vec2 canva_size;

in vec4 gl_FragCoord;

out vec4 fragColor;



float line_segment(vec2 p,vec2 a, vec2 b,float thickness){
  vec2 pa = p-a;
  vec2 pb = p-b;
  vec2 ba = b-a;

  float h = clamp( dot(pa,ba)/dot(ba,ba), 0.0, 1.0 );

  float comp = length(pa - ba*h);

  return smoothstep(0.0, thickness, comp);
}

void main() {

  float speed = length(velocity);
  vec2 last_pos = (position - velocity*speed );
  vec2 inv_last_pos = vec2(last_pos.x,resolution.y - last_pos.y);
  vec2 inv_position = vec2(position.x,resolution.y - position.y);
  vec2 inv_collision = vec2(collision_pos.x,resolution.y - collision_pos.y);


  if (line_segment(gl_FragCoord.xy,inv_position,inv_last_pos ,1.) != 1.){
    fragColor = vec4(1.,0.2,0.2,0.8);

    gl_FragDepth = z;
  }
  if (line_segment(gl_FragCoord.xy,inv_position,inv_collision,1.) != 1.){
    fragColor = vec4(1.,1.,1.,1.);
    gl_FragDepth = z;
  }
  else if (length(inv_position-gl_FragCoord.xy) <=(radius-1.)){
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

