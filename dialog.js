importScripts('webgl_api.js')

const { Application, Container, ContainerBuilder, Color, Label, LabelBuilder, ButtonBuilder, Button } = wasm_bindgen;

let app;

async function run() {
  await wasm_bindgen('/webgl_api_bg.wasm');
  if (app === undefined) {
    app = new App();
  }
  console.log("WORKER: done");
}

run();

closeDialog = function(value) {
  console.log("WORKER: closeDialog! " + value);
  app.container.remove_container(app.dialog);
}

openDialog = function(value) {
  console.log("WORKER: openDialog! " + value);
  let color = new Color();
  color.r = 0.8;
  color.g = 0.8;
  color.b = 0.8;
  let dialog = new ContainerBuilder().x(150.0).y(150.0).width(300.0).opacity(1.0).height(300.0).color(color).build();
  let button = new ButtonBuilder().x(30.0).y(30.0).text("Close dialog").build();
  dialog.add_button(button);
  app.dialog = dialog;
  app.container.add_container(dialog);
  app.application.add_listener(button, "onClicked", closeDialog);
}

class App {
  constructor() {
    this.application = new Application();
    let color = new Color();
    color.r = 1.0;
    color.g = 1.0;
    color.b = 1.0;
    this.container = new ContainerBuilder().x(0.0).y(0.0).width(700.0).opacity(1.0).height(700.0).color(color).build();
    let button = new ButtonBuilder().text("Open dialog").build();
    this.container.add_button(button);
    this.application.add_listener(button, "onClicked", openDialog);
    Application.set_root(this.container);
    console.log("WORKER: add");  
  }

}
