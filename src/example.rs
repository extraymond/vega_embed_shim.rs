use crate::render_chart;
use std::collections::HashMap;
use vega_lite_3::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn call_vega() {
    femme::start(log::LevelFilter::Info).unwrap();
    let doc = web_sys::window().unwrap().document().unwrap();
    let target = doc.get_element_by_id("viz").unwrap();

    if let Ok(chart) = gen_chart() {
        let mut option = HashMap::<String, String>::new();
        option.insert("renderer".to_string(), "svg".to_string());
        render_chart(&chart, target, &Some(option), Some("container"));
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
