import("fluid")
  .then((fluid) => {
    fluid.greet();
  })
  .catch((e) => console.error(e));
