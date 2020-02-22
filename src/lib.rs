mod utils;
use anyhow::Result;
use std::collections::HashMap;
use vega_lite_3::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn vegaEmbed(target: web_sys::Element, spec: JsValue, option: JsValue) -> js_sys::Promise;
}

pub fn render_chart(
    chart: &Vegalite,
    target: web_sys::Element,
    option: &Option<HashMap<&str, &str>>,
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

// #[wasm_bindgen(start)]
// pub fn main() {
// }

#[wasm_bindgen]
pub fn call_vega() {
    femme::start(log::LevelFilter::Info).unwrap();
    let doc = web_sys::window().unwrap().document().unwrap();
    let target = doc.get_element_by_id("viz").unwrap();

    if let Ok(chart) = gen_chart() {
        let mut option = HashMap::<&str, &str>::new();
        option.insert("renderer", "svg");
        render_chart(&chart, target, &Some(option));
    }
}

pub fn gen_chart() -> Result<Vegalite, Box<dyn std::error::Error>> {
    // the chart
    let chart = VegaliteBuilder::default()
        .title("Choropleth of Unemployment Rate per County")
        .data(
            UrlDataBuilder::default()
                .url("https://raw.githubusercontent.com/vega/vega-datasets/master/data/us-10m.json")
                .format(
                    DataFormatBuilder::default()
                        .data_format_type(DataFormatType::Topojson)
                        .feature("counties")
                        .build()?,
                )
                .build()?,
        )
        .mark(Mark::Geoshape)
        .transform(vec![TransformBuilder::default()
            .lookup("id")
            .from(LookupDataBuilder::default()
                .data(DataBuilder::default()
                    .url("https://raw.githubusercontent.com/vega/vega-datasets/master/data/unemployment.tsv")
                    .build()?)
                .key("id")
                .fields(vec!["rate".to_string()])
                .build()?)
            .build()?])
        .projection(ProjectionBuilder::default().projection_type(ProjectionType::AlbersUsa).build()?)
        .encoding(
            EncodingBuilder::default()
                .color(
                    DefWithConditionMarkPropFieldDefStringNullBuilder::default()
                        .field("rate")
                        .def_with_condition_mark_prop_field_def_string_null_type(StandardType::Quantitative)
                        .build()?,
                )
                .build()?,
        )
        .build()?;
    Ok(chart)
}
