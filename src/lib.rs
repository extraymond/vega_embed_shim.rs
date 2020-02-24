mod utils;
use anyhow::Result;
use futures_timer::Delay;
use std::collections::HashMap;
use std::time::Duration;
use vega_lite_3::*;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::*;

#[cfg(feature = "example")]
mod example;
#[cfg(feature = "example")]
use example::call_vega;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &JsValue);
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn vegaEmbed(target: web_sys::Element, spec: JsValue, option: JsValue) -> js_sys::Promise;
}

/// Render chart onto the web_sys::Element, with optional dict, allow resize if a container web_sys::Element is provided.
pub fn render_chart(
    chart: &Vegalite,
    target: web_sys::Element,
    option: &Option<HashMap<String, String>>,
    watch_resize: Option<web_sys::Element>,
) -> Result<(), Box<dyn std::error::Error>> {
    let spec = JsValue::from_serde(chart)?;
    let opt = match &option {
        Some(x) => JsValue::from_serde(x)?,
        None => JsValue::from(js_sys::Object::new()),
    };
    let fut: JsFuture = vegaEmbed(target, spec, opt).into();

    if let Some(target) = watch_resize {
        let target: web_sys::HtmlElement = target.unchecked_into();
        spawn_local(async move {
            let res = fut.await.unwrap();
            let view = js_sys::Reflect::get(&res, &JsValue::from_str("view")).unwrap();
            let mut dimension = [0_i32; 2];
            let width = js_sys::Function::from(
                js_sys::Reflect::get(&view, &JsValue::from_str("width")).unwrap(),
            );
            let height = js_sys::Function::from(
                js_sys::Reflect::get(&view, &JsValue::from_str("height")).unwrap(),
            );
            let run = js_sys::Function::from(
                js_sys::Reflect::get(&view, &JsValue::from_str("run")).unwrap(),
            );
            loop {
                Delay::new(Duration::from_millis(100)).await;
                let new_dimension = [target.offset_width(), target.offset_height()];
                if (dimension != new_dimension) && new_dimension != [0, 0] {
                    dimension = new_dimension;
                    width.call1(&view, &JsValue::from(dimension[0])).unwrap();
                    height.call1(&view, &JsValue::from(dimension[1])).unwrap();
                    run.call0(&view).unwrap();
                }
            }
        });
    }

    Ok(())
}
