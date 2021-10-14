import Renderer from "./renderer";

import("fluid")
  .then(({ FluidConfig, Fluid }) => {
    const fluidConfig = FluidConfig.new(8, 20);
    const fluid = Fluid.new(fluidConfig);
    let renderer = new Renderer(fluid);
    renderer.start();
  })
  .catch((e) => console.error(e));
