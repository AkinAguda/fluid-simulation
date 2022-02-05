import { Fluid, FluidConfig } from "fluid";
import { round } from "./utils";
import { RangeConfig, ConfigBox, ButtonConfig } from "./config";
import { vs1, vs2, fs1, fs2 } from "./shaders";
import {
  createProgram,
  createShader,
  getEventLocation,
  getMultipliers,
  getClientValues,
  setRectangle,
  resizeCanvasToDisplaySize,
  getDisplayDimensions,
} from "./utils";
import * as c from "./constants";

export default class Renderer {
  private canvas: HTMLCanvasElement;
  private gl: WebGLRenderingContext;
  private rangeResetHandlers: Array<() => void> = [];
  private addedDensity = c.DEFAULT_ADDED_DENSITY;
  private addedVelocity = c.DEFAULT_ADDED_VELOCITY;
  private mode = 0;
  private vertices: Float32Array;
  private fluid: Fluid;
  private densityPerGridSquare: Float32Array;
  private defaultMouseEventState = {
    mouseDown: false,
    dragging: false,
    pos: {
      x: 0,
      y: 0,
    },
  };
  mouseEventState = {
    ...this.defaultMouseEventState,
  };
  private webglData: ReturnType<typeof this.initializeWebGL>;

  constructor(canvas: HTMLCanvasElement) {
    resizeCanvasToDisplaySize(canvas);
    const [width, height] = getDisplayDimensions(canvas.width, canvas.height);
    this.canvas = document.getElementById("canvas") as HTMLCanvasElement;
    this.gl = this.canvas.getContext("webgl");
    const fluidConfig = FluidConfig.new(width, height, c.DEFAULT_DIFFUSION);
    this.fluid = Fluid.new(fluidConfig, c.DEFAULT_TIME_STEP);
    let nw = this.fluid.get_nw();
    let nh = this.fluid.get_nh();
    this.vertices = new Float32Array(nw * nh * 2);
    this.densityPerGridSquare = new Float32Array(nw * nh);
    this.addEventHandlers();
    this.webglData = this.initializeWebGL();
    new ConfigBox([
      new RangeConfig(
        {
          key: "dt",
          title: "Time Step",
          value: c.DEFAULT_TIME_STEP,
          min: c.DEFAULT_MIN_TIME_STEP,
          max: c.DEFAULT_MAX_TIME_STEP,
          step: c.DEFAULT_TIME_STEP_STEP,
          onInput: (value) => {
            this.fluid.set_dt(value);
          },
        },
        this.onRangeInstance(c.DEFAULT_TIME_STEP)
      ),
      new RangeConfig(
        {
          key: "addedD",
          title: "Added Density",
          value: this.addedDensity,
          min: c.DEFAULT_ADDED_DENSITY_MIN,
          max: c.DEFAULT_ADDED_DENSITY_MAX,
          step: 1,
          onInput: (value) => {
            this.addedDensity = value;
          },
        },
        this.onRangeInstance(c.DEFAULT_ADDED_DENSITY)
      ),
      new RangeConfig(
        {
          key: "addedV",
          title: "Added Velocity",
          value: this.addedVelocity,
          min: c.DEFAULT_ADDED_VELOCITY_MIN,
          max: c.DEFAULT_ADDED_VELOCITY_MAX,
          step: c.DEFAULT_ADDED_VELOCITY_STEP,
          onInput: (value) => {
            this.addedVelocity = value;
          },
        },
        this.onRangeInstance(c.DEFAULT_ADDED_VELOCITY)
      ),
      new RangeConfig(
        {
          key: "diff",
          title: "Diffusion",
          value: round(c.DEFAULT_DIFFUSION, 100),
          min: c.DEFAULT_MIN_DIFFUSION,
          max: c.DEFAULT_MAX_DIFFUSION,
          step: c.DEFAULT_DIFFUSION_STEP,
          onInput: (value) => {
            this.fluid.set_config_diffusion(value);
          },
        },
        this.onRangeInstance(round(c.DEFAULT_DIFFUSION, 100))
      ),
      new ButtonConfig({
        title: "Clear",
        onClick: () => {
          this.fluid.clear();
        },
      }),
      new ButtonConfig({
        title: "Reset",
        onClick: () => {
          this.rangeResetHandlers.forEach((handler) => handler());
        },
      }),
    ]);
  }

  onRangeInstance = (defaultValue: number) => (range: RangeConfig) => {
    this.rangeResetHandlers.push(() => {
      range.handleRangeInput(defaultValue);
      range.handleValueInput(defaultValue);
    });
  };

  addV = (x: number, y: number, clientX: number, clientY: number) => {
    const rect = this.canvas.getBoundingClientRect();
    const eventX = clientX - rect.left; //x position within the element.
    const eventY = clientY - rect.top; //y position within the element.
    let prevPos = this.mouseEventState.pos;
    const [multiX, multiY] = getMultipliers(
      prevPos.x,
      prevPos.y,
      eventX,
      eventY
    );
    this.fluid.add_velocity(
      this.fluid.ix(x, y),
      this.addedVelocity * multiX,
      this.addedVelocity * multiY
    );
    this.storeEventLocation(clientX, clientY);
  };

  addD = (x: number, y: number) => {
    this.fluid.add_density(this.fluid.ix(x, y), this.addedDensity);
  };

  storeEventLocation = (clientX: number, clientY: number) => {
    const rect = this.canvas.getBoundingClientRect();
    const x = clientX - rect.left; //x position within the element.
    const y = clientY - rect.top; //y position within the element.
    this.mouseEventState.pos = {
      x,
      y,
    };
  };

  handleEvent = (x: number, y: number, clientX: number, clientY: number) => {
    if (this.mode === 0) {
      this.addV(x, y, clientX, clientY);
      this.addD(x, y);
    } else if (this.mode === 1) {
      this.addV(x, y, clientX, clientY);
    } else if (this.mode === 2) {
      this.addD(x, y);
    }
  };

  addEventHandlers = () => {
    let nw = this.fluid.get_nw();
    let nh = this.fluid.get_nh();

    this.canvas.addEventListener("mousedown", (e) => {
      this.mouseEventState = { ...this.mouseEventState, mouseDown: true };
    });

    this.canvas.addEventListener("mousemove", (e) => {
      if (this.mouseEventState.mouseDown) {
        this.mouseEventState = { ...this.mouseEventState, dragging: true };
        const [clientX, clientY] = getClientValues(e);
        this.handleEvent(
          ...getEventLocation(
            nw,
            nh,
            (e.target as HTMLCanvasElement).getBoundingClientRect(),
            clientX,
            clientY
          ),
          clientX,
          clientY
        );
      }
    });

    this.canvas.addEventListener("touchmove", (e) => {
      if (this.mouseEventState.mouseDown) {
        this.mouseEventState = { ...this.mouseEventState, dragging: true };
        const [clientX, clientY] = getClientValues(e);
        this.handleEvent(
          ...getEventLocation(
            nw,
            nh,
            (e.target as HTMLCanvasElement).getBoundingClientRect(),
            clientX,
            clientY
          ),
          clientX,
          clientY
        );
      }
    });

    this.canvas.addEventListener("click", (e) => {
      const [clientX, clientY] = getClientValues(e);
      this.handleEvent(
        ...getEventLocation(
          nw,
          nh,
          (e.target as HTMLCanvasElement).getBoundingClientRect(),
          clientX,
          clientY
        ),
        clientX,
        clientY
      );
    });

    this.canvas.addEventListener("touchstart", (e) => {
      this.mouseEventState = { ...this.mouseEventState, mouseDown: true };
    });

    this.canvas.addEventListener("mouseup", () => {
      this.mouseEventState = { ...this.defaultMouseEventState };
    });

    this.canvas.addEventListener("touchend", () => {
      this.mouseEventState = { ...this.defaultMouseEventState };
    });

    this.canvas.addEventListener("mouseout", () => {
      this.mouseEventState = { ...this.defaultMouseEventState };
    });

    this.canvas.addEventListener("touchcancel", () => {
      this.mouseEventState = { ...this.defaultMouseEventState };
    });
  };

  private initializeWebGL = () => {
    const vertexShader1 = createShader(this.gl, this.gl.VERTEX_SHADER, vs1);

    const fragmentShader1 = createShader(this.gl, this.gl.FRAGMENT_SHADER, fs1);

    const program1 = createProgram(this.gl, vertexShader1, fragmentShader1);

    const vertexShader2 = createShader(this.gl, this.gl.VERTEX_SHADER, vs2);

    const fragmentShader2 = createShader(this.gl, this.gl.FRAGMENT_SHADER, fs2);

    const program2 = createProgram(this.gl, vertexShader2, fragmentShader2);

    const positionAttributeLocation = this.gl.getAttribLocation(
      program1,
      "a_position"
    );

    const densityAttributeLocation = this.gl.getAttribLocation(
      program1,
      "a_density"
    );

    const posAttributeLocation = this.gl.getAttribLocation(program2, "a_pos");

    const texAttributeLocation = this.gl.getAttribLocation(
      program2,
      "a_texCoord"
    );

    const resolutionUniformLocation = this.gl.getUniformLocation(
      program1,
      "u_resolution"
    );

    const canvasResolution = this.gl.getUniformLocation(
      program2,
      "u_canvasResolution"
    );

    const imageResolution = this.gl.getUniformLocation(
      program2,
      "u_imageResolution"
    );

    const positionBuffer = this.gl.createBuffer();

    const densityBuffer = this.gl.createBuffer();

    const posBuffer = this.gl.createBuffer();

    const texCoordBuffer = this.gl.createBuffer();

    const texture = this.gl.createTexture();

    this.gl.useProgram(program1);

    let nw = this.fluid.get_nw();
    let nh = this.fluid.get_nh();

    this.gl.uniform2f(resolutionUniformLocation, nw, nh);

    this.gl.useProgram(program2);

    this.gl.uniform2f(
      canvasResolution,
      this.gl.canvas.width,
      this.gl.canvas.height
    );

    this.gl.uniform2f(imageResolution, nw, nh);

    this.populateVertices();

    return {
      locations: {
        positionAttributeLocation,
        densityAttributeLocation,
        posAttributeLocation,
        texAttributeLocation,
      },
      buffers: {
        positionBuffer,
        densityBuffer,
        posBuffer,
        texCoordBuffer,
      },
      textureData: {
        texture: texture,
      },
      programs: {
        program1,
        program2,
      },
    };
  };

  private populateVertices = () => {
    let nw = this.fluid.get_nw();
    let nh = this.fluid.get_nh();

    let pointIndex = 0;
    const halfSquare = 0.5;
    for (let i = 1; i <= nh; i++) {
      for (let j = 1; j <= nw; j++) {
        this.vertices[pointIndex] = j - halfSquare;
        this.vertices[pointIndex + 1] = i - halfSquare;
        pointIndex += 2;
      }
    }
  };

  private rdenerToTexture = (): WebGLTexture => {
    const { gl } = this;
    let nw = this.fluid.get_nw();
    let nh = this.fluid.get_nh();

    // Texture and frame buffer code
    const targetTexture = gl.createTexture();
    gl.bindTexture(gl.TEXTURE_2D, targetTexture);

    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);

    gl.texImage2D(
      gl.TEXTURE_2D,
      0,
      gl.RGBA,
      nw,
      nh,
      0,
      gl.RGBA,
      gl.UNSIGNED_BYTE,
      null
    );

    const fb = gl.createFramebuffer();
    gl.bindFramebuffer(gl.FRAMEBUFFER, fb);

    const attachmentPoint = gl.COLOR_ATTACHMENT0;
    gl.framebufferTexture2D(
      gl.FRAMEBUFFER,
      attachmentPoint,
      gl.TEXTURE_2D,
      targetTexture,
      0
    );

    // Render code
    gl.useProgram(this.webglData.programs.program1);

    this.gl.bindBuffer(
      this.gl.ARRAY_BUFFER,
      this.webglData.buffers.positionBuffer
    );
    this.gl.bufferData(
      this.gl.ARRAY_BUFFER,
      this.vertices,
      this.gl.STATIC_DRAW
    );

    this.gl.bindBuffer(
      this.gl.ARRAY_BUFFER,
      this.webglData.buffers.densityBuffer
    );
    this.gl.bufferData(
      this.gl.ARRAY_BUFFER,
      this.densityPerGridSquare,
      this.gl.STATIC_DRAW
    );

    this.gl.bindBuffer(
      this.gl.ARRAY_BUFFER,
      this.webglData.buffers.positionBuffer
    );
    this.gl.enableVertexAttribArray(
      this.webglData.locations.positionAttributeLocation
    );
    this.gl.vertexAttribPointer(
      this.webglData.locations.positionAttributeLocation,
      2,
      this.gl.FLOAT,
      false,
      0,
      0
    );

    this.gl.bindBuffer(
      this.gl.ARRAY_BUFFER,
      this.webglData.buffers.densityBuffer
    );
    this.gl.enableVertexAttribArray(
      this.webglData.locations.densityAttributeLocation
    );

    this.gl.vertexAttribPointer(
      this.webglData.locations.densityAttributeLocation,
      1,
      this.gl.FLOAT,
      true,
      0,
      0
    );

    this.gl.viewport(0, 0, nw, nh);
    this.gl.clearColor(0, 0, 0, 0);
    this.gl.clear(this.gl.COLOR_BUFFER_BIT | this.gl.DEPTH_BUFFER_BIT);

    this.gl.drawArrays(this.gl.POINTS, 0, nw * nh);

    return targetTexture;
  };

  private renderToCanvas = () => {
    let nw = this.fluid.get_nw();
    let nh = this.fluid.get_nh();

    this.gl.useProgram(this.webglData.programs.program2);

    this.gl.bindBuffer(this.gl.ARRAY_BUFFER, this.webglData.buffers.posBuffer);

    setRectangle(this.gl, 0, 0, this.gl.canvas.width, this.gl.canvas.height);

    this.gl.bindBuffer(
      this.gl.ARRAY_BUFFER,
      this.webglData.buffers.texCoordBuffer
    );

    setRectangle(this.gl, 0, 0, nw, nh);

    this.gl.bindBuffer(this.gl.ARRAY_BUFFER, this.webglData.buffers.posBuffer);
    this.gl.enableVertexAttribArray(
      this.webglData.locations.posAttributeLocation
    );
    this.gl.vertexAttribPointer(
      this.webglData.locations.posAttributeLocation,
      2,
      this.gl.FLOAT,
      false,
      0,
      0
    );

    this.gl.bindBuffer(
      this.gl.ARRAY_BUFFER,
      this.webglData.buffers.texCoordBuffer
    );
    this.gl.enableVertexAttribArray(
      this.webglData.locations.texAttributeLocation
    );
    this.gl.vertexAttribPointer(
      this.webglData.locations.texAttributeLocation,
      2,
      this.gl.FLOAT,
      false,
      0,
      0
    );

    this.gl.bindFramebuffer(this.gl.FRAMEBUFFER, null);
    this.gl.viewport(0, 0, this.gl.canvas.width, this.gl.canvas.height);
    this.gl.drawArrays(this.gl.TRIANGLES, 0, 6);
  };

  private render = () => {
    this.fluid.simulate();
    let nw = this.fluid.get_nw();
    let nh = this.fluid.get_nh();
    let densIndex = 0;

    for (let i = 1; i <= nh; i++) {
      for (let j = 1; j <= nw; j++) {
        const index = this.fluid.ix(j, i);
        this.densityPerGridSquare[densIndex] =
          this.fluid.get_density_at_index(index);
        densIndex++;
      }
    }

    this.rdenerToTexture();
    this.renderToCanvas();
  };
  private draw = () => {
    this.render();
    requestAnimationFrame(this.draw);
  };

  start = () => {
    requestAnimationFrame(this.draw);
  };
}
