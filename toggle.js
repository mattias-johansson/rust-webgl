importScripts('webgl_api.js')

const { Application, Container, ContainerBuilder, Color, ToggleButton, ToggleButtonBuilder } = wasm_bindgen;

let app;

async function run() {
  await wasm_bindgen('/webgl_api_bg.wasm');
  app = new App();
  console.log("WORKER: done");
}
  
addButton = function(value) {
  console.log("WORKER: addButton! " + value);
  if (value == "true") {
    app.number_of_buttons++;
    y = 10 + app.number_of_buttons * 40;
    let button = new ToggleButtonBuilder().x(10.0).y(y).text("Knapp").build();
    app.container.add_toggle_button(button);
  }
}

run();

class App {
  constructor() {
    this.application = new Application();
    let color = new Color();
    color.r = 1.0;
    color.g = 1.0;
    color.b = 1.0;
    this.number_of_buttons = 0;
    this.container = new ContainerBuilder().x(0.0).y(0.0).width(700.0).opacity(1.0).height(700.0).color(color).build();
    let button = new ToggleButtonBuilder().x(10.0).y(10.0).text("Knapp").build();
    this.container.add_toggle_button(button);
    Application.set_root(this.container);
    console.log("WORKER: add");  
    this.application.add_tb_listener(button, "checked", addButton);  
  }

}
