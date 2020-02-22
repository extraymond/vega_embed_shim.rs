mod utils;
use anyhow::Result;
use std::collections::HashMap;
use vega_lite_3::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::*;

#[cfg(feature = "example")]
use example;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn vegaEmbed(target: web_sys::Element, spec: JsValue, option: JsValue) -> js_sys::Promise;
}

/// Render chart onto the web_sys::Element, with optional dict.
pub fn render_chart(
    chart: &Vegalite,
    target: web_sys::Element,
    option: &Option<HashMap<String, String>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let spec = JsValue::from_serde(chart)?;
    log::info!("chart ready");
    let opt = match &option {
        Some(x) => JsValue::from_serde(x)?,
        None => JsValue::from(js_sys::Object::new()),
    };
    let fut: JsFuture = vegaEmbed(target, spec, opt).into();
    spawn_local(async move {
        fut.await.unwrap();
    });

    Ok(())
}
