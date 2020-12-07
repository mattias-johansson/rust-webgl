importScripts('webgl_api.js')

const { Container, ContainerBuilder, Color } = wasm_bindgen;

async function run() {
  await wasm_bindgen('/webgl_api_bg.wasm');

  console.log("done");
}

self.onmessage = function(event) {

  let modul = event.data;

  console.log("WORKER: on message");
    run().then(response => {
      console.log("WORKER: after run");
      let color = new Color();
      console.log("WORKER: color created!");
      console.log("WORKER: new color: " + color.r);
      let container = new ContainerBuilder().x(2.0).build();
      console.log("WORKER: new containter: " + container.x);
      postMessage("TEST");
  });
}