import Renderer from "./renderer";

import("fluid")
  .then(({ FluidConfig, Fluid }) => {
    const fluidConfig = FluidConfig.new(64, 0.1, 0.4);
    const fluid = Fluid.new(fluidConfig);
    let renderer = new Renderer(fluid);
    renderer.render();
  })
  .catch((e) => console.error(e));
