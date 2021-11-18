importScripts('webgl_api.js')

const { Application, 
  Container, ContainerBuilder, 
  Color,
  ListView, ListViewBuilder } = wasm_bindgen;

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
    let listView = new ListViewBuilder().x(0.0).y(0.0).width(200.0).height(200.0).opacity(1.0).build();
    let color = new Color();
    color.r = 1.0;
    color.g = 1.0;
    color.b = 1.0;
    this.container = new ContainerBuilder().x(0.0).y(0.0).width(400.0).opacity(1.0).height(400.0).color(color).build();
    
    this.container.add_list_view(listView);
    Application.set_root(this.container);
    console.log("WORKER: add");  

    setTimeout(() => { listView.set_content("[" +
    "{\"label\": \"Adam\"}," +
    "{\"label\": \"Bertil\"}," +
    "{\"label\": \"Cesar\"}," +
    "{\"label\": \"David\"}," +
    "{\"label\": \"Erik\"}," +
    "{\"label\": \"Filip\"}," +
    "{\"label\": \"Gustav\"}," +
    "{\"label\": \"Filip\"}," +
    "{\"label\": \"Ivar\"}," +
    "{\"label\": \"Johan\"}," +
    "{\"label\": \"Karl\"}," +
    "{\"label\": \"Ludvig\"}," +
    "{\"label\": \"Martin\"}," +
    "{\"label\": \"Niklas\"}," +
    "{\"label\": \"Olof\"}," +
    "{\"label\": \"Petter\"}," +
    "{\"label\": \"Qvintus\"}," +
    "{\"label\": \"Rudolf\"}," +
    "{\"label\": \"Sigurd\"}," +
    "{\"label\": \"Tore\"}," +
    "{\"label\": \"Urban\"}," +
    "{\"label\": \"Viktor\"}," +
    "{\"label\": \"Wilhelm\"}," +
    "{\"label\": \"Xerxes\"}," +
    "{\"label\": \"Yngve\"}," +
    "{\"label\": \"Zäta\"}," +
    "{\"label\": \"Åke\"}," +
    "{\"label\": \"Ärlig\"}," +
    "{\"label\": \"Östen\"}]"); }, 2000);
  }

}
