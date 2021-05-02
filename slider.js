importScripts('webgl_api.js')

const { Application, Container, ContainerBuilder, Color, imageView, ImageViewBuilder, Slider, SliderBuilder, Label, LabelBuilder  } = wasm_bindgen;

let app;

async function run() {
  await wasm_bindgen('/webgl_api_bg.wasm');
  app = new App();
  console.log("WORKER: done");
}

log = function(value) {
//  console.log("WORKER: on click! " + value);
  app.label.text(value);
  app.imageView.set_opacity(value);
  app.imageView.set_x(value * 400);
  app.imageView.set_y(value * 400);
  app.imageView.set_width(700 - value * 400);
  app.imageView.set_height(700 - value * 400);
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
    this.imageView = new ImageViewBuilder().x(10.0).y(100.0).width(700.0).height(700.0).image("IMG_20160408_164451.jpg").opacity(0.0).build();
    let button = new SliderBuilder().x(10.0).y(10.0).text("Knapp").build();
    this.container.add_slider(button);
    this.label = new LabelBuilder().x(10.0).y(35.0).opacity(0.0).text("0").build();
    this.container.add_label(this.label);
    this.container.add_image_view(this.imageView);
    Application.set_root(this.container);
    this.application.add_slider_listener(button, "ImmediateValue", log);
    console.log("WORKER: add");  
  }


}
