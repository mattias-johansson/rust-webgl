importScripts('webgl_api.js')

const { Application, Container, ContainerBuilder, Color, ImageView, ImageViewBuilder, Button, ButtonBuilder } = wasm_bindgen;

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
    let imageView = new ImageViewBuilder().x(10.0).y(10.0).width(700.0).height(700.0).image("IMG_20160408_164451.jpg").opacity(1.0).build();
    let button = new ButtonBuilder().text("repaint").build();
    this.container.add_button(button);
    this.container.add_image_view(imageView);
    Application.set_root(this.container);
    console.log("WORKER: add");  
  }
}
