use web_sys::OffscreenCanvas;

#[web_rpc::service]
pub trait Actor {
    #[post(transfer(canvas))]
    fn draw(canvas: OffscreenCanvas) -> Result<(), ()>;
}