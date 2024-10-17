use js_sys::Uint8ClampedArray;
use wasm_bindgen::prelude::*;
use web_sys::{DedicatedWorkerGlobalScope, ImageData, OffscreenCanvas};

mod mandelbrot;

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;
const MAX_ITERATIONS: u32 = 1000;

struct ActorServiceImpl;
impl shared::Actor for ActorServiceImpl {
    fn draw(&self, canvas: OffscreenCanvas) -> Result<(), ()> {
        let data = mandelbrot::Generator::new(WIDTH, HEIGHT, MAX_ITERATIONS)
            .iter_bytes()
            .collect::<Vec<u8>>();
        /* convert the generated data to a ImageData via a Uint8ClampedArray */
        let data = Uint8ClampedArray::from(data.as_slice());
        let image_data = ImageData::new_with_js_u8_clamped_array(&data, WIDTH).unwrap();
        
        /* set the width and height and draw the pixels */
        canvas.set_width(WIDTH);
        canvas.set_height(HEIGHT);
        canvas.get_context("2d")
            .unwrap()
            .unwrap()
            .unchecked_into::<web_sys::OffscreenCanvasRenderingContext2d>()
            .put_image_data(&image_data, 0.0, 0.0)
            .unwrap();
        Ok(())
    }
}

#[wasm_bindgen(main)]
pub async fn main() {
    let scope = js_sys::global()
        .dyn_into::<DedicatedWorkerGlobalScope>()
        .expect("Could not get worker scope");
    let main_interface = web_rpc::Interface::new(scope).await;
    let server = web_rpc::Builder::new(main_interface)
        .with_service::<shared::ActorService<_>>(ActorServiceImpl)
        .build();
    server.await;
}
