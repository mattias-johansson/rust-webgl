importScripts('webgl_api.js')

const { Container, Color } = wasm_bindgen;

async function run() {
  await wasm_bindgen('/webgl_api_bg.wasm');

  console.log("done");
}

self.onmessage = function(event) {

  console.log("WORKER: on message");
    run().then(response => {
      console.log("WORKER: after run");
      let modul = event.data;
      let color = new Color();
      console.log("WORKER: color created!");
      console.log("WORKER: new color: " + color.r);
  });
}