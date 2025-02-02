use csv;
use serde::{Deserialize, Serialize};
use std::path::Path;
use vega_lite_4::*;

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub symbol: String,
    pub date: String,
    pub price: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // {
    //   "$schema": "https://vega.github.io/schema/vega-lite/v3.json",
    //   "description": "Google's stock price over time.",
    //   "data": {"url": "data/stocks.csv"},
    //   "transform": [{"filter": "datum.symbol==='GOOG'"}],
    //   "mark": "line",
    //   "encoding": {
    //     "x": {"field": "date", "type": "temporal"},
    //     "y": {"field": "price", "type": "quantitative"}
    //   }
    // }

    // input data: a CSV serialized to a `Vec<Item>`
    let mut rdr = csv::Reader::from_path(Path::new("examples/res/data/stocks.csv"))?;
    let values = rdr
        .deserialize()
        .into_iter()
        .collect::<Result<Vec<Item>, csv::Error>>()?;

    // the chart
    let chart = VegaliteBuilder::default()
        .title("Stock price")
        // .width(400.0)
        // .height(200.0)
        // .padding(Some(Padding::Double(5.0)))
        .description("Google's stock price over time.")
        .data(&values)
        .transform(vec![TransformBuilder::default()
            .filter("datum.symbol==='GOOG'")
            .build()?])
        .mark(Mark::Line)
        .encoding(
            EncodingBuilder::default()
                .x(XClassBuilder::default()
                    .field("date")
                    .def_type(StandardType::Temporal)
                    .build()?)
                .y(YClassBuilder::default()
                    .field("price")
                    .def_type(StandardType::Quantitative)
                    .build()?)
                .build()?,
        )
        .build()?;

    // display the chart using `showata`
    chart.show()?;

    // print the vega lite spec
    eprint!("{}", chart.to_string()?);

    Ok(())
}
