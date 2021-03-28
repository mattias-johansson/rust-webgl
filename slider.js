importScripts('webgl_api.js')

const { Application, Container, ContainerBuilder, Color, Slider, SliderBuilder, Label, LabelBuilder  } = wasm_bindgen;

let app;

async function run() {
  await wasm_bindgen('/webgl_api_bg.wasm');
  app = new App();
  console.log("WORKER: done");
}

log = function(value) {
  console.log("WORKER: on click! " + value);
  app.label.text(value);
};

run();

class App {
  constructor() {
    this.application = new Application();
    let color = new Color();
    color.r = 1.0;
    color.g = 1.0;
    color.b = 1.0;
    this.container = new ContainerBuilder().x(0.0).y(0.0).width(700.0).opacity(1.0).height(700.0).color(color).build();
    let button = new SliderBuilder().x(10.0).y(10.0).text("Knapp").build();
    this.container.add_slider(button);
    this.label = new LabelBuilder().x(10.0).y(50.0).text("0").build();
    this.container.add_label(this.label);
    Application.set_root(this.container);
    this.application.add_slider_listener(button, "ImmediateValue", log);
    console.log("WORKER: add");  
  }


}
