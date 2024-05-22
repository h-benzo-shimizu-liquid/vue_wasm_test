use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use js_sys::Array;
use js_sys::Function;
use js_sys::Promise;
use web_sys::console;
use web_sys::window;
use web_sys::Window;
use web_sys::Document;
use web_sys::Navigator;
use web_sys::Element;
use web_sys::HtmlVideoElement;
use web_sys::HtmlCanvasElement;
use web_sys::CanvasRenderingContext2d;
use web_sys::MediaDevices;
use web_sys::MediaStream;
use web_sys::MediaStreamConstraints;

#[wasm_bindgen]
pub fn hello_world(value: &str) {
  let value: String = format!("Hello, {}!", value);
  let value: JsValue = JsValue::from_str(&value);
  let value: Array = Array::from(&value);
  console::log(&value);
}

#[wasm_bindgen]
pub struct WasmCamera {
  #[wasm_bindgen(skip)]
  pub video: Option<HtmlVideoElement>,
}

#[wasm_bindgen]
impl WasmCamera {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    Self {
      video: None,
    }
  }

  pub async fn init(&mut self) -> Result<(), JsValue> {
    let window: Window = window().ok_or(JsValue::from_str("failed"))?;
    let document: Document = window.document().ok_or(JsValue::from_str("failed"))?;

    let mut media_constraints: MediaStreamConstraints = MediaStreamConstraints::new();
    media_constraints.audio(&JsValue::FALSE);
    media_constraints.video(&JsValue::TRUE);

    let navigator: Navigator = window.navigator();
    let media_devices: MediaDevices = navigator.media_devices()?;
    let stream_promise: Promise = media_devices.get_user_media_with_constraints(&media_constraints)?;
    let stream: JsValue = JsFuture::from(stream_promise).await?;
    let stream: MediaStream = stream.dyn_into::<MediaStream>()?;

    let video: Element = document.create_element("video")?;
    let video: HtmlVideoElement = video.dyn_into::<HtmlVideoElement>()?;
    JsFuture::from(Promise::new(&mut |resolve: Function, _reject: Function| {
      video.add_event_listener_with_callback("loadedmetadata", &resolve).unwrap();
      video.set_src_object(Option::Some(&stream));
    })).await?;

    JsFuture::from(video.play()?).await?;

    self.video = Some(video);
    return Result::Ok(());
  }

  pub fn draw(&self, canvas: HtmlCanvasElement, context: CanvasRenderingContext2d) -> Result<(), JsValue> {
    let video: &HtmlVideoElement = self.video.as_ref().ok_or(JsValue::from_str("failed"))?;
    let srcw: f64 = video.video_width().into();
    let srch: f64 = video.video_height().into();
    let dstw: f64 = canvas.width().into();
    let dsth: f64 = canvas.height().into();
    context.draw_image_with_html_video_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(&video, 0.0, 0.0, srcw, srch, 0.0, 0.0, dstw, dsth)?;
    return Result::Ok(());
  }
}
