importScripts('webgl_api.js')

const { Application, Container, ContainerBuilder, Color, Button, ButtonBuilder } = wasm_bindgen;

let app;

async function run() {
  await wasm_bindgen('/webgl_api_bg.wasm');
  app = new App();
  console.log("WORKER: done");
}

log = function(value) {
  console.log("WORKER: on click! " + value);
};

addButton = function(value) {
  console.log("WORKER: addButton! " + value);
  let button = new ButtonBuilder().x(150.0).y(30.0).text("New button").build();
  app.container.add_button(button);
}

run();

class App {
  constructor() {
    this.application = new Application();
    let color = new Color();
    color.r = 1.0;
    color.g = 1.0;
    color.b = 1.0;
    this.container = new ContainerBuilder().x(300.0).y(300.0).width(400.0).opacity(1.0).height(400.0).color(color).build();
    let button = new ButtonBuilder().x(0.0).y(0.0).text("Button").build();
    this.container.add_button(button);
    Application.set_root(this.container);
    this.application.add_listener(button, "onClicked", log);
    this.application.add_listener(button, "onClicked", addButton);  
    console.log("WORKER: add");  
  }


}
