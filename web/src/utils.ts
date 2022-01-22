export const getClientValues = (event: MouseEvent | TouchEvent) => {
  const clientX =
    event.type === "mousemove" || event.type === "click"
      ? (event as MouseEvent).clientX
      : (event as TouchEvent).changedTouches[
          (event as TouchEvent).changedTouches.length - 1
        ].clientX;
  const clientY =
    event.type === "mousemove" || event.type === "click"
      ? (event as MouseEvent).clientY
      : (event as TouchEvent).changedTouches[
          (event as TouchEvent).changedTouches.length - 1
        ].clientY;
  return [clientX, clientY];
};

export const getEventLocation = (
  n: number,
  rect: DOMRect,
  clientX: number,
  clientY: number
): [number, number] => {
  const x = clientX - rect.left; //x position within the element.
  const y = clientY - rect.top; //y position within the element.
  const hRatio = n / rect.height;
  const wRatio = n / rect.width;
  const convertedX = Math.round(x * wRatio);
  const convertedY = Math.round(y * hRatio);
  return [convertedX, convertedY];
};

// WEBGL FUNCTIONS
export const createShader = (
  gl: WebGLRenderingContext,
  type: number,
  source: string
) => {
  const shader = gl.createShader(type);
  gl.shaderSource(shader, source);
  gl.compileShader(shader);

  const success = gl.getShaderParameter(shader, gl.COMPILE_STATUS);
  if (success) {
    return shader;
  }
  console.error(gl.getShaderInfoLog(shader));

  gl.deleteShader(shader);
};

export const createProgram = (
  gl: WebGLRenderingContext,
  vertexShader: WebGLShader,
  fragmentShader: WebGLShader
) => {
  const program = gl.createProgram();
  gl.attachShader(program, vertexShader);
  gl.attachShader(program, fragmentShader);

  gl.linkProgram(program);

  const success = gl.getProgramParameter(program, gl.LINK_STATUS);
  if (success) {
    return program;
  }

  console.error(gl.getProgramInfoLog(program));

  gl.deleteProgram(program);
};

export const resizeCanvasToDisplaySize = (canvas: HTMLCanvasElement) => {
  // Lookup the size the browser is displaying the canvas in CSS pixels.
  const dpr = window.devicePixelRatio;
  const { width, height } = canvas.getBoundingClientRect();
  const displayWidth = Math.round(width * dpr);
  const displayHeight = Math.round(height * dpr);
  // Get the size the browser is displaying the canvas in device pixels.
  //    const [displayWidth, displayHeight] = canvasToDisplaySizeMap.get(canvas);

  // Check if the canvas is not the same size.
  const needResize =
    canvas.width != displayWidth || canvas.height != displayHeight;

  if (needResize) {
    // Make the canvas the same size
    canvas.width = displayWidth;
    canvas.height = displayHeight;
  }

  return needResize;
};

// Matrix Functions
export const m3 = {
  projection: function (width: number, height: number) {
    //TODO: Need to update projection matrix
    return [2 / width, 0, 0, 0, -2 / height, 0, -1, 1, 1];
  },

  multiply: (a: number[], b: number[]) => {
    const a00 = a[0 * 3 + 0];
    const a01 = a[0 * 3 + 1];
    const a02 = a[0 * 3 + 2];
    const a10 = a[1 * 3 + 0];
    const a11 = a[1 * 3 + 1];
    const a12 = a[1 * 3 + 2];
    const a20 = a[2 * 3 + 0];
    const a21 = a[2 * 3 + 1];
    const a22 = a[2 * 3 + 2];
    const b00 = b[0 * 3 + 0];
    const b01 = b[0 * 3 + 1];
    const b02 = b[0 * 3 + 2];
    const b10 = b[1 * 3 + 0];
    const b11 = b[1 * 3 + 1];
    const b12 = b[1 * 3 + 2];
    const b20 = b[2 * 3 + 0];
    const b21 = b[2 * 3 + 1];
    const b22 = b[2 * 3 + 2];
    return [
      b00 * a00 + b01 * a10 + b02 * a20,
      b00 * a01 + b01 * a11 + b02 * a21,
      b00 * a02 + b01 * a12 + b02 * a22,
      b10 * a00 + b11 * a10 + b12 * a20,
      b10 * a01 + b11 * a11 + b12 * a21,
      b10 * a02 + b11 * a12 + b12 * a22,
      b20 * a00 + b21 * a10 + b22 * a20,
      b20 * a01 + b21 * a11 + b22 * a21,
      b20 * a02 + b21 * a12 + b22 * a22,
    ];
  },
};

export const gaussSeidel1 = (
  functions: ((...args: number[]) => number)[],
  initialValues: number[],
  iter: number
): number[] => {
  const initialValuesClone = [...initialValues];
  for (let i = 0; i < iter; i++) {
    initialValues.forEach((_, index) => {
      initialValuesClone[index] = functions[index](...initialValuesClone);
    });
  }
  return initialValuesClone;
};

export const round = (number: number, precision: number) =>
  Math.round((number + Number.EPSILON) * precision) / precision;

export const lerp = (a: number, b: number, k: number) => a + k * (b - a);

export const getMultipliers = (
  x1: number,
  y1: number,
  x2: number,
  y2: number
): [number, number] => {
  let x = 0;
  let y = 0;
  if (x2 - x1 > 0) {
    x = 1;
  }
  if (x2 - x1 < 0) {
    x = -1;
  }
  if (y2 - y1 > 0) {
    y = 1;
  }
  if (y2 - y1 < 0) {
    y = -1;
  }
  return [x, y];
};

export const random = (min: number, max: number) =>
  Math.floor(Math.random() * (max - min)) + min;

export const setRectangle = (
  gl: WebGLRenderingContext,
  x: number,
  y: number,
  width: number,
  height: number
) => {
  const x1 = x;
  const x2 = x + width;
  const y1 = y;
  const y2 = y + height;
  gl.bufferData(
    gl.ARRAY_BUFFER,
    new Float32Array([x1, y1, x2, y1, x1, y2, x1, y2, x2, y1, x2, y2]),
    gl.STATIC_DRAW
  );
};

export const renderToATexture = (
  gl: WebGLRenderingContext,
  targetTexture: WebGLTexture,
  width: number,
  height: number
): [WebGLTexture, WebGLFramebuffer] => {
  gl.bindTexture(gl.TEXTURE_2D, targetTexture);
  const level = 0;
  const internalFormat = gl.RGBA;
  const border = 0;
  const format = gl.RGBA;
  const type = gl.UNSIGNED_BYTE;
  const data: ArrayBufferView = null;
  gl.texImage2D(
    gl.TEXTURE_2D,
    level,
    internalFormat,
    width,
    height,
    border,
    format,
    type,
    data
  );

  // set the filtering so we don't need mips
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
  gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);

  const fb = gl.createFramebuffer();
  gl.bindFramebuffer(gl.FRAMEBUFFER, fb);

  // attach the texture as the first color attachment
  const attachmentPoint = gl.COLOR_ATTACHMENT0;
  gl.framebufferTexture2D(
    gl.FRAMEBUFFER,
    attachmentPoint,
    gl.TEXTURE_2D,
    targetTexture,
    level
  );

  return [targetTexture, fb];
};
