import Renderer from "./renderer";

import("fluid")
  .then(({ FluidConfig, Fluid }) => {
    const fluidConfig = FluidConfig.new(150, 0.3);
    const fluid = Fluid.new(fluidConfig);
    let renderer = new Renderer(fluid);
    renderer.start();
  })
  .catch((e) => console.error(e));
