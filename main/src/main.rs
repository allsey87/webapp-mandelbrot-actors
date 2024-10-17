
use dominator::{html, with_node};
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

#[allow(non_snake_case)]
#[wasm_bindgen(inline_js = "
export function newActor() {
    return new Worker(new URL('../../actor.js', import.meta.url), {
        type: 'module'
    });
}
"
)]
extern "C" {
    #[wasm_bindgen(js_name = newActor, catch)]
    pub fn actor() -> Result<web_sys::Worker, JsValue>;
}

#[wasm_bindgen(main)]
pub async fn main() {
    let actor_worker = actor().expect("could not create actor worker");
    let actor_interface = web_rpc::Interface::new(actor_worker).await;
    let actor_client = web_rpc::Builder::new(actor_interface)
        .with_client::<shared::ActorClient>()
        .build();

    /* create the html for our page */
    dominator::append_dom(&dominator::body(), html!("div", {
        .child(html!("h1", { .text("Mandelbrot fractal") }))
        .child(html!("canvas" => HtmlCanvasElement, {
            .with_node!(canvas => {
                /* future polls a impl Future<Output = T> while the node is added to the
                   DOM. Under the hood, it is using wasm_bindgen_futures::spawn_local */
                .future(async move {
                    let offscreen_canvas = canvas.transfer_control_to_offscreen()
                        .expect("could not transfer control to offscreen");
                    actor_client.draw(offscreen_canvas).await
                        .expect("could not draw to canvas");
                    /* note: at this point actor_client is dropped, terminating the worker */
                })
            })
        }))
    }));
}
