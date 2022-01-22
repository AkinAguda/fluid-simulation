const convertToClipSpace = `
vec2 convertToClipSpace(vec2 position, vec2 resolution) {

    vec2 zeroToOne = position / resolution;
    
    vec2 zeroToTwo = zeroToOne * 2.0;
    
    vec2 clipSpace = vec2(zeroToTwo.x - 1.0, 1.0 - zeroToTwo.y);
    
    return clipSpace;
  }`;

export const vs1: string = `
attribute vec2 a_position;
attribute float a_density;

uniform vec2 u_resolution;

varying float v_density;

${convertToClipSpace}

void main() {
    gl_Position = vec4(convertToClipSpace(a_position, u_resolution), 0, 1);
    v_density = a_density;
}
`;

export const fs1: string = `
precision mediump float;

varying float v_density;

void main() {
  gl_FragColor = vec4(v_density * 1.0, v_density * 0.0, v_density * 1.0, 1);
}
`;

export const vs2: string = `
attribute vec2 a_pos;
attribute vec2 a_texCoord;

uniform vec2 u_canvasResolution;
uniform vec2 u_imageResolution;

varying vec2 v_texCoord;

${convertToClipSpace}

vec2 convertToTextureClipSpace(vec2 position, vec2 resolution) {

  vec2 zeroToOne = position / resolution;
  
  vec2 clipSpace = vec2(zeroToOne.x, 1.0 - zeroToOne.y);
  
  return clipSpace;
}

void main() {
  gl_Position = vec4(convertToClipSpace(a_pos, u_canvasResolution), 0, 1);
  v_texCoord = convertToTextureClipSpace(a_texCoord, u_imageResolution);
}
`;

export const fs2: string = `
precision mediump float;

uniform sampler2D u_texture;

varying vec2 v_texCoord;

void main() {
  gl_FragColor = texture2D(u_texture, v_texCoord).rgba;
}
`;
