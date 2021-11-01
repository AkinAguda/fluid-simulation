import Renderer from "./renderer";

import("fluid")
  .then(({ FluidConfig, Fluid }) => {
    const fluidConfig = FluidConfig.new(100, 1);
    const fluid = Fluid.new(fluidConfig, 0.3);
    let renderer = new Renderer(fluid);
    renderer.start();
  })
  .catch((e) => console.error(e));
