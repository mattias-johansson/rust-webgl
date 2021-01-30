importScripts('webgl_api.js')

const { Application, 
  Color, 
  Container, ContainerBuilder, 
  ImageView, ImageViewBuilder, 
  ScrollView, ScrollViewBuilder } = wasm_bindgen;

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
    let color2 = new Color();
    color2.r = 1.0;
    color2.g = 1.0;
    color2.b = 1.0;
    let color = new Color();
    color.r = 1.0;
    color.g = 1.0;
    color.b = 1.0;
    let scroller = new ScrollViewBuilder().width(200.0).height(200.0).build();
    this.container = new ContainerBuilder().x(0.0).y(0.0).width(400.0).opacity(1.0).height(400.0).color(color).build();
    this.container2 = new ContainerBuilder().translate_x(-100.0).translate_y(-100.0).width(700.0).opacity(1.0).height(700.0).color(color2).build();
    this.imageView = new ImageViewBuilder().x(0.0).y(0.0).width(700.0).height(700.0).image("IMG_20160408_164451.jpg").opacity(1.0).build();

    this.container2.add_image_view(this.imageView);
    scroller.set_content(this.container2);
    this.container.add_scroll_view(scroller);
    Application.set_root(this.container);
    console.log("WORKER: add");  
  }


}
