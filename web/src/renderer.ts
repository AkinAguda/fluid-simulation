import { Fluid, FluidConfig } from "fluid";
import {
  createProgram,
  createShader,
  m3,
  resizeCanvasToDisplaySize,
  getEventLocation,
  random,
  getMultipliers,
  getClientValues,
} from "./utils";

export default class Renderer {
  private canvas: HTMLCanvasElement;
  private gl: WebGLRenderingContext;
  private clearButton: HTMLButtonElement;
  private modeButton: HTMLButtonElement;
  private mode = 0;
  private vertices: Float32Array;
  private fluid: Fluid;
  private densityPerVertex: Float32Array;
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
  private webglData: {
    locations: {
      positionAttributeLocation: number | null;
      densityAttributeLocation: number | null;
      velocityAttributeLocation: number | null;
    };
    buffers: {
      positionBuffer: WebGLBuffer | null;
      densityBuffer: WebGLBuffer | null;
      velocityBuffer: WebGLBuffer | null;
    };
  };

  constructor(fluidConfig: FluidConfig, dt: 0.6) {
    this.canvas = document.getElementById("canvas") as HTMLCanvasElement;
    this.gl = this.canvas.getContext("webgl");
    this.clearButton = document.getElementById("clear") as HTMLButtonElement;
    resizeCanvasToDisplaySize(this.gl.canvas);
    this.gl.viewport(0, 0, this.gl.canvas.width, this.gl.canvas.height);
    this.gl.clearColor(0, 0, 0, 0);
    this.gl.clear(this.gl.COLOR_BUFFER_BIT);
    this.modeButton = document.getElementById("mode") as HTMLButtonElement;
    this.modeButton.innerHTML = "All";
    this.modeButton.onclick = () => {
      if (this.mode < 2) {
        this.mode += 1;
      } else {
        this.mode = 0;
      }
      if (this.mode === 0) {
        this.modeButton.innerHTML = "All";
      } else if (this.mode === 1) {
        this.modeButton.innerHTML = "Velocity";
      } else if (this.mode === 2) {
        this.modeButton.innerHTML = "Density";
      }
    };

    this.fluid = Fluid.new(fluidConfig, dt);
    this.clearButton.onclick = () => {
      this.fluid.clear();
    };
    this.vertices = new Float32Array(this.fluid.get_size() * 12);
    this.densityPerVertex = new Float32Array(this.fluid.get_size() * 6);
    this.webglData = {
      locations: {
        positionAttributeLocation: null,
        densityAttributeLocation: null,
        velocityAttributeLocation: null,
      },
      buffers: {
        positionBuffer: null,
        densityBuffer: null,
        velocityBuffer: null,
      },
    };
    this.addEventHandlers();
    this.initializeWebGL();
  }

  addV(x: number, y: number, clientX: number, clientY: number) {
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
    this.fluid.add_velocity(this.fluid.ix(x, y), 200 * multiX, 200 * multiY);
    this.storeEventLocation(clientX, clientY);
  }

  addD(x: number, y: number) {
    this.fluid.add_density(this.fluid.ix(x, y), random(5, 10));
  }

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

  addEventHandlers() {
    const n = this.fluid.get_n();
    this.canvas.addEventListener("mousedown", (e) => {
      this.mouseEventState = { ...this.mouseEventState, mouseDown: true };
    });

    this.canvas.addEventListener("mousemove", (e) => {
      if (this.mouseEventState.mouseDown) {
        this.mouseEventState = { ...this.mouseEventState, dragging: true };
        const [clientX, clientY] = getClientValues(e);
        this.handleEvent(
          ...getEventLocation(
            n,
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
            n,
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
          n,
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
  }

  private initializeWebGL() {
    const vsGLSL: string = `
    attribute vec2 a_position;
    attribute float a_density;
  
    // This matrix is only responsible for converting my pixel coords to clipspace
    uniform mat3 u_matrix;
  
    varying float v_density;
  
    void main() {
        vec2 position = (u_matrix * vec3(a_position, 1)).xy;
        gl_Position = vec4(position, 0, 1);
        v_density = a_density;
    }
  `;

    const fsGLSL: string = `
    precision mediump float;

    varying float v_density;

    void main() {
      gl_FragColor = vec4(v_density * 1.0, v_density * 0.0, v_density * 1.0, 1);
    }
  `;

    const vertexShader = createShader(this.gl, this.gl.VERTEX_SHADER, vsGLSL);

    const fragmentShader = createShader(
      this.gl,
      this.gl.FRAGMENT_SHADER,
      fsGLSL
    );

    const program = createProgram(this.gl, vertexShader, fragmentShader);

    this.webglData.locations.positionAttributeLocation =
      this.gl.getAttribLocation(program, "a_position");

    this.webglData.locations.densityAttributeLocation =
      this.gl.getAttribLocation(program, "a_density");

    const transformationMatrixLocation = this.gl.getUniformLocation(
      program,
      "u_matrix"
    );

    this.webglData.buffers.positionBuffer = this.gl.createBuffer();

    this.webglData.buffers.densityBuffer = this.gl.createBuffer();

    this.gl.useProgram(program);

    this.gl.uniformMatrix3fv(
      transformationMatrixLocation,
      false,
      m3.projection(this.gl.canvas.width, this.gl.canvas.width)
    );

    this.populateVertices();
  }

  private populateVertices() {
    let pointIndex = 0;
    let n = this.fluid.get_n();
    const halfSquare = this.gl.canvas.width / (n + 2) / 2;
    for (let i = 0; i < n + 2; i++) {
      for (let j = 0; j < n + 2; j++) {
        const center = [
          halfSquare * 2 * j + halfSquare,
          halfSquare * 2 * i + halfSquare,
        ];

        // Vertex 1 coords
        this.vertices[pointIndex] = center[0] - halfSquare;
        this.vertices[pointIndex + 1] = center[1] - halfSquare;

        // Vertex 2 coords
        this.vertices[pointIndex + 2] = center[0] + halfSquare;
        this.vertices[pointIndex + 3] = center[1] - halfSquare;

        // Vertex 3 coords
        this.vertices[pointIndex + 4] = center[0] - halfSquare;
        this.vertices[pointIndex + 5] = center[1] + halfSquare;

        // Vertex 4 coords
        this.vertices[pointIndex + 6] = center[0] - halfSquare;
        this.vertices[pointIndex + 7] = center[1] + halfSquare;

        // Vertex 5 coords
        this.vertices[pointIndex + 8] = center[0] + halfSquare;
        this.vertices[pointIndex + 9] = center[1] - halfSquare;

        // Vertex 6 coords
        this.vertices[pointIndex + 10] = center[0] + halfSquare;
        this.vertices[pointIndex + 11] = center[1] + halfSquare;

        pointIndex += 12;
      }
    }
  }

  private render() {
    this.fluid.simulate();
    let n = this.fluid.get_n();
    let size = this.fluid.get_size();
    for (let i = 1; i <= n; i++) {
      for (let j = 1; j <= n; j++) {
        const index = this.fluid.ix(i, j);
        for (let i = index * 6; i < index * 6 + 6; i++) {
          this.densityPerVertex[i] = this.fluid.get_density_at_index(index);
        }
      }
    }
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
      this.densityPerVertex,
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

    this.gl.drawArrays(this.gl.TRIANGLES, 0, 6 * size);
  }
  private draw = () => {
    this.render();
    requestAnimationFrame(this.draw);
  };

  start() {
    // setInterval(() => {
    // // Add any debug logs
    // }, 4000);
    requestAnimationFrame(this.draw);
  }
}
