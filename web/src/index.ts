import Renderer from "./renderer";

import("fluid")
  .then(({ FluidConfig }) => {
    const fluidConfig = FluidConfig.new(140, 0.8);
    let renderer = new Renderer(fluidConfig, 0.6);
    renderer.start();
  })
  .catch((e) => console.error(e));
