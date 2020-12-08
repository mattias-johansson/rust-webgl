importScripts('webgl_api.js')

const { Application, Container, ContainerBuilder, Color } = wasm_bindgen;

async function run() {
  await wasm_bindgen('/webgl_api_bg.wasm');

  console.log("WORKER: done");
  Application.add();
  console.log("WORKER: add");

}
run();
/*
self.onmessage = function(event) {

  let modul = event.data;

  console.log("WORKER: on message");
    run().then(response => {
      console.log("WORKER: after run");
      let color = new Color();
      console.log("WORKER: color created!");
      console.log("WORKER: new color: " + color.r);
      let container = new ContainerBuilder().x(2.0).color(color).build();
      console.log("WORKER: new containter: " + container.x);
      let message = JSON.stringify(container);
      console.log("WORKER: sending message: " + message);
      postMessage(message);
  });
}
*/