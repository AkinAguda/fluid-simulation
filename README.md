# Interactive fluid simulation with [navier stokes equations](https://en.wikipedia.org/wiki/Navier%E2%80%93Stokes_equations)<br>

![demo gif](https://res.cloudinary.com/nettik-technologies/video/upload/v1643672625/compressed-sim-gif.webm)

Live Demo https://akin-fluid-simulation.netlify.app/

## Resources<br>

Real-Time Fluid Dynamics for Games by Jos Stam <br>
Fluid Simulation SIGGRAPH 2007 Course Notes by Robert Bridson and Matthias Muller-Fischer<br>
Gonkee's [video](https://www.youtube.com/watch?v=qsYE1wMEMPA&t)<br>
3Blue1Brown's [video on divergence and curl](https://www.youtube.com/watch?v=rB83DpBJQsE&t)<br>
The Coding Train's [video](https://www.youtube.com/watch?v=alhpH6ECFvQ&t)<br>

## Contribution

Assuming you have [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) and [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) installed:

- Run `wasm-pack-build` to compile rust code to web assembly<br>
- `cd` into the `web` directory and run `npm run start:dev` the application will open up on `localhost:8000`
