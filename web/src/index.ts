import Renderer from "./renderer";

import("fluid")
  .then(({ FluidConfig, Fluid }) => {
    const fluidConfig = FluidConfig.new(140, 0.8);
    const fluid = Fluid.new(fluidConfig, 0.6);
    let renderer = new Renderer(fluid);
    renderer.start();
  })
  .catch((e) => console.error(e));
