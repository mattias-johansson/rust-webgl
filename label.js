importScripts('webgl_api.js')

const { Application, Container, ContainerBuilder, Color, Label, LabelBuilder } = wasm_bindgen;

let app;

async function run() {
  await wasm_bindgen('/webgl_api_bg.wasm');
  app = new App();
  console.log("WORKER: done");
}

run();

class App {
  constructor() {
    this.application = new Application();
    let color = new Color();
    color.r = 1.0;
    color.g = 1.0;
    color.b = 1.0;
    this.container = new ContainerBuilder().x(0.0).y(0.0).width(700.0).opacity(1.0).height(700.0).color(color).build();
    let label = new LabelBuilder().x(10.0).y(10.0).text("This is a label").build();
    this.container.add_label(label);
    Application.set_root(this.container);
    console.log("WORKER: add");  
  }

  log = function(value) {
    console.log("WORKER: on click! " + value);
  };
  
  addButton = function(value) {
    console.log("WORKER: addButton! " + value);
    let button = new ButtonBuilder().x(150.0).y(30.0).text("Ny knapp").build();
    app.container.add_button(button);
  }
}
