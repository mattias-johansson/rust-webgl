importScripts('webgl_api.js')

const { Application, Container, ContainerBuilder, Color, Button, ButtonBuilder} = wasm_bindgen;

async function run() {
  await wasm_bindgen('/webgl_api_bg.wasm');

  console.log("WORKER: done");
  let color = new Color();
  let container = new ContainerBuilder().x(300.0).y(300.0).width(400.0).opacity(1.0).height(400.0).color(color).build();
  let button = new ButtonBuilder().x(0.0).y(0.0).text("Knapp").build();
  container.add(button);
  Application.set_root(container);
  console.log("WORKER: add");

}
run();
