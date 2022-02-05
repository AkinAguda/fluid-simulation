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
  nw: number,
  nh: number,
  rect: DOMRect,
  clientX: number,
  clientY: number
): [number, number] => {
  const x = clientX - rect.left; //x position within the element.
  const y = clientY - rect.top; //y position within the element.
  const hRatio = nh / rect.height;
  const wRatio = nw / rect.width;
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

export const round = (number: number, precision: number) =>
  Math.round((number + Number.EPSILON) * precision) / precision;

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

export const getDisplayDimensions = (
  width: number,
  height: number
): number[] => {
  let count = 220;
  if (Math.max(width, height) / Math.min(width, height) <= 1.5) {
    count = 180;
  }
  if (width > height) {
    return [count, count / (width / height)];
  } else if (height > width) {
    return [count / (height / width), count];
  } else {
    return [width, height];
  }
};
