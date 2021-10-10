import { Fluid } from "fluid";
import {
  createProgram,
  createShader,
  m3,
  resizeCanvasToDisplaySize,
} from "./utils";

export default class Renderer {
  canvas: HTMLCanvasElement;
  gl: WebGLRenderingContext;
  resetButton: HTMLButtonElement;
  vertices: Float32Array;
  fluid: Fluid;
  densityPerVertex: Float32Array;
  webglData: {
    locations: {
      positionAttributeLocation: number | null;
      densityAttributeLocation: number | null;
    };
    buffers: {
      positionBuffer: WebGLBuffer | null;
      densityBuffer: WebGLBuffer | null;
    };
  };

  constructor(fluid: Fluid) {
    this.canvas = document.getElementById("canvas") as HTMLCanvasElement;
    this.gl = this.canvas.getContext("webgl");
    resizeCanvasToDisplaySize(this.gl.canvas);
    this.gl.viewport(0, 0, this.gl.canvas.width, this.gl.canvas.height);
    this.gl.clearColor(0, 0, 0, 0);
    this.gl.clear(this.gl.COLOR_BUFFER_BIT);

    this.resetButton = document.getElementById("reset") as HTMLButtonElement;
    this.fluid = fluid;
    this.vertices = new Float32Array(fluid.get_size() * 12);
    this.densityPerVertex = new Float32Array(fluid.get_size() * 6);
    this.webglData = {
      locations: {
        positionAttributeLocation: null,
        densityAttributeLocation: null,
      },
      buffers: {
        positionBuffer: null,
        densityBuffer: null,
      },
    };
    this.initializeWEBGL();
  }

  private initializeWEBGL() {
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
      gl_FragColor = vec4(v_density, v_density, v_density, 1);
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
          halfSquare * 2 * i + halfSquare,
          halfSquare * 2 * j + halfSquare,
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

  render() {
    let n = this.fluid.get_n();
    const ix = (x: number, y: number) => x + (n + 2) * y;
    let size = this.fluid.get_size();
    for (let i = 1; i <= n; i++) {
      for (let j = 1; j <= n; j++) {
        const index = ix(i, j);
        for (let i = index * 6; i < index * 6 + 6; i++) {
          this.densityPerVertex[i] = this.fluid.get_density_at_index(index);
        }
      }
    }
    // console.log(this.vertices, this.densityPerVertex);
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
}
