// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(missing_docs, clippy::large_enum_variant)]


use crate::removable_value::RemovableValue;
use derive_builder::Builder;
use derive_more::From;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A Vega-Lite top-level specification.
/// This is the root class for all Vega-Lite specifications.
/// (The json schema is generated from this type.)
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct Vegalite {
    /// URL to [JSON schema](http://json-schema.org/) for a Vega-Lite specification. Unless you
    /// have a reason to change this, use `https://vega.github.io/schema/vega-lite/v4.json`.
    /// Setting the `$schema` property allows automatic validation and autocomplete in editors
    /// that support JSON schema.
    #[serde(rename = "$schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        default = "Some(\"https://vega.github.io/schema/vega-lite/v4.0.2.json\".to_string())"
    )]
    pub schema: Option<String>,
    /// How the visualization size should be determined. If a string, should be one of `"pad"`,
    /// `"fit"` or `"none"`.
    /// Object values can additionally specify parameters for content sizing and automatic
    /// resizing.
    ///
    /// __Default value__: `pad`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub autosize: Option<Autosize>,
    /// CSS color property to use as the background of the entire view.
    ///
    /// __Default value:__ `"white"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub background: Option<String>,
    /// The bounds calculation method to use for determining the extent of a sub-plot. One of
    /// `full` (the default) or `flush`.
    ///
    /// - If set to `full`, the entire calculated bounds (including axes, title, and legend) will
    /// be used.
    /// - If set to `flush`, only the specified width and height values for the sub-view will be
    /// used. The `flush` setting can be useful when attempting to place sub-plots without axes
    /// or legends into a uniform grid structure.
    ///
    /// __Default value:__ `"full"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bounds: Option<BoundsEnum>,
    /// Vega-Lite configuration object. This property can only be defined at the top-level of a
    /// specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub config: Option<Config>,
    /// An object describing the data source. Set to `null` to ignore the parent's data source.
    /// If no data is set, it is derived from the parent.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub data: RemovableValue<UrlData>,
    /// A global data store for named datasets. This is a mapping from names to inline datasets.
    /// This can be an array of objects or primitive values or a string. Arrays of primitive
    /// values are ingested as objects with a `data` property.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub datasets: Option<HashMap<String, InlineDatasetValue>>,
    /// Description of this mark for commenting purpose.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub description: Option<String>,
    /// A key-value mapping between encoding channels and definition of fields.
    ///
    /// A shared key-value mapping between encoding channels and definition of fields in the
    /// underlying layers.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub encoding: Option<Encoding>,
    /// The height of a visualization.
    ///
    /// - For a plot with a continuous y-field, height should be a number.
    /// - For a plot with either a discrete y-field or no y-field, height can be either a number
    /// indicating a fixed height or an object in the form of `{step: number}` defining the
    /// height per discrete step. (No y-field is equivalent to having one discrete step.)
    /// - To enable responsive sizing on height, it should be set to `"container"`.
    ///
    /// __Default value:__ Based on `config.view.continuousHeight` for a plot with a continuous
    /// y-field and `config.view.discreteHeight` otherwise.
    ///
    /// __Note:__ For plots with [`row` and `column`
    /// channels](https://vega.github.io/vega-lite/docs/encoding.html#facet), this represents the
    /// height of a single view and the `"container"` option cannot be used.
    ///
    /// __See also:__ [`height`](https://vega.github.io/vega-lite/docs/size.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub height: Option<HeightUnion>,
    /// A string describing the mark type (one of `"bar"`, `"circle"`, `"square"`, `"tick"`,
    /// `"line"`,
    /// `"area"`, `"point"`, `"rule"`, `"geoshape"`, and `"text"`) or a [mark definition
    /// object](https://vega.github.io/vega-lite/docs/mark.html#mark-def).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub mark: Option<AnyMark>,
    /// Name of the visualization for later reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub name: Option<String>,
    /// The default visualization padding, in pixels, from the edge of the visualization canvas
    /// to the data rectangle. If a number, specifies padding for all sides.
    /// If an object, the value should have the format `{"left": 5, "top": 5, "right": 5,
    /// "bottom": 5}` to specify padding for each side of the visualization.
    ///
    /// __Default value__: `5`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub padding: Option<Padding>,
    /// An object defining properties of geographic projection, which will be applied to `shape`
    /// path for `"geoshape"` marks
    /// and to `latitude` and `"longitude"` channels for other marks.
    ///
    /// An object defining properties of the geographic projection shared by underlying layers.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub projection: Option<Projection>,
    /// Scale, axis, and legend resolutions for view composition specifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub resolve: Option<Resolve>,
    /// A key-value mapping between selection names and definitions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub selection: Option<HashMap<String, SelectionDef>>,
    /// Title for the plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title: Option<Text>,
    /// An array of data transformations such as filter and new field calculation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub transform: Option<Vec<Transform>>,
    /// Optional metadata that will be passed to Vega.
    /// This object is completely ignored by Vega and Vega-Lite and can be used for custom
    /// metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub usermeta: Option<HashMap<String, Option<serde_json::Value>>>,
    /// An object defining the view background's fill and stroke.
    ///
    /// __Default value:__ none (transparent)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub view: Option<ViewBackground>,
    /// The width of a visualization.
    ///
    /// - For a plot with a continuous x-field, width should be a number.
    /// - For a plot with either a discrete x-field or no x-field, width can be either a number
    /// indicating a fixed width or an object in the form of `{step: number}` defining the width
    /// per discrete step. (No x-field is equivalent to having one discrete step.)
    /// - To enable responsive sizing on width, it should be set to `"container"`.
    ///
    /// __Default value:__
    /// Based on `config.view.continuousWidth` for a plot with a continuous x-field and
    /// `config.view.discreteWidth` otherwise.
    ///
    /// __Note:__ For plots with [`row` and `column`
    /// channels](https://vega.github.io/vega-lite/docs/encoding.html#facet), this represents the
    /// width of a single view and the `"container"` option cannot be used.
    ///
    /// __See also:__ [`width`](https://vega.github.io/vega-lite/docs/size.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub width: Option<HeightUnion>,
    /// The alignment to apply to grid rows and columns.
    /// The supported string values are `"all"`, `"each"`, and `"none"`.
    ///
    /// - For `"none"`, a flow layout will be used, in which adjacent subviews are simply placed
    /// one after the other.
    /// - For `"each"`, subviews will be aligned into a clean grid structure, but each row or
    /// column may be of variable size.
    /// - For `"all"`, subviews will be aligned and each row or column will be sized identically
    /// based on the maximum observed size. String values for this property will be applied to
    /// both grid rows and columns.
    ///
    /// Alternatively, an object value of the form `{"row": string, "column": string}` can be
    /// used to supply different alignments for rows and columns.
    ///
    /// __Default value:__ `"all"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub align: Option<AlignUnion>,
    /// Boolean flag indicating if subviews should be centered relative to their respective rows
    /// or columns.
    ///
    /// An object value of the form `{"row": boolean, "column": boolean}` can be used to supply
    /// different centering values for rows and columns.
    ///
    /// __Default value:__ `false`
    ///
    /// Boolean flag indicating if subviews should be centered relative to their respective rows
    /// or columns.
    ///
    /// __Default value:__ `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub center: Option<VegaliteCenter>,
    /// The number of columns to include in the view composition layout.
    ///
    /// __Default value__: `undefined` -- An infinite number of columns (a single row) will be
    /// assumed. This is equivalent to
    /// `hconcat` (for `concat`) and to using the `column` channel (for `facet` and `repeat`).
    ///
    /// __Note__:
    ///
    /// 1) This property is only for:
    /// - the general (wrappable) `concat` operator (not `hconcat`/`vconcat`)
    /// - the `facet` and `repeat` operator with one field/repetition definition (without
    /// row/column nesting)
    ///
    /// 2) Setting the `columns` to `1` is equivalent to `vconcat` (for `concat`) and to using
    /// the `row` channel (for `facet` and `repeat`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub columns: Option<f64>,
    /// Definition for how to facet the data. One of:
    /// 1) [a field definition for faceting the plot by one
    /// field](https://vega.github.io/vega-lite/docs/facet.html#field-def)
    /// 2) [An object that maps `row` and `column` channels to their field
    /// definitions](https://vega.github.io/vega-lite/docs/facet.html#mapping)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub facet: Option<Facet>,
    /// The spacing in pixels between sub-views of the composition operator.
    /// An object of the form `{"row": number, "column": number}` can be used to set
    /// different spacing values for rows and columns.
    ///
    /// __Default value__: Depends on `"spacing"` property of [the view composition
    /// configuration](https://vega.github.io/vega-lite/docs/config.html#view-config) (`20` by
    /// default)
    ///
    /// The spacing in pixels between sub-views of the concat operator.
    ///
    /// __Default value__: `10`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub spacing: Option<Spacing>,
    /// A specification of the view that gets faceted.
    ///
    /// A specification of the view that gets repeated.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub spec: Option<SpecClass>,
    /// Layer or single view specifications to be layered.
    ///
    /// __Note__: Specifications inside `layer` cannot use `row` and `column` channels as
    /// layering facet specifications is not allowed. Instead, use the [facet
    /// operator](https://vega.github.io/vega-lite/docs/facet.html) and place a layer inside a
    /// facet.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub layer: Option<Vec<LayerSpec>>,
    /// Definition for fields to be repeated. One of:
    /// 1) An array of fields to be repeated. If `"repeat"` is an array, the field can be
    /// referred using `{"repeat": "repeat"}`
    /// 2) An object that mapped `"row"` and/or `"column"` to the listed of fields to be repeated
    /// along the particular orientations. The objects `{"repeat": "row"}` and `{"repeat":
    /// "column"}` can be used to refer to the repeated field respectively.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub repeat: Option<RepeatUnion>,
    /// A list of views to be concatenated.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub concat: Option<Vec<Spec>>,
    /// A list of views to be concatenated and put into a column.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub vconcat: Option<Vec<Spec>>,
    /// A list of views to be concatenated and put into a row.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub hconcat: Option<Vec<Spec>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct RowColLayoutAlign {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub column: Option<LayoutAlign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub row: Option<LayoutAlign>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct AutoSizeParams {
    /// Determines how size calculation should be performed, one of `"content"` or `"padding"`.
    /// The default setting (`"content"`) interprets the width and height settings as the data
    /// rectangle (plotting) dimensions, to which padding is then added. In contrast, the
    /// `"padding"` setting includes the padding within the view size calculations, such that the
    /// width and height settings indicate the **total** intended size of the view.
    ///
    /// __Default value__: `"content"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub contains: Option<Contains>,
    /// A boolean flag indicating if autosize layout should be re-calculated on every view
    /// update.
    ///
    /// __Default value__: `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub resize: Option<bool>,
    /// The sizing format type. One of `"pad"`, `"fit"`, `"fit-x"`, `"fit-y"`,  or `"none"`. See
    /// the [autosize type](https://vega.github.io/vega-lite/docs/size.html#autosize)
    /// documentation for descriptions of each.
    ///
    /// __Default value__: `"pad"`
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub auto_size_params_type: Option<AutosizeType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct RowColBoolean {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub column: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub row: Option<bool>,
}

/// A specification of the view that gets faceted.
///
/// A full layered plot specification, which may contains `encoding` and `projection`
/// properties that will be applied to underlying unit (single-view) specifications.
///
/// Unit spec that can have a composite mark and row or column channels (shorthand for a
/// facet spec).
///
/// A specification of the view that gets repeated.
///
/// Any specification in Vega-Lite.
///
/// Base interface for a facet specification.
///
/// Base interface for a repeat specification.
///
/// Base interface for a generalized concatenation specification.
///
/// Base interface for a vertical concatenation specification.
///
/// Base interface for a horizontal concatenation specification.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct SpecClass {
    /// An object describing the data source. Set to `null` to ignore the parent's data source.
    /// If no data is set, it is derived from the parent.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub data: RemovableValue<UrlData>,
    /// Description of this mark for commenting purpose.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub description: Option<String>,
    /// A shared key-value mapping between encoding channels and definition of fields in the
    /// underlying layers.
    ///
    /// A key-value mapping between encoding channels and definition of fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub encoding: Option<Encoding>,
    /// The height of a visualization.
    ///
    /// - For a plot with a continuous y-field, height should be a number.
    /// - For a plot with either a discrete y-field or no y-field, height can be either a number
    /// indicating a fixed height or an object in the form of `{step: number}` defining the
    /// height per discrete step. (No y-field is equivalent to having one discrete step.)
    /// - To enable responsive sizing on height, it should be set to `"container"`.
    ///
    /// __Default value:__ Based on `config.view.continuousHeight` for a plot with a continuous
    /// y-field and `config.view.discreteHeight` otherwise.
    ///
    /// __Note:__ For plots with [`row` and `column`
    /// channels](https://vega.github.io/vega-lite/docs/encoding.html#facet), this represents the
    /// height of a single view and the `"container"` option cannot be used.
    ///
    /// __See also:__ [`height`](https://vega.github.io/vega-lite/docs/size.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub height: Option<HeightUnion>,
    /// Layer or single view specifications to be layered.
    ///
    /// __Note__: Specifications inside `layer` cannot use `row` and `column` channels as
    /// layering facet specifications is not allowed. Instead, use the [facet
    /// operator](https://vega.github.io/vega-lite/docs/facet.html) and place a layer inside a
    /// facet.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub layer: Option<Vec<LayerSpec>>,
    /// Name of the visualization for later reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub name: Option<String>,
    /// An object defining properties of the geographic projection shared by underlying layers.
    ///
    /// An object defining properties of geographic projection, which will be applied to `shape`
    /// path for `"geoshape"` marks
    /// and to `latitude` and `"longitude"` channels for other marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub projection: Option<Projection>,
    /// Scale, axis, and legend resolutions for view composition specifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub resolve: Option<Resolve>,
    /// Title for the plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title: Option<Text>,
    /// An array of data transformations such as filter and new field calculation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub transform: Option<Vec<Transform>>,
    /// An object defining the view background's fill and stroke.
    ///
    /// __Default value:__ none (transparent)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub view: Option<ViewBackground>,
    /// The width of a visualization.
    ///
    /// - For a plot with a continuous x-field, width should be a number.
    /// - For a plot with either a discrete x-field or no x-field, width can be either a number
    /// indicating a fixed width or an object in the form of `{step: number}` defining the width
    /// per discrete step. (No x-field is equivalent to having one discrete step.)
    /// - To enable responsive sizing on width, it should be set to `"container"`.
    ///
    /// __Default value:__
    /// Based on `config.view.continuousWidth` for a plot with a continuous x-field and
    /// `config.view.discreteWidth` otherwise.
    ///
    /// __Note:__ For plots with [`row` and `column`
    /// channels](https://vega.github.io/vega-lite/docs/encoding.html#facet), this represents the
    /// width of a single view and the `"container"` option cannot be used.
    ///
    /// __See also:__ [`width`](https://vega.github.io/vega-lite/docs/size.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub width: Option<HeightUnion>,
    /// The bounds calculation method to use for determining the extent of a sub-plot. One of
    /// `full` (the default) or `flush`.
    ///
    /// - If set to `full`, the entire calculated bounds (including axes, title, and legend) will
    /// be used.
    /// - If set to `flush`, only the specified width and height values for the sub-view will be
    /// used. The `flush` setting can be useful when attempting to place sub-plots without axes
    /// or legends into a uniform grid structure.
    ///
    /// __Default value:__ `"full"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bounds: Option<BoundsEnum>,
    /// A string describing the mark type (one of `"bar"`, `"circle"`, `"square"`, `"tick"`,
    /// `"line"`,
    /// `"area"`, `"point"`, `"rule"`, `"geoshape"`, and `"text"`) or a [mark definition
    /// object](https://vega.github.io/vega-lite/docs/mark.html#mark-def).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub mark: Option<AnyMark>,
    /// A key-value mapping between selection names and definitions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub selection: Option<HashMap<String, SelectionDef>>,
    /// The alignment to apply to grid rows and columns.
    /// The supported string values are `"all"`, `"each"`, and `"none"`.
    ///
    /// - For `"none"`, a flow layout will be used, in which adjacent subviews are simply placed
    /// one after the other.
    /// - For `"each"`, subviews will be aligned into a clean grid structure, but each row or
    /// column may be of variable size.
    /// - For `"all"`, subviews will be aligned and each row or column will be sized identically
    /// based on the maximum observed size. String values for this property will be applied to
    /// both grid rows and columns.
    ///
    /// Alternatively, an object value of the form `{"row": string, "column": string}` can be
    /// used to supply different alignments for rows and columns.
    ///
    /// __Default value:__ `"all"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub align: Option<AlignUnion>,
    /// Boolean flag indicating if subviews should be centered relative to their respective rows
    /// or columns.
    ///
    /// An object value of the form `{"row": boolean, "column": boolean}` can be used to supply
    /// different centering values for rows and columns.
    ///
    /// __Default value:__ `false`
    ///
    /// Boolean flag indicating if subviews should be centered relative to their respective rows
    /// or columns.
    ///
    /// __Default value:__ `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub center: Option<VegaliteCenter>,
    /// The number of columns to include in the view composition layout.
    ///
    /// __Default value__: `undefined` -- An infinite number of columns (a single row) will be
    /// assumed. This is equivalent to
    /// `hconcat` (for `concat`) and to using the `column` channel (for `facet` and `repeat`).
    ///
    /// __Note__:
    ///
    /// 1) This property is only for:
    /// - the general (wrappable) `concat` operator (not `hconcat`/`vconcat`)
    /// - the `facet` and `repeat` operator with one field/repetition definition (without
    /// row/column nesting)
    ///
    /// 2) Setting the `columns` to `1` is equivalent to `vconcat` (for `concat`) and to using
    /// the `row` channel (for `facet` and `repeat`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub columns: Option<f64>,
    /// Definition for how to facet the data. One of:
    /// 1) [a field definition for faceting the plot by one
    /// field](https://vega.github.io/vega-lite/docs/facet.html#field-def)
    /// 2) [An object that maps `row` and `column` channels to their field
    /// definitions](https://vega.github.io/vega-lite/docs/facet.html#mapping)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub facet: Option<Facet>,
    /// The spacing in pixels between sub-views of the composition operator.
    /// An object of the form `{"row": number, "column": number}` can be used to set
    /// different spacing values for rows and columns.
    ///
    /// __Default value__: Depends on `"spacing"` property of [the view composition
    /// configuration](https://vega.github.io/vega-lite/docs/config.html#view-config) (`20` by
    /// default)
    ///
    /// The spacing in pixels between sub-views of the concat operator.
    ///
    /// __Default value__: `10`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub spacing: Option<Spacing>,
    /// A specification of the view that gets faceted.
    ///
    /// A specification of the view that gets repeated.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub spec: Option<Box<SpecClass>>,
    /// Definition for fields to be repeated. One of:
    /// 1) An array of fields to be repeated. If `"repeat"` is an array, the field can be
    /// referred using `{"repeat": "repeat"}`
    /// 2) An object that mapped `"row"` and/or `"column"` to the listed of fields to be repeated
    /// along the particular orientations. The objects `{"repeat": "row"}` and `{"repeat":
    /// "column"}` can be used to refer to the repeated field respectively.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub repeat: Option<RepeatUnion>,
    /// A list of views to be concatenated.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub concat: Option<Vec<Spec>>,
    /// A list of views to be concatenated and put into a column.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub vconcat: Option<Vec<Spec>>,
    /// A list of views to be concatenated and put into a row.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub hconcat: Option<Vec<Spec>>,
}

/// A specification of the view that gets repeated.
///
/// Any specification in Vega-Lite.
///
/// Unit spec that can have a composite mark and row or column channels (shorthand for a
/// facet spec).
///
/// A full layered plot specification, which may contains `encoding` and `projection`
/// properties that will be applied to underlying unit (single-view) specifications.
///
/// Base interface for a facet specification.
///
/// Base interface for a repeat specification.
///
/// Base interface for a generalized concatenation specification.
///
/// Base interface for a vertical concatenation specification.
///
/// Base interface for a horizontal concatenation specification.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct Spec {
    /// The bounds calculation method to use for determining the extent of a sub-plot. One of
    /// `full` (the default) or `flush`.
    ///
    /// - If set to `full`, the entire calculated bounds (including axes, title, and legend) will
    /// be used.
    /// - If set to `flush`, only the specified width and height values for the sub-view will be
    /// used. The `flush` setting can be useful when attempting to place sub-plots without axes
    /// or legends into a uniform grid structure.
    ///
    /// __Default value:__ `"full"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bounds: Option<BoundsEnum>,
    /// An object describing the data source. Set to `null` to ignore the parent's data source.
    /// If no data is set, it is derived from the parent.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub data: RemovableValue<UrlData>,
    /// Description of this mark for commenting purpose.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub description: Option<String>,
    /// A key-value mapping between encoding channels and definition of fields.
    ///
    /// A shared key-value mapping between encoding channels and definition of fields in the
    /// underlying layers.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub encoding: Option<Encoding>,
    /// The height of a visualization.
    ///
    /// - For a plot with a continuous y-field, height should be a number.
    /// - For a plot with either a discrete y-field or no y-field, height can be either a number
    /// indicating a fixed height or an object in the form of `{step: number}` defining the
    /// height per discrete step. (No y-field is equivalent to having one discrete step.)
    /// - To enable responsive sizing on height, it should be set to `"container"`.
    ///
    /// __Default value:__ Based on `config.view.continuousHeight` for a plot with a continuous
    /// y-field and `config.view.discreteHeight` otherwise.
    ///
    /// __Note:__ For plots with [`row` and `column`
    /// channels](https://vega.github.io/vega-lite/docs/encoding.html#facet), this represents the
    /// height of a single view and the `"container"` option cannot be used.
    ///
    /// __See also:__ [`height`](https://vega.github.io/vega-lite/docs/size.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub height: Option<HeightUnion>,
    /// A string describing the mark type (one of `"bar"`, `"circle"`, `"square"`, `"tick"`,
    /// `"line"`,
    /// `"area"`, `"point"`, `"rule"`, `"geoshape"`, and `"text"`) or a [mark definition
    /// object](https://vega.github.io/vega-lite/docs/mark.html#mark-def).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub mark: Option<AnyMark>,
    /// Name of the visualization for later reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub name: Option<String>,
    /// An object defining properties of geographic projection, which will be applied to `shape`
    /// path for `"geoshape"` marks
    /// and to `latitude` and `"longitude"` channels for other marks.
    ///
    /// An object defining properties of the geographic projection shared by underlying layers.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub projection: Option<Projection>,
    /// Scale, axis, and legend resolutions for view composition specifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub resolve: Option<Resolve>,
    /// A key-value mapping between selection names and definitions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub selection: Option<HashMap<String, SelectionDef>>,
    /// Title for the plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title: Option<Text>,
    /// An array of data transformations such as filter and new field calculation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub transform: Option<Vec<Transform>>,
    /// An object defining the view background's fill and stroke.
    ///
    /// __Default value:__ none (transparent)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub view: Option<ViewBackground>,
    /// The width of a visualization.
    ///
    /// - For a plot with a continuous x-field, width should be a number.
    /// - For a plot with either a discrete x-field or no x-field, width can be either a number
    /// indicating a fixed width or an object in the form of `{step: number}` defining the width
    /// per discrete step. (No x-field is equivalent to having one discrete step.)
    /// - To enable responsive sizing on width, it should be set to `"container"`.
    ///
    /// __Default value:__
    /// Based on `config.view.continuousWidth` for a plot with a continuous x-field and
    /// `config.view.discreteWidth` otherwise.
    ///
    /// __Note:__ For plots with [`row` and `column`
    /// channels](https://vega.github.io/vega-lite/docs/encoding.html#facet), this represents the
    /// width of a single view and the `"container"` option cannot be used.
    ///
    /// __See also:__ [`width`](https://vega.github.io/vega-lite/docs/size.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub width: Option<HeightUnion>,
    /// Layer or single view specifications to be layered.
    ///
    /// __Note__: Specifications inside `layer` cannot use `row` and `column` channels as
    /// layering facet specifications is not allowed. Instead, use the [facet
    /// operator](https://vega.github.io/vega-lite/docs/facet.html) and place a layer inside a
    /// facet.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub layer: Option<Vec<LayerSpec>>,
    /// The alignment to apply to grid rows and columns.
    /// The supported string values are `"all"`, `"each"`, and `"none"`.
    ///
    /// - For `"none"`, a flow layout will be used, in which adjacent subviews are simply placed
    /// one after the other.
    /// - For `"each"`, subviews will be aligned into a clean grid structure, but each row or
    /// column may be of variable size.
    /// - For `"all"`, subviews will be aligned and each row or column will be sized identically
    /// based on the maximum observed size. String values for this property will be applied to
    /// both grid rows and columns.
    ///
    /// Alternatively, an object value of the form `{"row": string, "column": string}` can be
    /// used to supply different alignments for rows and columns.
    ///
    /// __Default value:__ `"all"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub align: Option<AlignUnion>,
    /// Boolean flag indicating if subviews should be centered relative to their respective rows
    /// or columns.
    ///
    /// An object value of the form `{"row": boolean, "column": boolean}` can be used to supply
    /// different centering values for rows and columns.
    ///
    /// __Default value:__ `false`
    ///
    /// Boolean flag indicating if subviews should be centered relative to their respective rows
    /// or columns.
    ///
    /// __Default value:__ `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub center: Option<VegaliteCenter>,
    /// The number of columns to include in the view composition layout.
    ///
    /// __Default value__: `undefined` -- An infinite number of columns (a single row) will be
    /// assumed. This is equivalent to
    /// `hconcat` (for `concat`) and to using the `column` channel (for `facet` and `repeat`).
    ///
    /// __Note__:
    ///
    /// 1) This property is only for:
    /// - the general (wrappable) `concat` operator (not `hconcat`/`vconcat`)
    /// - the `facet` and `repeat` operator with one field/repetition definition (without
    /// row/column nesting)
    ///
    /// 2) Setting the `columns` to `1` is equivalent to `vconcat` (for `concat`) and to using
    /// the `row` channel (for `facet` and `repeat`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub columns: Option<f64>,
    /// Definition for how to facet the data. One of:
    /// 1) [a field definition for faceting the plot by one
    /// field](https://vega.github.io/vega-lite/docs/facet.html#field-def)
    /// 2) [An object that maps `row` and `column` channels to their field
    /// definitions](https://vega.github.io/vega-lite/docs/facet.html#mapping)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub facet: Option<Facet>,
    /// The spacing in pixels between sub-views of the composition operator.
    /// An object of the form `{"row": number, "column": number}` can be used to set
    /// different spacing values for rows and columns.
    ///
    /// __Default value__: Depends on `"spacing"` property of [the view composition
    /// configuration](https://vega.github.io/vega-lite/docs/config.html#view-config) (`20` by
    /// default)
    ///
    /// The spacing in pixels between sub-views of the concat operator.
    ///
    /// __Default value__: `10`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub spacing: Option<Spacing>,
    /// A specification of the view that gets faceted.
    ///
    /// A specification of the view that gets repeated.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub spec: Option<SpecClass>,
    /// Definition for fields to be repeated. One of:
    /// 1) An array of fields to be repeated. If `"repeat"` is an array, the field can be
    /// referred using `{"repeat": "repeat"}`
    /// 2) An object that mapped `"row"` and/or `"column"` to the listed of fields to be repeated
    /// along the particular orientations. The objects `{"repeat": "row"}` and `{"repeat":
    /// "column"}` can be used to refer to the repeated field respectively.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub repeat: Option<RepeatUnion>,
    /// A list of views to be concatenated.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub concat: Option<Vec<Spec>>,
    /// A list of views to be concatenated and put into a column.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub vconcat: Option<Vec<Spec>>,
    /// A list of views to be concatenated and put into a row.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub hconcat: Option<Vec<Spec>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct UrlData {
    /// An object that specifies the format for parsing the data.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub format: Option<DataFormat>,
    /// Provide a placeholder name and bind data at runtime.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub name: Option<String>,
    /// An URL from which to load the data set. Use the `format.type` property
    /// to ensure the loaded data is correctly parsed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub url: Option<String>,
    /// The full data set, included inline. This can be an array of objects or primitive values,
    /// an object, or a string.
    /// Arrays of primitive values are ingested as objects with a `data` property. Strings are
    /// parsed according to the specified format type.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub values: Option<UrlDataInlineDataset>,
    /// Generate a sequence of numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub sequence: Option<SequenceParams>,
    /// Generate sphere GeoJSON data for the full globe.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub sphere: Option<SphereUnion>,
    /// Generate graticule GeoJSON data for geographic reference lines.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub graticule: Option<Graticule>,
}

/// An object that specifies the format for parsing the data.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct DataFormat {
    /// If set to `null`, disable type inference based on the spec and only use type inference
    /// based on the data.
    /// Alternatively, a parsing directive object can be provided for explicit data types. Each
    /// property of the object corresponds to a field name, and the value to the desired data
    /// type (one of `"number"`, `"boolean"`, `"date"`, or null (do not parse the field)).
    /// For example, `"parse": {"modified_on": "date"}` parses the `modified_on` field in each
    /// input record a Date value.
    ///
    /// For `"date"`, we parse data based using Javascript's
    /// [`Date.parse()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/parse).
    /// For Specific date formats can be provided (e.g., `{foo: "date:'%m%d%Y'"}`), using the
    /// [d3-time-format syntax](https://github.com/d3/d3-time-format#locale_format). UTC date
    /// format parsing is supported similarly (e.g., `{foo: "utc:'%m%d%Y'"}`). See more about
    /// [UTC time](https://vega.github.io/vega-lite/docs/timeunit.html#utc)
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub parse: RemovableValue<HashMap<String, Option<String>>>,
    /// Type of input data: `"json"`, `"csv"`, `"tsv"`, `"dsv"`.
    ///
    /// __Default value:__  The default format type is determined by the extension of the file
    /// URL.
    /// If no extension is detected, `"json"` will be used by default.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub data_format_type: Option<DataFormatType>,
    /// The delimiter between records. The delimiter must be a single character (i.e., a single
    /// 16-bit code unit); so, ASCII delimiters are fine, but emoji delimiters are not.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub delimiter: Option<String>,
    /// The JSON property containing the desired data.
    /// This parameter can be used when the loaded JSON file may have surrounding structure or
    /// meta-data.
    /// For example `"property": "values.features"` is equivalent to retrieving
    /// `json.values.features`
    /// from the loaded JSON object.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub property: Option<String>,
    /// The name of the TopoJSON object set to convert to a GeoJSON feature collection.
    /// For example, in a map of the world, there may be an object set named `"countries"`.
    /// Using the feature property, we can extract this set and generate a GeoJSON feature object
    /// for each country.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub feature: Option<String>,
    /// The name of the TopoJSON object set to convert to mesh.
    /// Similar to the `feature` option, `mesh` extracts a named TopoJSON object set.
    /// Unlike the `feature` option, the corresponding geo data is returned as a single, unified
    /// mesh instance, not as individual GeoJSON features.
    /// Extracting a mesh is useful for more efficiently drawing borders or other geographic
    /// elements that you do not need to associate with specific regions such as individual
    /// countries, states or counties.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub mesh: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct GraticuleParams {
    /// Sets both the major and minor extents to the same values.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub extent: Option<Vec<Vec<f64>>>,
    /// The major extent of the graticule as a two-element array of coordinates.
    #[serde(rename = "extentMajor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub extent_major: Option<Vec<Vec<f64>>>,
    /// The minor extent of the graticule as a two-element array of coordinates.
    #[serde(rename = "extentMinor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub extent_minor: Option<Vec<Vec<f64>>>,
    /// The precision of the graticule in degrees.
    ///
    /// __Default value:__ `2.5`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub precision: Option<f64>,
    /// Sets both the major and minor step angles to the same values.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub step: Option<Vec<f64>>,
    /// The major step angles of the graticule.
    ///
    ///
    /// __Default value:__ `[90, 360]`
    #[serde(rename = "stepMajor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub step_major: Option<Vec<f64>>,
    /// The minor step angles of the graticule.
    ///
    /// __Default value:__ `[10, 10]`
    #[serde(rename = "stepMinor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub step_minor: Option<Vec<f64>>,
}

/// Generate a sequence of numbers.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct SequenceParams {
    /// The name of the generated sequence field.
    ///
    /// __Default value:__ `"data"`
    #[serde(rename = "as")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub sequence_params_as: Option<String>,
    /// The starting value of the sequence (inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub start: Option<f64>,
    /// The step value between sequence entries.
    ///
    /// __Default value:__ `1`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub step: Option<f64>,
    /// The ending value of the sequence (exclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stop: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct SphereClass {}

/// A key-value mapping between encoding channels and definition of fields.
///
/// A shared key-value mapping between encoding channels and definition of fields in the
/// underlying layers.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct Encoding {
    /// Color of the marks – either fill or stroke color based on  the `filled` property of mark
    /// definition.
    /// By default, `color` represents fill color for `"area"`, `"bar"`, `"tick"`,
    /// `"text"`, `"trail"`, `"circle"`, and `"square"` / stroke color for `"line"` and
    /// `"point"`.
    ///
    /// __Default value:__ If undefined, the default color depends on [mark
    /// config](https://vega.github.io/vega-lite/docs/config.html#mark)'s `color` property.
    ///
    /// _Note:_
    /// 1) For fine-grained control over both fill and stroke colors of the marks, please use the
    /// `fill` and `stroke` channels. The `fill` or `stroke` encodings have higher precedence
    /// than `color`, thus may override the `color` encoding if conflicting encodings are
    /// specified.
    /// 2) See the scale documentation for more information about customizing [color
    /// scheme](https://vega.github.io/vega-lite/docs/scale.html#scheme).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub color: Option<DefWithConditionMarkPropFieldDefGradientStringNull>,
    /// A field definition for the horizontal facet of trellis plots.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub column: Option<RowColumnEncodingFieldDef>,
    /// Additional levels of detail for grouping data in aggregate views and
    /// in line, trail, and area marks without mapping data to a specific visual channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub detail: Option<Detail>,
    /// A field definition for the (flexible) facet of trellis plots.
    ///
    /// If either `row` or `column` is specified, this channel will be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub facet: Option<FacetEncodingFieldDef>,
    /// Fill color of the marks.
    /// __Default value:__ If undefined, the default color depends on [mark
    /// config](https://vega.github.io/vega-lite/docs/config.html#mark)'s `color` property.
    ///
    /// _Note:_ The `fill` encoding has higher precedence than `color`, thus may override the
    /// `color` encoding if conflicting encodings are specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fill: Option<DefWithConditionMarkPropFieldDefGradientStringNull>,
    /// Fill opacity of the marks.
    ///
    /// __Default value:__ If undefined, the default opacity depends on [mark
    /// config](https://vega.github.io/vega-lite/docs/config.html#mark)'s `fillOpacity` property.
    #[serde(rename = "fillOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fill_opacity: Option<DefWithConditionMarkPropFieldDefNumber>,
    /// A URL to load upon mouse click.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub href: Option<HrefClass>,
    /// A data field to use as a unique key for data binding. When a visualization’s data is
    /// updated, the key value will be used to match data elements to existing mark instances.
    /// Use a key channel to enable object constancy for transitions over dynamic data.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub key: Option<TypedFieldDef>,
    /// Latitude position of geographically projected marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub latitude: Option<LatitudeClass>,
    /// Latitude-2 position for geographically projected ranged `"area"`, `"bar"`, `"rect"`, and
    /// `"rule"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub latitude2: Option<Latitude2Class>,
    /// Longitude position of geographically projected marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub longitude: Option<LatitudeClass>,
    /// Longitude-2 position for geographically projected ranged `"area"`, `"bar"`, `"rect"`,
    /// and  `"rule"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub longitude2: Option<Latitude2Class>,
    /// Opacity of the marks.
    ///
    /// __Default value:__ If undefined, the default opacity depends on [mark
    /// config](https://vega.github.io/vega-lite/docs/config.html#mark)'s `opacity` property.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub opacity: Option<DefWithConditionMarkPropFieldDefNumber>,
    /// Order of the marks.
    /// - For stacked marks, this `order` channel encodes [stack
    /// order](https://vega.github.io/vega-lite/docs/stack.html#order).
    /// - For line and trail marks, this `order` channel encodes order of data points in the
    /// lines. This can be useful for creating [a connected
    /// scatterplot](https://vega.github.io/vega-lite/examples/connected_scatterplot.html).
    /// Setting `order` to `{"value": null}` makes the line marks use the original order in the
    /// data sources.
    /// - Otherwise, this `order` channel encodes layer order of the marks.
    ///
    /// __Note__: In aggregate plots, `order` field should be `aggregate`d to avoid creating
    /// additional aggregation grouping.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub order: Option<Order>,
    /// A field definition for the vertical facet of trellis plots.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub row: Option<RowColumnEncodingFieldDef>,
    /// Shape of the mark.
    ///
    /// 1. For `point` marks the supported values include:
    /// - plotting shapes: `"circle"`, `"square"`, `"cross"`, `"diamond"`, `"triangle-up"`,
    /// `"triangle-down"`, `"triangle-right"`, or `"triangle-left"`.
    /// - the line symbol `"stroke"`
    /// - centered directional shapes `"arrow"`, `"wedge"`, or `"triangle"`
    /// - a custom [SVG path
    /// string](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths) (For correct
    /// sizing, custom shape paths should be defined within a square bounding box with
    /// coordinates ranging from -1 to 1 along both the x and y dimensions.)
    ///
    /// 2. For `geoshape` marks it should be a field definition of the geojson data
    ///
    /// __Default value:__ If undefined, the default shape depends on [mark
    /// config](https://vega.github.io/vega-lite/docs/config.html#point-config)'s `shape`
    /// property. (`"circle"` if unset.)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub shape: Option<DefWithConditionMarkPropFieldDefTypeForShapeStringNull>,
    /// Size of the mark.
    /// - For `"point"`, `"square"` and `"circle"`, – the symbol size, or pixel area of the mark.
    /// - For `"bar"` and `"tick"` – the bar and tick's size.
    /// - For `"text"` – the text's font size.
    /// - Size is unsupported for `"line"`, `"area"`, and `"rect"`. (Use `"trail"` instead of
    /// line with varying size)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub size: Option<DefWithConditionMarkPropFieldDefNumber>,
    /// Stroke color of the marks.
    /// __Default value:__ If undefined, the default color depends on [mark
    /// config](https://vega.github.io/vega-lite/docs/config.html#mark)'s `color` property.
    ///
    /// _Note:_ The `stroke` encoding has higher precedence than `color`, thus may override the
    /// `color` encoding if conflicting encodings are specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke: Option<DefWithConditionMarkPropFieldDefGradientStringNull>,
    /// Stroke opacity of the marks.
    ///
    /// __Default value:__ If undefined, the default opacity depends on [mark
    /// config](https://vega.github.io/vega-lite/docs/config.html#mark)'s `strokeOpacity`
    /// property.
    #[serde(rename = "strokeOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_opacity: Option<DefWithConditionMarkPropFieldDefNumber>,
    /// Stroke width of the marks.
    ///
    /// __Default value:__ If undefined, the default stroke width depends on [mark
    /// config](https://vega.github.io/vega-lite/docs/config.html#mark)'s `strokeWidth` property.
    #[serde(rename = "strokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_width: Option<DefWithConditionMarkPropFieldDefNumber>,
    /// Text of the `text` mark.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub text: Option<DefWithConditionStringFieldDefText>,
    /// The tooltip text to show upon mouse hover. Specifying `tooltip` encoding overrides [the
    /// `tooltip` property in the mark
    /// definition](https://vega.github.io/vega-lite/docs/mark.html#mark-def).
    ///
    /// See the [`tooltip`](https://vega.github.io/vega-lite/docs/tooltip.html) documentation for
    /// a detailed discussion about tooltip in Vega-Lite.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub tooltip: RemovableValue<Tooltip>,
    /// The URL of an image mark.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub url: Option<HrefClass>,
    /// X coordinates of the marks, or width of horizontal `"bar"` and `"area"` without specified
    /// `x2` or `width`.
    ///
    /// The `value` of this channel can be a number or a string `"width"` for the width of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub x: Option<XClass>,
    /// X2 coordinates for ranged `"area"`, `"bar"`, `"rect"`, and  `"rule"`.
    ///
    /// The `value` of this channel can be a number or a string `"width"` for the width of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub x2: Option<X2Class>,
    /// Error value of x coordinates for error specified `"errorbar"` and `"errorband"`.
    #[serde(rename = "xError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub x_error: Option<Latitude2Class>,
    /// Secondary error value of x coordinates for error specified `"errorbar"` and `"errorband"`.
    #[serde(rename = "xError2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub x_error2: Option<Latitude2Class>,
    /// Y coordinates of the marks, or height of vertical `"bar"` and `"area"` without specified
    /// `y2` or `height`.
    ///
    /// The `value` of this channel can be a number or a string `"height"` for the height of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub y: Option<YClass>,
    /// Y2 coordinates for ranged `"area"`, `"bar"`, `"rect"`, and  `"rule"`.
    ///
    /// The `value` of this channel can be a number or a string `"height"` for the height of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub y2: Option<Y2Class>,
    /// Error value of y coordinates for error specified `"errorbar"` and `"errorband"`.
    #[serde(rename = "yError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub y_error: Option<Latitude2Class>,
    /// Secondary error value of y coordinates for error specified `"errorbar"` and `"errorband"`.
    #[serde(rename = "yError2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub y_error2: Option<Latitude2Class>,
}

/// Color of the marks – either fill or stroke color based on  the `filled` property of mark
/// definition.
/// By default, `color` represents fill color for `"area"`, `"bar"`, `"tick"`,
/// `"text"`, `"trail"`, `"circle"`, and `"square"` / stroke color for `"line"` and
/// `"point"`.
///
/// __Default value:__ If undefined, the default color depends on [mark
/// config](https://vega.github.io/vega-lite/docs/config.html#mark)'s `color` property.
///
/// _Note:_
/// 1) For fine-grained control over both fill and stroke colors of the marks, please use the
/// `fill` and `stroke` channels. The `fill` or `stroke` encodings have higher precedence
/// than `color`, thus may override the `color` encoding if conflicting encodings are
/// specified.
/// 2) See the scale documentation for more information about customizing [color
/// scheme](https://vega.github.io/vega-lite/docs/scale.html#scheme).
///
/// Fill color of the marks.
/// __Default value:__ If undefined, the default color depends on [mark
/// config](https://vega.github.io/vega-lite/docs/config.html#mark)'s `color` property.
///
/// _Note:_ The `fill` encoding has higher precedence than `color`, thus may override the
/// `color` encoding if conflicting encodings are specified.
///
/// Stroke color of the marks.
/// __Default value:__ If undefined, the default color depends on [mark
/// config](https://vega.github.io/vega-lite/docs/config.html#mark)'s `color` property.
///
/// _Note:_ The `stroke` encoding has higher precedence than `color`, thus may override the
/// `color` encoding if conflicting encodings are specified.
///
/// A FieldDef with Condition<ValueDef>
/// {
/// condition: {value: ...},
/// field: ...,
/// ...
/// }
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct DefWithConditionMarkPropFieldDefGradientStringNull {
    /// Aggregation function for the field
    /// (e.g., `"mean"`, `"sum"`, `"median"`, `"min"`, `"max"`, `"count"`).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aggregate: Option<Aggregate>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - If `"binned"`, this indicates that the data for the `x` (or `y`) channel are already
    /// binned. You can map the bin-start field to `x` (or `y`) and the bin-end field to `x2` (or
    /// `y2`). The scale and axis will be formatted similar to binning in Vega-Lite.  To adjust
    /// the axis ticks based on the bin step, you can also set the axis's
    /// [`tickMinStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    ///
    /// __See also:__ [`bin`](https://vega.github.io/vega-lite/docs/bin.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bin: Option<PurpleBin>,
    /// One or more value definition(s) with [a selection or a test
    /// predicate](https://vega.github.io/vega-lite/docs/condition.html).
    ///
    /// __Note:__ A field definition's `condition` property can only contain [conditional value
    /// definitions](https://vega.github.io/vega-lite/docs/condition.html#value)
    /// since Vega-Lite only allows at most one encoded field per encoding channel.
    ///
    /// A field definition or one or more value definition(s) with a selection predicate.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub condition: Option<ColorCondition>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __See also:__ [`field`](https://vega.github.io/vega-lite/docs/field.html) documentation.
    ///
    /// __Notes:__
    /// 1)  Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects (e.g.,
    /// `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    /// 2) `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field: Option<Field>,
    /// An object defining properties of the legend.
    /// If `null`, the legend for the encoding channel will be removed.
    ///
    /// __Default value:__ If undefined, default [legend
    /// properties](https://vega.github.io/vega-lite/docs/legend.html) are applied.
    ///
    /// __See also:__ [`legend`](https://vega.github.io/vega-lite/docs/legend.html) documentation.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub legend: RemovableValue<Legend>,
    /// An object defining properties of the channel's scale, which is the function that
    /// transforms values in the data domain (numbers, dates, strings, etc) to visual values
    /// (pixels, colors, sizes) of the encoding channels.
    ///
    /// If `null`, the scale will be [disabled and the data value will be directly
    /// encoded](https://vega.github.io/vega-lite/docs/scale.html#disable).
    ///
    /// __Default value:__ If undefined, default [scale
    /// properties](https://vega.github.io/vega-lite/docs/scale.html) are applied.
    ///
    /// __See also:__ [`scale`](https://vega.github.io/vega-lite/docs/scale.html) documentation.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub scale: RemovableValue<Scale>,
    /// Sort order for the encoded field.
    ///
    /// For continuous fields (quantitative or temporal), `sort` can be either `"ascending"` or
    /// `"descending"`.
    ///
    /// For discrete fields, `sort` can be one of the following:
    /// - `"ascending"` or `"descending"` -- for sorting by the values' natural order in
    /// JavaScript.
    /// - [A string indicating an encoding channel name to sort
    /// by](https://vega.github.io/vega-lite/docs/sort.html#sort-by-encoding) (e.g., `"x"` or
    /// `"y"`) with an optional minus prefix for descending sort (e.g., `"-x"` to sort by
    /// x-field, descending). This channel string is short-form of [a sort-by-encoding
    /// definition](https://vega.github.io/vega-lite/docs/sort.html#sort-by-encoding). For
    /// example, `"sort": "-x"` is equivalent to `"sort": {"encoding": "x", "order":
    /// "descending"}`.
    /// - [A sort field definition](https://vega.github.io/vega-lite/docs/sort.html#sort-field)
    /// for sorting by another field.
    /// - [An array specifying the field values in preferred
    /// order](https://vega.github.io/vega-lite/docs/sort.html#sort-array). In this case, the
    /// sort order will obey the values in the array, followed by any unspecified values in their
    /// original order. For discrete time field, values in the sort array can be [date-time
    /// definition objects](struct.DateTime.html). In addition, for time units `"month"` and `"day"`,
    /// the values can be the month or day names (case insensitive) or their 3-letter initials
    /// (e.g., `"Mon"`, `"Tue"`).
    /// - `null` indicating no sort.
    ///
    /// __Default value:__ `"ascending"`
    ///
    /// __Note:__ `null` and sorting by another channel is not supported for `row` and `column`.
    ///
    /// __See also:__ [`sort`](https://vega.github.io/vega-lite/docs/sort.html) documentation.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub sort: RemovableValue<SortUnion>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html)
    /// documentation.
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`). If the field has an aggregate function, the function
    /// is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is binned or
    /// has a time unit applied, the applied function is shown in parentheses (e.g., `"Profit
    /// (binned)"`, `"Transaction Date (year-month)"`). Otherwise, the title is simply the field
    /// name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the
    /// [`fieldTitle`](https://vega.github.io/vega-lite/docs/config.html#top-level-config)
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub title: RemovableValue<PurpleText>,
    /// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
    /// `"nominal"`).
    /// It can also be a `"geojson"` type for encoding
    /// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
    ///
    ///
    /// __Note:__
    ///
    /// - Data values for a temporal field can be either a date-time string (e.g., `"2015-03-07
    /// 12:32:17"`, `"17:01"`, `"2015-03-16"`. `"2015"`) or a timestamp number (e.g.,
    /// `1552199579097`).
    /// - Data `type` describes the semantics of the data rather than the primitive data types
    /// (number, string, etc.). The same primitive data type can have different types of
    /// measurement. For example, numeric data can represent quantitative, ordinal, or nominal
    /// data.
    /// - When using with [`bin`](https://vega.github.io/vega-lite/docs/bin.html), the `type`
    /// property can be either `"quantitative"` (for using a linear bin scale) or [`"ordinal"`
    /// (for using an ordinal bin
    /// scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html), the
    /// `type` property can be either `"temporal"` (for using a temporal scale) or [`"ordinal"`
    /// (for using an ordinal scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html),
    /// the `type` property refers to the post-aggregation data type. For example, we can
    /// calculate count `distinct` of a categorical field `"cat"` using `{"aggregate":
    /// "distinct", "field": "cat", "type": "quantitative"}`. The `"type"` of the aggregate
    /// output is `"quantitative"`.
    /// - Secondary channels (e.g., `x2`, `y2`, `xError`, `yError`) do not have `type` as they
    /// have exactly the same type as their primary channels (e.g., `x`, `y`).
    ///
    /// __See also:__ [`type`](https://vega.github.io/vega-lite/docs/type.html) documentation.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub def_with_condition_mark_prop_field_def_gradient_string_null_type: Option<StandardType>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<ValueUnion>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ArgmDef {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub argmax: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub argmin: Option<String>,
}

/// Binning properties or boolean flag for determining whether to bin data or not.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct BinParams {
    /// A value in the binned domain at which to anchor the bins, shifting the bin boundaries if
    /// necessary to ensure that a boundary aligns with the anchor value.
    ///
    /// __Default value:__ the minimum bin extent value
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub anchor: Option<f64>,
    /// The number base to use for automatic bin determination (default is base 10).
    ///
    /// __Default value:__ `10`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub base: Option<f64>,
    /// When set to `true`, Vega-Lite treats the input data as already binned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub binned: Option<bool>,
    /// Scale factors indicating allowable subdivisions. The default value is [5, 2], which
    /// indicates that for base 10 numbers (the default base), the method may consider dividing
    /// bin sizes by 5 and/or 2. For example, for an initial step size of 10, the method can
    /// check if bin sizes of 2 (= 10/5), 5 (= 10/2), or 1 (= 10/(5*2)) might also satisfy the
    /// given constraints.
    ///
    /// __Default value:__ `[5, 2]`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub divide: Option<Vec<f64>>,
    /// A two-element (`[min, max]`) array indicating the range of desired bin values.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub extent: Option<BinExtent>,
    /// Maximum number of bins.
    ///
    /// __Default value:__ `6` for `row`, `column` and `shape` channels; `10` for other channels
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub maxbins: Option<f64>,
    /// A minimum allowable step size (particularly useful for integer values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub minstep: Option<f64>,
    /// If true, attempts to make the bin boundaries use human-friendly boundaries, such as
    /// multiples of ten.
    ///
    /// __Default value:__ `true`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub nice: Option<bool>,
    /// An exact step size to use between bins.
    ///
    /// __Note:__ If provided, options such as maxbins will be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub step: Option<f64>,
    /// An array of allowable step sizes to choose from.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub steps: Option<Vec<f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct BinExtentClass {
    /// The field name to extract selected values for, when a selection is
    /// [projected](https://vega.github.io/vega-lite/docs/project.html)
    /// over multiple fields or encodings.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field: Option<String>,
    /// The name of a selection.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub selection: Option<String>,
    /// The encoding channel to extract selected values for, when a selection is
    /// [projected](https://vega.github.io/vega-lite/docs/project.html)
    /// over multiple fields or encodings.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub encoding: Option<SingleDefUnitChannel>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ConditionalValueDefGradientStringNull {
    /// Predicate for triggering the condition
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub test: Option<PurpleLogicalOperandPredicate>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<ValueUnion>,
    /// A [selection name](https://vega.github.io/vega-lite/docs/selection.html), or a series of
    /// [composed selections](https://vega.github.io/vega-lite/docs/selection.html#compose).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub selection: Option<Box<PurpleSelectionOperand>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct Selection {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub not: Option<Box<PurpleSelectionOperand>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub and: Option<Vec<SelectionOperandElement>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub or: Option<Vec<SelectionOperandElement>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct Predicate {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub not: Option<PurpleLogicalOperandPredicate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub and: Option<Vec<LogicalOperandPredicateElement>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub or: Option<Vec<LogicalOperandPredicateElement>>,
    /// The value that the field should be equal to.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub equal: Option<SelectionInit>,
    /// Field to be filtered.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field: Option<String>,
    /// Time unit for the field to be filtered.
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit: Option<TimeUnit>,
    /// An array of inclusive minimum and maximum values
    /// for a field value of a data item to be included in the filtered data.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub range: Option<Vec<Option<PurpleRange>>>,
    /// A set of values that the `field`'s value should be a member of,
    /// for a data item included in the filtered data.
    #[serde(rename = "oneOf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub one_of: Option<Vec<Equal>>,
    /// The value that the field should be less than.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub lt: Option<Lt>,
    /// The value that the field should be greater than.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub gt: Option<Lt>,
    /// The value that the field should be less than or equals to.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub lte: Option<Lt>,
    /// The value that the field should be greater than or equals to.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub gte: Option<Lt>,
    /// If set to true the field's value has to be valid, meaning both not `null` and not
    /// [`NaN`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/NaN).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub valid: Option<bool>,
    /// Filter using a selection name.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub selection: Option<Box<PurpleSelectionOperand>>,
}

/// Object for defining datetime in Vega-Lite Filter.
/// If both month and quarter are provided, month has higher precedence.
/// `day` cannot be combined with other date.
/// We accept string for month and day names.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct DateTime {
    /// Integer value representing the date from 1-31.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub date: Option<f64>,
    /// Value representing the day of a week. This can be one of:
    /// (1) integer value -- `1` represents Monday;
    /// (2) case-insensitive day name (e.g., `"Monday"`);
    /// (3) case-insensitive, 3-character short day name (e.g., `"Mon"`).
    ///
    /// **Warning:** A DateTime definition object with `day`** should not be combined with
    /// `year`, `quarter`, `month`, or `date`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub day: Option<Day>,
    /// Integer value representing the hour of a day from 0-23.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub hours: Option<f64>,
    /// Integer value representing the millisecond segment of time.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub milliseconds: Option<f64>,
    /// Integer value representing the minute segment of time from 0-59.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub minutes: Option<f64>,
    /// One of:
    /// (1) integer value representing the month from `1`-`12`. `1` represents January;
    /// (2) case-insensitive month name (e.g., `"January"`);
    /// (3) case-insensitive, 3-character short month name (e.g., `"Jan"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub month: Option<Month>,
    /// Integer value representing the quarter of the year (from 1-4).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub quarter: Option<f64>,
    /// Integer value representing the second segment (0-59) of a time value
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub seconds: Option<f64>,
    /// A boolean flag indicating if date time is in utc time. If false, the date time is in
    /// local time
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub utc: Option<bool>,
    /// Integer value representing the year.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub year: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ValueLinearGradient {
    /// The type of gradient. Use `"linear"` for a linear gradient.
    ///
    /// The type of gradient. Use `"radial"` for a radial gradient.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub gradient: Option<Gradient>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub id: Option<String>,
    /// An array of gradient stops defining the gradient color sequence.
    pub stops: Vec<GradientStop>,
    /// The starting x-coordinate, in normalized [0, 1] coordinates, of the linear gradient.
    ///
    /// __Default value:__ `0`
    ///
    /// The x-coordinate, in normalized [0, 1] coordinates, for the center of the inner circle
    /// for the gradient.
    ///
    /// __Default value:__ `0.5`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub x1: Option<f64>,
    /// The ending x-coordinate, in normalized [0, 1] coordinates, of the linear gradient.
    ///
    /// __Default value:__ `1`
    ///
    /// The x-coordinate, in normalized [0, 1] coordinates, for the center of the outer circle
    /// for the gradient.
    ///
    /// __Default value:__ `0.5`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub x2: Option<f64>,
    /// The starting y-coordinate, in normalized [0, 1] coordinates, of the linear gradient.
    ///
    /// __Default value:__ `0`
    ///
    /// The y-coordinate, in normalized [0, 1] coordinates, for the center of the inner circle
    /// for the gradient.
    ///
    /// __Default value:__ `0.5`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub y1: Option<f64>,
    /// The ending y-coordinate, in normalized [0, 1] coordinates, of the linear gradient.
    ///
    /// __Default value:__ `0`
    ///
    /// The y-coordinate, in normalized [0, 1] coordinates, for the center of the outer circle
    /// for the gradient.
    ///
    /// __Default value:__ `0.5`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub y2: Option<f64>,
    /// The radius length, in normalized [0, 1] coordinates, of the inner circle for the
    /// gradient.
    ///
    /// __Default value:__ `0`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub r1: Option<f64>,
    /// The radius length, in normalized [0, 1] coordinates, of the outer circle for the
    /// gradient.
    ///
    /// __Default value:__ `0.5`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub r2: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct GradientStop {
    /// The color value at this point in the gradient.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub color: Option<String>,
    /// The offset fraction for the color stop, indicating its position within the gradient.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub offset: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ConditionalPredicateValueDefGradientStringNullClass {
    /// Predicate for triggering the condition
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub test: Option<PurpleLogicalOperandPredicate>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<ValueUnion>,
    /// A [selection name](https://vega.github.io/vega-lite/docs/selection.html), or a series of
    /// [composed selections](https://vega.github.io/vega-lite/docs/selection.html#compose).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub selection: Option<Box<PurpleSelectionOperand>>,
    /// Aggregation function for the field
    /// (e.g., `"mean"`, `"sum"`, `"median"`, `"min"`, `"max"`, `"count"`).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aggregate: Option<Aggregate>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - If `"binned"`, this indicates that the data for the `x` (or `y`) channel are already
    /// binned. You can map the bin-start field to `x` (or `y`) and the bin-end field to `x2` (or
    /// `y2`). The scale and axis will be formatted similar to binning in Vega-Lite.  To adjust
    /// the axis ticks based on the bin step, you can also set the axis's
    /// [`tickMinStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    ///
    /// __See also:__ [`bin`](https://vega.github.io/vega-lite/docs/bin.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bin: Option<PurpleBin>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __See also:__ [`field`](https://vega.github.io/vega-lite/docs/field.html) documentation.
    ///
    /// __Notes:__
    /// 1)  Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects (e.g.,
    /// `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    /// 2) `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field: Option<Field>,
    /// An object defining properties of the legend.
    /// If `null`, the legend for the encoding channel will be removed.
    ///
    /// __Default value:__ If undefined, default [legend
    /// properties](https://vega.github.io/vega-lite/docs/legend.html) are applied.
    ///
    /// __See also:__ [`legend`](https://vega.github.io/vega-lite/docs/legend.html) documentation.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub legend: RemovableValue<Legend>,
    /// An object defining properties of the channel's scale, which is the function that
    /// transforms values in the data domain (numbers, dates, strings, etc) to visual values
    /// (pixels, colors, sizes) of the encoding channels.
    ///
    /// If `null`, the scale will be [disabled and the data value will be directly
    /// encoded](https://vega.github.io/vega-lite/docs/scale.html#disable).
    ///
    /// __Default value:__ If undefined, default [scale
    /// properties](https://vega.github.io/vega-lite/docs/scale.html) are applied.
    ///
    /// __See also:__ [`scale`](https://vega.github.io/vega-lite/docs/scale.html) documentation.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub scale: RemovableValue<Scale>,
    /// Sort order for the encoded field.
    ///
    /// For continuous fields (quantitative or temporal), `sort` can be either `"ascending"` or
    /// `"descending"`.
    ///
    /// For discrete fields, `sort` can be one of the following:
    /// - `"ascending"` or `"descending"` -- for sorting by the values' natural order in
    /// JavaScript.
    /// - [A string indicating an encoding channel name to sort
    /// by](https://vega.github.io/vega-lite/docs/sort.html#sort-by-encoding) (e.g., `"x"` or
    /// `"y"`) with an optional minus prefix for descending sort (e.g., `"-x"` to sort by
    /// x-field, descending). This channel string is short-form of [a sort-by-encoding
    /// definition](https://vega.github.io/vega-lite/docs/sort.html#sort-by-encoding). For
    /// example, `"sort": "-x"` is equivalent to `"sort": {"encoding": "x", "order":
    /// "descending"}`.
    /// - [A sort field definition](https://vega.github.io/vega-lite/docs/sort.html#sort-field)
    /// for sorting by another field.
    /// - [An array specifying the field values in preferred
    /// order](https://vega.github.io/vega-lite/docs/sort.html#sort-array). In this case, the
    /// sort order will obey the values in the array, followed by any unspecified values in their
    /// original order. For discrete time field, values in the sort array can be [date-time
    /// definition objects](struct.DateTime.html). In addition, for time units `"month"` and `"day"`,
    /// the values can be the month or day names (case insensitive) or their 3-letter initials
    /// (e.g., `"Mon"`, `"Tue"`).
    /// - `null` indicating no sort.
    ///
    /// __Default value:__ `"ascending"`
    ///
    /// __Note:__ `null` and sorting by another channel is not supported for `row` and `column`.
    ///
    /// __See also:__ [`sort`](https://vega.github.io/vega-lite/docs/sort.html) documentation.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub sort: RemovableValue<SortUnion>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html)
    /// documentation.
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`). If the field has an aggregate function, the function
    /// is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is binned or
    /// has a time unit applied, the applied function is shown in parentheses (e.g., `"Profit
    /// (binned)"`, `"Transaction Date (year-month)"`). Otherwise, the title is simply the field
    /// name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the
    /// [`fieldTitle`](https://vega.github.io/vega-lite/docs/config.html#top-level-config)
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub title: RemovableValue<PurpleText>,
    /// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
    /// `"nominal"`).
    /// It can also be a `"geojson"` type for encoding
    /// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
    ///
    ///
    /// __Note:__
    ///
    /// - Data values for a temporal field can be either a date-time string (e.g., `"2015-03-07
    /// 12:32:17"`, `"17:01"`, `"2015-03-16"`. `"2015"`) or a timestamp number (e.g.,
    /// `1552199579097`).
    /// - Data `type` describes the semantics of the data rather than the primitive data types
    /// (number, string, etc.). The same primitive data type can have different types of
    /// measurement. For example, numeric data can represent quantitative, ordinal, or nominal
    /// data.
    /// - When using with [`bin`](https://vega.github.io/vega-lite/docs/bin.html), the `type`
    /// property can be either `"quantitative"` (for using a linear bin scale) or [`"ordinal"`
    /// (for using an ordinal bin
    /// scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html), the
    /// `type` property can be either `"temporal"` (for using a temporal scale) or [`"ordinal"`
    /// (for using an ordinal scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html),
    /// the `type` property refers to the post-aggregation data type. For example, we can
    /// calculate count `distinct` of a categorical field `"cat"` using `{"aggregate":
    /// "distinct", "field": "cat", "type": "quantitative"}`. The `"type"` of the aggregate
    /// output is `"quantitative"`.
    /// - Secondary channels (e.g., `x2`, `y2`, `xError`, `yError`) do not have `type` as they
    /// have exactly the same type as their primary channels (e.g., `x`, `y`).
    ///
    /// __See also:__ [`type`](https://vega.github.io/vega-lite/docs/type.html) documentation.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub conditional_type: Option<StandardType>,
}

/// Reference to a repeated value.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct RepeatRef {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub repeat: Option<RepeatEnum>,
}

/// Properties of a legend or boolean flag for determining whether to show it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct Legend {
    /// The height in pixels to clip symbol legend entries and limit their size.
    #[serde(rename = "clipHeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub clip_height: Option<f64>,
    /// The horizontal padding in pixels between symbol legend entries.
    ///
    /// __Default value:__ `10`.
    #[serde(rename = "columnPadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub column_padding: Option<f64>,
    /// The number of columns in which to arrange symbol legend entries. A value of `0` or lower
    /// indicates a single row with one column per entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub columns: Option<f64>,
    /// Corner radius for the full legend.
    #[serde(rename = "cornerRadius")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius: Option<f64>,
    /// The direction of the legend, one of `"vertical"` or `"horizontal"`.
    ///
    /// __Default value:__
    /// - For top-/bottom-`orient`ed legends, `"horizontal"`
    /// - For left-/right-`orient`ed legends, `"vertical"`
    /// - For top/bottom-left/right-`orient`ed legends, `"horizontal"` for gradient legends and
    /// `"vertical"` for symbol legends.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub direction: Option<Orientation>,
    /// Background fill color for the full legend.
    #[serde(rename = "fillColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fill_color: Option<String>,
    /// The text formatting pattern for labels of guides (axes, legends, headers) and text
    /// marks.
    ///
    /// - If the format type is `"number"` (e.g., for quantitative fields), this is D3's [number
    /// format pattern](https://github.com/d3/d3-format#locale_format).
    /// - If the format type is `"time"` (e.g., for temporal fields), this is D3's [time format
    /// pattern](https://github.com/d3/d3-time-format#locale_format).
    ///
    /// See the [format documentation](https://vega.github.io/vega-lite/docs/format.html) for
    /// more examples.
    ///
    /// __Default value:__  Derived from
    /// [numberFormat](https://vega.github.io/vega-lite/docs/config.html#format) config for
    /// number format and from
    /// [timeFormat](https://vega.github.io/vega-lite/docs/config.html#format) config for time
    /// format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub format: Option<String>,
    /// The format type for labels (`"number"` or `"time"`).
    ///
    /// __Default value:__
    /// - `"time"` for temporal fields and ordinal and nomimal fields with `timeUnit`.
    /// - `"number"` for quantitative fields as well as ordinal and nomimal fields without
    /// `timeUnit`.
    #[serde(rename = "formatType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub format_type: Option<FormatType>,
    /// The length in pixels of the primary axis of a color gradient. This value corresponds to
    /// the height of a vertical gradient or the width of a horizontal gradient.
    ///
    /// __Default value:__ `200`.
    #[serde(rename = "gradientLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub gradient_length: Option<f64>,
    /// Opacity of the color gradient.
    #[serde(rename = "gradientOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub gradient_opacity: Option<f64>,
    /// The color of the gradient stroke, can be in hex color code or regular color name.
    ///
    /// __Default value:__ `"lightGray"`.
    #[serde(rename = "gradientStrokeColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub gradient_stroke_color: Option<String>,
    /// The width of the gradient stroke, in pixels.
    ///
    /// __Default value:__ `0`.
    #[serde(rename = "gradientStrokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub gradient_stroke_width: Option<f64>,
    /// The thickness in pixels of the color gradient. This value corresponds to the width of a
    /// vertical gradient or the height of a horizontal gradient.
    ///
    /// __Default value:__ `16`.
    #[serde(rename = "gradientThickness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub gradient_thickness: Option<f64>,
    /// The alignment to apply to symbol legends rows and columns. The supported string values
    /// are `"all"`, `"each"` (the default), and `none`. For more information, see the [grid
    /// layout documentation](https://vega.github.io/vega/docs/layout).
    ///
    /// __Default value:__ `"each"`.
    #[serde(rename = "gridAlign")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub grid_align: Option<LayoutAlign>,
    /// The alignment of the legend label, can be left, center, or right.
    #[serde(rename = "labelAlign")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_align: Option<Align>,
    /// The position of the baseline of legend label, can be `"top"`, `"middle"`, `"bottom"`, or
    /// `"alphabetic"`.
    ///
    /// __Default value:__ `"middle"`.
    #[serde(rename = "labelBaseline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_baseline: Option<Baseline>,
    /// The color of the legend label, can be in hex color code or regular color name.
    #[serde(rename = "labelColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_color: Option<String>,
    /// [Vega expression](https://vega.github.io/vega/docs/expressions/) for customizing labels.
    ///
    /// __Note:__ The label text and value can be assessed via the `label` and `value` properties
    /// of the legend's backing `datum` object.
    #[serde(rename = "labelExpr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_expr: Option<String>,
    /// The font of the legend label.
    #[serde(rename = "labelFont")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_font: Option<String>,
    /// The font size of legend label.
    ///
    /// __Default value:__ `10`.
    #[serde(rename = "labelFontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_font_size: Option<f64>,
    /// The font style of legend label.
    #[serde(rename = "labelFontStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_font_style: Option<String>,
    /// The font weight of legend label.
    #[serde(rename = "labelFontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_font_weight: Option<FontWeight>,
    /// Maximum allowed pixel width of legend tick labels.
    ///
    /// __Default value:__ `160`.
    #[serde(rename = "labelLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_limit: Option<f64>,
    /// The offset of the legend label.
    #[serde(rename = "labelOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_offset: Option<f64>,
    /// Opacity of labels.
    #[serde(rename = "labelOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_opacity: Option<f64>,
    /// The strategy to use for resolving overlap of labels in gradient legends. If `false`, no
    /// overlap reduction is attempted. If set to `true` (default) or `"parity"`, a strategy of
    /// removing every other label is used. If set to `"greedy"`, a linear scan of the labels is
    /// performed, removing any label that overlaps with the last visible label (this often works
    /// better for log-scaled axes).
    ///
    /// __Default value:__ `true`.
    #[serde(rename = "labelOverlap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_overlap: Option<LabelOverlap>,
    /// Padding in pixels between the legend and legend labels.
    #[serde(rename = "labelPadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_padding: Option<f64>,
    /// The minimum separation that must be between label bounding boxes for them to be
    /// considered non-overlapping (default `0`). This property is ignored if *labelOverlap*
    /// resolution is not enabled.
    #[serde(rename = "labelSeparation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_separation: Option<f64>,
    /// Custom x-position for legend with orient "none".
    #[serde(rename = "legendX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub legend_x: Option<f64>,
    /// Custom y-position for legend with orient "none".
    #[serde(rename = "legendY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub legend_y: Option<f64>,
    /// The offset in pixels by which to displace the legend from the data rectangle and axes.
    ///
    /// __Default value:__ `18`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub offset: Option<f64>,
    /// The orientation of the legend, which determines how the legend is positioned within the
    /// scene. One of `"left"`, `"right"`, `"top"`, `"bottom"`, `"top-left"`, `"top-right"`,
    /// `"bottom-left"`, `"bottom-right"`, `"none"`.
    ///
    /// __Default value:__ `"right"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub orient: Option<LegendOrient>,
    /// The padding between the border and content of the legend group.
    ///
    /// __Default value:__ `0`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub padding: Option<f64>,
    /// The vertical padding in pixels between symbol legend entries.
    ///
    /// __Default value:__ `2`.
    #[serde(rename = "rowPadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub row_padding: Option<f64>,
    /// Border stroke color for the full legend.
    #[serde(rename = "strokeColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_color: Option<String>,
    /// An array of alternating [stroke, space] lengths for dashed symbol strokes.
    #[serde(rename = "symbolDash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub symbol_dash: Option<Vec<f64>>,
    /// The pixel offset at which to start drawing with the symbol stroke dash array.
    #[serde(rename = "symbolDashOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub symbol_dash_offset: Option<f64>,
    /// The color of the legend symbol,
    #[serde(rename = "symbolFillColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub symbol_fill_color: Option<String>,
    /// The maximum number of allowed entries for a symbol legend. Additional entries will be
    /// dropped.
    #[serde(rename = "symbolLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub symbol_limit: Option<f64>,
    /// Horizontal pixel offset for legend symbols.
    ///
    /// __Default value:__ `0`.
    #[serde(rename = "symbolOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub symbol_offset: Option<f64>,
    /// Opacity of the legend symbols.
    #[serde(rename = "symbolOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub symbol_opacity: Option<f64>,
    /// The size of the legend symbol, in pixels.
    ///
    /// __Default value:__ `100`.
    #[serde(rename = "symbolSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub symbol_size: Option<f64>,
    /// Stroke color for legend symbols.
    #[serde(rename = "symbolStrokeColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub symbol_stroke_color: Option<String>,
    /// The width of the symbol's stroke.
    ///
    /// __Default value:__ `1.5`.
    #[serde(rename = "symbolStrokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub symbol_stroke_width: Option<f64>,
    /// The symbol shape. One of the plotting shapes `circle` (default), `square`, `cross`,
    /// `diamond`, `triangle-up`, `triangle-down`, `triangle-right`, or `triangle-left`, the line
    /// symbol `stroke`, or one of the centered directional shapes `arrow`, `wedge`, or
    /// `triangle`. Alternatively, a custom [SVG path
    /// string](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths) can be provided.
    /// For correct sizing, custom shape paths should be defined within a square bounding box
    /// with coordinates ranging from -1 to 1 along both the x and y dimensions.
    ///
    /// __Default value:__ `"circle"`.
    #[serde(rename = "symbolType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub symbol_type: Option<String>,
    /// The desired number of tick values for quantitative legends.
    #[serde(rename = "tickCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tick_count: Option<TickCount>,
    /// The minimum desired step between legend ticks, in terms of scale domain values. For
    /// example, a value of `1` indicates that ticks should not be less than 1 unit apart. If
    /// `tickMinStep` is specified, the `tickCount` value will be adjusted, if necessary, to
    /// enforce the minimum step value.
    ///
    /// __Default value__: `undefined`
    #[serde(rename = "tickMinStep")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tick_min_step: Option<f64>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`). If the field has an aggregate function, the function
    /// is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is binned or
    /// has a time unit applied, the applied function is shown in parentheses (e.g., `"Profit
    /// (binned)"`, `"Transaction Date (year-month)"`). Otherwise, the title is simply the field
    /// name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the
    /// [`fieldTitle`](https://vega.github.io/vega-lite/docs/config.html#top-level-config)
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub title: RemovableValue<PurpleText>,
    /// Horizontal text alignment for legend titles.
    ///
    /// __Default value:__ `"left"`.
    #[serde(rename = "titleAlign")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_align: Option<Align>,
    /// Text anchor position for placing legend titles.
    #[serde(rename = "titleAnchor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_anchor: Option<TitleAnchorEnum>,
    /// Vertical text baseline for legend titles.
    ///
    /// __Default value:__ `"top"`.
    #[serde(rename = "titleBaseline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_baseline: Option<Baseline>,
    /// The color of the legend title, can be in hex color code or regular color name.
    #[serde(rename = "titleColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_color: Option<String>,
    /// The font of the legend title.
    #[serde(rename = "titleFont")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_font: Option<String>,
    /// The font size of the legend title.
    #[serde(rename = "titleFontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_font_size: Option<f64>,
    /// The font style of the legend title.
    #[serde(rename = "titleFontStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_font_style: Option<String>,
    /// The font weight of the legend title.
    /// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
    /// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
    #[serde(rename = "titleFontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_font_weight: Option<FontWeight>,
    /// Maximum allowed pixel width of legend titles.
    ///
    /// __Default value:__ `180`.
    #[serde(rename = "titleLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_limit: Option<f64>,
    /// Line height in pixels for multi-line title text.
    #[serde(rename = "titleLineHeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_line_height: Option<f64>,
    /// Opacity of the legend title.
    #[serde(rename = "titleOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_opacity: Option<f64>,
    /// Orientation of the legend title.
    #[serde(rename = "titleOrient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_orient: Option<Orient>,
    /// The padding, in pixels, between title and legend.
    ///
    /// __Default value:__ `5`.
    #[serde(rename = "titlePadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_padding: Option<f64>,
    /// The type of the legend. Use `"symbol"` to create a discrete legend and `"gradient"` for a
    /// continuous color gradient.
    ///
    /// __Default value:__ `"gradient"` for non-binned quantitative fields and temporal fields;
    /// `"symbol"` otherwise.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub legend_type: Option<LegendType>,
    /// Explicitly set the visible legend values.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub values: Option<Vec<Equal>>,
    /// A non-negative integer indicating the z-index of the legend.
    /// If zindex is 0, legend should be drawn behind all chart elements.
    /// To put them in front, use zindex = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub zindex: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct Scale {
    /// The alignment of the steps within the scale range.
    ///
    /// This value must lie in the range `[0,1]`. A value of `0.5` indicates that the steps
    /// should be centered within the range. A value of `0` or `1` may be used to shift the bands
    /// to one side, say to position them adjacent to an axis.
    ///
    /// __Default value:__ `0.5`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub align: Option<f64>,
    /// The logarithm base of the `log` scale (default `10`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub base: Option<f64>,
    /// An array of bin boundaries over the scale domain. If provided, axes and legends will use
    /// the bin boundaries to inform the choice of tick marks and text labels.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bins: Option<Vec<f64>>,
    /// If `true`, values that exceed the data domain are clamped to either the minimum or
    /// maximum range value
    ///
    /// __Default value:__ derived from the [scale
    /// config](https://vega.github.io/vega-lite/docs/config.html#scale-config)'s `clamp` (`true`
    /// by default).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub clamp: Option<bool>,
    /// A constant determining the slope of the symlog function around zero. Only used for
    /// `symlog` scales.
    ///
    /// __Default value:__ `1`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub constant: Option<f64>,
    /// Customized domain values.
    ///
    /// For _quantitative_ fields, `domain` can take the form of a two-element array with minimum
    /// and maximum values. [Piecewise
    /// scales](https://vega.github.io/vega-lite/docs/scale.html#piecewise) can be created by
    /// providing a `domain` with more than two entries.
    /// If the input field is aggregated, `domain` can also be a string value `"unaggregated"`,
    /// indicating that the domain should include the raw data values prior to the aggregation.
    ///
    /// For _temporal_ fields, `domain` can be a two-element array minimum and maximum values, in
    /// the form of either timestamps or the [DateTime definition
    /// objects](https://vega.github.io/vega-lite/docs/types.html#datetime).
    ///
    /// For _ordinal_ and _nominal_ fields, `domain` can be an array that lists valid input
    /// values.
    ///
    /// The `selection` property can be used to [interactively
    /// determine](https://vega.github.io/vega-lite/docs/selection.html#scale-domains) the scale
    /// domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub domain: Option<DomainUnion>,
    /// The exponent of the `pow` scale.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub exponent: Option<f64>,
    /// The interpolation method for range values. By default, a general interpolator for
    /// numbers, dates, strings and colors (in HCL space) is used. For color ranges, this
    /// property allows interpolation in alternative color spaces. Legal values include `rgb`,
    /// `hsl`, `hsl-long`, `lab`, `hcl`, `hcl-long`, `cubehelix` and `cubehelix-long` ('-long'
    /// variants use longer paths in polar coordinate spaces). If object-valued, this property
    /// accepts an object with a string-valued _type_ property and an optional numeric _gamma_
    /// property applicable to rgb and cubehelix interpolators. For more, see the [d3-interpolate
    /// documentation](https://github.com/d3/d3-interpolate).
    ///
    /// * __Default value:__ `hcl`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub interpolate: Option<InterpolateUnion>,
    /// Extending the domain so that it starts and ends on nice round values. This method
    /// typically modifies the scale’s domain, and may only extend the bounds to the nearest
    /// round value. Nicing is useful if the domain is computed from data and may be irregular.
    /// For example, for a domain of _[0.201479…, 0.996679…]_, a nice domain might be _[0.2,
    /// 1.0]_.
    ///
    /// For quantitative scales such as linear, `nice` can be either a boolean flag or a number.
    /// If `nice` is a number, it will represent a desired tick count. This allows greater
    /// control over the step size used to extend the bounds, guaranteeing that the returned
    /// ticks will exactly cover the domain.
    ///
    /// For temporal fields with time and utc scales, the `nice` value can be a string indicating
    /// the desired time interval. Legal values are `"millisecond"`, `"second"`, `"minute"`,
    /// `"hour"`, `"day"`, `"week"`, `"month"`, and `"year"`. Alternatively, `time` and `utc`
    /// scales can accept an object-valued interval specifier of the form `{"interval": "month",
    /// "step": 3}`, which includes a desired number of interval steps. Here, the domain would
    /// snap to quarter (Jan, Apr, Jul, Oct) boundaries.
    ///
    /// __Default value:__ `true` for unbinned _quantitative_ fields; `false` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub nice: Option<NiceUnion>,
    /// For _[continuous](https://vega.github.io/vega-lite/docs/scale.html#continuous)_ scales,
    /// expands the scale domain to accommodate the specified number of pixels on each of the
    /// scale range. The scale range must represent pixels for this parameter to function as
    /// intended. Padding adjustment is performed prior to all other adjustments, including the
    /// effects of the `zero`, `nice`, `domainMin`, and `domainMax` properties.
    ///
    /// For _[band](https://vega.github.io/vega-lite/docs/scale.html#band)_ scales, shortcut for
    /// setting `paddingInner` and `paddingOuter` to the same value.
    ///
    /// For _[point](https://vega.github.io/vega-lite/docs/scale.html#point)_ scales, alias for
    /// `paddingOuter`.
    ///
    /// __Default value:__ For _continuous_ scales, derived from the [scale
    /// config](https://vega.github.io/vega-lite/docs/scale.html#config)'s `continuousPadding`.
    /// For _band and point_ scales, see `paddingInner` and `paddingOuter`. By default, Vega-Lite
    /// sets padding such that _width/height = number of unique values * step_.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub padding: Option<f64>,
    /// The inner padding (spacing) within each band step of band scales, as a fraction of the
    /// step size. This value must lie in the range [0,1].
    ///
    /// For point scale, this property is invalid as point scales do not have internal band
    /// widths (only step sizes between bands).
    ///
    /// __Default value:__ derived from the [scale
    /// config](https://vega.github.io/vega-lite/docs/scale.html#config)'s `bandPaddingInner`.
    #[serde(rename = "paddingInner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub padding_inner: Option<f64>,
    /// The outer padding (spacing) at the ends of the range of band and point scales,
    /// as a fraction of the step size. This value must lie in the range [0,1].
    ///
    /// __Default value:__ derived from the [scale
    /// config](https://vega.github.io/vega-lite/docs/scale.html#config)'s `bandPaddingOuter` for
    /// band scales and `pointPadding` for point scales.
    /// By default, Vega-Lite sets outer padding such that _width/height = number of unique
    /// values * step_.
    #[serde(rename = "paddingOuter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub padding_outer: Option<f64>,
    /// The range of the scale. One of:
    ///
    /// - A string indicating a [pre-defined named scale
    /// range](https://vega.github.io/vega-lite/docs/scale.html#range-config) (e.g., example,
    /// `"symbol"`, or `"diverging"`).
    ///
    /// - For [continuous scales](https://vega.github.io/vega-lite/docs/scale.html#continuous),
    /// two-element array indicating  minimum and maximum values, or an array with more than two
    /// entries for specifying a [piecewise
    /// scale](https://vega.github.io/vega-lite/docs/scale.html#piecewise).
    ///
    /// - For [discrete](https://vega.github.io/vega-lite/docs/scale.html#discrete) and
    /// [discretizing](https://vega.github.io/vega-lite/docs/scale.html#discretizing) scales, an
    /// array of desired output values.
    ///
    /// __Notes:__
    ///
    /// 1) For color scales you can also specify a color
    /// [`scheme`](https://vega.github.io/vega-lite/docs/scale.html#scheme) instead of `range`.
    ///
    /// 2) Any directly specified `range` for `x` and `y` channels will be ignored. Range can be
    /// customized via the view's corresponding
    /// [size](https://vega.github.io/vega-lite/docs/size.html) (`width` and `height`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub range: Option<ScaleRange>,
    /// If `true`, rounds numeric output values to integers. This can be helpful for snapping to
    /// the pixel grid.
    ///
    /// __Default value:__ `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub round: Option<bool>,
    /// A string indicating a color
    /// [scheme](https://vega.github.io/vega-lite/docs/scale.html#scheme) name (e.g.,
    /// `"category10"` or `"blues"`) or a [scheme parameter
    /// object](https://vega.github.io/vega-lite/docs/scale.html#scheme-params).
    ///
    /// Discrete color schemes may be used with
    /// [discrete](https://vega.github.io/vega-lite/docs/scale.html#discrete) or
    /// [discretizing](https://vega.github.io/vega-lite/docs/scale.html#discretizing) scales.
    /// Continuous color schemes are intended for use with color scales.
    ///
    /// For the full list of supported schemes, please refer to the [Vega
    /// Scheme](https://vega.github.io/vega/docs/schemes/#reference) reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub scheme: Option<Scheme>,
    /// The type of scale. Vega-Lite supports the following categories of scale types:
    ///
    /// 1) [**Continuous Scales**](https://vega.github.io/vega-lite/docs/scale.html#continuous)
    /// -- mapping continuous domains to continuous output ranges
    /// ([`"linear"`](https://vega.github.io/vega-lite/docs/scale.html#linear),
    /// [`"pow"`](https://vega.github.io/vega-lite/docs/scale.html#pow),
    /// [`"sqrt"`](https://vega.github.io/vega-lite/docs/scale.html#sqrt),
    /// [`"symlog"`](https://vega.github.io/vega-lite/docs/scale.html#symlog),
    /// [`"log"`](https://vega.github.io/vega-lite/docs/scale.html#log),
    /// [`"time"`](https://vega.github.io/vega-lite/docs/scale.html#time),
    /// [`"utc"`](https://vega.github.io/vega-lite/docs/scale.html#utc).
    ///
    /// 2) [**Discrete Scales**](https://vega.github.io/vega-lite/docs/scale.html#discrete) --
    /// mapping discrete domains to discrete
    /// ([`"ordinal"`](https://vega.github.io/vega-lite/docs/scale.html#ordinal)) or continuous
    /// ([`"band"`](https://vega.github.io/vega-lite/docs/scale.html#band) and
    /// [`"point"`](https://vega.github.io/vega-lite/docs/scale.html#point)) output ranges.
    ///
    /// 3) [**Discretizing
    /// Scales**](https://vega.github.io/vega-lite/docs/scale.html#discretizing) -- mapping
    /// continuous domains to discrete output ranges
    /// [`"bin-ordinal"`](https://vega.github.io/vega-lite/docs/scale.html#bin-ordinal),
    /// [`"quantile"`](https://vega.github.io/vega-lite/docs/scale.html#quantile),
    /// [`"quantize"`](https://vega.github.io/vega-lite/docs/scale.html#quantize) and
    /// [`"threshold"`](https://vega.github.io/vega-lite/docs/scale.html#threshold).
    ///
    /// __Default value:__ please see the [scale type
    /// table](https://vega.github.io/vega-lite/docs/scale.html#type).
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub scale_type: Option<ScaleType>,
    /// If `true`, ensures that a zero baseline value is included in the scale domain.
    ///
    /// __Default value:__ `true` for x and y channels if the quantitative field is not binned
    /// and no custom `domain` is provided; `false` otherwise.
    ///
    /// __Note:__ Log, time, and utc scales do not support `zero`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub zero: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct DomainClass {
    /// The field name to extract selected values for, when a selection is
    /// [projected](https://vega.github.io/vega-lite/docs/project.html)
    /// over multiple fields or encodings.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field: Option<String>,
    /// The name of a selection.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub selection: Option<String>,
    /// The encoding channel to extract selected values for, when a selection is
    /// [projected](https://vega.github.io/vega-lite/docs/project.html)
    /// over multiple fields or encodings.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub encoding: Option<SingleDefUnitChannel>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ScaleInterpolateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub gamma: Option<f64>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub scale_interpolate_params_type: Option<ScaleInterpolateParamsType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct NiceClass {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub interval: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub step: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct SchemeParams {
    /// The number of colors to use in the scheme. This can be useful for scale types such as
    /// `"quantize"`, which use the length of the scale range to determine the number of discrete
    /// bins for the scale domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub count: Option<f64>,
    /// The extent of the color range to use. For example `[0.2, 1]` will rescale the color
    /// scheme such that color values in the range _[0, 0.2)_ are excluded from the scheme.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub extent: Option<Vec<f64>>,
    /// A color scheme name for ordinal scales (e.g., `"category10"` or `"blues"`).
    ///
    /// For the full list of supported schemes, please refer to the [Vega
    /// Scheme](https://vega.github.io/vega/docs/schemes/#reference) reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub name: Option<String>,
}

/// A sort definition for sorting a discrete scale in an encoding field definition.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct EncodingSortField {
    /// The data [field](https://vega.github.io/vega-lite/docs/field.html) to sort by.
    ///
    /// __Default value:__ If unspecified, defaults to the field specified in the outer data
    /// reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field: Option<Field>,
    /// An [aggregate operation](https://vega.github.io/vega-lite/docs/aggregate.html#ops) to
    /// perform on the field prior to sorting (e.g., `"count"`, `"mean"` and `"median"`).
    /// An aggregation is required when there are multiple values of the sort field for each
    /// encoded data field.
    /// The input data objects will be aggregated, grouped by the encoded data field.
    ///
    /// For a full list of operations, please see the documentation for
    /// [aggregate](https://vega.github.io/vega-lite/docs/aggregate.html#ops).
    ///
    /// __Default value:__ `"sum"` for stacked plots. Otherwise, `"mean"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub op: Option<NonArgAggregateOp>,
    /// The sort order. One of `"ascending"` (default), `"descending"`, or `null` (no not sort).
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub order: RemovableValue<SortOrder>,
    /// The [encoding channel](https://vega.github.io/vega-lite/docs/encoding.html#channels) to
    /// sort by (e.g., `"x"`, `"y"`)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub encoding: Option<SortByChannel>,
}

/// A field definition for the horizontal facet of trellis plots.
///
/// A field definition for the vertical facet of trellis plots.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct RowColumnEncodingFieldDef {
    /// Aggregation function for the field
    /// (e.g., `"mean"`, `"sum"`, `"median"`, `"min"`, `"max"`, `"count"`).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aggregate: Option<Aggregate>,
    /// The alignment to apply to row/column facet's subplot.
    /// The supported string values are `"all"`, `"each"`, and `"none"`.
    ///
    /// - For `"none"`, a flow layout will be used, in which adjacent subviews are simply placed
    /// one after the other.
    /// - For `"each"`, subviews will be aligned into a clean grid structure, but each row or
    /// column may be of variable size.
    /// - For `"all"`, subviews will be aligned and each row or column will be sized identically
    /// based on the maximum observed size. String values for this property will be applied to
    /// both grid rows and columns.
    ///
    /// __Default value:__ `"all"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub align: Option<LayoutAlign>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - If `"binned"`, this indicates that the data for the `x` (or `y`) channel are already
    /// binned. You can map the bin-start field to `x` (or `y`) and the bin-end field to `x2` (or
    /// `y2`). The scale and axis will be formatted similar to binning in Vega-Lite.  To adjust
    /// the axis ticks based on the bin step, you can also set the axis's
    /// [`tickMinStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    ///
    /// __See also:__ [`bin`](https://vega.github.io/vega-lite/docs/bin.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bin: Option<PurpleBin>,
    /// Boolean flag indicating if facet's subviews should be centered relative to their
    /// respective rows or columns.
    ///
    /// __Default value:__ `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub center: Option<bool>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __See also:__ [`field`](https://vega.github.io/vega-lite/docs/field.html) documentation.
    ///
    /// __Notes:__
    /// 1)  Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects (e.g.,
    /// `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    /// 2) `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field: Option<Field>,
    /// An object defining properties of a facet's header.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub header: Option<Header>,
    /// Sort order for the encoded field.
    ///
    /// For continuous fields (quantitative or temporal), `sort` can be either `"ascending"` or
    /// `"descending"`.
    ///
    /// For discrete fields, `sort` can be one of the following:
    /// - `"ascending"` or `"descending"` -- for sorting by the values' natural order in
    /// JavaScript.
    /// - [A sort field definition](https://vega.github.io/vega-lite/docs/sort.html#sort-field)
    /// for sorting by another field.
    /// - [An array specifying the field values in preferred
    /// order](https://vega.github.io/vega-lite/docs/sort.html#sort-array). In this case, the
    /// sort order will obey the values in the array, followed by any unspecified values in their
    /// original order. For discrete time field, values in the sort array can be [date-time
    /// definition objects](struct.DateTime.html). In addition, for time units `"month"` and `"day"`,
    /// the values can be the month or day names (case insensitive) or their 3-letter initials
    /// (e.g., `"Mon"`, `"Tue"`).
    /// - `null` indicating no sort.
    ///
    /// __Default value:__ `"ascending"`
    ///
    /// __Note:__ `null` is not supported for `row` and `column`.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub sort: RemovableValue<SortArray>,
    /// The spacing in pixels between facet's sub-views.
    ///
    /// __Default value__: Depends on `"spacing"` property of [the view composition
    /// configuration](https://vega.github.io/vega-lite/docs/config.html#view-config) (`20` by
    /// default)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub spacing: Option<f64>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html)
    /// documentation.
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`). If the field has an aggregate function, the function
    /// is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is binned or
    /// has a time unit applied, the applied function is shown in parentheses (e.g., `"Profit
    /// (binned)"`, `"Transaction Date (year-month)"`). Otherwise, the title is simply the field
    /// name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the
    /// [`fieldTitle`](https://vega.github.io/vega-lite/docs/config.html#top-level-config)
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub title: RemovableValue<PurpleText>,
    /// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
    /// `"nominal"`).
    /// It can also be a `"geojson"` type for encoding
    /// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
    ///
    ///
    /// __Note:__
    ///
    /// - Data values for a temporal field can be either a date-time string (e.g., `"2015-03-07
    /// 12:32:17"`, `"17:01"`, `"2015-03-16"`. `"2015"`) or a timestamp number (e.g.,
    /// `1552199579097`).
    /// - Data `type` describes the semantics of the data rather than the primitive data types
    /// (number, string, etc.). The same primitive data type can have different types of
    /// measurement. For example, numeric data can represent quantitative, ordinal, or nominal
    /// data.
    /// - When using with [`bin`](https://vega.github.io/vega-lite/docs/bin.html), the `type`
    /// property can be either `"quantitative"` (for using a linear bin scale) or [`"ordinal"`
    /// (for using an ordinal bin
    /// scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html), the
    /// `type` property can be either `"temporal"` (for using a temporal scale) or [`"ordinal"`
    /// (for using an ordinal scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html),
    /// the `type` property refers to the post-aggregation data type. For example, we can
    /// calculate count `distinct` of a categorical field `"cat"` using `{"aggregate":
    /// "distinct", "field": "cat", "type": "quantitative"}`. The `"type"` of the aggregate
    /// output is `"quantitative"`.
    /// - Secondary channels (e.g., `x2`, `y2`, `xError`, `yError`) do not have `type` as they
    /// have exactly the same type as their primary channels (e.g., `x`, `y`).
    ///
    /// __See also:__ [`type`](https://vega.github.io/vega-lite/docs/type.html) documentation.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub row_column_encoding_field_def_type: Option<StandardType>,
}

/// An object defining properties of a facet's header.
///
/// Headers of row / column channels for faceted plots.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct Header {
    /// The text formatting pattern for labels of guides (axes, legends, headers) and text
    /// marks.
    ///
    /// - If the format type is `"number"` (e.g., for quantitative fields), this is D3's [number
    /// format pattern](https://github.com/d3/d3-format#locale_format).
    /// - If the format type is `"time"` (e.g., for temporal fields), this is D3's [time format
    /// pattern](https://github.com/d3/d3-time-format#locale_format).
    ///
    /// See the [format documentation](https://vega.github.io/vega-lite/docs/format.html) for
    /// more examples.
    ///
    /// __Default value:__  Derived from
    /// [numberFormat](https://vega.github.io/vega-lite/docs/config.html#format) config for
    /// number format and from
    /// [timeFormat](https://vega.github.io/vega-lite/docs/config.html#format) config for time
    /// format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub format: Option<String>,
    /// The format type for labels (`"number"` or `"time"`).
    ///
    /// __Default value:__
    /// - `"time"` for temporal fields and ordinal and nomimal fields with `timeUnit`.
    /// - `"number"` for quantitative fields as well as ordinal and nomimal fields without
    /// `timeUnit`.
    #[serde(rename = "formatType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub format_type: Option<FormatType>,
    /// Horizontal text alignment of header labels. One of `"left"`, `"center"`, or `"right"`.
    #[serde(rename = "labelAlign")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_align: Option<Align>,
    /// The anchor position for placing the labels. One of `"start"`, `"middle"`, or `"end"`. For
    /// example, with a label orientation of top these anchor positions map to a left-, center-,
    /// or right-aligned label.
    #[serde(rename = "labelAnchor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_anchor: Option<TitleAnchorEnum>,
    /// The rotation angle of the header labels.
    ///
    /// __Default value:__ `0` for column header, `-90` for row header.
    #[serde(rename = "labelAngle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_angle: Option<f64>,
    /// The color of the header label, can be in hex color code or regular color name.
    #[serde(rename = "labelColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_color: Option<String>,
    /// [Vega expression](https://vega.github.io/vega/docs/expressions/) for customizing labels.
    ///
    /// __Note:__ The label text and value can be assessed via the `label` and `value` properties
    /// of the header's backing `datum` object.
    #[serde(rename = "labelExpr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_expr: Option<String>,
    /// The font of the header label.
    #[serde(rename = "labelFont")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_font: Option<String>,
    /// The font size of the header label, in pixels.
    #[serde(rename = "labelFontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_font_size: Option<f64>,
    /// The font style of the header label.
    #[serde(rename = "labelFontStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_font_style: Option<String>,
    /// The maximum length of the header label in pixels. The text value will be automatically
    /// truncated if the rendered size exceeds the limit.
    ///
    /// __Default value:__ `0`, indicating no limit
    #[serde(rename = "labelLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_limit: Option<f64>,
    /// The orientation of the header label. One of `"top"`, `"bottom"`, `"left"` or `"right"`.
    #[serde(rename = "labelOrient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_orient: Option<Orient>,
    /// The padding, in pixel, between facet header's label and the plot.
    ///
    /// __Default value:__ `10`
    #[serde(rename = "labelPadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_padding: Option<f64>,
    /// A boolean flag indicating if labels should be included as part of the header.
    ///
    /// __Default value:__ `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub labels: Option<bool>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`). If the field has an aggregate function, the function
    /// is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is binned or
    /// has a time unit applied, the applied function is shown in parentheses (e.g., `"Profit
    /// (binned)"`, `"Transaction Date (year-month)"`). Otherwise, the title is simply the field
    /// name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the
    /// [`fieldTitle`](https://vega.github.io/vega-lite/docs/config.html#top-level-config)
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub title: RemovableValue<PurpleText>,
    /// Horizontal text alignment (to the anchor) of header titles.
    #[serde(rename = "titleAlign")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_align: Option<Align>,
    /// The anchor position for placing the title. One of `"start"`, `"middle"`, or `"end"`. For
    /// example, with an orientation of top these anchor positions map to a left-, center-, or
    /// right-aligned title.
    #[serde(rename = "titleAnchor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_anchor: Option<TitleAnchorEnum>,
    /// The rotation angle of the header title.
    ///
    /// __Default value:__ `0`.
    #[serde(rename = "titleAngle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_angle: Option<f64>,
    /// Vertical text baseline for the header title. One of `"top"`, `"bottom"`, `"middle"`.
    ///
    /// __Default value:__ `"middle"`
    #[serde(rename = "titleBaseline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_baseline: Option<Baseline>,
    /// Color of the header title, can be in hex color code or regular color name.
    #[serde(rename = "titleColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_color: Option<String>,
    /// Font of the header title. (e.g., `"Helvetica Neue"`).
    #[serde(rename = "titleFont")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_font: Option<String>,
    /// Font size of the header title.
    #[serde(rename = "titleFontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_font_size: Option<f64>,
    /// The font style of the header title.
    #[serde(rename = "titleFontStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_font_style: Option<String>,
    /// Font weight of the header title.
    /// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
    /// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
    #[serde(rename = "titleFontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_font_weight: Option<FontWeight>,
    /// The maximum length of the header title in pixels. The text value will be automatically
    /// truncated if the rendered size exceeds the limit.
    ///
    /// __Default value:__ `0`, indicating no limit
    #[serde(rename = "titleLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_limit: Option<f64>,
    /// Line height in pixels for multi-line title text.
    #[serde(rename = "titleLineHeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_line_height: Option<f64>,
    /// The orientation of the header title. One of `"top"`, `"bottom"`, `"left"` or `"right"`.
    #[serde(rename = "titleOrient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_orient: Option<Orient>,
    /// The padding, in pixel, between facet header's title and the label.
    ///
    /// __Default value:__ `10`
    #[serde(rename = "titlePadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_padding: Option<f64>,
}

/// A sort definition for sorting a discrete scale in an encoding field definition.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct SortEncodingSortField {
    /// The data [field](https://vega.github.io/vega-lite/docs/field.html) to sort by.
    ///
    /// __Default value:__ If unspecified, defaults to the field specified in the outer data
    /// reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field: Option<Field>,
    /// An [aggregate operation](https://vega.github.io/vega-lite/docs/aggregate.html#ops) to
    /// perform on the field prior to sorting (e.g., `"count"`, `"mean"` and `"median"`).
    /// An aggregation is required when there are multiple values of the sort field for each
    /// encoded data field.
    /// The input data objects will be aggregated, grouped by the encoded data field.
    ///
    /// For a full list of operations, please see the documentation for
    /// [aggregate](https://vega.github.io/vega-lite/docs/aggregate.html#ops).
    ///
    /// __Default value:__ `"sum"` for stacked plots. Otherwise, `"mean"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub op: Option<NonArgAggregateOp>,
    /// The sort order. One of `"ascending"` (default), `"descending"`, or `null` (no not sort).
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub order: RemovableValue<SortOrder>,
}

/// Field Def without scale (and without bin: "binned" support).
///
/// Definition object for a data field, its type and transformation of an encoding channel.
///
/// A data field to use as a unique key for data binding. When a visualization’s data is
/// updated, the key value will be used to match data elements to existing mark instances.
/// Use a key channel to enable object constancy for transitions over dynamic data.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct TypedFieldDef {
    /// Aggregation function for the field
    /// (e.g., `"mean"`, `"sum"`, `"median"`, `"min"`, `"max"`, `"count"`).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aggregate: Option<Aggregate>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - If `"binned"`, this indicates that the data for the `x` (or `y`) channel are already
    /// binned. You can map the bin-start field to `x` (or `y`) and the bin-end field to `x2` (or
    /// `y2`). The scale and axis will be formatted similar to binning in Vega-Lite.  To adjust
    /// the axis ticks based on the bin step, you can also set the axis's
    /// [`tickMinStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    ///
    /// __See also:__ [`bin`](https://vega.github.io/vega-lite/docs/bin.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bin: Option<FluffyBin>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __See also:__ [`field`](https://vega.github.io/vega-lite/docs/field.html) documentation.
    ///
    /// __Notes:__
    /// 1)  Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects (e.g.,
    /// `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    /// 2) `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field: Option<Field>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html)
    /// documentation.
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`). If the field has an aggregate function, the function
    /// is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is binned or
    /// has a time unit applied, the applied function is shown in parentheses (e.g., `"Profit
    /// (binned)"`, `"Transaction Date (year-month)"`). Otherwise, the title is simply the field
    /// name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the
    /// [`fieldTitle`](https://vega.github.io/vega-lite/docs/config.html#top-level-config)
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub title: RemovableValue<PurpleText>,
    /// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
    /// `"nominal"`).
    /// It can also be a `"geojson"` type for encoding
    /// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
    ///
    ///
    /// __Note:__
    ///
    /// - Data values for a temporal field can be either a date-time string (e.g., `"2015-03-07
    /// 12:32:17"`, `"17:01"`, `"2015-03-16"`. `"2015"`) or a timestamp number (e.g.,
    /// `1552199579097`).
    /// - Data `type` describes the semantics of the data rather than the primitive data types
    /// (number, string, etc.). The same primitive data type can have different types of
    /// measurement. For example, numeric data can represent quantitative, ordinal, or nominal
    /// data.
    /// - When using with [`bin`](https://vega.github.io/vega-lite/docs/bin.html), the `type`
    /// property can be either `"quantitative"` (for using a linear bin scale) or [`"ordinal"`
    /// (for using an ordinal bin
    /// scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html), the
    /// `type` property can be either `"temporal"` (for using a temporal scale) or [`"ordinal"`
    /// (for using an ordinal scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html),
    /// the `type` property refers to the post-aggregation data type. For example, we can
    /// calculate count `distinct` of a categorical field `"cat"` using `{"aggregate":
    /// "distinct", "field": "cat", "type": "quantitative"}`. The `"type"` of the aggregate
    /// output is `"quantitative"`.
    /// - Secondary channels (e.g., `x2`, `y2`, `xError`, `yError`) do not have `type` as they
    /// have exactly the same type as their primary channels (e.g., `x`, `y`).
    ///
    /// __See also:__ [`type`](https://vega.github.io/vega-lite/docs/type.html) documentation.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub typed_field_def_type: Option<StandardType>,
}

/// A field definition for the (flexible) facet of trellis plots.
///
/// If either `row` or `column` is specified, this channel will be ignored.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct FacetEncodingFieldDef {
    /// Aggregation function for the field
    /// (e.g., `"mean"`, `"sum"`, `"median"`, `"min"`, `"max"`, `"count"`).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aggregate: Option<Aggregate>,
    /// The alignment to apply to grid rows and columns.
    /// The supported string values are `"all"`, `"each"`, and `"none"`.
    ///
    /// - For `"none"`, a flow layout will be used, in which adjacent subviews are simply placed
    /// one after the other.
    /// - For `"each"`, subviews will be aligned into a clean grid structure, but each row or
    /// column may be of variable size.
    /// - For `"all"`, subviews will be aligned and each row or column will be sized identically
    /// based on the maximum observed size. String values for this property will be applied to
    /// both grid rows and columns.
    ///
    /// Alternatively, an object value of the form `{"row": string, "column": string}` can be
    /// used to supply different alignments for rows and columns.
    ///
    /// __Default value:__ `"all"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub align: Option<AlignUnion>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - If `"binned"`, this indicates that the data for the `x` (or `y`) channel are already
    /// binned. You can map the bin-start field to `x` (or `y`) and the bin-end field to `x2` (or
    /// `y2`). The scale and axis will be formatted similar to binning in Vega-Lite.  To adjust
    /// the axis ticks based on the bin step, you can also set the axis's
    /// [`tickMinStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    ///
    /// __See also:__ [`bin`](https://vega.github.io/vega-lite/docs/bin.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bin: Option<PurpleBin>,
    /// The bounds calculation method to use for determining the extent of a sub-plot. One of
    /// `full` (the default) or `flush`.
    ///
    /// - If set to `full`, the entire calculated bounds (including axes, title, and legend) will
    /// be used.
    /// - If set to `flush`, only the specified width and height values for the sub-view will be
    /// used. The `flush` setting can be useful when attempting to place sub-plots without axes
    /// or legends into a uniform grid structure.
    ///
    /// __Default value:__ `"full"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bounds: Option<BoundsEnum>,
    /// Boolean flag indicating if subviews should be centered relative to their respective rows
    /// or columns.
    ///
    /// An object value of the form `{"row": boolean, "column": boolean}` can be used to supply
    /// different centering values for rows and columns.
    ///
    /// __Default value:__ `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub center: Option<VegaliteCenter>,
    /// The number of columns to include in the view composition layout.
    ///
    /// __Default value__: `undefined` -- An infinite number of columns (a single row) will be
    /// assumed. This is equivalent to
    /// `hconcat` (for `concat`) and to using the `column` channel (for `facet` and `repeat`).
    ///
    /// __Note__:
    ///
    /// 1) This property is only for:
    /// - the general (wrappable) `concat` operator (not `hconcat`/`vconcat`)
    /// - the `facet` and `repeat` operator with one field/repetition definition (without
    /// row/column nesting)
    ///
    /// 2) Setting the `columns` to `1` is equivalent to `vconcat` (for `concat`) and to using
    /// the `row` channel (for `facet` and `repeat`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub columns: Option<f64>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __See also:__ [`field`](https://vega.github.io/vega-lite/docs/field.html) documentation.
    ///
    /// __Notes:__
    /// 1)  Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects (e.g.,
    /// `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    /// 2) `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field: Option<Field>,
    /// An object defining properties of a facet's header.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub header: Option<Header>,
    /// Sort order for the encoded field.
    ///
    /// For continuous fields (quantitative or temporal), `sort` can be either `"ascending"` or
    /// `"descending"`.
    ///
    /// For discrete fields, `sort` can be one of the following:
    /// - `"ascending"` or `"descending"` -- for sorting by the values' natural order in
    /// JavaScript.
    /// - [A sort field definition](https://vega.github.io/vega-lite/docs/sort.html#sort-field)
    /// for sorting by another field.
    /// - [An array specifying the field values in preferred
    /// order](https://vega.github.io/vega-lite/docs/sort.html#sort-array). In this case, the
    /// sort order will obey the values in the array, followed by any unspecified values in their
    /// original order. For discrete time field, values in the sort array can be [date-time
    /// definition objects](struct.DateTime.html). In addition, for time units `"month"` and `"day"`,
    /// the values can be the month or day names (case insensitive) or their 3-letter initials
    /// (e.g., `"Mon"`, `"Tue"`).
    /// - `null` indicating no sort.
    ///
    /// __Default value:__ `"ascending"`
    ///
    /// __Note:__ `null` is not supported for `row` and `column`.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub sort: RemovableValue<SortArray>,
    /// The spacing in pixels between sub-views of the composition operator.
    /// An object of the form `{"row": number, "column": number}` can be used to set
    /// different spacing values for rows and columns.
    ///
    /// __Default value__: Depends on `"spacing"` property of [the view composition
    /// configuration](https://vega.github.io/vega-lite/docs/config.html#view-config) (`20` by
    /// default)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub spacing: Option<Spacing>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html)
    /// documentation.
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`). If the field has an aggregate function, the function
    /// is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is binned or
    /// has a time unit applied, the applied function is shown in parentheses (e.g., `"Profit
    /// (binned)"`, `"Transaction Date (year-month)"`). Otherwise, the title is simply the field
    /// name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the
    /// [`fieldTitle`](https://vega.github.io/vega-lite/docs/config.html#top-level-config)
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub title: RemovableValue<PurpleText>,
    /// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
    /// `"nominal"`).
    /// It can also be a `"geojson"` type for encoding
    /// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
    ///
    ///
    /// __Note:__
    ///
    /// - Data values for a temporal field can be either a date-time string (e.g., `"2015-03-07
    /// 12:32:17"`, `"17:01"`, `"2015-03-16"`. `"2015"`) or a timestamp number (e.g.,
    /// `1552199579097`).
    /// - Data `type` describes the semantics of the data rather than the primitive data types
    /// (number, string, etc.). The same primitive data type can have different types of
    /// measurement. For example, numeric data can represent quantitative, ordinal, or nominal
    /// data.
    /// - When using with [`bin`](https://vega.github.io/vega-lite/docs/bin.html), the `type`
    /// property can be either `"quantitative"` (for using a linear bin scale) or [`"ordinal"`
    /// (for using an ordinal bin
    /// scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html), the
    /// `type` property can be either `"temporal"` (for using a temporal scale) or [`"ordinal"`
    /// (for using an ordinal scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html),
    /// the `type` property refers to the post-aggregation data type. For example, we can
    /// calculate count `distinct` of a categorical field `"cat"` using `{"aggregate":
    /// "distinct", "field": "cat", "type": "quantitative"}`. The `"type"` of the aggregate
    /// output is `"quantitative"`.
    /// - Secondary channels (e.g., `x2`, `y2`, `xError`, `yError`) do not have `type` as they
    /// have exactly the same type as their primary channels (e.g., `x`, `y`).
    ///
    /// __See also:__ [`type`](https://vega.github.io/vega-lite/docs/type.html) documentation.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub facet_encoding_field_def_type: Option<StandardType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct RowColNumber {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub column: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub row: Option<f64>,
}

/// Fill opacity of the marks.
///
/// __Default value:__ If undefined, the default opacity depends on [mark
/// config](https://vega.github.io/vega-lite/docs/config.html#mark)'s `fillOpacity`
/// property.
///
/// Opacity of the marks.
///
/// __Default value:__ If undefined, the default opacity depends on [mark
/// config](https://vega.github.io/vega-lite/docs/config.html#mark)'s `opacity` property.
///
/// Size of the mark.
/// - For `"point"`, `"square"` and `"circle"`, – the symbol size, or pixel area of the mark.
/// - For `"bar"` and `"tick"` – the bar and tick's size.
/// - For `"text"` – the text's font size.
/// - Size is unsupported for `"line"`, `"area"`, and `"rect"`. (Use `"trail"` instead of
/// line with varying size)
///
/// Stroke opacity of the marks.
///
/// __Default value:__ If undefined, the default opacity depends on [mark
/// config](https://vega.github.io/vega-lite/docs/config.html#mark)'s `strokeOpacity`
/// property.
///
/// Stroke width of the marks.
///
/// __Default value:__ If undefined, the default stroke width depends on [mark
/// config](https://vega.github.io/vega-lite/docs/config.html#mark)'s `strokeWidth`
/// property.
///
/// A FieldDef with Condition<ValueDef>
/// {
/// condition: {value: ...},
/// field: ...,
/// ...
/// }
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct DefWithConditionMarkPropFieldDefNumber {
    /// Aggregation function for the field
    /// (e.g., `"mean"`, `"sum"`, `"median"`, `"min"`, `"max"`, `"count"`).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aggregate: Option<Aggregate>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - If `"binned"`, this indicates that the data for the `x` (or `y`) channel are already
    /// binned. You can map the bin-start field to `x` (or `y`) and the bin-end field to `x2` (or
    /// `y2`). The scale and axis will be formatted similar to binning in Vega-Lite.  To adjust
    /// the axis ticks based on the bin step, you can also set the axis's
    /// [`tickMinStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    ///
    /// __See also:__ [`bin`](https://vega.github.io/vega-lite/docs/bin.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bin: Option<PurpleBin>,
    /// One or more value definition(s) with [a selection or a test
    /// predicate](https://vega.github.io/vega-lite/docs/condition.html).
    ///
    /// __Note:__ A field definition's `condition` property can only contain [conditional value
    /// definitions](https://vega.github.io/vega-lite/docs/condition.html#value)
    /// since Vega-Lite only allows at most one encoded field per encoding channel.
    ///
    /// A field definition or one or more value definition(s) with a selection predicate.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub condition: Option<ConditionUnion>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __See also:__ [`field`](https://vega.github.io/vega-lite/docs/field.html) documentation.
    ///
    /// __Notes:__
    /// 1)  Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects (e.g.,
    /// `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    /// 2) `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field: Option<Field>,
    /// An object defining properties of the legend.
    /// If `null`, the legend for the encoding channel will be removed.
    ///
    /// __Default value:__ If undefined, default [legend
    /// properties](https://vega.github.io/vega-lite/docs/legend.html) are applied.
    ///
    /// __See also:__ [`legend`](https://vega.github.io/vega-lite/docs/legend.html) documentation.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub legend: RemovableValue<Legend>,
    /// An object defining properties of the channel's scale, which is the function that
    /// transforms values in the data domain (numbers, dates, strings, etc) to visual values
    /// (pixels, colors, sizes) of the encoding channels.
    ///
    /// If `null`, the scale will be [disabled and the data value will be directly
    /// encoded](https://vega.github.io/vega-lite/docs/scale.html#disable).
    ///
    /// __Default value:__ If undefined, default [scale
    /// properties](https://vega.github.io/vega-lite/docs/scale.html) are applied.
    ///
    /// __See also:__ [`scale`](https://vega.github.io/vega-lite/docs/scale.html) documentation.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub scale: RemovableValue<Scale>,
    /// Sort order for the encoded field.
    ///
    /// For continuous fields (quantitative or temporal), `sort` can be either `"ascending"` or
    /// `"descending"`.
    ///
    /// For discrete fields, `sort` can be one of the following:
    /// - `"ascending"` or `"descending"` -- for sorting by the values' natural order in
    /// JavaScript.
    /// - [A string indicating an encoding channel name to sort
    /// by](https://vega.github.io/vega-lite/docs/sort.html#sort-by-encoding) (e.g., `"x"` or
    /// `"y"`) with an optional minus prefix for descending sort (e.g., `"-x"` to sort by
    /// x-field, descending). This channel string is short-form of [a sort-by-encoding
    /// definition](https://vega.github.io/vega-lite/docs/sort.html#sort-by-encoding). For
    /// example, `"sort": "-x"` is equivalent to `"sort": {"encoding": "x", "order":
    /// "descending"}`.
    /// - [A sort field definition](https://vega.github.io/vega-lite/docs/sort.html#sort-field)
    /// for sorting by another field.
    /// - [An array specifying the field values in preferred
    /// order](https://vega.github.io/vega-lite/docs/sort.html#sort-array). In this case, the
    /// sort order will obey the values in the array, followed by any unspecified values in their
    /// original order. For discrete time field, values in the sort array can be [date-time
    /// definition objects](struct.DateTime.html). In addition, for time units `"month"` and `"day"`,
    /// the values can be the month or day names (case insensitive) or their 3-letter initials
    /// (e.g., `"Mon"`, `"Tue"`).
    /// - `null` indicating no sort.
    ///
    /// __Default value:__ `"ascending"`
    ///
    /// __Note:__ `null` and sorting by another channel is not supported for `row` and `column`.
    ///
    /// __See also:__ [`sort`](https://vega.github.io/vega-lite/docs/sort.html) documentation.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub sort: RemovableValue<SortUnion>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html)
    /// documentation.
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`). If the field has an aggregate function, the function
    /// is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is binned or
    /// has a time unit applied, the applied function is shown in parentheses (e.g., `"Profit
    /// (binned)"`, `"Transaction Date (year-month)"`). Otherwise, the title is simply the field
    /// name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the
    /// [`fieldTitle`](https://vega.github.io/vega-lite/docs/config.html#top-level-config)
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub title: RemovableValue<PurpleText>,
    /// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
    /// `"nominal"`).
    /// It can also be a `"geojson"` type for encoding
    /// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
    ///
    ///
    /// __Note:__
    ///
    /// - Data values for a temporal field can be either a date-time string (e.g., `"2015-03-07
    /// 12:32:17"`, `"17:01"`, `"2015-03-16"`. `"2015"`) or a timestamp number (e.g.,
    /// `1552199579097`).
    /// - Data `type` describes the semantics of the data rather than the primitive data types
    /// (number, string, etc.). The same primitive data type can have different types of
    /// measurement. For example, numeric data can represent quantitative, ordinal, or nominal
    /// data.
    /// - When using with [`bin`](https://vega.github.io/vega-lite/docs/bin.html), the `type`
    /// property can be either `"quantitative"` (for using a linear bin scale) or [`"ordinal"`
    /// (for using an ordinal bin
    /// scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html), the
    /// `type` property can be either `"temporal"` (for using a temporal scale) or [`"ordinal"`
    /// (for using an ordinal scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html),
    /// the `type` property refers to the post-aggregation data type. For example, we can
    /// calculate count `distinct` of a categorical field `"cat"` using `{"aggregate":
    /// "distinct", "field": "cat", "type": "quantitative"}`. The `"type"` of the aggregate
    /// output is `"quantitative"`.
    /// - Secondary channels (e.g., `x2`, `y2`, `xError`, `yError`) do not have `type` as they
    /// have exactly the same type as their primary channels (e.g., `x`, `y`).
    ///
    /// __See also:__ [`type`](https://vega.github.io/vega-lite/docs/type.html) documentation.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub def_with_condition_mark_prop_field_def_number_type: Option<StandardType>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ConditionalNumberValueDef {
    /// Predicate for triggering the condition
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub test: Option<PurpleLogicalOperandPredicate>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<f64>,
    /// A [selection name](https://vega.github.io/vega-lite/docs/selection.html), or a series of
    /// [composed selections](https://vega.github.io/vega-lite/docs/selection.html#compose).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub selection: Option<Box<PurpleSelectionOperand>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ConditionalDef {
    /// Predicate for triggering the condition
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub test: Option<PurpleLogicalOperandPredicate>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<f64>,
    /// A [selection name](https://vega.github.io/vega-lite/docs/selection.html), or a series of
    /// [composed selections](https://vega.github.io/vega-lite/docs/selection.html#compose).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub selection: Option<Box<PurpleSelectionOperand>>,
    /// Aggregation function for the field
    /// (e.g., `"mean"`, `"sum"`, `"median"`, `"min"`, `"max"`, `"count"`).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aggregate: Option<Aggregate>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - If `"binned"`, this indicates that the data for the `x` (or `y`) channel are already
    /// binned. You can map the bin-start field to `x` (or `y`) and the bin-end field to `x2` (or
    /// `y2`). The scale and axis will be formatted similar to binning in Vega-Lite.  To adjust
    /// the axis ticks based on the bin step, you can also set the axis's
    /// [`tickMinStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    ///
    /// __See also:__ [`bin`](https://vega.github.io/vega-lite/docs/bin.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bin: Option<PurpleBin>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __See also:__ [`field`](https://vega.github.io/vega-lite/docs/field.html) documentation.
    ///
    /// __Notes:__
    /// 1)  Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects (e.g.,
    /// `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    /// 2) `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field: Option<Field>,
    /// An object defining properties of the legend.
    /// If `null`, the legend for the encoding channel will be removed.
    ///
    /// __Default value:__ If undefined, default [legend
    /// properties](https://vega.github.io/vega-lite/docs/legend.html) are applied.
    ///
    /// __See also:__ [`legend`](https://vega.github.io/vega-lite/docs/legend.html) documentation.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub legend: RemovableValue<Legend>,
    /// An object defining properties of the channel's scale, which is the function that
    /// transforms values in the data domain (numbers, dates, strings, etc) to visual values
    /// (pixels, colors, sizes) of the encoding channels.
    ///
    /// If `null`, the scale will be [disabled and the data value will be directly
    /// encoded](https://vega.github.io/vega-lite/docs/scale.html#disable).
    ///
    /// __Default value:__ If undefined, default [scale
    /// properties](https://vega.github.io/vega-lite/docs/scale.html) are applied.
    ///
    /// __See also:__ [`scale`](https://vega.github.io/vega-lite/docs/scale.html) documentation.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub scale: RemovableValue<Scale>,
    /// Sort order for the encoded field.
    ///
    /// For continuous fields (quantitative or temporal), `sort` can be either `"ascending"` or
    /// `"descending"`.
    ///
    /// For discrete fields, `sort` can be one of the following:
    /// - `"ascending"` or `"descending"` -- for sorting by the values' natural order in
    /// JavaScript.
    /// - [A string indicating an encoding channel name to sort
    /// by](https://vega.github.io/vega-lite/docs/sort.html#sort-by-encoding) (e.g., `"x"` or
    /// `"y"`) with an optional minus prefix for descending sort (e.g., `"-x"` to sort by
    /// x-field, descending). This channel string is short-form of [a sort-by-encoding
    /// definition](https://vega.github.io/vega-lite/docs/sort.html#sort-by-encoding). For
    /// example, `"sort": "-x"` is equivalent to `"sort": {"encoding": "x", "order":
    /// "descending"}`.
    /// - [A sort field definition](https://vega.github.io/vega-lite/docs/sort.html#sort-field)
    /// for sorting by another field.
    /// - [An array specifying the field values in preferred
    /// order](https://vega.github.io/vega-lite/docs/sort.html#sort-array). In this case, the
    /// sort order will obey the values in the array, followed by any unspecified values in their
    /// original order. For discrete time field, values in the sort array can be [date-time
    /// definition objects](struct.DateTime.html). In addition, for time units `"month"` and `"day"`,
    /// the values can be the month or day names (case insensitive) or their 3-letter initials
    /// (e.g., `"Mon"`, `"Tue"`).
    /// - `null` indicating no sort.
    ///
    /// __Default value:__ `"ascending"`
    ///
    /// __Note:__ `null` and sorting by another channel is not supported for `row` and `column`.
    ///
    /// __See also:__ [`sort`](https://vega.github.io/vega-lite/docs/sort.html) documentation.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub sort: RemovableValue<SortUnion>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html)
    /// documentation.
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`). If the field has an aggregate function, the function
    /// is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is binned or
    /// has a time unit applied, the applied function is shown in parentheses (e.g., `"Profit
    /// (binned)"`, `"Transaction Date (year-month)"`). Otherwise, the title is simply the field
    /// name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the
    /// [`fieldTitle`](https://vega.github.io/vega-lite/docs/config.html#top-level-config)
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub title: RemovableValue<PurpleText>,
    /// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
    /// `"nominal"`).
    /// It can also be a `"geojson"` type for encoding
    /// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
    ///
    ///
    /// __Note:__
    ///
    /// - Data values for a temporal field can be either a date-time string (e.g., `"2015-03-07
    /// 12:32:17"`, `"17:01"`, `"2015-03-16"`. `"2015"`) or a timestamp number (e.g.,
    /// `1552199579097`).
    /// - Data `type` describes the semantics of the data rather than the primitive data types
    /// (number, string, etc.). The same primitive data type can have different types of
    /// measurement. For example, numeric data can represent quantitative, ordinal, or nominal
    /// data.
    /// - When using with [`bin`](https://vega.github.io/vega-lite/docs/bin.html), the `type`
    /// property can be either `"quantitative"` (for using a linear bin scale) or [`"ordinal"`
    /// (for using an ordinal bin
    /// scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html), the
    /// `type` property can be either `"temporal"` (for using a temporal scale) or [`"ordinal"`
    /// (for using an ordinal scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html),
    /// the `type` property refers to the post-aggregation data type. For example, we can
    /// calculate count `distinct` of a categorical field `"cat"` using `{"aggregate":
    /// "distinct", "field": "cat", "type": "quantitative"}`. The `"type"` of the aggregate
    /// output is `"quantitative"`.
    /// - Secondary channels (e.g., `x2`, `y2`, `xError`, `yError`) do not have `type` as they
    /// have exactly the same type as their primary channels (e.g., `x`, `y`).
    ///
    /// __See also:__ [`type`](https://vega.github.io/vega-lite/docs/type.html) documentation.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub conditional_def_type: Option<StandardType>,
}

/// A URL to load upon mouse click.
///
/// The URL of an image mark.
///
/// A FieldDef with Condition<ValueDef>
/// {
/// condition: {value: ...},
/// field: ...,
/// ...
/// }
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct HrefClass {
    /// Aggregation function for the field
    /// (e.g., `"mean"`, `"sum"`, `"median"`, `"min"`, `"max"`, `"count"`).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aggregate: Option<Aggregate>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - If `"binned"`, this indicates that the data for the `x` (or `y`) channel are already
    /// binned. You can map the bin-start field to `x` (or `y`) and the bin-end field to `x2` (or
    /// `y2`). The scale and axis will be formatted similar to binning in Vega-Lite.  To adjust
    /// the axis ticks based on the bin step, you can also set the axis's
    /// [`tickMinStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    ///
    /// __See also:__ [`bin`](https://vega.github.io/vega-lite/docs/bin.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bin: Option<FluffyBin>,
    /// One or more value definition(s) with [a selection or a test
    /// predicate](https://vega.github.io/vega-lite/docs/condition.html).
    ///
    /// __Note:__ A field definition's `condition` property can only contain [conditional value
    /// definitions](https://vega.github.io/vega-lite/docs/condition.html#value)
    /// since Vega-Lite only allows at most one encoded field per encoding channel.
    ///
    /// A field definition or one or more value definition(s) with a selection predicate.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub condition: Option<HrefCondition>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __See also:__ [`field`](https://vega.github.io/vega-lite/docs/field.html) documentation.
    ///
    /// __Notes:__
    /// 1)  Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects (e.g.,
    /// `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    /// 2) `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field: Option<Field>,
    /// The text formatting pattern for labels of guides (axes, legends, headers) and text
    /// marks.
    ///
    /// - If the format type is `"number"` (e.g., for quantitative fields), this is D3's [number
    /// format pattern](https://github.com/d3/d3-format#locale_format).
    /// - If the format type is `"time"` (e.g., for temporal fields), this is D3's [time format
    /// pattern](https://github.com/d3/d3-time-format#locale_format).
    ///
    /// See the [format documentation](https://vega.github.io/vega-lite/docs/format.html) for
    /// more examples.
    ///
    /// __Default value:__  Derived from
    /// [numberFormat](https://vega.github.io/vega-lite/docs/config.html#format) config for
    /// number format and from
    /// [timeFormat](https://vega.github.io/vega-lite/docs/config.html#format) config for time
    /// format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub format: Option<String>,
    /// The format type for labels (`"number"` or `"time"`).
    ///
    /// __Default value:__
    /// - `"time"` for temporal fields and ordinal and nomimal fields with `timeUnit`.
    /// - `"number"` for quantitative fields as well as ordinal and nomimal fields without
    /// `timeUnit`.
    #[serde(rename = "formatType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub format_type: Option<FormatType>,
    /// [Vega expression](https://vega.github.io/vega/docs/expressions/) for customizing labels
    /// text.
    ///
    /// __Note:__ The label text and value can be assessed via the `label` and `value` properties
    /// of the axis's backing `datum` object.
    #[serde(rename = "labelExpr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_expr: Option<String>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html)
    /// documentation.
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`). If the field has an aggregate function, the function
    /// is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is binned or
    /// has a time unit applied, the applied function is shown in parentheses (e.g., `"Profit
    /// (binned)"`, `"Transaction Date (year-month)"`). Otherwise, the title is simply the field
    /// name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the
    /// [`fieldTitle`](https://vega.github.io/vega-lite/docs/config.html#top-level-config)
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub title: RemovableValue<PurpleText>,
    /// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
    /// `"nominal"`).
    /// It can also be a `"geojson"` type for encoding
    /// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
    ///
    ///
    /// __Note:__
    ///
    /// - Data values for a temporal field can be either a date-time string (e.g., `"2015-03-07
    /// 12:32:17"`, `"17:01"`, `"2015-03-16"`. `"2015"`) or a timestamp number (e.g.,
    /// `1552199579097`).
    /// - Data `type` describes the semantics of the data rather than the primitive data types
    /// (number, string, etc.). The same primitive data type can have different types of
    /// measurement. For example, numeric data can represent quantitative, ordinal, or nominal
    /// data.
    /// - When using with [`bin`](https://vega.github.io/vega-lite/docs/bin.html), the `type`
    /// property can be either `"quantitative"` (for using a linear bin scale) or [`"ordinal"`
    /// (for using an ordinal bin
    /// scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html), the
    /// `type` property can be either `"temporal"` (for using a temporal scale) or [`"ordinal"`
    /// (for using an ordinal scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html),
    /// the `type` property refers to the post-aggregation data type. For example, we can
    /// calculate count `distinct` of a categorical field `"cat"` using `{"aggregate":
    /// "distinct", "field": "cat", "type": "quantitative"}`. The `"type"` of the aggregate
    /// output is `"quantitative"`.
    /// - Secondary channels (e.g., `x2`, `y2`, `xError`, `yError`) do not have `type` as they
    /// have exactly the same type as their primary channels (e.g., `x`, `y`).
    ///
    /// __See also:__ [`type`](https://vega.github.io/vega-lite/docs/type.html) documentation.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field_def_with_condition_string_field_def_string_type: Option<StandardType>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ConditionElement {
    /// Predicate for triggering the condition
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub test: Option<PurpleLogicalOperandPredicate>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<String>,
    /// A [selection name](https://vega.github.io/vega-lite/docs/selection.html), or a series of
    /// [composed selections](https://vega.github.io/vega-lite/docs/selection.html#compose).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub selection: Option<Box<PurpleSelectionOperand>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ConditionalPredicateValueDefStringClass {
    /// Predicate for triggering the condition
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub test: Option<PurpleLogicalOperandPredicate>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<String>,
    /// A [selection name](https://vega.github.io/vega-lite/docs/selection.html), or a series of
    /// [composed selections](https://vega.github.io/vega-lite/docs/selection.html#compose).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub selection: Option<Box<PurpleSelectionOperand>>,
    /// Aggregation function for the field
    /// (e.g., `"mean"`, `"sum"`, `"median"`, `"min"`, `"max"`, `"count"`).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aggregate: Option<Aggregate>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - If `"binned"`, this indicates that the data for the `x` (or `y`) channel are already
    /// binned. You can map the bin-start field to `x` (or `y`) and the bin-end field to `x2` (or
    /// `y2`). The scale and axis will be formatted similar to binning in Vega-Lite.  To adjust
    /// the axis ticks based on the bin step, you can also set the axis's
    /// [`tickMinStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    ///
    /// __See also:__ [`bin`](https://vega.github.io/vega-lite/docs/bin.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bin: Option<PurpleBin>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __See also:__ [`field`](https://vega.github.io/vega-lite/docs/field.html) documentation.
    ///
    /// __Notes:__
    /// 1)  Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects (e.g.,
    /// `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    /// 2) `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field: Option<Field>,
    /// An object defining properties of the legend.
    /// If `null`, the legend for the encoding channel will be removed.
    ///
    /// __Default value:__ If undefined, default [legend
    /// properties](https://vega.github.io/vega-lite/docs/legend.html) are applied.
    ///
    /// __See also:__ [`legend`](https://vega.github.io/vega-lite/docs/legend.html) documentation.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub legend: RemovableValue<Legend>,
    /// An object defining properties of the channel's scale, which is the function that
    /// transforms values in the data domain (numbers, dates, strings, etc) to visual values
    /// (pixels, colors, sizes) of the encoding channels.
    ///
    /// If `null`, the scale will be [disabled and the data value will be directly
    /// encoded](https://vega.github.io/vega-lite/docs/scale.html#disable).
    ///
    /// __Default value:__ If undefined, default [scale
    /// properties](https://vega.github.io/vega-lite/docs/scale.html) are applied.
    ///
    /// __See also:__ [`scale`](https://vega.github.io/vega-lite/docs/scale.html) documentation.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub scale: RemovableValue<Scale>,
    /// Sort order for the encoded field.
    ///
    /// For continuous fields (quantitative or temporal), `sort` can be either `"ascending"` or
    /// `"descending"`.
    ///
    /// For discrete fields, `sort` can be one of the following:
    /// - `"ascending"` or `"descending"` -- for sorting by the values' natural order in
    /// JavaScript.
    /// - [A string indicating an encoding channel name to sort
    /// by](https://vega.github.io/vega-lite/docs/sort.html#sort-by-encoding) (e.g., `"x"` or
    /// `"y"`) with an optional minus prefix for descending sort (e.g., `"-x"` to sort by
    /// x-field, descending). This channel string is short-form of [a sort-by-encoding
    /// definition](https://vega.github.io/vega-lite/docs/sort.html#sort-by-encoding). For
    /// example, `"sort": "-x"` is equivalent to `"sort": {"encoding": "x", "order":
    /// "descending"}`.
    /// - [A sort field definition](https://vega.github.io/vega-lite/docs/sort.html#sort-field)
    /// for sorting by another field.
    /// - [An array specifying the field values in preferred
    /// order](https://vega.github.io/vega-lite/docs/sort.html#sort-array). In this case, the
    /// sort order will obey the values in the array, followed by any unspecified values in their
    /// original order. For discrete time field, values in the sort array can be [date-time
    /// definition objects](struct.DateTime.html). In addition, for time units `"month"` and `"day"`,
    /// the values can be the month or day names (case insensitive) or their 3-letter initials
    /// (e.g., `"Mon"`, `"Tue"`).
    /// - `null` indicating no sort.
    ///
    /// __Default value:__ `"ascending"`
    ///
    /// __Note:__ `null` and sorting by another channel is not supported for `row` and `column`.
    ///
    /// __See also:__ [`sort`](https://vega.github.io/vega-lite/docs/sort.html) documentation.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub sort: RemovableValue<SortUnion>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html)
    /// documentation.
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`). If the field has an aggregate function, the function
    /// is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is binned or
    /// has a time unit applied, the applied function is shown in parentheses (e.g., `"Profit
    /// (binned)"`, `"Transaction Date (year-month)"`). Otherwise, the title is simply the field
    /// name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the
    /// [`fieldTitle`](https://vega.github.io/vega-lite/docs/config.html#top-level-config)
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub title: RemovableValue<PurpleText>,
    /// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
    /// `"nominal"`).
    /// It can also be a `"geojson"` type for encoding
    /// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
    ///
    ///
    /// __Note:__
    ///
    /// - Data values for a temporal field can be either a date-time string (e.g., `"2015-03-07
    /// 12:32:17"`, `"17:01"`, `"2015-03-16"`. `"2015"`) or a timestamp number (e.g.,
    /// `1552199579097`).
    /// - Data `type` describes the semantics of the data rather than the primitive data types
    /// (number, string, etc.). The same primitive data type can have different types of
    /// measurement. For example, numeric data can represent quantitative, ordinal, or nominal
    /// data.
    /// - When using with [`bin`](https://vega.github.io/vega-lite/docs/bin.html), the `type`
    /// property can be either `"quantitative"` (for using a linear bin scale) or [`"ordinal"`
    /// (for using an ordinal bin
    /// scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html), the
    /// `type` property can be either `"temporal"` (for using a temporal scale) or [`"ordinal"`
    /// (for using an ordinal scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html),
    /// the `type` property refers to the post-aggregation data type. For example, we can
    /// calculate count `distinct` of a categorical field `"cat"` using `{"aggregate":
    /// "distinct", "field": "cat", "type": "quantitative"}`. The `"type"` of the aggregate
    /// output is `"quantitative"`.
    /// - Secondary channels (e.g., `x2`, `y2`, `xError`, `yError`) do not have `type` as they
    /// have exactly the same type as their primary channels (e.g., `x`, `y`).
    ///
    /// __See also:__ [`type`](https://vega.github.io/vega-lite/docs/type.html) documentation.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub conditional_type: Option<StandardType>,
}

/// Latitude position of geographically projected marks.
///
/// Longitude position of geographically projected marks.
///
/// Definition object for a constant value (primitive value or gradient definition) of an
/// encoding channel.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct LatitudeClass {
    /// Aggregation function for the field
    /// (e.g., `"mean"`, `"sum"`, `"median"`, `"min"`, `"max"`, `"count"`).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aggregate: Option<Aggregate>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - If `"binned"`, this indicates that the data for the `x` (or `y`) channel are already
    /// binned. You can map the bin-start field to `x` (or `y`) and the bin-end field to `x2` (or
    /// `y2`). The scale and axis will be formatted similar to binning in Vega-Lite.  To adjust
    /// the axis ticks based on the bin step, you can also set the axis's
    /// [`tickMinStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    ///
    /// __See also:__ [`bin`](https://vega.github.io/vega-lite/docs/bin.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bin: Option<serde_json::Value>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __See also:__ [`field`](https://vega.github.io/vega-lite/docs/field.html) documentation.
    ///
    /// __Notes:__
    /// 1)  Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects (e.g.,
    /// `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    /// 2) `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field: Option<Field>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html)
    /// documentation.
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`). If the field has an aggregate function, the function
    /// is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is binned or
    /// has a time unit applied, the applied function is shown in parentheses (e.g., `"Profit
    /// (binned)"`, `"Transaction Date (year-month)"`). Otherwise, the title is simply the field
    /// name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the
    /// [`fieldTitle`](https://vega.github.io/vega-lite/docs/config.html#top-level-config)
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub title: RemovableValue<PurpleText>,
    /// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
    /// `"nominal"`).
    /// It can also be a `"geojson"` type for encoding
    /// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
    ///
    ///
    /// __Note:__
    ///
    /// - Data values for a temporal field can be either a date-time string (e.g., `"2015-03-07
    /// 12:32:17"`, `"17:01"`, `"2015-03-16"`. `"2015"`) or a timestamp number (e.g.,
    /// `1552199579097`).
    /// - Data `type` describes the semantics of the data rather than the primitive data types
    /// (number, string, etc.). The same primitive data type can have different types of
    /// measurement. For example, numeric data can represent quantitative, ordinal, or nominal
    /// data.
    /// - When using with [`bin`](https://vega.github.io/vega-lite/docs/bin.html), the `type`
    /// property can be either `"quantitative"` (for using a linear bin scale) or [`"ordinal"`
    /// (for using an ordinal bin
    /// scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html), the
    /// `type` property can be either `"temporal"` (for using a temporal scale) or [`"ordinal"`
    /// (for using an ordinal scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html),
    /// the `type` property refers to the post-aggregation data type. For example, we can
    /// calculate count `distinct` of a categorical field `"cat"` using `{"aggregate":
    /// "distinct", "field": "cat", "type": "quantitative"}`. The `"type"` of the aggregate
    /// output is `"quantitative"`.
    /// - Secondary channels (e.g., `x2`, `y2`, `xError`, `yError`) do not have `type` as they
    /// have exactly the same type as their primary channels (e.g., `x`, `y`).
    ///
    /// __See also:__ [`type`](https://vega.github.io/vega-lite/docs/type.html) documentation.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub def_type: Option<LatitudeType>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<f64>,
}

/// Latitude-2 position for geographically projected ranged `"area"`, `"bar"`, `"rect"`, and
/// `"rule"`.
///
/// Longitude-2 position for geographically projected ranged `"area"`, `"bar"`, `"rect"`,
/// and  `"rule"`.
///
/// Error value of x coordinates for error specified `"errorbar"` and `"errorband"`.
///
/// Secondary error value of x coordinates for error specified `"errorbar"` and
/// `"errorband"`.
///
/// Error value of y coordinates for error specified `"errorbar"` and `"errorband"`.
///
/// Secondary error value of y coordinates for error specified `"errorbar"` and
/// `"errorband"`.
///
/// A field definition of a secondary channel that shares a scale with another primary
/// channel. For example, `x2`, `xError` and `xError2` share the same scale with `x`.
///
/// Definition object for a constant value (primitive value or gradient definition) of an
/// encoding channel.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct Latitude2Class {
    /// Aggregation function for the field
    /// (e.g., `"mean"`, `"sum"`, `"median"`, `"min"`, `"max"`, `"count"`).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aggregate: Option<Aggregate>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - If `"binned"`, this indicates that the data for the `x` (or `y`) channel are already
    /// binned. You can map the bin-start field to `x` (or `y`) and the bin-end field to `x2` (or
    /// `y2`). The scale and axis will be formatted similar to binning in Vega-Lite.  To adjust
    /// the axis ticks based on the bin step, you can also set the axis's
    /// [`tickMinStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    ///
    /// __See also:__ [`bin`](https://vega.github.io/vega-lite/docs/bin.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bin: Option<serde_json::Value>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __See also:__ [`field`](https://vega.github.io/vega-lite/docs/field.html) documentation.
    ///
    /// __Notes:__
    /// 1)  Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects (e.g.,
    /// `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    /// 2) `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field: Option<Field>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html)
    /// documentation.
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`). If the field has an aggregate function, the function
    /// is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is binned or
    /// has a time unit applied, the applied function is shown in parentheses (e.g., `"Profit
    /// (binned)"`, `"Transaction Date (year-month)"`). Otherwise, the title is simply the field
    /// name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the
    /// [`fieldTitle`](https://vega.github.io/vega-lite/docs/config.html#top-level-config)
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub title: RemovableValue<PurpleText>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct OrderFieldDef {
    /// Aggregation function for the field
    /// (e.g., `"mean"`, `"sum"`, `"median"`, `"min"`, `"max"`, `"count"`).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aggregate: Option<Aggregate>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - If `"binned"`, this indicates that the data for the `x` (or `y`) channel are already
    /// binned. You can map the bin-start field to `x` (or `y`) and the bin-end field to `x2` (or
    /// `y2`). The scale and axis will be formatted similar to binning in Vega-Lite.  To adjust
    /// the axis ticks based on the bin step, you can also set the axis's
    /// [`tickMinStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    ///
    /// __See also:__ [`bin`](https://vega.github.io/vega-lite/docs/bin.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bin: Option<FluffyBin>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __See also:__ [`field`](https://vega.github.io/vega-lite/docs/field.html) documentation.
    ///
    /// __Notes:__
    /// 1)  Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects (e.g.,
    /// `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    /// 2) `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field: Option<Field>,
    /// The sort order. One of `"ascending"` (default) or `"descending"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub sort: Option<SortOrder>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html)
    /// documentation.
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`). If the field has an aggregate function, the function
    /// is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is binned or
    /// has a time unit applied, the applied function is shown in parentheses (e.g., `"Profit
    /// (binned)"`, `"Transaction Date (year-month)"`). Otherwise, the title is simply the field
    /// name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the
    /// [`fieldTitle`](https://vega.github.io/vega-lite/docs/config.html#top-level-config)
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub title: RemovableValue<PurpleText>,
    /// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
    /// `"nominal"`).
    /// It can also be a `"geojson"` type for encoding
    /// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
    ///
    ///
    /// __Note:__
    ///
    /// - Data values for a temporal field can be either a date-time string (e.g., `"2015-03-07
    /// 12:32:17"`, `"17:01"`, `"2015-03-16"`. `"2015"`) or a timestamp number (e.g.,
    /// `1552199579097`).
    /// - Data `type` describes the semantics of the data rather than the primitive data types
    /// (number, string, etc.). The same primitive data type can have different types of
    /// measurement. For example, numeric data can represent quantitative, ordinal, or nominal
    /// data.
    /// - When using with [`bin`](https://vega.github.io/vega-lite/docs/bin.html), the `type`
    /// property can be either `"quantitative"` (for using a linear bin scale) or [`"ordinal"`
    /// (for using an ordinal bin
    /// scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html), the
    /// `type` property can be either `"temporal"` (for using a temporal scale) or [`"ordinal"`
    /// (for using an ordinal scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html),
    /// the `type` property refers to the post-aggregation data type. For example, we can
    /// calculate count `distinct` of a categorical field `"cat"` using `{"aggregate":
    /// "distinct", "field": "cat", "type": "quantitative"}`. The `"type"` of the aggregate
    /// output is `"quantitative"`.
    /// - Secondary channels (e.g., `x2`, `y2`, `xError`, `yError`) do not have `type` as they
    /// have exactly the same type as their primary channels (e.g., `x`, `y`).
    ///
    /// __See also:__ [`type`](https://vega.github.io/vega-lite/docs/type.html) documentation.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub order_field_def_type: Option<StandardType>,
}

/// Definition object for a constant value (primitive value or gradient definition) of an
/// encoding channel.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct OrderFieldDefClass {
    /// Aggregation function for the field
    /// (e.g., `"mean"`, `"sum"`, `"median"`, `"min"`, `"max"`, `"count"`).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aggregate: Option<Aggregate>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - If `"binned"`, this indicates that the data for the `x` (or `y`) channel are already
    /// binned. You can map the bin-start field to `x` (or `y`) and the bin-end field to `x2` (or
    /// `y2`). The scale and axis will be formatted similar to binning in Vega-Lite.  To adjust
    /// the axis ticks based on the bin step, you can also set the axis's
    /// [`tickMinStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    ///
    /// __See also:__ [`bin`](https://vega.github.io/vega-lite/docs/bin.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bin: Option<FluffyBin>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __See also:__ [`field`](https://vega.github.io/vega-lite/docs/field.html) documentation.
    ///
    /// __Notes:__
    /// 1)  Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects (e.g.,
    /// `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    /// 2) `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field: Option<Field>,
    /// The sort order. One of `"ascending"` (default) or `"descending"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub sort: Option<SortOrder>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html)
    /// documentation.
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`). If the field has an aggregate function, the function
    /// is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is binned or
    /// has a time unit applied, the applied function is shown in parentheses (e.g., `"Profit
    /// (binned)"`, `"Transaction Date (year-month)"`). Otherwise, the title is simply the field
    /// name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the
    /// [`fieldTitle`](https://vega.github.io/vega-lite/docs/config.html#top-level-config)
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub title: RemovableValue<PurpleText>,
    /// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
    /// `"nominal"`).
    /// It can also be a `"geojson"` type for encoding
    /// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
    ///
    ///
    /// __Note:__
    ///
    /// - Data values for a temporal field can be either a date-time string (e.g., `"2015-03-07
    /// 12:32:17"`, `"17:01"`, `"2015-03-16"`. `"2015"`) or a timestamp number (e.g.,
    /// `1552199579097`).
    /// - Data `type` describes the semantics of the data rather than the primitive data types
    /// (number, string, etc.). The same primitive data type can have different types of
    /// measurement. For example, numeric data can represent quantitative, ordinal, or nominal
    /// data.
    /// - When using with [`bin`](https://vega.github.io/vega-lite/docs/bin.html), the `type`
    /// property can be either `"quantitative"` (for using a linear bin scale) or [`"ordinal"`
    /// (for using an ordinal bin
    /// scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html), the
    /// `type` property can be either `"temporal"` (for using a temporal scale) or [`"ordinal"`
    /// (for using an ordinal scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html),
    /// the `type` property refers to the post-aggregation data type. For example, we can
    /// calculate count `distinct` of a categorical field `"cat"` using `{"aggregate":
    /// "distinct", "field": "cat", "type": "quantitative"}`. The `"type"` of the aggregate
    /// output is `"quantitative"`.
    /// - Secondary channels (e.g., `x2`, `y2`, `xError`, `yError`) do not have `type` as they
    /// have exactly the same type as their primary channels (e.g., `x`, `y`).
    ///
    /// __See also:__ [`type`](https://vega.github.io/vega-lite/docs/type.html) documentation.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub def_type: Option<StandardType>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<f64>,
}

/// Shape of the mark.
///
/// 1. For `point` marks the supported values include:
/// - plotting shapes: `"circle"`, `"square"`, `"cross"`, `"diamond"`, `"triangle-up"`,
/// `"triangle-down"`, `"triangle-right"`, or `"triangle-left"`.
/// - the line symbol `"stroke"`
/// - centered directional shapes `"arrow"`, `"wedge"`, or `"triangle"`
/// - a custom [SVG path
/// string](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths) (For correct
/// sizing, custom shape paths should be defined within a square bounding box with
/// coordinates ranging from -1 to 1 along both the x and y dimensions.)
///
/// 2. For `geoshape` marks it should be a field definition of the geojson data
///
/// __Default value:__ If undefined, the default shape depends on [mark
/// config](https://vega.github.io/vega-lite/docs/config.html#point-config)'s `shape`
/// property. (`"circle"` if unset.)
///
/// A FieldDef with Condition<ValueDef>
/// {
/// condition: {value: ...},
/// field: ...,
/// ...
/// }
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct DefWithConditionMarkPropFieldDefTypeForShapeStringNull {
    /// Aggregation function for the field
    /// (e.g., `"mean"`, `"sum"`, `"median"`, `"min"`, `"max"`, `"count"`).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aggregate: Option<Aggregate>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - If `"binned"`, this indicates that the data for the `x` (or `y`) channel are already
    /// binned. You can map the bin-start field to `x` (or `y`) and the bin-end field to `x2` (or
    /// `y2`). The scale and axis will be formatted similar to binning in Vega-Lite.  To adjust
    /// the axis ticks based on the bin step, you can also set the axis's
    /// [`tickMinStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    ///
    /// __See also:__ [`bin`](https://vega.github.io/vega-lite/docs/bin.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bin: Option<PurpleBin>,
    /// One or more value definition(s) with [a selection or a test
    /// predicate](https://vega.github.io/vega-lite/docs/condition.html).
    ///
    /// __Note:__ A field definition's `condition` property can only contain [conditional value
    /// definitions](https://vega.github.io/vega-lite/docs/condition.html#value)
    /// since Vega-Lite only allows at most one encoded field per encoding channel.
    ///
    /// A field definition or one or more value definition(s) with a selection predicate.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub condition: Option<ShapeCondition>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __See also:__ [`field`](https://vega.github.io/vega-lite/docs/field.html) documentation.
    ///
    /// __Notes:__
    /// 1)  Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects (e.g.,
    /// `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    /// 2) `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field: Option<Field>,
    /// An object defining properties of the legend.
    /// If `null`, the legend for the encoding channel will be removed.
    ///
    /// __Default value:__ If undefined, default [legend
    /// properties](https://vega.github.io/vega-lite/docs/legend.html) are applied.
    ///
    /// __See also:__ [`legend`](https://vega.github.io/vega-lite/docs/legend.html) documentation.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub legend: RemovableValue<Legend>,
    /// An object defining properties of the channel's scale, which is the function that
    /// transforms values in the data domain (numbers, dates, strings, etc) to visual values
    /// (pixels, colors, sizes) of the encoding channels.
    ///
    /// If `null`, the scale will be [disabled and the data value will be directly
    /// encoded](https://vega.github.io/vega-lite/docs/scale.html#disable).
    ///
    /// __Default value:__ If undefined, default [scale
    /// properties](https://vega.github.io/vega-lite/docs/scale.html) are applied.
    ///
    /// __See also:__ [`scale`](https://vega.github.io/vega-lite/docs/scale.html) documentation.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub scale: RemovableValue<Scale>,
    /// Sort order for the encoded field.
    ///
    /// For continuous fields (quantitative or temporal), `sort` can be either `"ascending"` or
    /// `"descending"`.
    ///
    /// For discrete fields, `sort` can be one of the following:
    /// - `"ascending"` or `"descending"` -- for sorting by the values' natural order in
    /// JavaScript.
    /// - [A string indicating an encoding channel name to sort
    /// by](https://vega.github.io/vega-lite/docs/sort.html#sort-by-encoding) (e.g., `"x"` or
    /// `"y"`) with an optional minus prefix for descending sort (e.g., `"-x"` to sort by
    /// x-field, descending). This channel string is short-form of [a sort-by-encoding
    /// definition](https://vega.github.io/vega-lite/docs/sort.html#sort-by-encoding). For
    /// example, `"sort": "-x"` is equivalent to `"sort": {"encoding": "x", "order":
    /// "descending"}`.
    /// - [A sort field definition](https://vega.github.io/vega-lite/docs/sort.html#sort-field)
    /// for sorting by another field.
    /// - [An array specifying the field values in preferred
    /// order](https://vega.github.io/vega-lite/docs/sort.html#sort-array). In this case, the
    /// sort order will obey the values in the array, followed by any unspecified values in their
    /// original order. For discrete time field, values in the sort array can be [date-time
    /// definition objects](struct.DateTime.html). In addition, for time units `"month"` and `"day"`,
    /// the values can be the month or day names (case insensitive) or their 3-letter initials
    /// (e.g., `"Mon"`, `"Tue"`).
    /// - `null` indicating no sort.
    ///
    /// __Default value:__ `"ascending"`
    ///
    /// __Note:__ `null` and sorting by another channel is not supported for `row` and `column`.
    ///
    /// __See also:__ [`sort`](https://vega.github.io/vega-lite/docs/sort.html) documentation.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub sort: RemovableValue<SortUnion>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html)
    /// documentation.
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`). If the field has an aggregate function, the function
    /// is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is binned or
    /// has a time unit applied, the applied function is shown in parentheses (e.g., `"Profit
    /// (binned)"`, `"Transaction Date (year-month)"`). Otherwise, the title is simply the field
    /// name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the
    /// [`fieldTitle`](https://vega.github.io/vega-lite/docs/config.html#top-level-config)
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub title: RemovableValue<PurpleText>,
    /// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
    /// `"nominal"`).
    /// It can also be a `"geojson"` type for encoding
    /// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
    ///
    ///
    /// __Note:__
    ///
    /// - Data values for a temporal field can be either a date-time string (e.g., `"2015-03-07
    /// 12:32:17"`, `"17:01"`, `"2015-03-16"`. `"2015"`) or a timestamp number (e.g.,
    /// `1552199579097`).
    /// - Data `type` describes the semantics of the data rather than the primitive data types
    /// (number, string, etc.). The same primitive data type can have different types of
    /// measurement. For example, numeric data can represent quantitative, ordinal, or nominal
    /// data.
    /// - When using with [`bin`](https://vega.github.io/vega-lite/docs/bin.html), the `type`
    /// property can be either `"quantitative"` (for using a linear bin scale) or [`"ordinal"`
    /// (for using an ordinal bin
    /// scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html), the
    /// `type` property can be either `"temporal"` (for using a temporal scale) or [`"ordinal"`
    /// (for using an ordinal scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html),
    /// the `type` property refers to the post-aggregation data type. For example, we can
    /// calculate count `distinct` of a categorical field `"cat"` using `{"aggregate":
    /// "distinct", "field": "cat", "type": "quantitative"}`. The `"type"` of the aggregate
    /// output is `"quantitative"`.
    /// - Secondary channels (e.g., `x2`, `y2`, `xError`, `yError`) do not have `type` as they
    /// have exactly the same type as their primary channels (e.g., `x`, `y`).
    ///
    /// __See also:__ [`type`](https://vega.github.io/vega-lite/docs/type.html) documentation.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub def_with_condition_mark_prop_field_def_type_for_shape_string_null_type:
        Option<TypeForShape>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ConditionalStringValueDef {
    /// Predicate for triggering the condition
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub test: Option<PurpleLogicalOperandPredicate>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<String>,
    /// A [selection name](https://vega.github.io/vega-lite/docs/selection.html), or a series of
    /// [composed selections](https://vega.github.io/vega-lite/docs/selection.html#compose).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub selection: Option<Box<PurpleSelectionOperand>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ConditionalPredicateMarkPropFieldDefTypeForShapeClass {
    /// Predicate for triggering the condition
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub test: Option<PurpleLogicalOperandPredicate>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<String>,
    /// A [selection name](https://vega.github.io/vega-lite/docs/selection.html), or a series of
    /// [composed selections](https://vega.github.io/vega-lite/docs/selection.html#compose).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub selection: Option<Box<PurpleSelectionOperand>>,
    /// Aggregation function for the field
    /// (e.g., `"mean"`, `"sum"`, `"median"`, `"min"`, `"max"`, `"count"`).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aggregate: Option<Aggregate>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - If `"binned"`, this indicates that the data for the `x` (or `y`) channel are already
    /// binned. You can map the bin-start field to `x` (or `y`) and the bin-end field to `x2` (or
    /// `y2`). The scale and axis will be formatted similar to binning in Vega-Lite.  To adjust
    /// the axis ticks based on the bin step, you can also set the axis's
    /// [`tickMinStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    ///
    /// __See also:__ [`bin`](https://vega.github.io/vega-lite/docs/bin.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bin: Option<PurpleBin>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __See also:__ [`field`](https://vega.github.io/vega-lite/docs/field.html) documentation.
    ///
    /// __Notes:__
    /// 1)  Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects (e.g.,
    /// `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    /// 2) `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field: Option<Field>,
    /// An object defining properties of the legend.
    /// If `null`, the legend for the encoding channel will be removed.
    ///
    /// __Default value:__ If undefined, default [legend
    /// properties](https://vega.github.io/vega-lite/docs/legend.html) are applied.
    ///
    /// __See also:__ [`legend`](https://vega.github.io/vega-lite/docs/legend.html) documentation.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub legend: RemovableValue<Legend>,
    /// An object defining properties of the channel's scale, which is the function that
    /// transforms values in the data domain (numbers, dates, strings, etc) to visual values
    /// (pixels, colors, sizes) of the encoding channels.
    ///
    /// If `null`, the scale will be [disabled and the data value will be directly
    /// encoded](https://vega.github.io/vega-lite/docs/scale.html#disable).
    ///
    /// __Default value:__ If undefined, default [scale
    /// properties](https://vega.github.io/vega-lite/docs/scale.html) are applied.
    ///
    /// __See also:__ [`scale`](https://vega.github.io/vega-lite/docs/scale.html) documentation.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub scale: RemovableValue<Scale>,
    /// Sort order for the encoded field.
    ///
    /// For continuous fields (quantitative or temporal), `sort` can be either `"ascending"` or
    /// `"descending"`.
    ///
    /// For discrete fields, `sort` can be one of the following:
    /// - `"ascending"` or `"descending"` -- for sorting by the values' natural order in
    /// JavaScript.
    /// - [A string indicating an encoding channel name to sort
    /// by](https://vega.github.io/vega-lite/docs/sort.html#sort-by-encoding) (e.g., `"x"` or
    /// `"y"`) with an optional minus prefix for descending sort (e.g., `"-x"` to sort by
    /// x-field, descending). This channel string is short-form of [a sort-by-encoding
    /// definition](https://vega.github.io/vega-lite/docs/sort.html#sort-by-encoding). For
    /// example, `"sort": "-x"` is equivalent to `"sort": {"encoding": "x", "order":
    /// "descending"}`.
    /// - [A sort field definition](https://vega.github.io/vega-lite/docs/sort.html#sort-field)
    /// for sorting by another field.
    /// - [An array specifying the field values in preferred
    /// order](https://vega.github.io/vega-lite/docs/sort.html#sort-array). In this case, the
    /// sort order will obey the values in the array, followed by any unspecified values in their
    /// original order. For discrete time field, values in the sort array can be [date-time
    /// definition objects](struct.DateTime.html). In addition, for time units `"month"` and `"day"`,
    /// the values can be the month or day names (case insensitive) or their 3-letter initials
    /// (e.g., `"Mon"`, `"Tue"`).
    /// - `null` indicating no sort.
    ///
    /// __Default value:__ `"ascending"`
    ///
    /// __Note:__ `null` and sorting by another channel is not supported for `row` and `column`.
    ///
    /// __See also:__ [`sort`](https://vega.github.io/vega-lite/docs/sort.html) documentation.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub sort: RemovableValue<SortUnion>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html)
    /// documentation.
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`). If the field has an aggregate function, the function
    /// is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is binned or
    /// has a time unit applied, the applied function is shown in parentheses (e.g., `"Profit
    /// (binned)"`, `"Transaction Date (year-month)"`). Otherwise, the title is simply the field
    /// name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the
    /// [`fieldTitle`](https://vega.github.io/vega-lite/docs/config.html#top-level-config)
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub title: RemovableValue<PurpleText>,
    /// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
    /// `"nominal"`).
    /// It can also be a `"geojson"` type for encoding
    /// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
    ///
    ///
    /// __Note:__
    ///
    /// - Data values for a temporal field can be either a date-time string (e.g., `"2015-03-07
    /// 12:32:17"`, `"17:01"`, `"2015-03-16"`. `"2015"`) or a timestamp number (e.g.,
    /// `1552199579097`).
    /// - Data `type` describes the semantics of the data rather than the primitive data types
    /// (number, string, etc.). The same primitive data type can have different types of
    /// measurement. For example, numeric data can represent quantitative, ordinal, or nominal
    /// data.
    /// - When using with [`bin`](https://vega.github.io/vega-lite/docs/bin.html), the `type`
    /// property can be either `"quantitative"` (for using a linear bin scale) or [`"ordinal"`
    /// (for using an ordinal bin
    /// scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html), the
    /// `type` property can be either `"temporal"` (for using a temporal scale) or [`"ordinal"`
    /// (for using an ordinal scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html),
    /// the `type` property refers to the post-aggregation data type. For example, we can
    /// calculate count `distinct` of a categorical field `"cat"` using `{"aggregate":
    /// "distinct", "field": "cat", "type": "quantitative"}`. The `"type"` of the aggregate
    /// output is `"quantitative"`.
    /// - Secondary channels (e.g., `x2`, `y2`, `xError`, `yError`) do not have `type` as they
    /// have exactly the same type as their primary channels (e.g., `x`, `y`).
    ///
    /// __See also:__ [`type`](https://vega.github.io/vega-lite/docs/type.html) documentation.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub conditional_type: Option<TypeForShape>,
}

/// Text of the `text` mark.
///
/// A FieldDef with Condition<ValueDef>
/// {
/// condition: {value: ...},
/// field: ...,
/// ...
/// }
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct DefWithConditionStringFieldDefText {
    /// Aggregation function for the field
    /// (e.g., `"mean"`, `"sum"`, `"median"`, `"min"`, `"max"`, `"count"`).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aggregate: Option<Aggregate>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - If `"binned"`, this indicates that the data for the `x` (or `y`) channel are already
    /// binned. You can map the bin-start field to `x` (or `y`) and the bin-end field to `x2` (or
    /// `y2`). The scale and axis will be formatted similar to binning in Vega-Lite.  To adjust
    /// the axis ticks based on the bin step, you can also set the axis's
    /// [`tickMinStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    ///
    /// __See also:__ [`bin`](https://vega.github.io/vega-lite/docs/bin.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bin: Option<FluffyBin>,
    /// One or more value definition(s) with [a selection or a test
    /// predicate](https://vega.github.io/vega-lite/docs/condition.html).
    ///
    /// __Note:__ A field definition's `condition` property can only contain [conditional value
    /// definitions](https://vega.github.io/vega-lite/docs/condition.html#value)
    /// since Vega-Lite only allows at most one encoded field per encoding channel.
    ///
    /// A field definition or one or more value definition(s) with a selection predicate.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub condition: Option<TextCondition>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __See also:__ [`field`](https://vega.github.io/vega-lite/docs/field.html) documentation.
    ///
    /// __Notes:__
    /// 1)  Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects (e.g.,
    /// `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    /// 2) `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field: Option<Field>,
    /// The text formatting pattern for labels of guides (axes, legends, headers) and text
    /// marks.
    ///
    /// - If the format type is `"number"` (e.g., for quantitative fields), this is D3's [number
    /// format pattern](https://github.com/d3/d3-format#locale_format).
    /// - If the format type is `"time"` (e.g., for temporal fields), this is D3's [time format
    /// pattern](https://github.com/d3/d3-time-format#locale_format).
    ///
    /// See the [format documentation](https://vega.github.io/vega-lite/docs/format.html) for
    /// more examples.
    ///
    /// __Default value:__  Derived from
    /// [numberFormat](https://vega.github.io/vega-lite/docs/config.html#format) config for
    /// number format and from
    /// [timeFormat](https://vega.github.io/vega-lite/docs/config.html#format) config for time
    /// format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub format: Option<String>,
    /// The format type for labels (`"number"` or `"time"`).
    ///
    /// __Default value:__
    /// - `"time"` for temporal fields and ordinal and nomimal fields with `timeUnit`.
    /// - `"number"` for quantitative fields as well as ordinal and nomimal fields without
    /// `timeUnit`.
    #[serde(rename = "formatType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub format_type: Option<FormatType>,
    /// [Vega expression](https://vega.github.io/vega/docs/expressions/) for customizing labels
    /// text.
    ///
    /// __Note:__ The label text and value can be assessed via the `label` and `value` properties
    /// of the axis's backing `datum` object.
    #[serde(rename = "labelExpr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_expr: Option<String>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html)
    /// documentation.
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`). If the field has an aggregate function, the function
    /// is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is binned or
    /// has a time unit applied, the applied function is shown in parentheses (e.g., `"Profit
    /// (binned)"`, `"Transaction Date (year-month)"`). Otherwise, the title is simply the field
    /// name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the
    /// [`fieldTitle`](https://vega.github.io/vega-lite/docs/config.html#top-level-config)
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub title: RemovableValue<PurpleText>,
    /// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
    /// `"nominal"`).
    /// It can also be a `"geojson"` type for encoding
    /// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
    ///
    ///
    /// __Note:__
    ///
    /// - Data values for a temporal field can be either a date-time string (e.g., `"2015-03-07
    /// 12:32:17"`, `"17:01"`, `"2015-03-16"`. `"2015"`) or a timestamp number (e.g.,
    /// `1552199579097`).
    /// - Data `type` describes the semantics of the data rather than the primitive data types
    /// (number, string, etc.). The same primitive data type can have different types of
    /// measurement. For example, numeric data can represent quantitative, ordinal, or nominal
    /// data.
    /// - When using with [`bin`](https://vega.github.io/vega-lite/docs/bin.html), the `type`
    /// property can be either `"quantitative"` (for using a linear bin scale) or [`"ordinal"`
    /// (for using an ordinal bin
    /// scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html), the
    /// `type` property can be either `"temporal"` (for using a temporal scale) or [`"ordinal"`
    /// (for using an ordinal scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html),
    /// the `type` property refers to the post-aggregation data type. For example, we can
    /// calculate count `distinct` of a categorical field `"cat"` using `{"aggregate":
    /// "distinct", "field": "cat", "type": "quantitative"}`. The `"type"` of the aggregate
    /// output is `"quantitative"`.
    /// - Secondary channels (e.g., `x2`, `y2`, `xError`, `yError`) do not have `type` as they
    /// have exactly the same type as their primary channels (e.g., `x`, `y`).
    ///
    /// __See also:__ [`type`](https://vega.github.io/vega-lite/docs/type.html) documentation.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub def_with_condition_string_field_def_text_type: Option<StandardType>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<PurpleText>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ConditionalValueDefText {
    /// Predicate for triggering the condition
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub test: Option<PurpleLogicalOperandPredicate>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<ConditionalValueDefTextText>,
    /// A [selection name](https://vega.github.io/vega-lite/docs/selection.html), or a series of
    /// [composed selections](https://vega.github.io/vega-lite/docs/selection.html#compose).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub selection: Option<Box<PurpleSelectionOperand>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ConditionalPredicateValueDefTextClass {
    /// Predicate for triggering the condition
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub test: Option<PurpleLogicalOperandPredicate>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<PurpleText>,
    /// A [selection name](https://vega.github.io/vega-lite/docs/selection.html), or a series of
    /// [composed selections](https://vega.github.io/vega-lite/docs/selection.html#compose).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub selection: Option<Box<PurpleSelectionOperand>>,
    /// Aggregation function for the field
    /// (e.g., `"mean"`, `"sum"`, `"median"`, `"min"`, `"max"`, `"count"`).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aggregate: Option<Aggregate>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - If `"binned"`, this indicates that the data for the `x` (or `y`) channel are already
    /// binned. You can map the bin-start field to `x` (or `y`) and the bin-end field to `x2` (or
    /// `y2`). The scale and axis will be formatted similar to binning in Vega-Lite.  To adjust
    /// the axis ticks based on the bin step, you can also set the axis's
    /// [`tickMinStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    ///
    /// __See also:__ [`bin`](https://vega.github.io/vega-lite/docs/bin.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bin: Option<FluffyBin>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __See also:__ [`field`](https://vega.github.io/vega-lite/docs/field.html) documentation.
    ///
    /// __Notes:__
    /// 1)  Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects (e.g.,
    /// `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    /// 2) `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field: Option<Field>,
    /// The text formatting pattern for labels of guides (axes, legends, headers) and text
    /// marks.
    ///
    /// - If the format type is `"number"` (e.g., for quantitative fields), this is D3's [number
    /// format pattern](https://github.com/d3/d3-format#locale_format).
    /// - If the format type is `"time"` (e.g., for temporal fields), this is D3's [time format
    /// pattern](https://github.com/d3/d3-time-format#locale_format).
    ///
    /// See the [format documentation](https://vega.github.io/vega-lite/docs/format.html) for
    /// more examples.
    ///
    /// __Default value:__  Derived from
    /// [numberFormat](https://vega.github.io/vega-lite/docs/config.html#format) config for
    /// number format and from
    /// [timeFormat](https://vega.github.io/vega-lite/docs/config.html#format) config for time
    /// format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub format: Option<String>,
    /// The format type for labels (`"number"` or `"time"`).
    ///
    /// __Default value:__
    /// - `"time"` for temporal fields and ordinal and nomimal fields with `timeUnit`.
    /// - `"number"` for quantitative fields as well as ordinal and nomimal fields without
    /// `timeUnit`.
    #[serde(rename = "formatType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub format_type: Option<FormatType>,
    /// [Vega expression](https://vega.github.io/vega/docs/expressions/) for customizing labels
    /// text.
    ///
    /// __Note:__ The label text and value can be assessed via the `label` and `value` properties
    /// of the axis's backing `datum` object.
    #[serde(rename = "labelExpr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_expr: Option<String>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html)
    /// documentation.
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`). If the field has an aggregate function, the function
    /// is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is binned or
    /// has a time unit applied, the applied function is shown in parentheses (e.g., `"Profit
    /// (binned)"`, `"Transaction Date (year-month)"`). Otherwise, the title is simply the field
    /// name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the
    /// [`fieldTitle`](https://vega.github.io/vega-lite/docs/config.html#top-level-config)
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub title: RemovableValue<PurpleText>,
    /// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
    /// `"nominal"`).
    /// It can also be a `"geojson"` type for encoding
    /// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
    ///
    ///
    /// __Note:__
    ///
    /// - Data values for a temporal field can be either a date-time string (e.g., `"2015-03-07
    /// 12:32:17"`, `"17:01"`, `"2015-03-16"`. `"2015"`) or a timestamp number (e.g.,
    /// `1552199579097`).
    /// - Data `type` describes the semantics of the data rather than the primitive data types
    /// (number, string, etc.). The same primitive data type can have different types of
    /// measurement. For example, numeric data can represent quantitative, ordinal, or nominal
    /// data.
    /// - When using with [`bin`](https://vega.github.io/vega-lite/docs/bin.html), the `type`
    /// property can be either `"quantitative"` (for using a linear bin scale) or [`"ordinal"`
    /// (for using an ordinal bin
    /// scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html), the
    /// `type` property can be either `"temporal"` (for using a temporal scale) or [`"ordinal"`
    /// (for using an ordinal scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html),
    /// the `type` property refers to the post-aggregation data type. For example, we can
    /// calculate count `distinct` of a categorical field `"cat"` using `{"aggregate":
    /// "distinct", "field": "cat", "type": "quantitative"}`. The `"type"` of the aggregate
    /// output is `"quantitative"`.
    /// - Secondary channels (e.g., `x2`, `y2`, `xError`, `yError`) do not have `type` as they
    /// have exactly the same type as their primary channels (e.g., `x`, `y`).
    ///
    /// __See also:__ [`type`](https://vega.github.io/vega-lite/docs/type.html) documentation.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub conditional_type: Option<StandardType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct StringFieldDef {
    /// Aggregation function for the field
    /// (e.g., `"mean"`, `"sum"`, `"median"`, `"min"`, `"max"`, `"count"`).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aggregate: Option<Aggregate>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - If `"binned"`, this indicates that the data for the `x` (or `y`) channel are already
    /// binned. You can map the bin-start field to `x` (or `y`) and the bin-end field to `x2` (or
    /// `y2`). The scale and axis will be formatted similar to binning in Vega-Lite.  To adjust
    /// the axis ticks based on the bin step, you can also set the axis's
    /// [`tickMinStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    ///
    /// __See also:__ [`bin`](https://vega.github.io/vega-lite/docs/bin.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bin: Option<FluffyBin>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __See also:__ [`field`](https://vega.github.io/vega-lite/docs/field.html) documentation.
    ///
    /// __Notes:__
    /// 1)  Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects (e.g.,
    /// `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    /// 2) `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field: Option<Field>,
    /// The text formatting pattern for labels of guides (axes, legends, headers) and text
    /// marks.
    ///
    /// - If the format type is `"number"` (e.g., for quantitative fields), this is D3's [number
    /// format pattern](https://github.com/d3/d3-format#locale_format).
    /// - If the format type is `"time"` (e.g., for temporal fields), this is D3's [time format
    /// pattern](https://github.com/d3/d3-time-format#locale_format).
    ///
    /// See the [format documentation](https://vega.github.io/vega-lite/docs/format.html) for
    /// more examples.
    ///
    /// __Default value:__  Derived from
    /// [numberFormat](https://vega.github.io/vega-lite/docs/config.html#format) config for
    /// number format and from
    /// [timeFormat](https://vega.github.io/vega-lite/docs/config.html#format) config for time
    /// format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub format: Option<String>,
    /// The format type for labels (`"number"` or `"time"`).
    ///
    /// __Default value:__
    /// - `"time"` for temporal fields and ordinal and nomimal fields with `timeUnit`.
    /// - `"number"` for quantitative fields as well as ordinal and nomimal fields without
    /// `timeUnit`.
    #[serde(rename = "formatType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub format_type: Option<FormatType>,
    /// [Vega expression](https://vega.github.io/vega/docs/expressions/) for customizing labels
    /// text.
    ///
    /// __Note:__ The label text and value can be assessed via the `label` and `value` properties
    /// of the axis's backing `datum` object.
    #[serde(rename = "labelExpr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_expr: Option<String>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html)
    /// documentation.
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`). If the field has an aggregate function, the function
    /// is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is binned or
    /// has a time unit applied, the applied function is shown in parentheses (e.g., `"Profit
    /// (binned)"`, `"Transaction Date (year-month)"`). Otherwise, the title is simply the field
    /// name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the
    /// [`fieldTitle`](https://vega.github.io/vega-lite/docs/config.html#top-level-config)
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub title: RemovableValue<PurpleText>,
    /// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
    /// `"nominal"`).
    /// It can also be a `"geojson"` type for encoding
    /// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
    ///
    ///
    /// __Note:__
    ///
    /// - Data values for a temporal field can be either a date-time string (e.g., `"2015-03-07
    /// 12:32:17"`, `"17:01"`, `"2015-03-16"`. `"2015"`) or a timestamp number (e.g.,
    /// `1552199579097`).
    /// - Data `type` describes the semantics of the data rather than the primitive data types
    /// (number, string, etc.). The same primitive data type can have different types of
    /// measurement. For example, numeric data can represent quantitative, ordinal, or nominal
    /// data.
    /// - When using with [`bin`](https://vega.github.io/vega-lite/docs/bin.html), the `type`
    /// property can be either `"quantitative"` (for using a linear bin scale) or [`"ordinal"`
    /// (for using an ordinal bin
    /// scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html), the
    /// `type` property can be either `"temporal"` (for using a temporal scale) or [`"ordinal"`
    /// (for using an ordinal scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html),
    /// the `type` property refers to the post-aggregation data type. For example, we can
    /// calculate count `distinct` of a categorical field `"cat"` using `{"aggregate":
    /// "distinct", "field": "cat", "type": "quantitative"}`. The `"type"` of the aggregate
    /// output is `"quantitative"`.
    /// - Secondary channels (e.g., `x2`, `y2`, `xError`, `yError`) do not have `type` as they
    /// have exactly the same type as their primary channels (e.g., `x`, `y`).
    ///
    /// __See also:__ [`type`](https://vega.github.io/vega-lite/docs/type.html) documentation.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub string_field_def_type: Option<StandardType>,
}

/// A FieldDef with Condition<ValueDef>
/// {
/// condition: {value: ...},
/// field: ...,
/// ...
/// }
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct FieldDefWithConditionStringFieldDefString {
    /// Aggregation function for the field
    /// (e.g., `"mean"`, `"sum"`, `"median"`, `"min"`, `"max"`, `"count"`).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aggregate: Option<Aggregate>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - If `"binned"`, this indicates that the data for the `x` (or `y`) channel are already
    /// binned. You can map the bin-start field to `x` (or `y`) and the bin-end field to `x2` (or
    /// `y2`). The scale and axis will be formatted similar to binning in Vega-Lite.  To adjust
    /// the axis ticks based on the bin step, you can also set the axis's
    /// [`tickMinStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    ///
    /// __See also:__ [`bin`](https://vega.github.io/vega-lite/docs/bin.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bin: Option<FluffyBin>,
    /// One or more value definition(s) with [a selection or a test
    /// predicate](https://vega.github.io/vega-lite/docs/condition.html).
    ///
    /// __Note:__ A field definition's `condition` property can only contain [conditional value
    /// definitions](https://vega.github.io/vega-lite/docs/condition.html#value)
    /// since Vega-Lite only allows at most one encoded field per encoding channel.
    ///
    /// A field definition or one or more value definition(s) with a selection predicate.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub condition: Option<HrefCondition>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __See also:__ [`field`](https://vega.github.io/vega-lite/docs/field.html) documentation.
    ///
    /// __Notes:__
    /// 1)  Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects (e.g.,
    /// `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    /// 2) `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field: Option<Field>,
    /// The text formatting pattern for labels of guides (axes, legends, headers) and text
    /// marks.
    ///
    /// - If the format type is `"number"` (e.g., for quantitative fields), this is D3's [number
    /// format pattern](https://github.com/d3/d3-format#locale_format).
    /// - If the format type is `"time"` (e.g., for temporal fields), this is D3's [time format
    /// pattern](https://github.com/d3/d3-time-format#locale_format).
    ///
    /// See the [format documentation](https://vega.github.io/vega-lite/docs/format.html) for
    /// more examples.
    ///
    /// __Default value:__  Derived from
    /// [numberFormat](https://vega.github.io/vega-lite/docs/config.html#format) config for
    /// number format and from
    /// [timeFormat](https://vega.github.io/vega-lite/docs/config.html#format) config for time
    /// format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub format: Option<String>,
    /// The format type for labels (`"number"` or `"time"`).
    ///
    /// __Default value:__
    /// - `"time"` for temporal fields and ordinal and nomimal fields with `timeUnit`.
    /// - `"number"` for quantitative fields as well as ordinal and nomimal fields without
    /// `timeUnit`.
    #[serde(rename = "formatType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub format_type: Option<FormatType>,
    /// [Vega expression](https://vega.github.io/vega/docs/expressions/) for customizing labels
    /// text.
    ///
    /// __Note:__ The label text and value can be assessed via the `label` and `value` properties
    /// of the axis's backing `datum` object.
    #[serde(rename = "labelExpr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_expr: Option<String>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html)
    /// documentation.
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`). If the field has an aggregate function, the function
    /// is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is binned or
    /// has a time unit applied, the applied function is shown in parentheses (e.g., `"Profit
    /// (binned)"`, `"Transaction Date (year-month)"`). Otherwise, the title is simply the field
    /// name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the
    /// [`fieldTitle`](https://vega.github.io/vega-lite/docs/config.html#top-level-config)
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub title: RemovableValue<PurpleText>,
    /// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
    /// `"nominal"`).
    /// It can also be a `"geojson"` type for encoding
    /// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
    ///
    ///
    /// __Note:__
    ///
    /// - Data values for a temporal field can be either a date-time string (e.g., `"2015-03-07
    /// 12:32:17"`, `"17:01"`, `"2015-03-16"`. `"2015"`) or a timestamp number (e.g.,
    /// `1552199579097`).
    /// - Data `type` describes the semantics of the data rather than the primitive data types
    /// (number, string, etc.). The same primitive data type can have different types of
    /// measurement. For example, numeric data can represent quantitative, ordinal, or nominal
    /// data.
    /// - When using with [`bin`](https://vega.github.io/vega-lite/docs/bin.html), the `type`
    /// property can be either `"quantitative"` (for using a linear bin scale) or [`"ordinal"`
    /// (for using an ordinal bin
    /// scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html), the
    /// `type` property can be either `"temporal"` (for using a temporal scale) or [`"ordinal"`
    /// (for using an ordinal scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html),
    /// the `type` property refers to the post-aggregation data type. For example, we can
    /// calculate count `distinct` of a categorical field `"cat"` using `{"aggregate":
    /// "distinct", "field": "cat", "type": "quantitative"}`. The `"type"` of the aggregate
    /// output is `"quantitative"`.
    /// - Secondary channels (e.g., `x2`, `y2`, `xError`, `yError`) do not have `type` as they
    /// have exactly the same type as their primary channels (e.g., `x`, `y`).
    ///
    /// __See also:__ [`type`](https://vega.github.io/vega-lite/docs/type.html) documentation.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field_def_with_condition_string_field_def_string_type: Option<StandardType>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<String>,
}

/// X coordinates of the marks, or width of horizontal `"bar"` and `"area"` without specified
/// `x2` or `width`.
///
/// The `value` of this channel can be a number or a string `"width"` for the width of the
/// plot.
///
/// Definition object for a constant value (primitive value or gradient definition) of an
/// encoding channel.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct XClass {
    /// Aggregation function for the field
    /// (e.g., `"mean"`, `"sum"`, `"median"`, `"min"`, `"max"`, `"count"`).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aggregate: Option<Aggregate>,
    /// An object defining properties of axis's gridlines, ticks and labels.
    /// If `null`, the axis for the encoding channel will be removed.
    ///
    /// __Default value:__ If undefined, default [axis
    /// properties](https://vega.github.io/vega-lite/docs/axis.html) are applied.
    ///
    /// __See also:__ [`axis`](https://vega.github.io/vega-lite/docs/axis.html) documentation.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub axis: RemovableValue<Axis>,
    /// For rect-based marks (`rect`, `bar`, and `image`), mark size relative to bandwidth of
    /// [band scales](https://vega.github.io/vega-lite/docs/scale.html#band) or time units. If
    /// set to `1`, the mark size is set to the bandwidth or the time unit interval. If set to
    /// `0.5`, the mark size is half of the bandwidth or the time unit interval.
    ///
    /// For other marks, relative position on a band of a stacked, binned, time unit or band
    /// scale. If set to `0`, the marks will be positioned at the beginning of the band. If set
    /// to `0.5`, the marks will be positioned in the middle of the band.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub band: Option<f64>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - If `"binned"`, this indicates that the data for the `x` (or `y`) channel are already
    /// binned. You can map the bin-start field to `x` (or `y`) and the bin-end field to `x2` (or
    /// `y2`). The scale and axis will be formatted similar to binning in Vega-Lite.  To adjust
    /// the axis ticks based on the bin step, you can also set the axis's
    /// [`tickMinStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    ///
    /// __See also:__ [`bin`](https://vega.github.io/vega-lite/docs/bin.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bin: Option<FluffyBin>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __See also:__ [`field`](https://vega.github.io/vega-lite/docs/field.html) documentation.
    ///
    /// __Notes:__
    /// 1)  Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects (e.g.,
    /// `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    /// 2) `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field: Option<Field>,
    /// An object defining the properties of the Impute Operation to be applied.
    /// The field value of the other positional channel is taken as `key` of the `Impute`
    /// Operation.
    /// The field of the `color` channel if specified is used as `groupby` of the `Impute`
    /// Operation.
    ///
    /// __See also:__ [`impute`](https://vega.github.io/vega-lite/docs/impute.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub impute: Option<ImputeParams>,
    /// An object defining properties of the channel's scale, which is the function that
    /// transforms values in the data domain (numbers, dates, strings, etc) to visual values
    /// (pixels, colors, sizes) of the encoding channels.
    ///
    /// If `null`, the scale will be [disabled and the data value will be directly
    /// encoded](https://vega.github.io/vega-lite/docs/scale.html#disable).
    ///
    /// __Default value:__ If undefined, default [scale
    /// properties](https://vega.github.io/vega-lite/docs/scale.html) are applied.
    ///
    /// __See also:__ [`scale`](https://vega.github.io/vega-lite/docs/scale.html) documentation.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub scale: RemovableValue<Scale>,
    /// Sort order for the encoded field.
    ///
    /// For continuous fields (quantitative or temporal), `sort` can be either `"ascending"` or
    /// `"descending"`.
    ///
    /// For discrete fields, `sort` can be one of the following:
    /// - `"ascending"` or `"descending"` -- for sorting by the values' natural order in
    /// JavaScript.
    /// - [A string indicating an encoding channel name to sort
    /// by](https://vega.github.io/vega-lite/docs/sort.html#sort-by-encoding) (e.g., `"x"` or
    /// `"y"`) with an optional minus prefix for descending sort (e.g., `"-x"` to sort by
    /// x-field, descending). This channel string is short-form of [a sort-by-encoding
    /// definition](https://vega.github.io/vega-lite/docs/sort.html#sort-by-encoding). For
    /// example, `"sort": "-x"` is equivalent to `"sort": {"encoding": "x", "order":
    /// "descending"}`.
    /// - [A sort field definition](https://vega.github.io/vega-lite/docs/sort.html#sort-field)
    /// for sorting by another field.
    /// - [An array specifying the field values in preferred
    /// order](https://vega.github.io/vega-lite/docs/sort.html#sort-array). In this case, the
    /// sort order will obey the values in the array, followed by any unspecified values in their
    /// original order. For discrete time field, values in the sort array can be [date-time
    /// definition objects](struct.DateTime.html). In addition, for time units `"month"` and `"day"`,
    /// the values can be the month or day names (case insensitive) or their 3-letter initials
    /// (e.g., `"Mon"`, `"Tue"`).
    /// - `null` indicating no sort.
    ///
    /// __Default value:__ `"ascending"`
    ///
    /// __Note:__ `null` and sorting by another channel is not supported for `row` and `column`.
    ///
    /// __See also:__ [`sort`](https://vega.github.io/vega-lite/docs/sort.html) documentation.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub sort: RemovableValue<SortUnion>,
    /// Type of stacking offset if the field should be stacked.
    /// `stack` is only applicable for `x` and `y` channels with continuous domains.
    /// For example, `stack` of `y` can be used to customize stacking for a vertical bar chart.
    ///
    /// `stack` can be one of the following values:
    /// - `"zero"` or `true`: stacking with baseline offset at zero value of the scale (for
    /// creating typical stacked [bar](https://vega.github.io/vega-lite/docs/stack.html#bar) and
    /// [area](https://vega.github.io/vega-lite/docs/stack.html#area) chart).
    /// - `"normalize"` - stacking with normalized domain (for creating [normalized stacked bar
    /// and area charts](https://vega.github.io/vega-lite/docs/stack.html#normalized). <br/>
    /// -`"center"` - stacking with center baseline (for
    /// [streamgraph](https://vega.github.io/vega-lite/docs/stack.html#streamgraph)).
    /// - `null` or `false` - No-stacking. This will produce layered
    /// [bar](https://vega.github.io/vega-lite/docs/stack.html#layered-bar-chart) and area
    /// chart.
    ///
    /// __Default value:__ `zero` for plots with all of the following conditions are true:
    /// (1) the mark is `bar` or `area`;
    /// (2) the stacked measure channel (x or y) has a linear scale;
    /// (3) At least one of non-position channels mapped to an unaggregated field that is
    /// different from x and y. Otherwise, `null` by default.
    ///
    /// __See also:__ [`stack`](https://vega.github.io/vega-lite/docs/stack.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stack: Option<Stack>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html)
    /// documentation.
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`). If the field has an aggregate function, the function
    /// is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is binned or
    /// has a time unit applied, the applied function is shown in parentheses (e.g., `"Profit
    /// (binned)"`, `"Transaction Date (year-month)"`). Otherwise, the title is simply the field
    /// name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the
    /// [`fieldTitle`](https://vega.github.io/vega-lite/docs/config.html#top-level-config)
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub title: RemovableValue<PurpleText>,
    /// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
    /// `"nominal"`).
    /// It can also be a `"geojson"` type for encoding
    /// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
    ///
    ///
    /// __Note:__
    ///
    /// - Data values for a temporal field can be either a date-time string (e.g., `"2015-03-07
    /// 12:32:17"`, `"17:01"`, `"2015-03-16"`. `"2015"`) or a timestamp number (e.g.,
    /// `1552199579097`).
    /// - Data `type` describes the semantics of the data rather than the primitive data types
    /// (number, string, etc.). The same primitive data type can have different types of
    /// measurement. For example, numeric data can represent quantitative, ordinal, or nominal
    /// data.
    /// - When using with [`bin`](https://vega.github.io/vega-lite/docs/bin.html), the `type`
    /// property can be either `"quantitative"` (for using a linear bin scale) or [`"ordinal"`
    /// (for using an ordinal bin
    /// scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html), the
    /// `type` property can be either `"temporal"` (for using a temporal scale) or [`"ordinal"`
    /// (for using an ordinal scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html),
    /// the `type` property refers to the post-aggregation data type. For example, we can
    /// calculate count `distinct` of a categorical field `"cat"` using `{"aggregate":
    /// "distinct", "field": "cat", "type": "quantitative"}`. The `"type"` of the aggregate
    /// output is `"quantitative"`.
    /// - Secondary channels (e.g., `x2`, `y2`, `xError`, `yError`) do not have `type` as they
    /// have exactly the same type as their primary channels (e.g., `x`, `y`).
    ///
    /// __See also:__ [`type`](https://vega.github.io/vega-lite/docs/type.html) documentation.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub def_type: Option<StandardType>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<XUnion>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct Axis {
    /// An interpolation fraction indicating where, for `band` scales, axis ticks should be
    /// positioned. A value of `0` places ticks at the left edge of their bands. A value of `0.5`
    /// places ticks in the middle of their bands.
    ///
    /// __Default value:__ `0.5`
    #[serde(rename = "bandPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub band_position: Option<f64>,
    /// A boolean flag indicating if the domain (the axis baseline) should be included as part of
    /// the axis.
    ///
    /// __Default value:__ `true`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub domain: Option<bool>,
    /// Color of axis domain line.
    ///
    /// __Default value:__ `"gray"`.
    #[serde(rename = "domainColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub domain_color: Option<String>,
    /// An array of alternating [stroke, space] lengths for dashed domain lines.
    #[serde(rename = "domainDash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub domain_dash: Option<Vec<f64>>,
    /// The pixel offset at which to start drawing with the domain dash array.
    #[serde(rename = "domainDashOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub domain_dash_offset: Option<f64>,
    /// Opacity of the axis domain line.
    #[serde(rename = "domainOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub domain_opacity: Option<f64>,
    /// Stroke width of axis domain line
    ///
    /// __Default value:__ `1`
    #[serde(rename = "domainWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub domain_width: Option<f64>,
    /// The text formatting pattern for labels of guides (axes, legends, headers) and text
    /// marks.
    ///
    /// - If the format type is `"number"` (e.g., for quantitative fields), this is D3's [number
    /// format pattern](https://github.com/d3/d3-format#locale_format).
    /// - If the format type is `"time"` (e.g., for temporal fields), this is D3's [time format
    /// pattern](https://github.com/d3/d3-time-format#locale_format).
    ///
    /// See the [format documentation](https://vega.github.io/vega-lite/docs/format.html) for
    /// more examples.
    ///
    /// __Default value:__  Derived from
    /// [numberFormat](https://vega.github.io/vega-lite/docs/config.html#format) config for
    /// number format and from
    /// [timeFormat](https://vega.github.io/vega-lite/docs/config.html#format) config for time
    /// format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub format: Option<String>,
    /// The format type for labels (`"number"` or `"time"`).
    ///
    /// __Default value:__
    /// - `"time"` for temporal fields and ordinal and nomimal fields with `timeUnit`.
    /// - `"number"` for quantitative fields as well as ordinal and nomimal fields without
    /// `timeUnit`.
    #[serde(rename = "formatType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub format_type: Option<FormatType>,
    /// A boolean flag indicating if grid lines should be included as part of the axis
    ///
    /// __Default value:__ `true` for [continuous
    /// scales](https://vega.github.io/vega-lite/docs/scale.html#continuous) that are not binned;
    /// otherwise, `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub grid: Option<bool>,
    #[serde(rename = "gridColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub grid_color: Option<Color>,
    #[serde(rename = "gridDash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub grid_dash: Option<Dash>,
    #[serde(rename = "gridDashOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub grid_dash_offset: Option<GridDashOffset>,
    #[serde(rename = "gridOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub grid_opacity: Option<GridOpacity>,
    #[serde(rename = "gridWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub grid_width: Option<GridWidth>,
    #[serde(rename = "labelAlign")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_align: Option<LabelAlign>,
    /// The rotation angle of the axis labels.
    ///
    /// __Default value:__ `-90` for nominal and ordinal fields; `0` otherwise.
    #[serde(rename = "labelAngle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_angle: Option<f64>,
    #[serde(rename = "labelBaseline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_baseline: Option<TextBaseline>,
    /// Indicates if labels should be hidden if they exceed the axis range. If `false` (the
    /// default) no bounds overlap analysis is performed. If `true`, labels will be hidden if
    /// they exceed the axis range by more than 1 pixel. If this property is a number, it
    /// specifies the pixel tolerance: the maximum amount by which a label bounding box may
    /// exceed the axis range.
    ///
    /// __Default value:__ `false`.
    #[serde(rename = "labelBound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_bound: Option<Label>,
    #[serde(rename = "labelColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_color: Option<Color>,
    /// [Vega expression](https://vega.github.io/vega/docs/expressions/) for customizing labels.
    ///
    /// __Note:__ The label text and value can be assessed via the `label` and `value` properties
    /// of the axis's backing `datum` object.
    #[serde(rename = "labelExpr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_expr: Option<String>,
    /// Indicates if the first and last axis labels should be aligned flush with the scale range.
    /// Flush alignment for a horizontal axis will left-align the first label and right-align the
    /// last label. For vertical axes, bottom and top text baselines are applied instead. If this
    /// property is a number, it also indicates the number of pixels by which to offset the first
    /// and last labels; for example, a value of 2 will flush-align the first and last labels and
    /// also push them 2 pixels outward from the center of the axis. The additional adjustment
    /// can sometimes help the labels better visually group with corresponding axis ticks.
    ///
    /// __Default value:__ `true` for axis of a continuous x-scale. Otherwise, `false`.
    #[serde(rename = "labelFlush")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_flush: Option<Label>,
    /// Indicates the number of pixels by which to offset flush-adjusted labels. For example, a
    /// value of `2` will push flush-adjusted labels 2 pixels outward from the center of the
    /// axis. Offsets can help the labels better visually group with corresponding axis ticks.
    ///
    /// __Default value:__ `0`.
    #[serde(rename = "labelFlushOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_flush_offset: Option<f64>,
    #[serde(rename = "labelFont")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_font: Option<LabelFont>,
    #[serde(rename = "labelFontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_font_size: Option<GridWidth>,
    #[serde(rename = "labelFontStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_font_style: Option<LabelFontStyle>,
    #[serde(rename = "labelFontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_font_weight: Option<LabelFontWeightUnion>,
    /// Maximum allowed pixel width of axis tick labels.
    ///
    /// __Default value:__ `180`
    #[serde(rename = "labelLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_limit: Option<f64>,
    #[serde(rename = "labelOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_opacity: Option<GridDashOffset>,
    /// The strategy to use for resolving overlap of axis labels. If `false` (the default), no
    /// overlap reduction is attempted. If set to `true` or `"parity"`, a strategy of removing
    /// every other label is used (this works well for standard linear axes). If set to
    /// `"greedy"`, a linear scan of the labels is performed, removing any labels that overlaps
    /// with the last visible label (this often works better for log-scaled axes).
    ///
    /// __Default value:__ `true` for non-nominal fields with non-log scales; `"greedy"` for log
    /// scales; otherwise `false`.
    #[serde(rename = "labelOverlap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_overlap: Option<LabelOverlap>,
    /// The padding, in pixels, between axis and text labels.
    ///
    /// __Default value:__ `2`
    #[serde(rename = "labelPadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_padding: Option<f64>,
    /// A boolean flag indicating if labels should be included as part of the axis.
    ///
    /// __Default value:__ `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub labels: Option<bool>,
    /// The minimum separation that must be between label bounding boxes for them to be
    /// considered non-overlapping (default `0`). This property is ignored if *labelOverlap*
    /// resolution is not enabled.
    #[serde(rename = "labelSeparation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_separation: Option<f64>,
    /// The maximum extent in pixels that axis ticks and labels should use. This determines a
    /// maximum offset value for axis titles.
    ///
    /// __Default value:__ `undefined`.
    #[serde(rename = "maxExtent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub max_extent: Option<f64>,
    /// The minimum extent in pixels that axis ticks and labels should use. This determines a
    /// minimum offset value for axis titles.
    ///
    /// __Default value:__ `30` for y-axis; `undefined` for x-axis.
    #[serde(rename = "minExtent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub min_extent: Option<f64>,
    /// The offset, in pixels, by which to displace the axis from the edge of the enclosing group
    /// or data rectangle.
    ///
    /// __Default value:__ derived from the [axis
    /// config](https://vega.github.io/vega-lite/docs/config.html#facet-scale-config)'s `offset`
    /// (`0` by default)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub offset: Option<f64>,
    /// The orientation of the axis. One of `"top"`, `"bottom"`, `"left"` or `"right"`. The
    /// orientation can be used to further specialize the axis type (e.g., a y-axis oriented
    /// towards the right edge of the chart).
    ///
    /// __Default value:__ `"bottom"` for x-axes and `"left"` for y-axes.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub orient: Option<Orient>,
    /// The anchor position of the axis in pixels. For x-axes with top or bottom orientation,
    /// this sets the axis group x coordinate. For y-axes with left or right orientation, this
    /// sets the axis group y coordinate.
    ///
    /// __Default value__: `0`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub position: Option<f64>,
    /// For band scales, indicates if ticks and grid lines should be placed at the center of a
    /// band (default) or at the band extents to indicate intervals.
    #[serde(rename = "tickBand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tick_band: Option<TickBand>,
    #[serde(rename = "tickColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tick_color: Option<Color>,
    /// A desired number of ticks, for axes visualizing quantitative scales. The resulting number
    /// may be different so that values are "nice" (multiples of 2, 5, 10) and lie within the
    /// underlying scale's range.
    ///
    /// __Default value__: Determine using a formula `ceil(width/40)` for x and `ceil(height/40)`
    /// for y.
    #[serde(rename = "tickCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tick_count: Option<f64>,
    #[serde(rename = "tickDash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tick_dash: Option<Dash>,
    #[serde(rename = "tickDashOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tick_dash_offset: Option<GridDashOffset>,
    /// Boolean flag indicating if an extra axis tick should be added for the initial position of
    /// the axis. This flag is useful for styling axes for `band` scales such that ticks are
    /// placed on band boundaries rather in the middle of a band. Use in conjunction with
    /// `"bandPosition": 1` and an axis `"padding"` value of `0`.
    #[serde(rename = "tickExtra")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tick_extra: Option<bool>,
    /// The minimum desired step between axis ticks, in terms of scale domain values. For
    /// example, a value of `1` indicates that ticks should not be less than 1 unit apart. If
    /// `tickMinStep` is specified, the `tickCount` value will be adjusted, if necessary, to
    /// enforce the minimum step value.
    ///
    /// __Default value__: `undefined`
    #[serde(rename = "tickMinStep")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tick_min_step: Option<f64>,
    /// Position offset in pixels to apply to ticks, labels, and gridlines.
    #[serde(rename = "tickOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tick_offset: Option<f64>,
    #[serde(rename = "tickOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tick_opacity: Option<GridDashOffset>,
    /// Boolean flag indicating if pixel position values should be rounded to the nearest
    /// integer.
    ///
    /// __Default value:__ `true`
    #[serde(rename = "tickRound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tick_round: Option<bool>,
    /// Boolean value that determines whether the axis should include ticks.
    ///
    /// __Default value:__ `true`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ticks: Option<bool>,
    /// The size in pixels of axis ticks.
    ///
    /// __Default value:__ `5`
    #[serde(rename = "tickSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tick_size: Option<f64>,
    #[serde(rename = "tickWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tick_width: Option<GridWidth>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`). If the field has an aggregate function, the function
    /// is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is binned or
    /// has a time unit applied, the applied function is shown in parentheses (e.g., `"Profit
    /// (binned)"`, `"Transaction Date (year-month)"`). Otherwise, the title is simply the field
    /// name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the
    /// [`fieldTitle`](https://vega.github.io/vega-lite/docs/config.html#top-level-config)
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub title: RemovableValue<PurpleText>,
    /// Horizontal text alignment of axis titles.
    #[serde(rename = "titleAlign")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_align: Option<Align>,
    /// Text anchor position for placing axis titles.
    #[serde(rename = "titleAnchor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_anchor: Option<TitleAnchorEnum>,
    /// Angle in degrees of axis titles.
    #[serde(rename = "titleAngle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_angle: Option<f64>,
    /// Vertical text baseline for axis titles.
    #[serde(rename = "titleBaseline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_baseline: Option<Baseline>,
    /// Color of the title, can be in hex color code or regular color name.
    #[serde(rename = "titleColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_color: Option<String>,
    /// Font of the title. (e.g., `"Helvetica Neue"`).
    #[serde(rename = "titleFont")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_font: Option<String>,
    /// Font size of the title.
    #[serde(rename = "titleFontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_font_size: Option<f64>,
    /// Font style of the title.
    #[serde(rename = "titleFontStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_font_style: Option<String>,
    /// Font weight of the title.
    /// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
    /// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
    #[serde(rename = "titleFontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_font_weight: Option<FontWeight>,
    /// Maximum allowed pixel width of axis titles.
    #[serde(rename = "titleLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_limit: Option<f64>,
    /// Line height in pixels for multi-line title text.
    #[serde(rename = "titleLineHeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_line_height: Option<f64>,
    /// Opacity of the axis title.
    #[serde(rename = "titleOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_opacity: Option<f64>,
    /// The padding, in pixels, between title and axis.
    #[serde(rename = "titlePadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_padding: Option<f64>,
    /// X-coordinate of the axis title relative to the axis group.
    #[serde(rename = "titleX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_x: Option<f64>,
    /// Y-coordinate of the axis title relative to the axis group.
    #[serde(rename = "titleY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_y: Option<f64>,
    /// Translation offset in pixels applied to the axis group mark x and y. If specified,
    /// overrides the default behavior of a 0.5 offset to pixel-align stroked lines.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub translate: Option<f64>,
    /// Explicitly set the visible axis tick values.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub values: Option<Vec<Equal>>,
    /// A non-negative integer indicating the z-index of the axis.
    /// If zindex is 0, axes should be drawn behind all chart elements.
    /// To put them in front, set `zindex` to `1` or more.
    ///
    /// __Default value:__ `0` (behind the marks).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub zindex: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ConditionalAxisPropertyColorNull {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub condition: Option<ConditionalAxisPropertyColorNullCondition>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ConditionalPredicateValueDefColorNull {
    /// Predicate for triggering the condition
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub test: Option<LogicalOperandPredicateElement>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ConditionalAxisPropertyNumberNull {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub condition: Option<ConditionalAxisPropertyNumberNullCondition>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<Vec<f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ConditionalPredicateValueDefNumberNull {
    /// Predicate for triggering the condition
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub test: Option<LogicalOperandPredicateElement>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<Vec<f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ConditionalAxisPropertyNumberNullClass {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub condition: Option<ConditionalAxisPropertyNumberNullConditionUnion>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ConditionalPredicateValueDefNumberNullElement {
    /// Predicate for triggering the condition
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub test: Option<LogicalOperandPredicateElement>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ConditionalAxisPropertyTextBaselineNull {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub condition: Option<ConditionalAxisPropertyTextBaselineNullCondition>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<Baseline>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ConditionalPredicateValueDefTextBaselineNull {
    /// Predicate for triggering the condition
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub test: Option<LogicalOperandPredicateElement>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<Baseline>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ConditionalAxisPropertyStringNull {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub condition: Option<ConditionalAxisPropertyStringNullCondition>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ConditionalPredicateStringValueDef {
    /// Predicate for triggering the condition
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub test: Option<LogicalOperandPredicateElement>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ConditionalAxisPropertyFontStyleNull {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub condition: Option<ConditionalAxisPropertyFontStyleNullCondition>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ConditionalPredicateValueDefFontStyleNull {
    /// Predicate for triggering the condition
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub test: Option<LogicalOperandPredicateElement>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ConditionalAxisPropertyFontWeightNull {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub condition: Option<ConditionalAxisPropertyFontWeightNullCondition>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<FontWeight>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ConditionalPredicateValueDefFontWeightNull {
    /// Predicate for triggering the condition
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub test: Option<LogicalOperandPredicateElement>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<FontWeight>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ImputeParams {
    /// A frame specification as a two-element array used to control the window over which the
    /// specified method is applied. The array entries should either be a number indicating the
    /// offset from the current data object, or null to indicate unbounded rows preceding or
    /// following the current data object. For example, the value `[-5, 5]` indicates that the
    /// window should include five objects preceding and five objects following the current
    /// object.
    ///
    /// __Default value:__:  `[null, null]` indicating that the window includes all objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub frame: Option<Vec<Option<f64>>>,
    /// Defines the key values that should be considered for imputation.
    /// An array of key values or an object defining a [number
    /// sequence](https://vega.github.io/vega-lite/docs/impute.html#sequence-def).
    ///
    /// If provided, this will be used in addition to the key values observed within the input
    /// data. If not provided, the values will be derived from all unique values of the `key`
    /// field. For `impute` in `encoding`, the key field is the x-field if the y-field is
    /// imputed, or vice versa.
    ///
    /// If there is no impute grouping, this property _must_ be specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub keyvals: Option<Keyvals>,
    /// The imputation method to use for the field value of imputed data objects.
    /// One of `"value"`, `"mean"`, `"median"`, `"max"` or `"min"`.
    ///
    /// __Default value:__  `"value"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub method: Option<ImputeParamsMethod>,
    /// The field value to use when the imputation `method` is `"value"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ImputeSequence {
    /// The starting value of the sequence.
    /// __Default value:__ `0`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub start: Option<f64>,
    /// The step value between sequence entries.
    /// __Default value:__ `1` or `-1` if `stop < start`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub step: Option<f64>,
    /// The ending value(exclusive) of the sequence.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stop: Option<f64>,
}

/// X2 coordinates for ranged `"area"`, `"bar"`, `"rect"`, and  `"rule"`.
///
/// The `value` of this channel can be a number or a string `"width"` for the width of the
/// plot.
///
/// A field definition of a secondary channel that shares a scale with another primary
/// channel. For example, `x2`, `xError` and `xError2` share the same scale with `x`.
///
/// Definition object for a constant value (primitive value or gradient definition) of an
/// encoding channel.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct X2Class {
    /// Aggregation function for the field
    /// (e.g., `"mean"`, `"sum"`, `"median"`, `"min"`, `"max"`, `"count"`).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aggregate: Option<Aggregate>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - If `"binned"`, this indicates that the data for the `x` (or `y`) channel are already
    /// binned. You can map the bin-start field to `x` (or `y`) and the bin-end field to `x2` (or
    /// `y2`). The scale and axis will be formatted similar to binning in Vega-Lite.  To adjust
    /// the axis ticks based on the bin step, you can also set the axis's
    /// [`tickMinStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    ///
    /// __See also:__ [`bin`](https://vega.github.io/vega-lite/docs/bin.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bin: Option<serde_json::Value>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __See also:__ [`field`](https://vega.github.io/vega-lite/docs/field.html) documentation.
    ///
    /// __Notes:__
    /// 1)  Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects (e.g.,
    /// `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    /// 2) `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field: Option<Field>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html)
    /// documentation.
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`). If the field has an aggregate function, the function
    /// is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is binned or
    /// has a time unit applied, the applied function is shown in parentheses (e.g., `"Profit
    /// (binned)"`, `"Transaction Date (year-month)"`). Otherwise, the title is simply the field
    /// name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the
    /// [`fieldTitle`](https://vega.github.io/vega-lite/docs/config.html#top-level-config)
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub title: RemovableValue<PurpleText>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<XUnion>,
}

/// Y coordinates of the marks, or height of vertical `"bar"` and `"area"` without specified
/// `y2` or `height`.
///
/// The `value` of this channel can be a number or a string `"height"` for the height of the
/// plot.
///
/// Definition object for a constant value (primitive value or gradient definition) of an
/// encoding channel.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct YClass {
    /// Aggregation function for the field
    /// (e.g., `"mean"`, `"sum"`, `"median"`, `"min"`, `"max"`, `"count"`).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aggregate: Option<Aggregate>,
    /// An object defining properties of axis's gridlines, ticks and labels.
    /// If `null`, the axis for the encoding channel will be removed.
    ///
    /// __Default value:__ If undefined, default [axis
    /// properties](https://vega.github.io/vega-lite/docs/axis.html) are applied.
    ///
    /// __See also:__ [`axis`](https://vega.github.io/vega-lite/docs/axis.html) documentation.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub axis: RemovableValue<Axis>,
    /// For rect-based marks (`rect`, `bar`, and `image`), mark size relative to bandwidth of
    /// [band scales](https://vega.github.io/vega-lite/docs/scale.html#band) or time units. If
    /// set to `1`, the mark size is set to the bandwidth or the time unit interval. If set to
    /// `0.5`, the mark size is half of the bandwidth or the time unit interval.
    ///
    /// For other marks, relative position on a band of a stacked, binned, time unit or band
    /// scale. If set to `0`, the marks will be positioned at the beginning of the band. If set
    /// to `0.5`, the marks will be positioned in the middle of the band.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub band: Option<f64>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - If `"binned"`, this indicates that the data for the `x` (or `y`) channel are already
    /// binned. You can map the bin-start field to `x` (or `y`) and the bin-end field to `x2` (or
    /// `y2`). The scale and axis will be formatted similar to binning in Vega-Lite.  To adjust
    /// the axis ticks based on the bin step, you can also set the axis's
    /// [`tickMinStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    ///
    /// __See also:__ [`bin`](https://vega.github.io/vega-lite/docs/bin.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bin: Option<FluffyBin>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __See also:__ [`field`](https://vega.github.io/vega-lite/docs/field.html) documentation.
    ///
    /// __Notes:__
    /// 1)  Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects (e.g.,
    /// `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    /// 2) `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field: Option<Field>,
    /// An object defining the properties of the Impute Operation to be applied.
    /// The field value of the other positional channel is taken as `key` of the `Impute`
    /// Operation.
    /// The field of the `color` channel if specified is used as `groupby` of the `Impute`
    /// Operation.
    ///
    /// __See also:__ [`impute`](https://vega.github.io/vega-lite/docs/impute.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub impute: Option<ImputeParams>,
    /// An object defining properties of the channel's scale, which is the function that
    /// transforms values in the data domain (numbers, dates, strings, etc) to visual values
    /// (pixels, colors, sizes) of the encoding channels.
    ///
    /// If `null`, the scale will be [disabled and the data value will be directly
    /// encoded](https://vega.github.io/vega-lite/docs/scale.html#disable).
    ///
    /// __Default value:__ If undefined, default [scale
    /// properties](https://vega.github.io/vega-lite/docs/scale.html) are applied.
    ///
    /// __See also:__ [`scale`](https://vega.github.io/vega-lite/docs/scale.html) documentation.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub scale: RemovableValue<Scale>,
    /// Sort order for the encoded field.
    ///
    /// For continuous fields (quantitative or temporal), `sort` can be either `"ascending"` or
    /// `"descending"`.
    ///
    /// For discrete fields, `sort` can be one of the following:
    /// - `"ascending"` or `"descending"` -- for sorting by the values' natural order in
    /// JavaScript.
    /// - [A string indicating an encoding channel name to sort
    /// by](https://vega.github.io/vega-lite/docs/sort.html#sort-by-encoding) (e.g., `"x"` or
    /// `"y"`) with an optional minus prefix for descending sort (e.g., `"-x"` to sort by
    /// x-field, descending). This channel string is short-form of [a sort-by-encoding
    /// definition](https://vega.github.io/vega-lite/docs/sort.html#sort-by-encoding). For
    /// example, `"sort": "-x"` is equivalent to `"sort": {"encoding": "x", "order":
    /// "descending"}`.
    /// - [A sort field definition](https://vega.github.io/vega-lite/docs/sort.html#sort-field)
    /// for sorting by another field.
    /// - [An array specifying the field values in preferred
    /// order](https://vega.github.io/vega-lite/docs/sort.html#sort-array). In this case, the
    /// sort order will obey the values in the array, followed by any unspecified values in their
    /// original order. For discrete time field, values in the sort array can be [date-time
    /// definition objects](struct.DateTime.html). In addition, for time units `"month"` and `"day"`,
    /// the values can be the month or day names (case insensitive) or their 3-letter initials
    /// (e.g., `"Mon"`, `"Tue"`).
    /// - `null` indicating no sort.
    ///
    /// __Default value:__ `"ascending"`
    ///
    /// __Note:__ `null` and sorting by another channel is not supported for `row` and `column`.
    ///
    /// __See also:__ [`sort`](https://vega.github.io/vega-lite/docs/sort.html) documentation.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub sort: RemovableValue<SortUnion>,
    /// Type of stacking offset if the field should be stacked.
    /// `stack` is only applicable for `x` and `y` channels with continuous domains.
    /// For example, `stack` of `y` can be used to customize stacking for a vertical bar chart.
    ///
    /// `stack` can be one of the following values:
    /// - `"zero"` or `true`: stacking with baseline offset at zero value of the scale (for
    /// creating typical stacked [bar](https://vega.github.io/vega-lite/docs/stack.html#bar) and
    /// [area](https://vega.github.io/vega-lite/docs/stack.html#area) chart).
    /// - `"normalize"` - stacking with normalized domain (for creating [normalized stacked bar
    /// and area charts](https://vega.github.io/vega-lite/docs/stack.html#normalized). <br/>
    /// -`"center"` - stacking with center baseline (for
    /// [streamgraph](https://vega.github.io/vega-lite/docs/stack.html#streamgraph)).
    /// - `null` or `false` - No-stacking. This will produce layered
    /// [bar](https://vega.github.io/vega-lite/docs/stack.html#layered-bar-chart) and area
    /// chart.
    ///
    /// __Default value:__ `zero` for plots with all of the following conditions are true:
    /// (1) the mark is `bar` or `area`;
    /// (2) the stacked measure channel (x or y) has a linear scale;
    /// (3) At least one of non-position channels mapped to an unaggregated field that is
    /// different from x and y. Otherwise, `null` by default.
    ///
    /// __See also:__ [`stack`](https://vega.github.io/vega-lite/docs/stack.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stack: Option<Stack>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html)
    /// documentation.
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`). If the field has an aggregate function, the function
    /// is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is binned or
    /// has a time unit applied, the applied function is shown in parentheses (e.g., `"Profit
    /// (binned)"`, `"Transaction Date (year-month)"`). Otherwise, the title is simply the field
    /// name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the
    /// [`fieldTitle`](https://vega.github.io/vega-lite/docs/config.html#top-level-config)
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub title: RemovableValue<PurpleText>,
    /// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
    /// `"nominal"`).
    /// It can also be a `"geojson"` type for encoding
    /// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
    ///
    ///
    /// __Note:__
    ///
    /// - Data values for a temporal field can be either a date-time string (e.g., `"2015-03-07
    /// 12:32:17"`, `"17:01"`, `"2015-03-16"`. `"2015"`) or a timestamp number (e.g.,
    /// `1552199579097`).
    /// - Data `type` describes the semantics of the data rather than the primitive data types
    /// (number, string, etc.). The same primitive data type can have different types of
    /// measurement. For example, numeric data can represent quantitative, ordinal, or nominal
    /// data.
    /// - When using with [`bin`](https://vega.github.io/vega-lite/docs/bin.html), the `type`
    /// property can be either `"quantitative"` (for using a linear bin scale) or [`"ordinal"`
    /// (for using an ordinal bin
    /// scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html), the
    /// `type` property can be either `"temporal"` (for using a temporal scale) or [`"ordinal"`
    /// (for using an ordinal scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html),
    /// the `type` property refers to the post-aggregation data type. For example, we can
    /// calculate count `distinct` of a categorical field `"cat"` using `{"aggregate":
    /// "distinct", "field": "cat", "type": "quantitative"}`. The `"type"` of the aggregate
    /// output is `"quantitative"`.
    /// - Secondary channels (e.g., `x2`, `y2`, `xError`, `yError`) do not have `type` as they
    /// have exactly the same type as their primary channels (e.g., `x`, `y`).
    ///
    /// __See also:__ [`type`](https://vega.github.io/vega-lite/docs/type.html) documentation.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub def_type: Option<StandardType>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<YUnion>,
}

/// Y2 coordinates for ranged `"area"`, `"bar"`, `"rect"`, and  `"rule"`.
///
/// The `value` of this channel can be a number or a string `"height"` for the height of the
/// plot.
///
/// A field definition of a secondary channel that shares a scale with another primary
/// channel. For example, `x2`, `xError` and `xError2` share the same scale with `x`.
///
/// Definition object for a constant value (primitive value or gradient definition) of an
/// encoding channel.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct Y2Class {
    /// Aggregation function for the field
    /// (e.g., `"mean"`, `"sum"`, `"median"`, `"min"`, `"max"`, `"count"`).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aggregate: Option<Aggregate>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - If `"binned"`, this indicates that the data for the `x` (or `y`) channel are already
    /// binned. You can map the bin-start field to `x` (or `y`) and the bin-end field to `x2` (or
    /// `y2`). The scale and axis will be formatted similar to binning in Vega-Lite.  To adjust
    /// the axis ticks based on the bin step, you can also set the axis's
    /// [`tickMinStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    ///
    /// __See also:__ [`bin`](https://vega.github.io/vega-lite/docs/bin.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bin: Option<serde_json::Value>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __See also:__ [`field`](https://vega.github.io/vega-lite/docs/field.html) documentation.
    ///
    /// __Notes:__
    /// 1)  Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects (e.g.,
    /// `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    /// 2) `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field: Option<Field>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html)
    /// documentation.
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`). If the field has an aggregate function, the function
    /// is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is binned or
    /// has a time unit applied, the applied function is shown in parentheses (e.g., `"Profit
    /// (binned)"`, `"Transaction Date (year-month)"`). Otherwise, the title is simply the field
    /// name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the
    /// [`fieldTitle`](https://vega.github.io/vega-lite/docs/config.html#top-level-config)
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub title: RemovableValue<PurpleText>,
    /// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
    /// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
    /// between `0` to `1` for opacity).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<YUnion>,
}

/// Definition for how to facet the data. One of:
/// 1) [a field definition for faceting the plot by one
/// field](https://vega.github.io/vega-lite/docs/facet.html#field-def)
/// 2) [An object that maps `row` and `column` channels to their field
/// definitions](https://vega.github.io/vega-lite/docs/facet.html#mapping)
///
/// A field definition for the horizontal facet of trellis plots.
///
/// A field definition for the vertical facet of trellis plots.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct Facet {
    /// Aggregation function for the field
    /// (e.g., `"mean"`, `"sum"`, `"median"`, `"min"`, `"max"`, `"count"`).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aggregate: Option<Aggregate>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - If `"binned"`, this indicates that the data for the `x` (or `y`) channel are already
    /// binned. You can map the bin-start field to `x` (or `y`) and the bin-end field to `x2` (or
    /// `y2`). The scale and axis will be formatted similar to binning in Vega-Lite.  To adjust
    /// the axis ticks based on the bin step, you can also set the axis's
    /// [`tickMinStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    ///
    /// __See also:__ [`bin`](https://vega.github.io/vega-lite/docs/bin.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bin: Option<PurpleBin>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __See also:__ [`field`](https://vega.github.io/vega-lite/docs/field.html) documentation.
    ///
    /// __Notes:__
    /// 1)  Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects (e.g.,
    /// `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    /// 2) `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field: Option<Field>,
    /// An object defining properties of a facet's header.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub header: Option<Header>,
    /// Sort order for the encoded field.
    ///
    /// For continuous fields (quantitative or temporal), `sort` can be either `"ascending"` or
    /// `"descending"`.
    ///
    /// For discrete fields, `sort` can be one of the following:
    /// - `"ascending"` or `"descending"` -- for sorting by the values' natural order in
    /// JavaScript.
    /// - [A sort field definition](https://vega.github.io/vega-lite/docs/sort.html#sort-field)
    /// for sorting by another field.
    /// - [An array specifying the field values in preferred
    /// order](https://vega.github.io/vega-lite/docs/sort.html#sort-array). In this case, the
    /// sort order will obey the values in the array, followed by any unspecified values in their
    /// original order. For discrete time field, values in the sort array can be [date-time
    /// definition objects](struct.DateTime.html). In addition, for time units `"month"` and `"day"`,
    /// the values can be the month or day names (case insensitive) or their 3-letter initials
    /// (e.g., `"Mon"`, `"Tue"`).
    /// - `null` indicating no sort.
    ///
    /// __Default value:__ `"ascending"`
    ///
    /// __Note:__ `null` is not supported for `row` and `column`.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub sort: RemovableValue<SortArray>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html)
    /// documentation.
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`). If the field has an aggregate function, the function
    /// is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is binned or
    /// has a time unit applied, the applied function is shown in parentheses (e.g., `"Profit
    /// (binned)"`, `"Transaction Date (year-month)"`). Otherwise, the title is simply the field
    /// name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the
    /// [`fieldTitle`](https://vega.github.io/vega-lite/docs/config.html#top-level-config)
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub title: RemovableValue<PurpleText>,
    /// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
    /// `"nominal"`).
    /// It can also be a `"geojson"` type for encoding
    /// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
    ///
    ///
    /// __Note:__
    ///
    /// - Data values for a temporal field can be either a date-time string (e.g., `"2015-03-07
    /// 12:32:17"`, `"17:01"`, `"2015-03-16"`. `"2015"`) or a timestamp number (e.g.,
    /// `1552199579097`).
    /// - Data `type` describes the semantics of the data rather than the primitive data types
    /// (number, string, etc.). The same primitive data type can have different types of
    /// measurement. For example, numeric data can represent quantitative, ordinal, or nominal
    /// data.
    /// - When using with [`bin`](https://vega.github.io/vega-lite/docs/bin.html), the `type`
    /// property can be either `"quantitative"` (for using a linear bin scale) or [`"ordinal"`
    /// (for using an ordinal bin
    /// scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html), the
    /// `type` property can be either `"temporal"` (for using a temporal scale) or [`"ordinal"`
    /// (for using an ordinal scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html),
    /// the `type` property refers to the post-aggregation data type. For example, we can
    /// calculate count `distinct` of a categorical field `"cat"` using `{"aggregate":
    /// "distinct", "field": "cat", "type": "quantitative"}`. The `"type"` of the aggregate
    /// output is `"quantitative"`.
    /// - Secondary channels (e.g., `x2`, `y2`, `xError`, `yError`) do not have `type` as they
    /// have exactly the same type as their primary channels (e.g., `x`, `y`).
    ///
    /// __See also:__ [`type`](https://vega.github.io/vega-lite/docs/type.html) documentation.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub facet_type: Option<StandardType>,
    /// A field definition for the horizontal facet of trellis plots.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub column: Option<FacetFieldDef>,
    /// A field definition for the vertical facet of trellis plots.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub row: Option<FacetFieldDef>,
}

/// A field definition for the horizontal facet of trellis plots.
///
/// A field definition for the vertical facet of trellis plots.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct FacetFieldDef {
    /// Aggregation function for the field
    /// (e.g., `"mean"`, `"sum"`, `"median"`, `"min"`, `"max"`, `"count"`).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aggregate: Option<Aggregate>,
    /// A flag for binning a `quantitative` field, [an object defining binning
    /// parameters](https://vega.github.io/vega-lite/docs/bin.html#params), or indicating that
    /// the data for `x` or `y` channel are binned before they are imported into Vega-Lite
    /// (`"binned"`).
    ///
    /// - If `true`, default [binning parameters](https://vega.github.io/vega-lite/docs/bin.html)
    /// will be applied.
    ///
    /// - If `"binned"`, this indicates that the data for the `x` (or `y`) channel are already
    /// binned. You can map the bin-start field to `x` (or `y`) and the bin-end field to `x2` (or
    /// `y2`). The scale and axis will be formatted similar to binning in Vega-Lite.  To adjust
    /// the axis ticks based on the bin step, you can also set the axis's
    /// [`tickMinStep`](https://vega.github.io/vega-lite/docs/axis.html#ticks) property.
    ///
    /// __Default value:__ `false`
    ///
    /// __See also:__ [`bin`](https://vega.github.io/vega-lite/docs/bin.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bin: Option<PurpleBin>,
    /// __Required.__ A string defining the name of the field from which to pull a data value
    /// or an object defining iterated values from the
    /// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
    ///
    /// __See also:__ [`field`](https://vega.github.io/vega-lite/docs/field.html) documentation.
    ///
    /// __Notes:__
    /// 1)  Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects (e.g.,
    /// `"field": "foo.bar"` and `"field": "foo['bar']"`).
    /// If field names contain dots or brackets but are not nested, you can use `\\` to escape
    /// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
    /// See more details about escaping in the [field
    /// documentation](https://vega.github.io/vega-lite/docs/field.html).
    /// 2) `field` is not required if `aggregate` is `count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field: Option<Field>,
    /// An object defining properties of a facet's header.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub header: Option<Header>,
    /// Sort order for the encoded field.
    ///
    /// For continuous fields (quantitative or temporal), `sort` can be either `"ascending"` or
    /// `"descending"`.
    ///
    /// For discrete fields, `sort` can be one of the following:
    /// - `"ascending"` or `"descending"` -- for sorting by the values' natural order in
    /// JavaScript.
    /// - [A sort field definition](https://vega.github.io/vega-lite/docs/sort.html#sort-field)
    /// for sorting by another field.
    /// - [An array specifying the field values in preferred
    /// order](https://vega.github.io/vega-lite/docs/sort.html#sort-array). In this case, the
    /// sort order will obey the values in the array, followed by any unspecified values in their
    /// original order. For discrete time field, values in the sort array can be [date-time
    /// definition objects](struct.DateTime.html). In addition, for time units `"month"` and `"day"`,
    /// the values can be the month or day names (case insensitive) or their 3-letter initials
    /// (e.g., `"Mon"`, `"Tue"`).
    /// - `null` indicating no sort.
    ///
    /// __Default value:__ `"ascending"`
    ///
    /// __Note:__ `null` is not supported for `row` and `column`.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub sort: RemovableValue<SortArray>,
    /// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
    /// or [a temporal field that gets casted as
    /// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
    ///
    /// __Default value:__ `undefined` (None)
    ///
    /// __See also:__ [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html)
    /// documentation.
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit: Option<TimeUnit>,
    /// A title for the field. If `null`, the title will be removed.
    ///
    /// __Default value:__  derived from the field's name and transformation function
    /// (`aggregate`, `bin` and `timeUnit`). If the field has an aggregate function, the function
    /// is displayed as part of the title (e.g., `"Sum of Profit"`). If the field is binned or
    /// has a time unit applied, the applied function is shown in parentheses (e.g., `"Profit
    /// (binned)"`, `"Transaction Date (year-month)"`). Otherwise, the title is simply the field
    /// name.
    ///
    /// __Notes__:
    ///
    /// 1) You can customize the default field title format by providing the
    /// [`fieldTitle`](https://vega.github.io/vega-lite/docs/config.html#top-level-config)
    /// property in the [config](https://vega.github.io/vega-lite/docs/config.html) or
    /// [`fieldTitle` function via the `compile` function's
    /// options](https://vega.github.io/vega-lite/docs/compile.html#field-title).
    ///
    /// 2) If both field definition's `title` and axis, header, or legend `title` are defined,
    /// axis/header/legend title will be used.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub title: RemovableValue<PurpleText>,
    /// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
    /// `"nominal"`).
    /// It can also be a `"geojson"` type for encoding
    /// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
    ///
    ///
    /// __Note:__
    ///
    /// - Data values for a temporal field can be either a date-time string (e.g., `"2015-03-07
    /// 12:32:17"`, `"17:01"`, `"2015-03-16"`. `"2015"`) or a timestamp number (e.g.,
    /// `1552199579097`).
    /// - Data `type` describes the semantics of the data rather than the primitive data types
    /// (number, string, etc.). The same primitive data type can have different types of
    /// measurement. For example, numeric data can represent quantitative, ordinal, or nominal
    /// data.
    /// - When using with [`bin`](https://vega.github.io/vega-lite/docs/bin.html), the `type`
    /// property can be either `"quantitative"` (for using a linear bin scale) or [`"ordinal"`
    /// (for using an ordinal bin
    /// scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html), the
    /// `type` property can be either `"temporal"` (for using a temporal scale) or [`"ordinal"`
    /// (for using an ordinal scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
    /// - When using with [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html),
    /// the `type` property refers to the post-aggregation data type. For example, we can
    /// calculate count `distinct` of a categorical field `"cat"` using `{"aggregate":
    /// "distinct", "field": "cat", "type": "quantitative"}`. The `"type"` of the aggregate
    /// output is `"quantitative"`.
    /// - Secondary channels (e.g., `x2`, `y2`, `xError`, `yError`) do not have `type` as they
    /// have exactly the same type as their primary channels (e.g., `x`, `y`).
    ///
    /// __See also:__ [`type`](https://vega.github.io/vega-lite/docs/type.html) documentation.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub facet_field_def_type: Option<StandardType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct Step {
    /// The size (width/height) per discrete step.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub step: Option<f64>,
}

/// A full layered plot specification, which may contains `encoding` and `projection`
/// properties that will be applied to underlying unit (single-view) specifications.
///
/// A unit specification, which can contain either [primitive marks or composite
/// marks](https://vega.github.io/vega-lite/docs/mark.html#types).
///
/// Base interface for a unit (single-view) specification.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct LayerSpec {
    /// An object describing the data source. Set to `null` to ignore the parent's data source.
    /// If no data is set, it is derived from the parent.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub data: RemovableValue<UrlData>,
    /// Description of this mark for commenting purpose.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub description: Option<String>,
    /// A shared key-value mapping between encoding channels and definition of fields in the
    /// underlying layers.
    ///
    /// A key-value mapping between encoding channels and definition of fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub encoding: Option<LayerEncoding>,
    /// The height of a visualization.
    ///
    /// - For a plot with a continuous y-field, height should be a number.
    /// - For a plot with either a discrete y-field or no y-field, height can be either a number
    /// indicating a fixed height or an object in the form of `{step: number}` defining the
    /// height per discrete step. (No y-field is equivalent to having one discrete step.)
    /// - To enable responsive sizing on height, it should be set to `"container"`.
    ///
    /// __Default value:__ Based on `config.view.continuousHeight` for a plot with a continuous
    /// y-field and `config.view.discreteHeight` otherwise.
    ///
    /// __Note:__ For plots with [`row` and `column`
    /// channels](https://vega.github.io/vega-lite/docs/encoding.html#facet), this represents the
    /// height of a single view and the `"container"` option cannot be used.
    ///
    /// __See also:__ [`height`](https://vega.github.io/vega-lite/docs/size.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub height: Option<HeightUnion>,
    /// Layer or single view specifications to be layered.
    ///
    /// __Note__: Specifications inside `layer` cannot use `row` and `column` channels as
    /// layering facet specifications is not allowed. Instead, use the [facet
    /// operator](https://vega.github.io/vega-lite/docs/facet.html) and place a layer inside a
    /// facet.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub layer: Option<Vec<LayerSpec>>,
    /// Name of the visualization for later reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub name: Option<String>,
    /// An object defining properties of the geographic projection shared by underlying layers.
    ///
    /// An object defining properties of geographic projection, which will be applied to `shape`
    /// path for `"geoshape"` marks
    /// and to `latitude` and `"longitude"` channels for other marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub projection: Option<Projection>,
    /// Scale, axis, and legend resolutions for view composition specifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub resolve: Option<Resolve>,
    /// Title for the plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title: Option<Text>,
    /// An array of data transformations such as filter and new field calculation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub transform: Option<Vec<Transform>>,
    /// An object defining the view background's fill and stroke.
    ///
    /// __Default value:__ none (transparent)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub view: Option<ViewBackground>,
    /// The width of a visualization.
    ///
    /// - For a plot with a continuous x-field, width should be a number.
    /// - For a plot with either a discrete x-field or no x-field, width can be either a number
    /// indicating a fixed width or an object in the form of `{step: number}` defining the width
    /// per discrete step. (No x-field is equivalent to having one discrete step.)
    /// - To enable responsive sizing on width, it should be set to `"container"`.
    ///
    /// __Default value:__
    /// Based on `config.view.continuousWidth` for a plot with a continuous x-field and
    /// `config.view.discreteWidth` otherwise.
    ///
    /// __Note:__ For plots with [`row` and `column`
    /// channels](https://vega.github.io/vega-lite/docs/encoding.html#facet), this represents the
    /// width of a single view and the `"container"` option cannot be used.
    ///
    /// __See also:__ [`width`](https://vega.github.io/vega-lite/docs/size.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub width: Option<HeightUnion>,
    /// A string describing the mark type (one of `"bar"`, `"circle"`, `"square"`, `"tick"`,
    /// `"line"`,
    /// `"area"`, `"point"`, `"rule"`, `"geoshape"`, and `"text"`) or a [mark definition
    /// object](https://vega.github.io/vega-lite/docs/mark.html#mark-def).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub mark: Option<AnyMark>,
    /// A key-value mapping between selection names and definitions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub selection: Option<HashMap<String, SelectionDef>>,
}

/// A shared key-value mapping between encoding channels and definition of fields in the
/// underlying layers.
///
/// A key-value mapping between encoding channels and definition of fields.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct LayerEncoding {
    /// Color of the marks – either fill or stroke color based on  the `filled` property of mark
    /// definition.
    /// By default, `color` represents fill color for `"area"`, `"bar"`, `"tick"`,
    /// `"text"`, `"trail"`, `"circle"`, and `"square"` / stroke color for `"line"` and
    /// `"point"`.
    ///
    /// __Default value:__ If undefined, the default color depends on [mark
    /// config](https://vega.github.io/vega-lite/docs/config.html#mark)'s `color` property.
    ///
    /// _Note:_
    /// 1) For fine-grained control over both fill and stroke colors of the marks, please use the
    /// `fill` and `stroke` channels. The `fill` or `stroke` encodings have higher precedence
    /// than `color`, thus may override the `color` encoding if conflicting encodings are
    /// specified.
    /// 2) See the scale documentation for more information about customizing [color
    /// scheme](https://vega.github.io/vega-lite/docs/scale.html#scheme).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub color: Option<DefWithConditionMarkPropFieldDefGradientStringNull>,
    /// Additional levels of detail for grouping data in aggregate views and
    /// in line, trail, and area marks without mapping data to a specific visual channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub detail: Option<Detail>,
    /// Fill color of the marks.
    /// __Default value:__ If undefined, the default color depends on [mark
    /// config](https://vega.github.io/vega-lite/docs/config.html#mark)'s `color` property.
    ///
    /// _Note:_ The `fill` encoding has higher precedence than `color`, thus may override the
    /// `color` encoding if conflicting encodings are specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fill: Option<DefWithConditionMarkPropFieldDefGradientStringNull>,
    /// Fill opacity of the marks.
    ///
    /// __Default value:__ If undefined, the default opacity depends on [mark
    /// config](https://vega.github.io/vega-lite/docs/config.html#mark)'s `fillOpacity` property.
    #[serde(rename = "fillOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fill_opacity: Option<DefWithConditionMarkPropFieldDefNumber>,
    /// A URL to load upon mouse click.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub href: Option<HrefClass>,
    /// A data field to use as a unique key for data binding. When a visualization’s data is
    /// updated, the key value will be used to match data elements to existing mark instances.
    /// Use a key channel to enable object constancy for transitions over dynamic data.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub key: Option<TypedFieldDef>,
    /// Latitude position of geographically projected marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub latitude: Option<LatitudeClass>,
    /// Latitude-2 position for geographically projected ranged `"area"`, `"bar"`, `"rect"`, and
    /// `"rule"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub latitude2: Option<Latitude2Class>,
    /// Longitude position of geographically projected marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub longitude: Option<LatitudeClass>,
    /// Longitude-2 position for geographically projected ranged `"area"`, `"bar"`, `"rect"`,
    /// and  `"rule"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub longitude2: Option<Latitude2Class>,
    /// Opacity of the marks.
    ///
    /// __Default value:__ If undefined, the default opacity depends on [mark
    /// config](https://vega.github.io/vega-lite/docs/config.html#mark)'s `opacity` property.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub opacity: Option<DefWithConditionMarkPropFieldDefNumber>,
    /// Order of the marks.
    /// - For stacked marks, this `order` channel encodes [stack
    /// order](https://vega.github.io/vega-lite/docs/stack.html#order).
    /// - For line and trail marks, this `order` channel encodes order of data points in the
    /// lines. This can be useful for creating [a connected
    /// scatterplot](https://vega.github.io/vega-lite/examples/connected_scatterplot.html).
    /// Setting `order` to `{"value": null}` makes the line marks use the original order in the
    /// data sources.
    /// - Otherwise, this `order` channel encodes layer order of the marks.
    ///
    /// __Note__: In aggregate plots, `order` field should be `aggregate`d to avoid creating
    /// additional aggregation grouping.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub order: Option<Order>,
    /// Shape of the mark.
    ///
    /// 1. For `point` marks the supported values include:
    /// - plotting shapes: `"circle"`, `"square"`, `"cross"`, `"diamond"`, `"triangle-up"`,
    /// `"triangle-down"`, `"triangle-right"`, or `"triangle-left"`.
    /// - the line symbol `"stroke"`
    /// - centered directional shapes `"arrow"`, `"wedge"`, or `"triangle"`
    /// - a custom [SVG path
    /// string](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths) (For correct
    /// sizing, custom shape paths should be defined within a square bounding box with
    /// coordinates ranging from -1 to 1 along both the x and y dimensions.)
    ///
    /// 2. For `geoshape` marks it should be a field definition of the geojson data
    ///
    /// __Default value:__ If undefined, the default shape depends on [mark
    /// config](https://vega.github.io/vega-lite/docs/config.html#point-config)'s `shape`
    /// property. (`"circle"` if unset.)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub shape: Option<DefWithConditionMarkPropFieldDefTypeForShapeStringNull>,
    /// Size of the mark.
    /// - For `"point"`, `"square"` and `"circle"`, – the symbol size, or pixel area of the mark.
    /// - For `"bar"` and `"tick"` – the bar and tick's size.
    /// - For `"text"` – the text's font size.
    /// - Size is unsupported for `"line"`, `"area"`, and `"rect"`. (Use `"trail"` instead of
    /// line with varying size)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub size: Option<DefWithConditionMarkPropFieldDefNumber>,
    /// Stroke color of the marks.
    /// __Default value:__ If undefined, the default color depends on [mark
    /// config](https://vega.github.io/vega-lite/docs/config.html#mark)'s `color` property.
    ///
    /// _Note:_ The `stroke` encoding has higher precedence than `color`, thus may override the
    /// `color` encoding if conflicting encodings are specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke: Option<DefWithConditionMarkPropFieldDefGradientStringNull>,
    /// Stroke opacity of the marks.
    ///
    /// __Default value:__ If undefined, the default opacity depends on [mark
    /// config](https://vega.github.io/vega-lite/docs/config.html#mark)'s `strokeOpacity`
    /// property.
    #[serde(rename = "strokeOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_opacity: Option<DefWithConditionMarkPropFieldDefNumber>,
    /// Stroke width of the marks.
    ///
    /// __Default value:__ If undefined, the default stroke width depends on [mark
    /// config](https://vega.github.io/vega-lite/docs/config.html#mark)'s `strokeWidth` property.
    #[serde(rename = "strokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_width: Option<DefWithConditionMarkPropFieldDefNumber>,
    /// Text of the `text` mark.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub text: Option<DefWithConditionStringFieldDefText>,
    /// The tooltip text to show upon mouse hover. Specifying `tooltip` encoding overrides [the
    /// `tooltip` property in the mark
    /// definition](https://vega.github.io/vega-lite/docs/mark.html#mark-def).
    ///
    /// See the [`tooltip`](https://vega.github.io/vega-lite/docs/tooltip.html) documentation for
    /// a detailed discussion about tooltip in Vega-Lite.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub tooltip: RemovableValue<Tooltip>,
    /// The URL of an image mark.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub url: Option<HrefClass>,
    /// X coordinates of the marks, or width of horizontal `"bar"` and `"area"` without specified
    /// `x2` or `width`.
    ///
    /// The `value` of this channel can be a number or a string `"width"` for the width of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub x: Option<XClass>,
    /// X2 coordinates for ranged `"area"`, `"bar"`, `"rect"`, and  `"rule"`.
    ///
    /// The `value` of this channel can be a number or a string `"width"` for the width of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub x2: Option<X2Class>,
    /// Error value of x coordinates for error specified `"errorbar"` and `"errorband"`.
    #[serde(rename = "xError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub x_error: Option<Latitude2Class>,
    /// Secondary error value of x coordinates for error specified `"errorbar"` and `"errorband"`.
    #[serde(rename = "xError2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub x_error2: Option<Latitude2Class>,
    /// Y coordinates of the marks, or height of vertical `"bar"` and `"area"` without specified
    /// `y2` or `height`.
    ///
    /// The `value` of this channel can be a number or a string `"height"` for the height of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub y: Option<YClass>,
    /// Y2 coordinates for ranged `"area"`, `"bar"`, `"rect"`, and  `"rule"`.
    ///
    /// The `value` of this channel can be a number or a string `"height"` for the height of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub y2: Option<Y2Class>,
    /// Error value of y coordinates for error specified `"errorbar"` and `"errorband"`.
    #[serde(rename = "yError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub y_error: Option<Latitude2Class>,
    /// Secondary error value of y coordinates for error specified `"errorbar"` and `"errorband"`.
    #[serde(rename = "yError2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub y_error2: Option<Latitude2Class>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct MarkDefClass {
    #[serde(rename = "box")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub def_box: Option<DefBox>,
    /// Whether a composite mark be clipped to the enclosing group’s width and height.
    ///
    /// Whether a mark be clipped to the enclosing group’s width and height.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub clip: Option<bool>,
    /// Default color.
    ///
    /// __Default value:__ <span style="color: #4682b4;">&#9632;</span> `"#4682b4"`
    ///
    /// __Note:__
    /// - This property cannot be used in a [style
    /// config](https://vega.github.io/vega-lite/docs/mark.html#style-config).
    /// - The `fill` and `stroke` properties have higher precedence than `color` and will
    /// override `color`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub color: Option<ColorUnion>,
    /// The extent of the whiskers. Available options include:
    /// - `"min-max"`: min and max are the lower and upper whiskers respectively.
    /// - A number representing multiple of the interquartile range. This number will be
    /// multiplied by the IQR to determine whisker boundary, which spans from the smallest data
    /// to the largest data within the range _[Q1 - k * IQR, Q3 + k * IQR]_ where _Q1_ and _Q3_
    /// are the first and third quartiles while _IQR_ is the interquartile range (_Q3-Q1_).
    ///
    /// __Default value:__ `1.5`.
    ///
    /// The extent of the rule. Available options include:
    /// - `"ci"`: Extend the rule to the confidence interval of the mean.
    /// - `"stderr"`: The size of rule are set to the value of standard error, extending from the
    /// mean.
    /// - `"stdev"`: The size of rule are set to the value of standard deviation, extending from
    /// the mean.
    /// - `"iqr"`: Extend the rule to the q1 and q3.
    ///
    /// __Default value:__ `"stderr"`.
    ///
    /// The extent of the band. Available options include:
    /// - `"ci"`: Extend the band to the confidence interval of the mean.
    /// - `"stderr"`: The size of band are set to the value of standard error, extending from the
    /// mean.
    /// - `"stdev"`: The size of band are set to the value of standard deviation, extending from
    /// the mean.
    /// - `"iqr"`: Extend the band to the q1 and q3.
    ///
    /// __Default value:__ `"stderr"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub extent: Option<MarkDefExtent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub median: Option<DefBox>,
    /// The opacity (value between [0,1]) of the mark.
    ///
    /// The overall opacity (value between [0,1]).
    ///
    /// __Default value:__ `0.7` for non-aggregate plots with `point`, `tick`, `circle`, or
    /// `square` marks or layered `bar` charts and `1` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub opacity: Option<f64>,
    /// Orientation of the box plot. This is normally automatically determined based on types of
    /// fields on x and y channels. However, an explicit `orient` be specified when the
    /// orientation is ambiguous.
    ///
    /// __Default value:__ `"vertical"`.
    ///
    /// Orientation of the error bar. This is normally automatically determined, but can be
    /// specified when the orientation is ambiguous and cannot be automatically determined.
    ///
    /// Orientation of the error band. This is normally automatically determined, but can be
    /// specified when the orientation is ambiguous and cannot be automatically determined.
    ///
    /// The orientation of a non-stacked bar, tick, area, and line charts.
    /// The value is either horizontal (default) or vertical.
    /// - For bar, rule and tick, this determines whether the size of the bar and tick
    /// should be applied to x or y dimension.
    /// - For area, this property determines the orient property of the Vega output.
    /// - For line and trail marks, this property determines the sort order of the points in the
    /// line
    /// if `config.sortLineBy` is not specified.
    /// For stacked charts, this is always determined by the orientation of the stack;
    /// therefore explicitly specified value will be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub orient: Option<Orientation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub outliers: Option<DefBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub rule: Option<DefBox>,
    /// Size of the box and median tick of a box plot
    ///
    /// Default size for marks.
    /// - For `point`/`circle`/`square`, this represents the pixel area of the marks. For
    /// example: in the case of circles, the radius is determined in part by the square root of
    /// the size value.
    /// - For `bar`, this represents the band size of the bar, in pixels.
    /// - For `text`, this represents the font size, in pixels.
    ///
    /// __Default value:__
    /// - `30` for point, circle, square marks; width/height's `step`
    /// - `2` for bar marks with discrete dimensions;
    /// - `5` for bar marks with continuous dimensions;
    /// - `11` for text marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ticks: Option<DefBox>,
    /// The mark type. This could a primitive mark type
    /// (one of `"bar"`, `"circle"`, `"square"`, `"tick"`, `"line"`,
    /// `"area"`, `"point"`, `"geoshape"`, `"rule"`, and `"text"`)
    /// or a composite mark type (`"boxplot"`, `"errorband"`, `"errorbar"`).
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub def_type: Option<Mark>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub band: Option<DefBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub borders: Option<DefBox>,
    /// The line interpolation method for the error band. One of the following:
    /// - `"linear"`: piecewise linear segments, as in a polyline.
    /// - `"linear-closed"`: close the linear segments to form a polygon.
    /// - `"step"`: a piecewise constant function (a step function) consisting of alternating
    /// horizontal and vertical lines. The y-value changes at the midpoint of each pair of
    /// adjacent x-values.
    /// - `"step-before"`: a piecewise constant function (a step function) consisting of
    /// alternating horizontal and vertical lines. The y-value changes before the x-value.
    /// - `"step-after"`: a piecewise constant function (a step function) consisting of
    /// alternating horizontal and vertical lines. The y-value changes after the x-value.
    /// - `"basis"`: a B-spline, with control point duplication on the ends.
    /// - `"basis-open"`: an open B-spline; may not intersect the start or end.
    /// - `"basis-closed"`: a closed B-spline, as in a loop.
    /// - `"cardinal"`: a Cardinal spline, with control point duplication on the ends.
    /// - `"cardinal-open"`: an open Cardinal spline; may not intersect the start or end, but
    /// will intersect other control points.
    /// - `"cardinal-closed"`: a closed Cardinal spline, as in a loop.
    /// - `"bundle"`: equivalent to basis, except the tension parameter is used to straighten the
    /// spline.
    /// - `"monotone"`: cubic interpolation that preserves monotonicity in y.
    ///
    /// The line interpolation method to use for line and area marks. One of the following:
    /// - `"linear"`: piecewise linear segments, as in a polyline.
    /// - `"linear-closed"`: close the linear segments to form a polygon.
    /// - `"step"`: alternate between horizontal and vertical segments, as in a step function.
    /// - `"step-before"`: alternate between vertical and horizontal segments, as in a step
    /// function.
    /// - `"step-after"`: alternate between horizontal and vertical segments, as in a step
    /// function.
    /// - `"basis"`: a B-spline, with control point duplication on the ends.
    /// - `"basis-open"`: an open B-spline; may not intersect the start or end.
    /// - `"basis-closed"`: a closed B-spline, as in a loop.
    /// - `"cardinal"`: a Cardinal spline, with control point duplication on the ends.
    /// - `"cardinal-open"`: an open Cardinal spline; may not intersect the start or end, but
    /// will intersect other control points.
    /// - `"cardinal-closed"`: a closed Cardinal spline, as in a loop.
    /// - `"bundle"`: equivalent to basis, except the tension parameter is used to straighten the
    /// spline.
    /// - `"monotone"`: cubic interpolation that preserves monotonicity in y.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub interpolate: Option<Interpolate>,
    /// The tension parameter for the interpolation type of the error band.
    ///
    /// Depending on the interpolation type, sets the tension parameter (for line and area marks).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tension: Option<f64>,
    /// The horizontal alignment of the text or ranged marks (area, bar, image, rect, rule). One
    /// of `"left"`, `"right"`, `"center"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub align: Option<Align>,
    /// The rotation angle of the text, in degrees.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub angle: Option<f64>,
    /// Whether to keep aspect ratio of image marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aspect: Option<bool>,
    /// The vertical alignment of the text or ranged marks (area, bar, image, rect, rule). One of
    /// `"top"`, `"middle"`, `"bottom"`.
    ///
    /// __Default value:__ `"middle"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub baseline: Option<Baseline>,
    /// Offset between bars for binned field. The ideal value for this is either 0 (preferred by
    /// statisticians) or 1 (Vega-Lite default, D3 example style).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "binSpacing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bin_spacing: Option<f64>,
    /// The radius in pixels of rounded rectangle corners.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadius")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius: Option<f64>,
    /// The radius in pixels of rounded rectangle bottom left corner.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadiusBottomLeft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius_bottom_left: Option<f64>,
    /// The radius in pixels of rounded rectangle bottom right corner.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadiusBottomRight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius_bottom_right: Option<f64>,
    /// The radius in pixels of rounded rectangle top right corner.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadiusTopLeft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius_top_left: Option<f64>,
    /// The radius in pixels of rounded rectangle top left corner.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadiusTopRight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius_top_right: Option<f64>,
    /// The mouse cursor used over the mark. Any valid [CSS cursor
    /// type](https://developer.mozilla.org/en-US/docs/Web/CSS/cursor#Values) can be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub cursor: Option<Cursor>,
    /// The direction of the text. One of `"ltr"` (left-to-right) or `"rtl"` (right-to-left).
    /// This property determines on which side is truncated in response to the limit parameter.
    ///
    /// __Default value:__ `"ltr"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dir: Option<Dir>,
    /// The horizontal offset, in pixels, between the text label and its anchor point. The offset
    /// is applied after rotation by the _angle_ property.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dx: Option<f64>,
    /// The vertical offset, in pixels, between the text label and its anchor point. The offset
    /// is applied after rotation by the _angle_ property.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dy: Option<f64>,
    /// The ellipsis string for text truncated in response to the limit parameter.
    ///
    /// __Default value:__ `"…"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ellipsis: Option<String>,
    /// Default Fill Color. This has higher precedence than `config.color`.
    ///
    /// __Default value:__ (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fill: Option<FillUnion>,
    /// Whether the mark's color should be used as fill color instead of stroke color.
    ///
    /// __Default value:__ `false` for all `point`, `line`, and `rule` marks as well as
    /// `geoshape` marks for
    /// [`graticule`](https://vega.github.io/vega-lite/docs/data.html#graticule) data sources;
    /// otherwise, `true`.
    ///
    /// __Note:__ This property cannot be used in a [style
    /// config](https://vega.github.io/vega-lite/docs/mark.html#style-config).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub filled: Option<bool>,
    /// The fill opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "fillOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fill_opacity: Option<f64>,
    /// The typeface to set the text in (e.g., `"Helvetica Neue"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font: Option<String>,
    /// The font size, in pixels.
    #[serde(rename = "fontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font_size: Option<f64>,
    /// The font style (e.g., `"italic"`).
    #[serde(rename = "fontStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font_style: Option<String>,
    /// The font weight.
    /// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
    /// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
    #[serde(rename = "fontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font_weight: Option<FontWeight>,
    /// Height of the marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub height: Option<f64>,
    /// A URL to load upon mouse click. If defined, the mark acts as a hyperlink.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub href: Option<String>,
    /// Defines how Vega-Lite should handle marks for invalid values (`null` and `NaN`).
    /// - If set to `"filter"` (default), all data items with null values will be skipped (for
    /// line, trail, and area marks) or filtered (for other marks).
    /// - If `null`, all data items are included. In this case, invalid values will be
    /// interpreted as zeroes.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub invalid: RemovableValue<Invalid>,
    /// The maximum length of the text mark in pixels. The text value will be automatically
    /// truncated if the rendered size exceeds the limit.
    ///
    /// __Default value:__ `0`, indicating no limit
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub limit: Option<f64>,
    /// A flag for overlaying line on top of area marks, or an object defining the properties of
    /// the overlayed lines.
    ///
    /// - If this value is an empty object (`{}`) or `true`, lines with default properties will
    /// be used.
    ///
    /// - If this value is `false`, no lines would be automatically added to area marks.
    ///
    /// __Default value:__ `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub line: Option<Line>,
    /// A delimiter, such as a newline character, upon which to break text strings into multiple
    /// lines. This property will be ignored if the text property is array-valued.
    #[serde(rename = "lineBreak")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub line_break: Option<String>,
    /// The height, in pixels, of each line of text in a multi-line text mark.
    #[serde(rename = "lineHeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub line_height: Option<f64>,
    /// For line and trail marks, this `order` property can be set to `null` or `false` to make
    /// the lines use the original order in the data sources.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub order: Option<bool>,
    /// A flag for overlaying points on top of line or area marks, or an object defining the
    /// properties of the overlayed points.
    ///
    /// - If this property is `"transparent"`, transparent points will be used (for enhancing
    /// tooltips and selections).
    ///
    /// - If this property is an empty object (`{}`) or `true`, filled points with default
    /// properties will be used.
    ///
    /// - If this property is `false`, no points would be automatically added to line or area
    /// marks.
    ///
    /// __Default value:__ `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub point: Option<PointUnion>,
    /// Polar coordinate radial offset, in pixels, of the text label from the origin determined
    /// by the `x` and `y` properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub radius: Option<f64>,
    /// Shape of the point marks. Supported values include:
    /// - plotting shapes: `"circle"`, `"square"`, `"cross"`, `"diamond"`, `"triangle-up"`,
    /// `"triangle-down"`, `"triangle-right"`, or `"triangle-left"`.
    /// - the line symbol `"stroke"`
    /// - centered directional shapes `"arrow"`, `"wedge"`, or `"triangle"`
    /// - a custom [SVG path
    /// string](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths) (For correct
    /// sizing, custom shape paths should be defined within a square bounding box with
    /// coordinates ranging from -1 to 1 along both the x and y dimensions.)
    ///
    /// __Default value:__ `"circle"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub shape: Option<String>,
    /// Default Stroke Color. This has higher precedence than `config.color`.
    ///
    /// __Default value:__ (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke: Option<FillUnion>,
    /// The stroke cap for line ending style. One of `"butt"`, `"round"`, or `"square"`.
    ///
    /// __Default value:__ `"square"`
    #[serde(rename = "strokeCap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_cap: Option<StrokeCap>,
    /// An array of alternating stroke, space lengths for creating dashed or dotted lines.
    #[serde(rename = "strokeDash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_dash: Option<Vec<f64>>,
    /// The offset (in pixels) into which to begin drawing with the stroke dash array.
    #[serde(rename = "strokeDashOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_dash_offset: Option<f64>,
    /// The stroke line join method. One of `"miter"`, `"round"` or `"bevel"`.
    ///
    /// __Default value:__ `"miter"`
    #[serde(rename = "strokeJoin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_join: Option<StrokeJoin>,
    /// The miter limit at which to bevel a line join.
    #[serde(rename = "strokeMiterLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_miter_limit: Option<f64>,
    /// The stroke opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "strokeOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_opacity: Option<f64>,
    /// The stroke width, in pixels.
    #[serde(rename = "strokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_width: Option<f64>,
    /// A string or array of strings indicating the name of custom styles to apply to the mark. A
    /// style is a named collection of mark property defaults defined within the [style
    /// configuration](https://vega.github.io/vega-lite/docs/mark.html#style-config). If style is
    /// an array, later styles will override earlier styles. Any [mark
    /// properties](https://vega.github.io/vega-lite/docs/encoding.html#mark-prop) explicitly
    /// defined within the `encoding` will override a style default.
    ///
    /// __Default value:__ The mark's name. For example, a bar mark will have style `"bar"` by
    /// default.
    /// __Note:__ Any specified style will augment the default style. For example, a bar mark
    /// with `"style": "foo"` will receive from `config.style.bar` and `config.style.foo` (the
    /// specified style `"foo"` has higher precedence).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub style: Option<PurpleText>,
    /// Placeholder text if the `text` channel is not specified
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub text: Option<PurpleText>,
    /// Polar coordinate angle, in radians, of the text label from the origin determined by the
    /// `x` and `y` properties. Values for `theta` follow the same convention of `arc` mark
    /// `startAngle` and `endAngle` properties: angles are measured in radians, with `0`
    /// indicating "north".
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub theta: Option<f64>,
    /// Thickness of the tick mark.
    ///
    /// __Default value:__  `1`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub thickness: Option<f64>,
    /// Default relative band size for a time unit. If set to `1`, the bandwidth of the marks
    /// will be equal to the time unit band step.
    /// If set to `0.5`, bandwidth of the marks will be half of the time unit band step.
    #[serde(rename = "timeUnitBand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit_band: Option<f64>,
    /// Default relative band position for a time unit. If set to `0`, the marks will be
    /// positioned at the beginning of the time unit band step.
    /// If set to `0.5`, the marks will be positioned in the middle of the time unit band step.
    #[serde(rename = "timeUnitBandPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit_band_position: Option<f64>,
    /// The tooltip text string to show upon mouse hover or an object defining which fields
    /// should the tooltip be derived from.
    ///
    /// - If `tooltip` is `true` or `{"content": "encoding"}`, then all fields from `encoding`
    /// will be used.
    /// - If `tooltip` is `{"content": "data"}`, then all fields that appear in the highlighted
    /// data point will be used.
    /// - If set to `null` or `false`, then no tooltip will be used.
    ///
    /// See the [`tooltip`](https://vega.github.io/vega-lite/docs/tooltip.html) documentation for
    /// a detailed discussion about tooltip  in Vega-Lite.
    ///
    /// __Default value:__ `null`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tooltip: Option<Value>,
    /// Width of the marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub width: Option<f64>,
    /// X coordinates of the marks, or width of horizontal `"bar"` and `"area"` without specified
    /// `x2` or `width`.
    ///
    /// The `value` of this channel can be a number or a string `"width"` for the width of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub x: Option<XUnion>,
    /// X2 coordinates for ranged `"area"`, `"bar"`, `"rect"`, and  `"rule"`.
    ///
    /// The `value` of this channel can be a number or a string `"width"` for the width of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub x2: Option<XUnion>,
    /// Offset for x2-position.
    #[serde(rename = "x2Offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub x2_offset: Option<f64>,
    /// Offset for x-position.
    #[serde(rename = "xOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub x_offset: Option<f64>,
    /// Y coordinates of the marks, or height of vertical `"bar"` and `"area"` without specified
    /// `y2` or `height`.
    ///
    /// The `value` of this channel can be a number or a string `"height"` for the height of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub y: Option<YUnion>,
    /// Y2 coordinates for ranged `"area"`, `"bar"`, `"rect"`, and  `"rule"`.
    ///
    /// The `value` of this channel can be a number or a string `"height"` for the height of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub y2: Option<YUnion>,
    /// Offset for y2-position.
    #[serde(rename = "y2Offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub y2_offset: Option<f64>,
    /// Offset for y-position.
    #[serde(rename = "yOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub y_offset: Option<f64>,
}

/// Circle-Specific Config
///
/// Geoshape-Specific Config
///
/// Mark Config
///
/// Point-Specific Config
///
/// Rule-Specific Config
///
/// Square-Specific Config
///
/// Text-Specific Config
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct MarkConfig {
    /// The horizontal alignment of the text or ranged marks (area, bar, image, rect, rule). One
    /// of `"left"`, `"right"`, `"center"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub align: Option<Align>,
    /// The rotation angle of the text, in degrees.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub angle: Option<f64>,
    /// Whether to keep aspect ratio of image marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aspect: Option<bool>,
    /// The vertical alignment of the text or ranged marks (area, bar, image, rect, rule). One of
    /// `"top"`, `"middle"`, `"bottom"`.
    ///
    /// __Default value:__ `"middle"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub baseline: Option<Baseline>,
    /// Default color.
    ///
    /// __Default value:__ <span style="color: #4682b4;">&#9632;</span> `"#4682b4"`
    ///
    /// __Note:__
    /// - This property cannot be used in a [style
    /// config](https://vega.github.io/vega-lite/docs/mark.html#style-config).
    /// - The `fill` and `stroke` properties have higher precedence than `color` and will
    /// override `color`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub color: Option<ColorUnion>,
    /// The radius in pixels of rounded rectangle corners.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadius")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius: Option<f64>,
    /// The radius in pixels of rounded rectangle bottom left corner.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadiusBottomLeft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius_bottom_left: Option<f64>,
    /// The radius in pixels of rounded rectangle bottom right corner.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadiusBottomRight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius_bottom_right: Option<f64>,
    /// The radius in pixels of rounded rectangle top right corner.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadiusTopLeft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius_top_left: Option<f64>,
    /// The radius in pixels of rounded rectangle top left corner.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadiusTopRight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius_top_right: Option<f64>,
    /// The mouse cursor used over the mark. Any valid [CSS cursor
    /// type](https://developer.mozilla.org/en-US/docs/Web/CSS/cursor#Values) can be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub cursor: Option<Cursor>,
    /// The direction of the text. One of `"ltr"` (left-to-right) or `"rtl"` (right-to-left).
    /// This property determines on which side is truncated in response to the limit parameter.
    ///
    /// __Default value:__ `"ltr"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dir: Option<Dir>,
    /// The horizontal offset, in pixels, between the text label and its anchor point. The offset
    /// is applied after rotation by the _angle_ property.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dx: Option<f64>,
    /// The vertical offset, in pixels, between the text label and its anchor point. The offset
    /// is applied after rotation by the _angle_ property.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dy: Option<f64>,
    /// The ellipsis string for text truncated in response to the limit parameter.
    ///
    /// __Default value:__ `"…"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ellipsis: Option<String>,
    /// Default Fill Color. This has higher precedence than `config.color`.
    ///
    /// __Default value:__ (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fill: Option<FillUnion>,
    /// Whether the mark's color should be used as fill color instead of stroke color.
    ///
    /// __Default value:__ `false` for all `point`, `line`, and `rule` marks as well as
    /// `geoshape` marks for
    /// [`graticule`](https://vega.github.io/vega-lite/docs/data.html#graticule) data sources;
    /// otherwise, `true`.
    ///
    /// __Note:__ This property cannot be used in a [style
    /// config](https://vega.github.io/vega-lite/docs/mark.html#style-config).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub filled: Option<bool>,
    /// The fill opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "fillOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fill_opacity: Option<f64>,
    /// The typeface to set the text in (e.g., `"Helvetica Neue"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font: Option<String>,
    /// The font size, in pixels.
    #[serde(rename = "fontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font_size: Option<f64>,
    /// The font style (e.g., `"italic"`).
    #[serde(rename = "fontStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font_style: Option<String>,
    /// The font weight.
    /// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
    /// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
    #[serde(rename = "fontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font_weight: Option<FontWeight>,
    /// Height of the marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub height: Option<f64>,
    /// A URL to load upon mouse click. If defined, the mark acts as a hyperlink.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub href: Option<String>,
    /// The line interpolation method to use for line and area marks. One of the following:
    /// - `"linear"`: piecewise linear segments, as in a polyline.
    /// - `"linear-closed"`: close the linear segments to form a polygon.
    /// - `"step"`: alternate between horizontal and vertical segments, as in a step function.
    /// - `"step-before"`: alternate between vertical and horizontal segments, as in a step
    /// function.
    /// - `"step-after"`: alternate between horizontal and vertical segments, as in a step
    /// function.
    /// - `"basis"`: a B-spline, with control point duplication on the ends.
    /// - `"basis-open"`: an open B-spline; may not intersect the start or end.
    /// - `"basis-closed"`: a closed B-spline, as in a loop.
    /// - `"cardinal"`: a Cardinal spline, with control point duplication on the ends.
    /// - `"cardinal-open"`: an open Cardinal spline; may not intersect the start or end, but
    /// will intersect other control points.
    /// - `"cardinal-closed"`: a closed Cardinal spline, as in a loop.
    /// - `"bundle"`: equivalent to basis, except the tension parameter is used to straighten the
    /// spline.
    /// - `"monotone"`: cubic interpolation that preserves monotonicity in y.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub interpolate: Option<Interpolate>,
    /// Defines how Vega-Lite should handle marks for invalid values (`null` and `NaN`).
    /// - If set to `"filter"` (default), all data items with null values will be skipped (for
    /// line, trail, and area marks) or filtered (for other marks).
    /// - If `null`, all data items are included. In this case, invalid values will be
    /// interpreted as zeroes.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub invalid: RemovableValue<Invalid>,
    /// The maximum length of the text mark in pixels. The text value will be automatically
    /// truncated if the rendered size exceeds the limit.
    ///
    /// __Default value:__ `0`, indicating no limit
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub limit: Option<f64>,
    /// A delimiter, such as a newline character, upon which to break text strings into multiple
    /// lines. This property will be ignored if the text property is array-valued.
    #[serde(rename = "lineBreak")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub line_break: Option<String>,
    /// The height, in pixels, of each line of text in a multi-line text mark.
    #[serde(rename = "lineHeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub line_height: Option<f64>,
    /// The overall opacity (value between [0,1]).
    ///
    /// __Default value:__ `0.7` for non-aggregate plots with `point`, `tick`, `circle`, or
    /// `square` marks or layered `bar` charts and `1` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub opacity: Option<f64>,
    /// For line and trail marks, this `order` property can be set to `null` or `false` to make
    /// the lines use the original order in the data sources.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub order: Option<bool>,
    /// The orientation of a non-stacked bar, tick, area, and line charts.
    /// The value is either horizontal (default) or vertical.
    /// - For bar, rule and tick, this determines whether the size of the bar and tick
    /// should be applied to x or y dimension.
    /// - For area, this property determines the orient property of the Vega output.
    /// - For line and trail marks, this property determines the sort order of the points in the
    /// line
    /// if `config.sortLineBy` is not specified.
    /// For stacked charts, this is always determined by the orientation of the stack;
    /// therefore explicitly specified value will be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub orient: Option<Orientation>,
    /// Polar coordinate radial offset, in pixels, of the text label from the origin determined
    /// by the `x` and `y` properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub radius: Option<f64>,
    /// Shape of the point marks. Supported values include:
    /// - plotting shapes: `"circle"`, `"square"`, `"cross"`, `"diamond"`, `"triangle-up"`,
    /// `"triangle-down"`, `"triangle-right"`, or `"triangle-left"`.
    /// - the line symbol `"stroke"`
    /// - centered directional shapes `"arrow"`, `"wedge"`, or `"triangle"`
    /// - a custom [SVG path
    /// string](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths) (For correct
    /// sizing, custom shape paths should be defined within a square bounding box with
    /// coordinates ranging from -1 to 1 along both the x and y dimensions.)
    ///
    /// __Default value:__ `"circle"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub shape: Option<String>,
    /// Default size for marks.
    /// - For `point`/`circle`/`square`, this represents the pixel area of the marks. For
    /// example: in the case of circles, the radius is determined in part by the square root of
    /// the size value.
    /// - For `bar`, this represents the band size of the bar, in pixels.
    /// - For `text`, this represents the font size, in pixels.
    ///
    /// __Default value:__
    /// - `30` for point, circle, square marks; width/height's `step`
    /// - `2` for bar marks with discrete dimensions;
    /// - `5` for bar marks with continuous dimensions;
    /// - `11` for text marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub size: Option<f64>,
    /// Default Stroke Color. This has higher precedence than `config.color`.
    ///
    /// __Default value:__ (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke: Option<FillUnion>,
    /// The stroke cap for line ending style. One of `"butt"`, `"round"`, or `"square"`.
    ///
    /// __Default value:__ `"square"`
    #[serde(rename = "strokeCap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_cap: Option<StrokeCap>,
    /// An array of alternating stroke, space lengths for creating dashed or dotted lines.
    #[serde(rename = "strokeDash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_dash: Option<Vec<f64>>,
    /// The offset (in pixels) into which to begin drawing with the stroke dash array.
    #[serde(rename = "strokeDashOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_dash_offset: Option<f64>,
    /// The stroke line join method. One of `"miter"`, `"round"` or `"bevel"`.
    ///
    /// __Default value:__ `"miter"`
    #[serde(rename = "strokeJoin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_join: Option<StrokeJoin>,
    /// The miter limit at which to bevel a line join.
    #[serde(rename = "strokeMiterLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_miter_limit: Option<f64>,
    /// The stroke opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "strokeOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_opacity: Option<f64>,
    /// The stroke width, in pixels.
    #[serde(rename = "strokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_width: Option<f64>,
    /// Depending on the interpolation type, sets the tension parameter (for line and area marks).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tension: Option<f64>,
    /// Placeholder text if the `text` channel is not specified
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub text: Option<PurpleText>,
    /// Polar coordinate angle, in radians, of the text label from the origin determined by the
    /// `x` and `y` properties. Values for `theta` follow the same convention of `arc` mark
    /// `startAngle` and `endAngle` properties: angles are measured in radians, with `0`
    /// indicating "north".
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub theta: Option<f64>,
    /// Default relative band size for a time unit. If set to `1`, the bandwidth of the marks
    /// will be equal to the time unit band step.
    /// If set to `0.5`, bandwidth of the marks will be half of the time unit band step.
    #[serde(rename = "timeUnitBand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit_band: Option<f64>,
    /// Default relative band position for a time unit. If set to `0`, the marks will be
    /// positioned at the beginning of the time unit band step.
    /// If set to `0.5`, the marks will be positioned in the middle of the time unit band step.
    #[serde(rename = "timeUnitBandPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit_band_position: Option<f64>,
    /// The tooltip text string to show upon mouse hover or an object defining which fields
    /// should the tooltip be derived from.
    ///
    /// - If `tooltip` is `true` or `{"content": "encoding"}`, then all fields from `encoding`
    /// will be used.
    /// - If `tooltip` is `{"content": "data"}`, then all fields that appear in the highlighted
    /// data point will be used.
    /// - If set to `null` or `false`, then no tooltip will be used.
    ///
    /// See the [`tooltip`](https://vega.github.io/vega-lite/docs/tooltip.html) documentation for
    /// a detailed discussion about tooltip  in Vega-Lite.
    ///
    /// __Default value:__ `null`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tooltip: Option<Value>,
    /// Width of the marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub width: Option<f64>,
    /// X coordinates of the marks, or width of horizontal `"bar"` and `"area"` without specified
    /// `x2` or `width`.
    ///
    /// The `value` of this channel can be a number or a string `"width"` for the width of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub x: Option<XUnion>,
    /// X2 coordinates for ranged `"area"`, `"bar"`, `"rect"`, and  `"rule"`.
    ///
    /// The `value` of this channel can be a number or a string `"width"` for the width of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub x2: Option<XUnion>,
    /// Y coordinates of the marks, or height of vertical `"bar"` and `"area"` without specified
    /// `y2` or `height`.
    ///
    /// The `value` of this channel can be a number or a string `"height"` for the height of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub y: Option<YUnion>,
    /// Y2 coordinates for ranged `"area"`, `"bar"`, `"rect"`, and  `"rule"`.
    ///
    /// The `value` of this channel can be a number or a string `"height"` for the height of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub y2: Option<YUnion>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ColorLinearGradient {
    /// The type of gradient. Use `"linear"` for a linear gradient.
    ///
    /// The type of gradient. Use `"radial"` for a radial gradient.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub gradient: Option<Gradient>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub id: Option<String>,
    /// An array of gradient stops defining the gradient color sequence.
    pub stops: Vec<GradientStop>,
    /// The starting x-coordinate, in normalized [0, 1] coordinates, of the linear gradient.
    ///
    /// __Default value:__ `0`
    ///
    /// The x-coordinate, in normalized [0, 1] coordinates, for the center of the inner circle
    /// for the gradient.
    ///
    /// __Default value:__ `0.5`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub x1: Option<f64>,
    /// The ending x-coordinate, in normalized [0, 1] coordinates, of the linear gradient.
    ///
    /// __Default value:__ `1`
    ///
    /// The x-coordinate, in normalized [0, 1] coordinates, for the center of the outer circle
    /// for the gradient.
    ///
    /// __Default value:__ `0.5`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub x2: Option<f64>,
    /// The starting y-coordinate, in normalized [0, 1] coordinates, of the linear gradient.
    ///
    /// __Default value:__ `0`
    ///
    /// The y-coordinate, in normalized [0, 1] coordinates, for the center of the inner circle
    /// for the gradient.
    ///
    /// __Default value:__ `0.5`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub y1: Option<f64>,
    /// The ending y-coordinate, in normalized [0, 1] coordinates, of the linear gradient.
    ///
    /// __Default value:__ `0`
    ///
    /// The y-coordinate, in normalized [0, 1] coordinates, for the center of the outer circle
    /// for the gradient.
    ///
    /// __Default value:__ `0.5`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub y2: Option<f64>,
    /// The radius length, in normalized [0, 1] coordinates, of the inner circle for the
    /// gradient.
    ///
    /// __Default value:__ `0`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub r1: Option<f64>,
    /// The radius length, in normalized [0, 1] coordinates, of the outer circle for the
    /// gradient.
    ///
    /// __Default value:__ `0.5`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub r2: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct FillLinearGradient {
    /// The type of gradient. Use `"linear"` for a linear gradient.
    ///
    /// The type of gradient. Use `"radial"` for a radial gradient.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub gradient: Option<Gradient>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub id: Option<String>,
    /// An array of gradient stops defining the gradient color sequence.
    pub stops: Vec<GradientStop>,
    /// The starting x-coordinate, in normalized [0, 1] coordinates, of the linear gradient.
    ///
    /// __Default value:__ `0`
    ///
    /// The x-coordinate, in normalized [0, 1] coordinates, for the center of the inner circle
    /// for the gradient.
    ///
    /// __Default value:__ `0.5`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub x1: Option<f64>,
    /// The ending x-coordinate, in normalized [0, 1] coordinates, of the linear gradient.
    ///
    /// __Default value:__ `1`
    ///
    /// The x-coordinate, in normalized [0, 1] coordinates, for the center of the outer circle
    /// for the gradient.
    ///
    /// __Default value:__ `0.5`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub x2: Option<f64>,
    /// The starting y-coordinate, in normalized [0, 1] coordinates, of the linear gradient.
    ///
    /// __Default value:__ `0`
    ///
    /// The y-coordinate, in normalized [0, 1] coordinates, for the center of the inner circle
    /// for the gradient.
    ///
    /// __Default value:__ `0.5`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub y1: Option<f64>,
    /// The ending y-coordinate, in normalized [0, 1] coordinates, of the linear gradient.
    ///
    /// __Default value:__ `0`
    ///
    /// The y-coordinate, in normalized [0, 1] coordinates, for the center of the outer circle
    /// for the gradient.
    ///
    /// __Default value:__ `0.5`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub y2: Option<f64>,
    /// The radius length, in normalized [0, 1] coordinates, of the inner circle for the
    /// gradient.
    ///
    /// __Default value:__ `0`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub r1: Option<f64>,
    /// The radius length, in normalized [0, 1] coordinates, of the outer circle for the
    /// gradient.
    ///
    /// __Default value:__ `0.5`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub r2: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct TooltipContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub content: Option<Content>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct OverlayMarkDef {
    /// The horizontal alignment of the text or ranged marks (area, bar, image, rect, rule). One
    /// of `"left"`, `"right"`, `"center"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub align: Option<Align>,
    /// The rotation angle of the text, in degrees.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub angle: Option<f64>,
    /// Whether to keep aspect ratio of image marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aspect: Option<bool>,
    /// The vertical alignment of the text or ranged marks (area, bar, image, rect, rule). One of
    /// `"top"`, `"middle"`, `"bottom"`.
    ///
    /// __Default value:__ `"middle"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub baseline: Option<Baseline>,
    /// Whether a mark be clipped to the enclosing group’s width and height.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub clip: Option<bool>,
    /// Default color.
    ///
    /// __Default value:__ <span style="color: #4682b4;">&#9632;</span> `"#4682b4"`
    ///
    /// __Note:__
    /// - This property cannot be used in a [style
    /// config](https://vega.github.io/vega-lite/docs/mark.html#style-config).
    /// - The `fill` and `stroke` properties have higher precedence than `color` and will
    /// override `color`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub color: Option<ColorUnion>,
    /// The radius in pixels of rounded rectangle corners.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadius")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius: Option<f64>,
    /// The radius in pixels of rounded rectangle bottom left corner.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadiusBottomLeft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius_bottom_left: Option<f64>,
    /// The radius in pixels of rounded rectangle bottom right corner.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadiusBottomRight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius_bottom_right: Option<f64>,
    /// The radius in pixels of rounded rectangle top right corner.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadiusTopLeft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius_top_left: Option<f64>,
    /// The radius in pixels of rounded rectangle top left corner.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadiusTopRight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius_top_right: Option<f64>,
    /// The mouse cursor used over the mark. Any valid [CSS cursor
    /// type](https://developer.mozilla.org/en-US/docs/Web/CSS/cursor#Values) can be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub cursor: Option<Cursor>,
    /// The direction of the text. One of `"ltr"` (left-to-right) or `"rtl"` (right-to-left).
    /// This property determines on which side is truncated in response to the limit parameter.
    ///
    /// __Default value:__ `"ltr"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dir: Option<Dir>,
    /// The horizontal offset, in pixels, between the text label and its anchor point. The offset
    /// is applied after rotation by the _angle_ property.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dx: Option<f64>,
    /// The vertical offset, in pixels, between the text label and its anchor point. The offset
    /// is applied after rotation by the _angle_ property.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dy: Option<f64>,
    /// The ellipsis string for text truncated in response to the limit parameter.
    ///
    /// __Default value:__ `"…"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ellipsis: Option<String>,
    /// Default Fill Color. This has higher precedence than `config.color`.
    ///
    /// __Default value:__ (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fill: Option<FillUnion>,
    /// Whether the mark's color should be used as fill color instead of stroke color.
    ///
    /// __Default value:__ `false` for all `point`, `line`, and `rule` marks as well as
    /// `geoshape` marks for
    /// [`graticule`](https://vega.github.io/vega-lite/docs/data.html#graticule) data sources;
    /// otherwise, `true`.
    ///
    /// __Note:__ This property cannot be used in a [style
    /// config](https://vega.github.io/vega-lite/docs/mark.html#style-config).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub filled: Option<bool>,
    /// The fill opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "fillOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fill_opacity: Option<f64>,
    /// The typeface to set the text in (e.g., `"Helvetica Neue"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font: Option<String>,
    /// The font size, in pixels.
    #[serde(rename = "fontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font_size: Option<f64>,
    /// The font style (e.g., `"italic"`).
    #[serde(rename = "fontStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font_style: Option<String>,
    /// The font weight.
    /// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
    /// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
    #[serde(rename = "fontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font_weight: Option<FontWeight>,
    /// Height of the marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub height: Option<f64>,
    /// A URL to load upon mouse click. If defined, the mark acts as a hyperlink.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub href: Option<String>,
    /// The line interpolation method to use for line and area marks. One of the following:
    /// - `"linear"`: piecewise linear segments, as in a polyline.
    /// - `"linear-closed"`: close the linear segments to form a polygon.
    /// - `"step"`: alternate between horizontal and vertical segments, as in a step function.
    /// - `"step-before"`: alternate between vertical and horizontal segments, as in a step
    /// function.
    /// - `"step-after"`: alternate between horizontal and vertical segments, as in a step
    /// function.
    /// - `"basis"`: a B-spline, with control point duplication on the ends.
    /// - `"basis-open"`: an open B-spline; may not intersect the start or end.
    /// - `"basis-closed"`: a closed B-spline, as in a loop.
    /// - `"cardinal"`: a Cardinal spline, with control point duplication on the ends.
    /// - `"cardinal-open"`: an open Cardinal spline; may not intersect the start or end, but
    /// will intersect other control points.
    /// - `"cardinal-closed"`: a closed Cardinal spline, as in a loop.
    /// - `"bundle"`: equivalent to basis, except the tension parameter is used to straighten the
    /// spline.
    /// - `"monotone"`: cubic interpolation that preserves monotonicity in y.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub interpolate: Option<Interpolate>,
    /// Defines how Vega-Lite should handle marks for invalid values (`null` and `NaN`).
    /// - If set to `"filter"` (default), all data items with null values will be skipped (for
    /// line, trail, and area marks) or filtered (for other marks).
    /// - If `null`, all data items are included. In this case, invalid values will be
    /// interpreted as zeroes.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub invalid: RemovableValue<Invalid>,
    /// The maximum length of the text mark in pixels. The text value will be automatically
    /// truncated if the rendered size exceeds the limit.
    ///
    /// __Default value:__ `0`, indicating no limit
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub limit: Option<f64>,
    /// A delimiter, such as a newline character, upon which to break text strings into multiple
    /// lines. This property will be ignored if the text property is array-valued.
    #[serde(rename = "lineBreak")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub line_break: Option<String>,
    /// The height, in pixels, of each line of text in a multi-line text mark.
    #[serde(rename = "lineHeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub line_height: Option<f64>,
    /// The overall opacity (value between [0,1]).
    ///
    /// __Default value:__ `0.7` for non-aggregate plots with `point`, `tick`, `circle`, or
    /// `square` marks or layered `bar` charts and `1` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub opacity: Option<f64>,
    /// For line and trail marks, this `order` property can be set to `null` or `false` to make
    /// the lines use the original order in the data sources.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub order: Option<bool>,
    /// The orientation of a non-stacked bar, tick, area, and line charts.
    /// The value is either horizontal (default) or vertical.
    /// - For bar, rule and tick, this determines whether the size of the bar and tick
    /// should be applied to x or y dimension.
    /// - For area, this property determines the orient property of the Vega output.
    /// - For line and trail marks, this property determines the sort order of the points in the
    /// line
    /// if `config.sortLineBy` is not specified.
    /// For stacked charts, this is always determined by the orientation of the stack;
    /// therefore explicitly specified value will be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub orient: Option<Orientation>,
    /// Polar coordinate radial offset, in pixels, of the text label from the origin determined
    /// by the `x` and `y` properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub radius: Option<f64>,
    /// Shape of the point marks. Supported values include:
    /// - plotting shapes: `"circle"`, `"square"`, `"cross"`, `"diamond"`, `"triangle-up"`,
    /// `"triangle-down"`, `"triangle-right"`, or `"triangle-left"`.
    /// - the line symbol `"stroke"`
    /// - centered directional shapes `"arrow"`, `"wedge"`, or `"triangle"`
    /// - a custom [SVG path
    /// string](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths) (For correct
    /// sizing, custom shape paths should be defined within a square bounding box with
    /// coordinates ranging from -1 to 1 along both the x and y dimensions.)
    ///
    /// __Default value:__ `"circle"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub shape: Option<String>,
    /// Default size for marks.
    /// - For `point`/`circle`/`square`, this represents the pixel area of the marks. For
    /// example: in the case of circles, the radius is determined in part by the square root of
    /// the size value.
    /// - For `bar`, this represents the band size of the bar, in pixels.
    /// - For `text`, this represents the font size, in pixels.
    ///
    /// __Default value:__
    /// - `30` for point, circle, square marks; width/height's `step`
    /// - `2` for bar marks with discrete dimensions;
    /// - `5` for bar marks with continuous dimensions;
    /// - `11` for text marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub size: Option<f64>,
    /// Default Stroke Color. This has higher precedence than `config.color`.
    ///
    /// __Default value:__ (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke: Option<FillUnion>,
    /// The stroke cap for line ending style. One of `"butt"`, `"round"`, or `"square"`.
    ///
    /// __Default value:__ `"square"`
    #[serde(rename = "strokeCap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_cap: Option<StrokeCap>,
    /// An array of alternating stroke, space lengths for creating dashed or dotted lines.
    #[serde(rename = "strokeDash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_dash: Option<Vec<f64>>,
    /// The offset (in pixels) into which to begin drawing with the stroke dash array.
    #[serde(rename = "strokeDashOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_dash_offset: Option<f64>,
    /// The stroke line join method. One of `"miter"`, `"round"` or `"bevel"`.
    ///
    /// __Default value:__ `"miter"`
    #[serde(rename = "strokeJoin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_join: Option<StrokeJoin>,
    /// The miter limit at which to bevel a line join.
    #[serde(rename = "strokeMiterLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_miter_limit: Option<f64>,
    /// The stroke opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "strokeOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_opacity: Option<f64>,
    /// The stroke width, in pixels.
    #[serde(rename = "strokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_width: Option<f64>,
    /// A string or array of strings indicating the name of custom styles to apply to the mark. A
    /// style is a named collection of mark property defaults defined within the [style
    /// configuration](https://vega.github.io/vega-lite/docs/mark.html#style-config). If style is
    /// an array, later styles will override earlier styles. Any [mark
    /// properties](https://vega.github.io/vega-lite/docs/encoding.html#mark-prop) explicitly
    /// defined within the `encoding` will override a style default.
    ///
    /// __Default value:__ The mark's name. For example, a bar mark will have style `"bar"` by
    /// default.
    /// __Note:__ Any specified style will augment the default style. For example, a bar mark
    /// with `"style": "foo"` will receive from `config.style.bar` and `config.style.foo` (the
    /// specified style `"foo"` has higher precedence).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub style: Option<PurpleText>,
    /// Depending on the interpolation type, sets the tension parameter (for line and area marks).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tension: Option<f64>,
    /// Placeholder text if the `text` channel is not specified
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub text: Option<PurpleText>,
    /// Polar coordinate angle, in radians, of the text label from the origin determined by the
    /// `x` and `y` properties. Values for `theta` follow the same convention of `arc` mark
    /// `startAngle` and `endAngle` properties: angles are measured in radians, with `0`
    /// indicating "north".
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub theta: Option<f64>,
    /// Default relative band size for a time unit. If set to `1`, the bandwidth of the marks
    /// will be equal to the time unit band step.
    /// If set to `0.5`, bandwidth of the marks will be half of the time unit band step.
    #[serde(rename = "timeUnitBand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit_band: Option<f64>,
    /// Default relative band position for a time unit. If set to `0`, the marks will be
    /// positioned at the beginning of the time unit band step.
    /// If set to `0.5`, the marks will be positioned in the middle of the time unit band step.
    #[serde(rename = "timeUnitBandPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit_band_position: Option<f64>,
    /// The tooltip text string to show upon mouse hover or an object defining which fields
    /// should the tooltip be derived from.
    ///
    /// - If `tooltip` is `true` or `{"content": "encoding"}`, then all fields from `encoding`
    /// will be used.
    /// - If `tooltip` is `{"content": "data"}`, then all fields that appear in the highlighted
    /// data point will be used.
    /// - If set to `null` or `false`, then no tooltip will be used.
    ///
    /// See the [`tooltip`](https://vega.github.io/vega-lite/docs/tooltip.html) documentation for
    /// a detailed discussion about tooltip  in Vega-Lite.
    ///
    /// __Default value:__ `null`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tooltip: Option<Value>,
    /// Width of the marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub width: Option<f64>,
    /// X coordinates of the marks, or width of horizontal `"bar"` and `"area"` without specified
    /// `x2` or `width`.
    ///
    /// The `value` of this channel can be a number or a string `"width"` for the width of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub x: Option<XUnion>,
    /// X2 coordinates for ranged `"area"`, `"bar"`, `"rect"`, and  `"rule"`.
    ///
    /// The `value` of this channel can be a number or a string `"width"` for the width of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub x2: Option<XUnion>,
    /// Offset for x2-position.
    #[serde(rename = "x2Offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub x2_offset: Option<f64>,
    /// Offset for x-position.
    #[serde(rename = "xOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub x_offset: Option<f64>,
    /// Y coordinates of the marks, or height of vertical `"bar"` and `"area"` without specified
    /// `y2` or `height`.
    ///
    /// The `value` of this channel can be a number or a string `"height"` for the height of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub y: Option<YUnion>,
    /// Y2 coordinates for ranged `"area"`, `"bar"`, `"rect"`, and  `"rule"`.
    ///
    /// The `value` of this channel can be a number or a string `"height"` for the height of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub y2: Option<YUnion>,
    /// Offset for y2-position.
    #[serde(rename = "y2Offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub y2_offset: Option<f64>,
    /// Offset for y-position.
    #[serde(rename = "yOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub y_offset: Option<f64>,
}

/// Projection configuration, which determines default properties for all
/// [projections](https://vega.github.io/vega-lite/docs/projection.html). For a full list of
/// projection configuration options, please see the [corresponding section of the projection
/// documentation](https://vega.github.io/vega-lite/docs/projection.html#config).
///
/// Any property of Projection can be in config
///
/// An object defining properties of geographic projection, which will be applied to `shape`
/// path for `"geoshape"` marks
/// and to `latitude` and `"longitude"` channels for other marks.
///
/// An object defining properties of the geographic projection shared by underlying layers.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct Projection {
    /// The projection’s center to the specified center, a two-element array of longitude and
    /// latitude in degrees.
    ///
    /// __Default value:__ `[0, 0]`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub center: Option<Vec<f64>>,
    /// The projection’s clipping circle radius to the specified angle in degrees. If `null`,
    /// switches to [antimeridian](http://bl.ocks.org/mbostock/3788999) cutting rather than
    /// small-circle clipping.
    #[serde(rename = "clipAngle")]
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub clip_angle: RemovableValue<f64>,
    /// The projection’s viewport clip extent to the specified bounds in pixels. The extent
    /// bounds are specified as an array `[[x0, y0], [x1, y1]]`, where `x0` is the left-side of
    /// the viewport, `y0` is the top, `x1` is the right and `y1` is the bottom. If `null`, no
    /// viewport clipping is performed.
    #[serde(rename = "clipExtent")]
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub clip_extent: RemovableValue<Vec<Vec<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub coefficient: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub distance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fraction: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub lobes: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub parallel: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub parallels: Option<Vec<f64>>,
    /// The threshold for the projection’s [adaptive
    /// resampling](http://bl.ocks.org/mbostock/3795544) to the specified value in pixels. This
    /// value corresponds to the [Douglas–Peucker
    /// distance](http://en.wikipedia.org/wiki/Ramer%E2%80%93Douglas%E2%80%93Peucker_algorithm).
    /// If precision is not specified, returns the projection’s current resampling precision
    /// which defaults to `√0.5 ≅ 0.70710…`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub precision: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub radius: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ratio: Option<f64>,
    #[serde(rename = "reflectX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub reflect_x: Option<bool>,
    #[serde(rename = "reflectY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub reflect_y: Option<bool>,
    /// The projection’s three-axis rotation to the specified angles, which must be a two- or
    /// three-element array of numbers [`lambda`, `phi`, `gamma`] specifying the rotation angles
    /// in degrees about each spherical axis. (These correspond to yaw, pitch and roll.)
    ///
    /// __Default value:__ `[0, 0, 0]`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub rotate: Option<Vec<f64>>,
    /// The projection's scale (zoom) value, overriding automatic fitting.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub spacing: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tilt: Option<f64>,
    /// The projection's translation (pan) value, overriding automatic fitting.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub translate: Option<Vec<f64>>,
    /// The cartographic projection to use. This value is case-insensitive, for example
    /// `"albers"` and `"Albers"` indicate the same projection type. You can find all valid
    /// projection types [in the
    /// documentation](https://vega.github.io/vega-lite/docs/projection.html#projection-types).
    ///
    /// __Default value:__ `mercator`
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub projection_type: Option<ProjectionType>,
}

/// Scale, axis, and legend resolutions for view composition specifications.
///
/// Defines how scales, axes, and legends from different specs should be combined. Resolve is
/// a mapping from `scale`, `axis`, and `legend` to a mapping from channels to resolutions.
/// Scales and guides can be resolved to be `"independent"` or `"shared"`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct Resolve {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub axis: Option<AxisResolveMap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub legend: Option<LegendResolveMap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub scale: Option<ScaleResolveMap>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct AxisResolveMap {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub x: Option<ResolveMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub y: Option<ResolveMode>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct LegendResolveMap {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub color: Option<ResolveMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fill: Option<ResolveMode>,
    #[serde(rename = "fillOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fill_opacity: Option<ResolveMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub opacity: Option<ResolveMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub shape: Option<ResolveMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub size: Option<ResolveMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke: Option<ResolveMode>,
    #[serde(rename = "strokeOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_opacity: Option<ResolveMode>,
    #[serde(rename = "strokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_width: Option<ResolveMode>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ScaleResolveMap {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub color: Option<ResolveMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fill: Option<ResolveMode>,
    #[serde(rename = "fillOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fill_opacity: Option<ResolveMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub opacity: Option<ResolveMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub shape: Option<ResolveMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub size: Option<ResolveMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke: Option<ResolveMode>,
    #[serde(rename = "strokeOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_opacity: Option<ResolveMode>,
    #[serde(rename = "strokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_width: Option<ResolveMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub x: Option<ResolveMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub y: Option<ResolveMode>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct SelectionDef {
    /// When set, a selection is populated by input elements (also known as dynamic query
    /// widgets)
    /// or by interacting with the corresponding legend. Direct manipulation interaction is
    /// disabled by default;
    /// to re-enable it, set the selection's
    /// [`on`](https://vega.github.io/vega-lite/docs/selection.html#common-selection-properties)
    /// property.
    ///
    /// Legend bindings are restricted to selections that only specify a single field or
    /// encoding.
    ///
    /// Query widget binding takes the form of Vega's [input element binding
    /// definition](https://vega.github.io/vega/docs/signals/#bind)
    /// or can be a mapping between projected field/encodings and binding definitions.
    ///
    /// __See also:__ [`bind`](https://vega.github.io/vega-lite/docs/bind.html) documentation.
    ///
    /// When set, a selection is populated by interacting with the corresponding legend. Direct
    /// manipulation interaction is disabled by default;
    /// to re-enable it, set the selection's
    /// [`on`](https://vega.github.io/vega-lite/docs/selection.html#common-selection-properties)
    /// property.
    ///
    /// Legend bindings are restricted to selections that only specify a single field or
    /// encoding.
    ///
    /// Establishes a two-way binding between the interval selection and the scales
    /// used within the same view. This allows a user to interactively pan and
    /// zoom the view.
    ///
    /// __See also:__ [`bind`](https://vega.github.io/vega-lite/docs/bind.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bind: Option<BindUnion>,
    /// Clears the selection, emptying it of all values. Can be a
    /// [Event Stream](https://vega.github.io/vega/docs/event-streams/) or `false` to disable.
    ///
    /// __Default value:__ `dblclick`.
    ///
    /// __See also:__ [`clear`](https://vega.github.io/vega-lite/docs/clear.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub clear: Option<ClearUnion>,
    /// By default, `all` data values are considered to lie within an empty selection.
    /// When set to `none`, empty selections contain no data values.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub empty: Option<Empty>,
    /// An array of encoding channels. The corresponding data field values
    /// must match for a data tuple to fall within the selection.
    ///
    /// __See also:__ [`encodings`](https://vega.github.io/vega-lite/docs/project.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub encodings: Option<Vec<SingleDefUnitChannel>>,
    /// An array of field names whose values must match for a data tuple to
    /// fall within the selection.
    ///
    /// __See also:__ [`fields`](https://vega.github.io/vega-lite/docs/project.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fields: Option<Vec<String>>,
    /// Initialize the selection with a mapping between [projected channels or field
    /// names](https://vega.github.io/vega-lite/docs/project.html) and initial values.
    ///
    /// __See also:__ [`init`](https://vega.github.io/vega-lite/docs/init.html) documentation.
    ///
    /// Initialize the selection with a mapping between [projected channels or field
    /// names](https://vega.github.io/vega-lite/docs/project.html) and an initial
    /// value (or array of values).
    ///
    /// __See also:__ [`init`](https://vega.github.io/vega-lite/docs/init.html) documentation.
    ///
    /// Initialize the selection with a mapping between [projected channels or field
    /// names](https://vega.github.io/vega-lite/docs/project.html) and arrays of
    /// initial values.
    ///
    /// __See also:__ [`init`](https://vega.github.io/vega-lite/docs/init.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub init: Option<Init>,
    /// When true, an invisible voronoi diagram is computed to accelerate discrete
    /// selection. The data value _nearest_ the mouse cursor is added to the selection.
    ///
    /// __See also:__ [`nearest`](https://vega.github.io/vega-lite/docs/nearest.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub nearest: Option<bool>,
    /// A [Vega event stream](https://vega.github.io/vega/docs/event-streams/) (object or
    /// selector) that triggers the selection.
    /// For interval selections, the event stream must specify a [start and
    /// end](https://vega.github.io/vega/docs/event-streams/#between-filters).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub on: Option<OnUnion>,
    /// With layered and multi-view displays, a strategy that determines how
    /// selections' data queries are resolved when applied in a filter transform,
    /// conditional encoding rule, or scale domain.
    ///
    /// __See also:__ [`resolve`](https://vega.github.io/vega-lite/docs/selection-resolve.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub resolve: Option<SelectionResolution>,
    /// Determines the default event processing and data query for the selection. Vega-Lite
    /// currently supports three selection types:
    ///
    /// - `"single"` -- to select a single discrete data value on `click`.
    /// - `"multi"` -- to select multiple discrete data value; the first value is selected on
    /// `click` and additional values toggled on shift-`click`.
    /// - `"interval"` -- to select a continuous range of data values on `drag`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub selection_def_type: Option<SelectionDefType>,
    /// Controls whether data values should be toggled or only ever inserted into
    /// multi selections. Can be `true`, `false` (for insertion only), or a
    /// [Vega expression](https://vega.github.io/vega/docs/expressions/).
    ///
    /// __Default value:__ `true`, which corresponds to `event.shiftKey` (i.e.,
    /// data values are toggled when a user interacts with the shift-key pressed).
    ///
    /// __See also:__ [`toggle`](https://vega.github.io/vega-lite/docs/toggle.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub toggle: Option<Translate>,
    /// An interval selection also adds a rectangle mark to depict the
    /// extents of the interval. The `mark` property can be used to customize the
    /// appearance of the mark.
    ///
    /// __See also:__ [`mark`](https://vega.github.io/vega-lite/docs/selection-mark.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub mark: Option<BrushConfig>,
    /// When truthy, allows a user to interactively move an interval selection
    /// back-and-forth. Can be `true`, `false` (to disable panning), or a
    /// [Vega event stream definition](https://vega.github.io/vega/docs/event-streams/)
    /// which must include a start and end event to trigger continuous panning.
    ///
    /// __Default value:__ `true`, which corresponds to
    /// `[mousedown, window:mouseup] > window:mousemove!` which corresponds to
    /// clicks and dragging within an interval selection to reposition it.
    ///
    /// __See also:__ [`translate`](https://vega.github.io/vega-lite/docs/translate.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub translate: Option<Translate>,
    /// When truthy, allows a user to interactively resize an interval selection.
    /// Can be `true`, `false` (to disable zooming), or a [Vega event stream
    /// definition](https://vega.github.io/vega/docs/event-streams/). Currently,
    /// only `wheel` events are supported.
    ///
    /// __Default value:__ `true`, which corresponds to `wheel!`.
    ///
    /// __See also:__ [`zoom`](https://vega.github.io/vega-lite/docs/zoom.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub zoom: Option<Translate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct PurpleBinding {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub debounce: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub element: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub input: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub binding_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub labels: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub options: Option<Vec<Option<serde_json::Value>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub step: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub autocomplete: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub between: Option<Vec<Stream>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub consume: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub filter: Option<PurpleText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub markname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub marktype: Option<MarkType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub source: Option<Source>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub throttle: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stream: Option<Stream>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub merge: Option<Vec<Stream>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct Stream {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub between: Option<Vec<Stream>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub consume: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub debounce: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub filter: Option<PurpleText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub markname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub marktype: Option<MarkType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub source: Option<Source>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub throttle: Option<f64>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stream_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stream: Option<Box<Stream>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub merge: Option<Vec<Stream>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ClearDerivedStream {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub between: Option<Vec<Stream>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub consume: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub debounce: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub filter: Option<PurpleText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub markname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub marktype: Option<MarkType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub source: Option<Source>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub throttle: Option<f64>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ed_stream_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stream: Option<Stream>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub merge: Option<Vec<Stream>>,
}

/// An interval selection also adds a rectangle mark to depict the
/// extents of the interval. The `mark` property can be used to customize the
/// appearance of the mark.
///
/// __See also:__ [`mark`](https://vega.github.io/vega-lite/docs/selection-mark.html)
/// documentation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct BrushConfig {
    /// The fill color of the interval mark.
    ///
    /// __Default value:__ `"#333333"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fill: Option<String>,
    /// The fill opacity of the interval mark (a value between 0 and 1).
    ///
    /// __Default value:__ `0.125`
    #[serde(rename = "fillOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fill_opacity: Option<f64>,
    /// The stroke color of the interval mark.
    ///
    /// __Default value:__ `"#ffffff"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke: Option<String>,
    /// An array of alternating stroke and space lengths,
    /// for creating dashed or dotted lines.
    #[serde(rename = "strokeDash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_dash: Option<Vec<f64>>,
    /// The offset (in pixels) with which to begin drawing the stroke dash array.
    #[serde(rename = "strokeDashOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_dash_offset: Option<f64>,
    /// The stroke opacity of the interval mark (a value between `0` and `1`).
    #[serde(rename = "strokeOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_opacity: Option<f64>,
    /// The stroke width of the interval mark.
    #[serde(rename = "strokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_width: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct OnDerivedStream {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub between: Option<Vec<Stream>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub consume: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub debounce: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub filter: Option<PurpleText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub markname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub marktype: Option<MarkType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub source: Option<Source>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub throttle: Option<f64>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ed_stream_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stream: Option<Stream>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub merge: Option<Vec<Stream>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct TitleParams {
    /// Horizontal text alignment for title text. One of `"left"`, `"center"`, or `"right"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub align: Option<Align>,
    /// The anchor position for placing the title. One of `"start"`, `"middle"`, or `"end"`. For
    /// example, with an orientation of top these anchor positions map to a left-, center-, or
    /// right-aligned title.
    ///
    /// __Default value:__ `"middle"` for
    /// [single](https://vega.github.io/vega-lite/docs/spec.html) and
    /// [layered](https://vega.github.io/vega-lite/docs/layer.html) views.
    /// `"start"` for other composite views.
    ///
    /// __Note:__ [For now](https://github.com/vega/vega-lite/issues/2875), `anchor` is only
    /// customizable only for [single](https://vega.github.io/vega-lite/docs/spec.html) and
    /// [layered](https://vega.github.io/vega-lite/docs/layer.html) views. For other composite
    /// views, `anchor` is always `"start"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub anchor: Option<TitleAnchorEnum>,
    /// Angle in degrees of title and subtitle text.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub angle: Option<f64>,
    /// Vertical text baseline for title and subtitle text. One of `"top"`, `"middle"`,
    /// `"bottom"`, or `"alphabetic"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub baseline: Option<Baseline>,
    /// Text color for title text.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub color: Option<String>,
    /// Delta offset for title and subtitle text x-coordinate.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dx: Option<f64>,
    /// Delta offset for title and subtitle text y-coordinate.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dy: Option<f64>,
    /// Font name for title text.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font: Option<String>,
    /// Font size in pixels for title text.
    #[serde(rename = "fontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font_size: Option<f64>,
    /// Font style for title text.
    #[serde(rename = "fontStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font_style: Option<String>,
    /// Font weight for title text.
    /// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
    /// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
    #[serde(rename = "fontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font_weight: Option<FontWeight>,
    /// The reference frame for the anchor position, one of `"bounds"` (to anchor relative to the
    /// full bounding box) or `"group"` (to anchor relative to the group width or height).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub frame: Option<String>,
    /// The maximum allowed length in pixels of title and subtitle text.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub limit: Option<f64>,
    /// Line height in pixels for multi-line title text.
    #[serde(rename = "lineHeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub line_height: Option<f64>,
    /// The orthogonal offset in pixels by which to displace the title group from its position
    /// along the edge of the chart.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub offset: Option<f64>,
    /// Default title orientation (`"top"`, `"bottom"`, `"left"`, or `"right"`)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub orient: Option<TitleOrient>,
    /// A [mark style property](https://vega.github.io/vega-lite/docs/config.html#style) to apply
    /// to the title text mark.
    ///
    /// __Default value:__ `"group-title"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub style: Option<PurpleText>,
    /// The subtitle Text.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub subtitle: Option<PurpleText>,
    /// Text color for subtitle text.
    #[serde(rename = "subtitleColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub subtitle_color: Option<String>,
    /// Font name for subtitle text.
    #[serde(rename = "subtitleFont")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub subtitle_font: Option<String>,
    /// Font size in pixels for subtitle text.
    #[serde(rename = "subtitleFontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub subtitle_font_size: Option<f64>,
    /// Font style for subtitle text.
    #[serde(rename = "subtitleFontStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub subtitle_font_style: Option<String>,
    /// Font weight for subtitle text.
    /// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
    /// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
    #[serde(rename = "subtitleFontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub subtitle_font_weight: Option<FontWeight>,
    /// Line height in pixels for multi-line subtitle text.
    #[serde(rename = "subtitleLineHeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub subtitle_line_height: Option<f64>,
    /// The padding in pixels between title and subtitle text.
    #[serde(rename = "subtitlePadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub subtitle_padding: Option<f64>,
    /// The title text.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub text: Option<ConditionalValueDefTextText>,
    /// The integer z-index indicating the layering of the title group relative to other axis,
    /// mark and legend groups.
    ///
    /// __Default value:__ `0`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub zindex: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct Transform {
    /// Array of objects that define fields to aggregate.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aggregate: Option<Vec<AggregatedFieldDef>>,
    /// The data fields to group by. If not specified, a single group containing all data objects
    /// will be used.
    ///
    /// An optional array of fields by which to group the values.
    /// Imputation will then be performed on a per-group basis.
    ///
    /// The data fields for partitioning the data objects into separate groups. If unspecified,
    /// all data points will be in a single group.
    ///
    /// The data fields to group by.
    ///
    /// The data fields for partitioning the data objects into separate windows. If unspecified,
    /// all data points will be in a single window.
    ///
    /// The optional data fields to group by. If not specified, a single group containing all
    /// data objects will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub groupby: Option<Vec<String>>,
    /// The output fields at which to write the start and end bin values.
    ///
    /// The field for storing the computed formula value.
    ///
    /// The output fields for the sample value and corresponding density estimate.
    ///
    /// __Default value:__ `["value", "density"]`
    ///
    /// The output field names for extracted array values.
    ///
    /// __Default value:__ The field name of the corresponding array field
    ///
    /// The output field names for the key and value properties produced by the fold transform.
    /// __Default value:__ `["key", "value"]`
    ///
    /// The output field names for the smoothed points generated by the loess transform.
    ///
    /// __Default value:__ The field names of the input x and y values.
    ///
    /// The output fields on which to store the looked up data values.
    ///
    /// For data lookups, this property may be left blank if `from.fields`
    /// has been specified (those field names will be used); if `from.fields`
    /// has not been specified, `as` must be a string.
    ///
    /// For selection lookups, this property is optional: if unspecified,
    /// looked up values will be stored under a property named for the selection;
    /// and if specified, it must correspond to `from.fields`.
    ///
    /// The output field names for the probability and quantile values.
    ///
    /// __Default value:__ `["prob", "value"]`
    ///
    /// The output field names for the smoothed points generated by the regression transform.
    ///
    /// __Default value:__ The field names of the input x and y values.
    ///
    /// The output field to write the timeUnit value.
    ///
    /// Output field names. This can be either a string or an array of strings with two elements
    /// denoting the name for the fields for stack start and stack end respectively.
    /// If a single string(e.g., `"val"`) is provided, the end field will be `"val_end"`.
    #[serde(rename = "as")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub transform_as: Option<PurpleText>,
    /// An object indicating bin properties, or simply `true` for using default bin parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bin: Option<PurpleBin>,
    /// The data field to bin.
    ///
    /// The data field to apply time unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field: Option<String>,
    /// A [expression](https://vega.github.io/vega-lite/docs/types.html#expression) string. Use
    /// the variable `datum` to refer to the current data object.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub calculate: Option<String>,
    /// The bandwidth (standard deviation) of the Gaussian kernel. If unspecified or set to zero,
    /// the bandwidth value is automatically estimated from the input data using Scott’s rule.
    ///
    /// A bandwidth parameter in the range `[0, 1]` that determines the amount of smoothing.
    ///
    /// __Default value:__ `0.3`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bandwidth: Option<f64>,
    /// A boolean flag indicating if the output values should be probability estimates (false) or
    /// smoothed counts (true).
    ///
    /// __Default value:__ `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub counts: Option<bool>,
    /// A boolean flag indicating whether to produce density estimates (false) or cumulative
    /// density estimates (true).
    ///
    /// __Default value:__ `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub cumulative: Option<bool>,
    /// The data field for which to perform density estimation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub density: Option<String>,
    /// A [min, max] domain from which to sample the distribution. If unspecified, the extent
    /// will be determined by the observed minimum and maximum values of the density value
    /// field.
    ///
    /// A [min, max] domain over the independent (x) field for the starting and ending points of
    /// the generated trend line.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub extent: Option<Vec<f64>>,
    /// The maximum number of samples to take along the extent domain for plotting the density.
    ///
    /// __Default value:__ `200`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub maxsteps: Option<f64>,
    /// The minimum number of samples to take along the extent domain for plotting the density.
    ///
    /// __Default value:__ `25`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub minsteps: Option<f64>,
    /// The exact number of samples to take along the extent domain for plotting the density. If
    /// specified, overrides both minsteps and maxsteps to set an exact number of uniform
    /// samples. Potentially useful in conjunction with a fixed extent to ensure consistent
    /// sample points for stacked densities.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub steps: Option<f64>,
    /// The `filter` property must be one of the predicate definitions:
    ///
    /// 1) an [expression](https://vega.github.io/vega-lite/docs/types.html#expression) string,
    /// where `datum` can be used to refer to the current data object
    ///
    /// 2) one of the field predicates:
    /// [`equal`](https://vega.github.io/vega-lite/docs/filter.html#equal-predicate),
    /// [`lt`](https://vega.github.io/vega-lite/docs/filter.html#lt-predicate),
    /// [`lte`](https://vega.github.io/vega-lite/docs/filter.html#lte-predicate),
    /// [`gt`](https://vega.github.io/vega-lite/docs/filter.html#gt-predicate),
    /// [`gte`](https://vega.github.io/vega-lite/docs/filter.html#gte-predicate),
    /// [`range`](https://vega.github.io/vega-lite/docs/filter.html#range-predicate),
    /// [`oneOf`](https://vega.github.io/vega-lite/docs/filter.html#one-of-predicate),
    /// or [`valid`](https://vega.github.io/vega-lite/docs/filter.html#valid-predicate),
    ///
    /// 3) a [selection
    /// predicate](https://vega.github.io/vega-lite/docs/filter.html#selection-predicate)
    ///
    /// 4) a logical operand that combines (1), (2), or (3).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub filter: Option<PurpleLogicalOperandPredicate>,
    /// An array of one or more data fields containing arrays to flatten.
    /// If multiple fields are specified, their array values should have a parallel structure,
    /// ideally with the same length.
    /// If the lengths of parallel arrays do not match,
    /// the longest array will be used with `null` values added for missing entries.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub flatten: Option<Vec<String>>,
    /// An array of data fields indicating the properties to fold.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fold: Option<Vec<String>>,
    /// A frame specification as a two-element array used to control the window over which the
    /// specified method is applied. The array entries should either be a number indicating the
    /// offset from the current data object, or null to indicate unbounded rows preceding or
    /// following the current data object. For example, the value `[-5, 5]` indicates that the
    /// window should include five objects preceding and five objects following the current
    /// object.
    ///
    /// __Default value:__:  `[null, null]` indicating that the window includes all objects.
    ///
    /// A frame specification as a two-element array indicating how the sliding window should
    /// proceed. The array entries should either be a number indicating the offset from the
    /// current data object, or null to indicate unbounded rows preceding or following the
    /// current data object. The default value is `[null, 0]`, indicating that the sliding window
    /// includes the current object and all preceding objects. The value `[-5, 5]` indicates that
    /// the window should include five objects preceding and five objects following the current
    /// object. Finally, `[null, null]` indicates that the window frame should always include all
    /// data objects. If you this frame and want to assign the same value to add objects, you can
    /// use the simpler [join aggregate
    /// transform](https://vega.github.io/vega-lite/docs/joinaggregate.html). The only operators
    /// affected are the aggregation operations and the `first_value`, `last_value`, and
    /// `nth_value` window operations. The other window operations are not affected by this.
    ///
    /// __Default value:__:  `[null, 0]` (includes the current object and all preceding objects)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub frame: Option<Vec<Option<f64>>>,
    /// The data field for which the missing values should be imputed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub impute: Option<String>,
    /// A key field that uniquely identifies data objects within a group.
    /// Missing key values (those occurring in the data but not in the current group) will be
    /// imputed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub key: Option<String>,
    /// Defines the key values that should be considered for imputation.
    /// An array of key values or an object defining a [number
    /// sequence](https://vega.github.io/vega-lite/docs/impute.html#sequence-def).
    ///
    /// If provided, this will be used in addition to the key values observed within the input
    /// data. If not provided, the values will be derived from all unique values of the `key`
    /// field. For `impute` in `encoding`, the key field is the x-field if the y-field is
    /// imputed, or vice versa.
    ///
    /// If there is no impute grouping, this property _must_ be specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub keyvals: Option<Keyvals>,
    /// The imputation method to use for the field value of imputed data objects.
    /// One of `"value"`, `"mean"`, `"median"`, `"max"` or `"min"`.
    ///
    /// __Default value:__  `"value"`
    ///
    /// The functional form of the regression model. One of `"linear"`, `"log"`, `"exp"`,
    /// `"pow"`, `"quad"`, or `"poly"`.
    ///
    /// __Default value:__ `"linear"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub method: Option<TransformMethod>,
    /// The field value to use when the imputation `method` is `"value"`.
    ///
    /// The data field to populate pivoted fields. The aggregate values of this field become the
    /// values of the new pivoted fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub value: Option<serde_json::Value>,
    /// The definition of the fields in the join aggregate, and what calculations to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub joinaggregate: Option<Vec<JoinAggregateFieldDef>>,
    /// The data field of the dependent variable to smooth.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub loess: Option<String>,
    /// The data field of the independent variable to use a predictor.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub on: Option<String>,
    /// The default value to use if lookup fails.
    ///
    /// __Default value:__ `null`
    #[serde(rename = "default")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub transform_default: Option<String>,
    /// Data source or selection for secondary data reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub from: Option<Lookup>,
    /// Key in primary data source.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub lookup: Option<String>,
    /// An array of probabilities in the range (0, 1) for which to compute quantile values. If
    /// not specified, the *step* parameter will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub probs: Option<Vec<f64>>,
    /// The data field for which to perform quantile estimation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub quantile: Option<String>,
    /// A probability step size (default 0.01) for sampling quantile values. All values from
    /// one-half the step size up to 1 (exclusive) will be sampled. This parameter is only used
    /// if the *probs* parameter is not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub step: Option<f64>,
    /// The polynomial order (number of coefficients) for the 'poly' method.
    ///
    /// __Default value:__ `3`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub order: Option<f64>,
    /// A boolean flag indicating if the transform should return the regression model parameters
    /// (one object per group), rather than trend line points.
    /// The resulting objects include a `coef` array of fitted coefficient values (starting with
    /// the intercept term and then including terms of increasing order)
    /// and an `rSquared` value (indicating the total variance explained by the model).
    ///
    /// __Default value:__ `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub params: Option<bool>,
    /// The data field of the dependent variable to predict.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub regression: Option<String>,
    /// The timeUnit.
    #[serde(rename = "timeUnit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit: Option<TimeUnit>,
    /// The maximum number of data objects to include in the sample.
    ///
    /// __Default value:__ `1000`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub sample: Option<f64>,
    /// Mode for stacking marks. One of `"zero"` (default), `"center"`, or `"normalize"`.
    /// The `"zero"` offset will stack starting at `0`. The `"center"` offset will center the
    /// stacks. The `"normalize"` offset will compute percentage values for each stack point,
    /// with output values in the range `[0,1]`.
    ///
    /// __Default value:__ `"zero"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub offset: Option<StackOffset>,
    /// Field that determines the order of leaves in the stacked charts.
    ///
    /// A sort field definition for sorting data objects within a window. If two data objects are
    /// considered equal by the comparator, they are considered "peer" values of equal rank. If
    /// sort is not specified, the order is undefined: data objects are processed in the order
    /// they are observed and none are considered peers (the ignorePeers parameter is ignored and
    /// treated as if set to `true`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub sort: Option<Vec<SortField>>,
    /// The field which is stacked.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stack: Option<String>,
    /// Indicates if the sliding window frame should ignore peer values (data that are considered
    /// identical by the sort criteria). The default is false, causing the window frame to expand
    /// to include all peer values. If set to true, the window frame will be defined by offset
    /// values only. This setting only affects those operations that depend on the window frame,
    /// namely aggregation operations and the first_value, last_value, and nth_value window
    /// operations.
    ///
    /// __Default value:__ `false`
    #[serde(rename = "ignorePeers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ignore_peers: Option<bool>,
    /// The definition of the fields in the window, and what calculations to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub window: Option<Vec<WindowFieldDef>>,
    /// An optional parameter indicating the maximum number of pivoted fields to generate.
    /// The default (`0`) applies no limit. The pivoted `pivot` names are sorted in ascending
    /// order prior to enforcing the limit.
    /// __Default value:__ `0`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub limit: Option<f64>,
    /// The aggregation operation to apply to grouped `value` field values.
    /// __Default value:__ `sum`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub op: Option<String>,
    /// The data field to pivot on. The unique values of this field become new field names in the
    /// output stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub pivot: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct AggregatedFieldDef {
    /// The output field names to use for each aggregated field.
    #[serde(rename = "as")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aggregated_field_def_as: Option<String>,
    /// The data field for which to compute aggregate function. This is required for all
    /// aggregation operations except `"count"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field: Option<String>,
    /// The aggregation operation to apply to the fields (e.g., `"sum"`, `"average"`, or
    /// `"count"`).
    /// See the [full list of supported aggregation
    /// operations](https://vega.github.io/vega-lite/docs/aggregate.html#ops)
    /// for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub op: Option<AggregateOp>,
}

/// Data source or selection for secondary data reference.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct Lookup {
    /// Secondary data source to lookup in.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub data: Option<Data>,
    /// Fields in foreign data or selection to lookup.
    /// If not specified, the entire object is queried.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fields: Option<Vec<String>>,
    /// Key in data to lookup.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub key: Option<String>,
    /// Selection name to look up.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub selection: Option<String>,
}

/// Secondary data source to lookup in.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct Data {
    /// An object that specifies the format for parsing the data.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub format: Option<DataFormat>,
    /// Provide a placeholder name and bind data at runtime.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub name: Option<String>,
    /// An URL from which to load the data set. Use the `format.type` property
    /// to ensure the loaded data is correctly parsed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub url: Option<String>,
    /// The full data set, included inline. This can be an array of objects or primitive values,
    /// an object, or a string.
    /// Arrays of primitive values are ingested as objects with a `data` property. Strings are
    /// parsed according to the specified format type.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub values: Option<UrlDataInlineDataset>,
    /// Generate a sequence of numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub sequence: Option<SequenceParams>,
    /// Generate sphere GeoJSON data for the full globe.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub sphere: Option<SphereUnion>,
    /// Generate graticule GeoJSON data for geographic reference lines.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub graticule: Option<Graticule>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct JoinAggregateFieldDef {
    /// The output name for the join aggregate operation.
    #[serde(rename = "as")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub join_aggregate_field_def_as: Option<String>,
    /// The data field for which to compute the aggregate function. This can be omitted for
    /// functions that do not operate over a field such as `"count"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field: Option<String>,
    /// The aggregation operation to apply (e.g., `"sum"`, `"average"` or `"count"`). See the
    /// list of all supported operations
    /// [here](https://vega.github.io/vega-lite/docs/aggregate.html#ops).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub op: Option<AggregateOp>,
}

/// A sort definition for transform
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct SortField {
    /// The name of the field to sort.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field: Option<String>,
    /// Whether to sort the field in ascending or descending order. One of `"ascending"`
    /// (default), `"descending"`, or `null` (no not sort).
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub order: RemovableValue<SortOrder>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct WindowFieldDef {
    /// The output name for the window operation.
    #[serde(rename = "as")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub window_field_def_as: Option<String>,
    /// The data field for which to compute the aggregate or window function. This can be omitted
    /// for window functions that do not operate over a field such as `"count"`, `"rank"`,
    /// `"dense_rank"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field: Option<String>,
    /// The window or aggregation operation to apply within a window (e.g., `"rank"`, `"lead"`,
    /// `"sum"`, `"average"` or `"count"`). See the list of all supported operations
    /// [here](https://vega.github.io/vega-lite/docs/window.html#ops).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub op: Option<Op>,
    /// Parameter values for the window functions. Parameter values can be omitted for operations
    /// that do not accept a parameter.
    ///
    /// See the list of all supported operations and their parameters
    /// [here](https://vega.github.io/vega-lite/docs/transforms/window.html).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub param: Option<f64>,
}

/// An object defining the view background's fill and stroke.
///
/// __Default value:__ none (transparent)
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ViewBackground {
    /// The radius in pixels of rounded rectangle corners.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadius")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius: Option<f64>,
    /// The fill color.
    ///
    /// __Default value:__ `undefined`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fill: Option<String>,
    /// The fill opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "fillOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fill_opacity: Option<f64>,
    /// The overall opacity (value between [0,1]).
    ///
    /// __Default value:__ `0.7` for non-aggregate plots with `point`, `tick`, `circle`, or
    /// `square` marks or layered `bar` charts and `1` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub opacity: Option<f64>,
    /// The stroke color.
    ///
    /// __Default value:__ `"#ddd"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke: Option<String>,
    /// The stroke cap for line ending style. One of `"butt"`, `"round"`, or `"square"`.
    ///
    /// __Default value:__ `"square"`
    #[serde(rename = "strokeCap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_cap: Option<StrokeCap>,
    /// An array of alternating stroke, space lengths for creating dashed or dotted lines.
    #[serde(rename = "strokeDash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_dash: Option<Vec<f64>>,
    /// The offset (in pixels) into which to begin drawing with the stroke dash array.
    #[serde(rename = "strokeDashOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_dash_offset: Option<f64>,
    /// The stroke line join method. One of `"miter"`, `"round"` or `"bevel"`.
    ///
    /// __Default value:__ `"miter"`
    #[serde(rename = "strokeJoin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_join: Option<StrokeJoin>,
    /// The miter limit at which to bevel a line join.
    #[serde(rename = "strokeMiterLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_miter_limit: Option<f64>,
    /// The stroke opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "strokeOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_opacity: Option<f64>,
    /// The stroke width, in pixels.
    #[serde(rename = "strokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_width: Option<f64>,
    /// A string or array of strings indicating the name of custom styles to apply to the view
    /// background. A style is a named collection of mark property defaults defined within the
    /// [style configuration](https://vega.github.io/vega-lite/docs/mark.html#style-config). If
    /// style is an array, later styles will override earlier styles.
    ///
    /// __Default value:__ `"cell"`
    /// __Note:__ Any specified view background properties will augment the default style.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub style: Option<PurpleText>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct RepeatMapping {
    /// An array of fields to be repeated horizontally.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub column: Option<Vec<String>>,
    /// An array of fields to be repeated vertically.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub row: Option<Vec<String>>,
}

/// Vega-Lite configuration object. This property can only be defined at the top-level of a
/// specification.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct Config {
    /// Area-Specific Config
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub area: Option<AreaConfig>,
    /// How the visualization size should be determined. If a string, should be one of `"pad"`,
    /// `"fit"` or `"none"`.
    /// Object values can additionally specify parameters for content sizing and automatic
    /// resizing.
    ///
    /// __Default value__: `pad`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub autosize: Option<Autosize>,
    /// Axis configuration, which determines default properties for all `x` and `y`
    /// [axes](https://vega.github.io/vega-lite/docs/axis.html). For a full list of axis
    /// configuration options, please see the [corresponding section of the axis
    /// documentation](https://vega.github.io/vega-lite/docs/axis.html#config).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub axis: Option<AxisConfig>,
    /// Specific axis config for axes with "band" scales.
    #[serde(rename = "axisBand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub axis_band: Option<AxisConfig>,
    /// Specific axis config for x-axis along the bottom edge of the chart.
    #[serde(rename = "axisBottom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub axis_bottom: Option<AxisConfig>,
    /// Specific axis config for y-axis along the left edge of the chart.
    #[serde(rename = "axisLeft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub axis_left: Option<AxisConfig>,
    /// Specific axis config for y-axis along the right edge of the chart.
    #[serde(rename = "axisRight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub axis_right: Option<AxisConfig>,
    /// Specific axis config for x-axis along the top edge of the chart.
    #[serde(rename = "axisTop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub axis_top: Option<AxisConfig>,
    /// X-axis specific config.
    #[serde(rename = "axisX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub axis_x: Option<AxisConfig>,
    /// Y-axis specific config.
    #[serde(rename = "axisY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub axis_y: Option<AxisConfig>,
    /// CSS color property to use as the background of the entire view.
    ///
    /// __Default value:__ `"white"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub background: Option<String>,
    /// Bar-Specific Config
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bar: Option<RectConfig>,
    /// Box Config
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub boxplot: Option<BoxPlotConfig>,
    /// Circle-Specific Config
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub circle: Option<MarkConfig>,
    /// Default configuration for all concatenation view composition operators (`concat`,
    /// `hconcat`, and `vconcat`)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub concat: Option<CompositionConfig>,
    /// Default axis and legend title for count fields.
    ///
    /// __Default value:__ `'Count of Records`.
    #[serde(rename = "countTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub count_title: Option<String>,
    /// ErrorBand Config
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub errorband: Option<ErrorBandConfig>,
    /// ErrorBar Config
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub errorbar: Option<ErrorBarConfig>,
    /// Default configuration for the `facet` view composition operator
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub facet: Option<CompositionConfig>,
    /// Defines how Vega-Lite generates title for fields. There are three possible styles:
    /// - `"verbal"` (Default) - displays function in a verbal style (e.g., "Sum of field",
    /// "Year-month of date", "field (binned)").
    /// - `"function"` - displays function using parentheses and capitalized texts (e.g.,
    /// "SUM(field)", "YEARMONTH(date)", "BIN(field)").
    /// - `"plain"` - displays only the field name without functions (e.g., "field", "date",
    /// "field").
    #[serde(rename = "fieldTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub field_title: Option<FieldTitle>,
    /// Geoshape-Specific Config
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub geoshape: Option<MarkConfig>,
    /// Header configuration, which determines default properties for all
    /// [headers](https://vega.github.io/vega-lite/docs/header.html).
    ///
    /// For a full list of header configuration options, please see the [corresponding section of
    /// in the header documentation](https://vega.github.io/vega-lite/docs/header.html#config).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub header: Option<HeaderConfig>,
    /// Header configuration, which determines default properties for column
    /// [headers](https://vega.github.io/vega-lite/docs/header.html).
    ///
    /// For a full list of header configuration options, please see the [corresponding section of
    /// in the header documentation](https://vega.github.io/vega-lite/docs/header.html#config).
    #[serde(rename = "headerColumn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub header_column: Option<HeaderConfig>,
    /// Header configuration, which determines default properties for non-row/column facet
    /// [headers](https://vega.github.io/vega-lite/docs/header.html).
    ///
    /// For a full list of header configuration options, please see the [corresponding section of
    /// in the header documentation](https://vega.github.io/vega-lite/docs/header.html#config).
    #[serde(rename = "headerFacet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub header_facet: Option<HeaderConfig>,
    /// Header configuration, which determines default properties for row
    /// [headers](https://vega.github.io/vega-lite/docs/header.html).
    ///
    /// For a full list of header configuration options, please see the [corresponding section of
    /// in the header documentation](https://vega.github.io/vega-lite/docs/header.html#config).
    #[serde(rename = "headerRow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub header_row: Option<HeaderConfig>,
    /// Image-specific Config
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub image: Option<RectConfig>,
    /// Legend configuration, which determines default properties for all
    /// [legends](https://vega.github.io/vega-lite/docs/legend.html). For a full list of legend
    /// configuration options, please see the [corresponding section of in the legend
    /// documentation](https://vega.github.io/vega-lite/docs/legend.html#config).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub legend: Option<LegendConfig>,
    /// Line-Specific Config
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub line: Option<LineConfig>,
    /// Mark Config
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub mark: Option<MarkConfig>,
    /// D3 Number format for guide labels and text marks. For example "s" for SI units. Use [D3's
    /// number format pattern](https://github.com/d3/d3-format#locale_format).
    #[serde(rename = "numberFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub number_format: Option<String>,
    /// The default visualization padding, in pixels, from the edge of the visualization canvas
    /// to the data rectangle. If a number, specifies padding for all sides.
    /// If an object, the value should have the format `{"left": 5, "top": 5, "right": 5,
    /// "bottom": 5}` to specify padding for each side of the visualization.
    ///
    /// __Default value__: `5`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub padding: Option<Padding>,
    /// Point-Specific Config
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub point: Option<MarkConfig>,
    /// Projection configuration, which determines default properties for all
    /// [projections](https://vega.github.io/vega-lite/docs/projection.html). For a full list of
    /// projection configuration options, please see the [corresponding section of the projection
    /// documentation](https://vega.github.io/vega-lite/docs/projection.html#config).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub projection: Option<Projection>,
    /// An object hash that defines default range arrays or schemes for using with scales.
    /// For a full list of scale range configuration options, please see the [corresponding
    /// section of the scale
    /// documentation](https://vega.github.io/vega-lite/docs/scale.html#config).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub range: Option<RangeConfig>,
    /// Rect-Specific Config
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub rect: Option<RectConfig>,
    /// Default configuration for the `repeat` view composition operator
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub repeat: Option<CompositionConfig>,
    /// Rule-Specific Config
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub rule: Option<MarkConfig>,
    /// Scale configuration determines default properties for all
    /// [scales](https://vega.github.io/vega-lite/docs/scale.html). For a full list of scale
    /// configuration options, please see the [corresponding section of the scale
    /// documentation](https://vega.github.io/vega-lite/docs/scale.html#config).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub scale: Option<ScaleConfig>,
    /// An object hash for defining default properties for each type of selections.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub selection: Option<SelectionConfig>,
    /// Square-Specific Config
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub square: Option<MarkConfig>,
    /// An object hash that defines key-value mappings to determine default properties for marks
    /// with a given [style](https://vega.github.io/vega-lite/docs/mark.html#mark-def). The keys
    /// represent styles names; the values have to be valid [mark configuration
    /// objects](https://vega.github.io/vega-lite/docs/mark.html#config).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub style: Option<HashMap<String, BaseMarkConfig>>,
    /// Text-Specific Config
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub text: Option<MarkConfig>,
    /// Tick-Specific Config
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tick: Option<TickConfig>,
    /// Default time format for raw time values (without time units) in text marks, legend labels
    /// and header labels.
    ///
    /// __Default value:__ `"%b %d, %Y"`
    /// __Note:__ Axes automatically determine format each label automatically so this config
    /// would not affect axes.
    #[serde(rename = "timeFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_format: Option<String>,
    /// Title configuration, which determines default properties for all
    /// [titles](https://vega.github.io/vega-lite/docs/title.html). For a full list of title
    /// configuration options, please see the [corresponding section of the title
    /// documentation](https://vega.github.io/vega-lite/docs/title.html#config).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title: Option<ExcludeMappedValueRefBaseTitle>,
    /// Trail-Specific Config
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub trail: Option<LineConfig>,
    /// Default properties for [single view
    /// plots](https://vega.github.io/vega-lite/docs/spec.html#single).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub view: Option<ViewConfig>,
}

/// Area-Specific Config
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct AreaConfig {
    /// The horizontal alignment of the text or ranged marks (area, bar, image, rect, rule). One
    /// of `"left"`, `"right"`, `"center"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub align: Option<Align>,
    /// The rotation angle of the text, in degrees.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub angle: Option<f64>,
    /// Whether to keep aspect ratio of image marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aspect: Option<bool>,
    /// The vertical alignment of the text or ranged marks (area, bar, image, rect, rule). One of
    /// `"top"`, `"middle"`, `"bottom"`.
    ///
    /// __Default value:__ `"middle"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub baseline: Option<Baseline>,
    /// Default color.
    ///
    /// __Default value:__ <span style="color: #4682b4;">&#9632;</span> `"#4682b4"`
    ///
    /// __Note:__
    /// - This property cannot be used in a [style
    /// config](https://vega.github.io/vega-lite/docs/mark.html#style-config).
    /// - The `fill` and `stroke` properties have higher precedence than `color` and will
    /// override `color`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub color: Option<ColorUnion>,
    /// The radius in pixels of rounded rectangle corners.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadius")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius: Option<f64>,
    /// The radius in pixels of rounded rectangle bottom left corner.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadiusBottomLeft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius_bottom_left: Option<f64>,
    /// The radius in pixels of rounded rectangle bottom right corner.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadiusBottomRight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius_bottom_right: Option<f64>,
    /// The radius in pixels of rounded rectangle top right corner.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadiusTopLeft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius_top_left: Option<f64>,
    /// The radius in pixels of rounded rectangle top left corner.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadiusTopRight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius_top_right: Option<f64>,
    /// The mouse cursor used over the mark. Any valid [CSS cursor
    /// type](https://developer.mozilla.org/en-US/docs/Web/CSS/cursor#Values) can be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub cursor: Option<Cursor>,
    /// The direction of the text. One of `"ltr"` (left-to-right) or `"rtl"` (right-to-left).
    /// This property determines on which side is truncated in response to the limit parameter.
    ///
    /// __Default value:__ `"ltr"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dir: Option<Dir>,
    /// The horizontal offset, in pixels, between the text label and its anchor point. The offset
    /// is applied after rotation by the _angle_ property.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dx: Option<f64>,
    /// The vertical offset, in pixels, between the text label and its anchor point. The offset
    /// is applied after rotation by the _angle_ property.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dy: Option<f64>,
    /// The ellipsis string for text truncated in response to the limit parameter.
    ///
    /// __Default value:__ `"…"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ellipsis: Option<String>,
    /// Default Fill Color. This has higher precedence than `config.color`.
    ///
    /// __Default value:__ (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fill: Option<FillUnion>,
    /// Whether the mark's color should be used as fill color instead of stroke color.
    ///
    /// __Default value:__ `false` for all `point`, `line`, and `rule` marks as well as
    /// `geoshape` marks for
    /// [`graticule`](https://vega.github.io/vega-lite/docs/data.html#graticule) data sources;
    /// otherwise, `true`.
    ///
    /// __Note:__ This property cannot be used in a [style
    /// config](https://vega.github.io/vega-lite/docs/mark.html#style-config).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub filled: Option<bool>,
    /// The fill opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "fillOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fill_opacity: Option<f64>,
    /// The typeface to set the text in (e.g., `"Helvetica Neue"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font: Option<String>,
    /// The font size, in pixels.
    #[serde(rename = "fontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font_size: Option<f64>,
    /// The font style (e.g., `"italic"`).
    #[serde(rename = "fontStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font_style: Option<String>,
    /// The font weight.
    /// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
    /// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
    #[serde(rename = "fontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font_weight: Option<FontWeight>,
    /// Height of the marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub height: Option<f64>,
    /// A URL to load upon mouse click. If defined, the mark acts as a hyperlink.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub href: Option<String>,
    /// The line interpolation method to use for line and area marks. One of the following:
    /// - `"linear"`: piecewise linear segments, as in a polyline.
    /// - `"linear-closed"`: close the linear segments to form a polygon.
    /// - `"step"`: alternate between horizontal and vertical segments, as in a step function.
    /// - `"step-before"`: alternate between vertical and horizontal segments, as in a step
    /// function.
    /// - `"step-after"`: alternate between horizontal and vertical segments, as in a step
    /// function.
    /// - `"basis"`: a B-spline, with control point duplication on the ends.
    /// - `"basis-open"`: an open B-spline; may not intersect the start or end.
    /// - `"basis-closed"`: a closed B-spline, as in a loop.
    /// - `"cardinal"`: a Cardinal spline, with control point duplication on the ends.
    /// - `"cardinal-open"`: an open Cardinal spline; may not intersect the start or end, but
    /// will intersect other control points.
    /// - `"cardinal-closed"`: a closed Cardinal spline, as in a loop.
    /// - `"bundle"`: equivalent to basis, except the tension parameter is used to straighten the
    /// spline.
    /// - `"monotone"`: cubic interpolation that preserves monotonicity in y.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub interpolate: Option<Interpolate>,
    /// Defines how Vega-Lite should handle marks for invalid values (`null` and `NaN`).
    /// - If set to `"filter"` (default), all data items with null values will be skipped (for
    /// line, trail, and area marks) or filtered (for other marks).
    /// - If `null`, all data items are included. In this case, invalid values will be
    /// interpreted as zeroes.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub invalid: RemovableValue<Invalid>,
    /// The maximum length of the text mark in pixels. The text value will be automatically
    /// truncated if the rendered size exceeds the limit.
    ///
    /// __Default value:__ `0`, indicating no limit
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub limit: Option<f64>,
    /// A flag for overlaying line on top of area marks, or an object defining the properties of
    /// the overlayed lines.
    ///
    /// - If this value is an empty object (`{}`) or `true`, lines with default properties will
    /// be used.
    ///
    /// - If this value is `false`, no lines would be automatically added to area marks.
    ///
    /// __Default value:__ `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub line: Option<Line>,
    /// A delimiter, such as a newline character, upon which to break text strings into multiple
    /// lines. This property will be ignored if the text property is array-valued.
    #[serde(rename = "lineBreak")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub line_break: Option<String>,
    /// The height, in pixels, of each line of text in a multi-line text mark.
    #[serde(rename = "lineHeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub line_height: Option<f64>,
    /// The overall opacity (value between [0,1]).
    ///
    /// __Default value:__ `0.7` for non-aggregate plots with `point`, `tick`, `circle`, or
    /// `square` marks or layered `bar` charts and `1` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub opacity: Option<f64>,
    /// For line and trail marks, this `order` property can be set to `null` or `false` to make
    /// the lines use the original order in the data sources.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub order: Option<bool>,
    /// The orientation of a non-stacked bar, tick, area, and line charts.
    /// The value is either horizontal (default) or vertical.
    /// - For bar, rule and tick, this determines whether the size of the bar and tick
    /// should be applied to x or y dimension.
    /// - For area, this property determines the orient property of the Vega output.
    /// - For line and trail marks, this property determines the sort order of the points in the
    /// line
    /// if `config.sortLineBy` is not specified.
    /// For stacked charts, this is always determined by the orientation of the stack;
    /// therefore explicitly specified value will be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub orient: Option<Orientation>,
    /// A flag for overlaying points on top of line or area marks, or an object defining the
    /// properties of the overlayed points.
    ///
    /// - If this property is `"transparent"`, transparent points will be used (for enhancing
    /// tooltips and selections).
    ///
    /// - If this property is an empty object (`{}`) or `true`, filled points with default
    /// properties will be used.
    ///
    /// - If this property is `false`, no points would be automatically added to line or area
    /// marks.
    ///
    /// __Default value:__ `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub point: Option<PointUnion>,
    /// Polar coordinate radial offset, in pixels, of the text label from the origin determined
    /// by the `x` and `y` properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub radius: Option<f64>,
    /// Shape of the point marks. Supported values include:
    /// - plotting shapes: `"circle"`, `"square"`, `"cross"`, `"diamond"`, `"triangle-up"`,
    /// `"triangle-down"`, `"triangle-right"`, or `"triangle-left"`.
    /// - the line symbol `"stroke"`
    /// - centered directional shapes `"arrow"`, `"wedge"`, or `"triangle"`
    /// - a custom [SVG path
    /// string](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths) (For correct
    /// sizing, custom shape paths should be defined within a square bounding box with
    /// coordinates ranging from -1 to 1 along both the x and y dimensions.)
    ///
    /// __Default value:__ `"circle"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub shape: Option<String>,
    /// Default size for marks.
    /// - For `point`/`circle`/`square`, this represents the pixel area of the marks. For
    /// example: in the case of circles, the radius is determined in part by the square root of
    /// the size value.
    /// - For `bar`, this represents the band size of the bar, in pixels.
    /// - For `text`, this represents the font size, in pixels.
    ///
    /// __Default value:__
    /// - `30` for point, circle, square marks; width/height's `step`
    /// - `2` for bar marks with discrete dimensions;
    /// - `5` for bar marks with continuous dimensions;
    /// - `11` for text marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub size: Option<f64>,
    /// Default Stroke Color. This has higher precedence than `config.color`.
    ///
    /// __Default value:__ (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke: Option<FillUnion>,
    /// The stroke cap for line ending style. One of `"butt"`, `"round"`, or `"square"`.
    ///
    /// __Default value:__ `"square"`
    #[serde(rename = "strokeCap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_cap: Option<StrokeCap>,
    /// An array of alternating stroke, space lengths for creating dashed or dotted lines.
    #[serde(rename = "strokeDash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_dash: Option<Vec<f64>>,
    /// The offset (in pixels) into which to begin drawing with the stroke dash array.
    #[serde(rename = "strokeDashOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_dash_offset: Option<f64>,
    /// The stroke line join method. One of `"miter"`, `"round"` or `"bevel"`.
    ///
    /// __Default value:__ `"miter"`
    #[serde(rename = "strokeJoin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_join: Option<StrokeJoin>,
    /// The miter limit at which to bevel a line join.
    #[serde(rename = "strokeMiterLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_miter_limit: Option<f64>,
    /// The stroke opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "strokeOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_opacity: Option<f64>,
    /// The stroke width, in pixels.
    #[serde(rename = "strokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_width: Option<f64>,
    /// Depending on the interpolation type, sets the tension parameter (for line and area marks).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tension: Option<f64>,
    /// Placeholder text if the `text` channel is not specified
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub text: Option<PurpleText>,
    /// Polar coordinate angle, in radians, of the text label from the origin determined by the
    /// `x` and `y` properties. Values for `theta` follow the same convention of `arc` mark
    /// `startAngle` and `endAngle` properties: angles are measured in radians, with `0`
    /// indicating "north".
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub theta: Option<f64>,
    /// Default relative band size for a time unit. If set to `1`, the bandwidth of the marks
    /// will be equal to the time unit band step.
    /// If set to `0.5`, bandwidth of the marks will be half of the time unit band step.
    #[serde(rename = "timeUnitBand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit_band: Option<f64>,
    /// Default relative band position for a time unit. If set to `0`, the marks will be
    /// positioned at the beginning of the time unit band step.
    /// If set to `0.5`, the marks will be positioned in the middle of the time unit band step.
    #[serde(rename = "timeUnitBandPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit_band_position: Option<f64>,
    /// The tooltip text string to show upon mouse hover or an object defining which fields
    /// should the tooltip be derived from.
    ///
    /// - If `tooltip` is `true` or `{"content": "encoding"}`, then all fields from `encoding`
    /// will be used.
    /// - If `tooltip` is `{"content": "data"}`, then all fields that appear in the highlighted
    /// data point will be used.
    /// - If set to `null` or `false`, then no tooltip will be used.
    ///
    /// See the [`tooltip`](https://vega.github.io/vega-lite/docs/tooltip.html) documentation for
    /// a detailed discussion about tooltip  in Vega-Lite.
    ///
    /// __Default value:__ `null`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tooltip: Option<Value>,
    /// Width of the marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub width: Option<f64>,
    /// X coordinates of the marks, or width of horizontal `"bar"` and `"area"` without specified
    /// `x2` or `width`.
    ///
    /// The `value` of this channel can be a number or a string `"width"` for the width of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub x: Option<XUnion>,
    /// X2 coordinates for ranged `"area"`, `"bar"`, `"rect"`, and  `"rule"`.
    ///
    /// The `value` of this channel can be a number or a string `"width"` for the width of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub x2: Option<XUnion>,
    /// Y coordinates of the marks, or height of vertical `"bar"` and `"area"` without specified
    /// `y2` or `height`.
    ///
    /// The `value` of this channel can be a number or a string `"height"` for the height of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub y: Option<YUnion>,
    /// Y2 coordinates for ranged `"area"`, `"bar"`, `"rect"`, and  `"rule"`.
    ///
    /// The `value` of this channel can be a number or a string `"height"` for the height of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub y2: Option<YUnion>,
}

/// Axis configuration, which determines default properties for all `x` and `y`
/// [axes](https://vega.github.io/vega-lite/docs/axis.html). For a full list of axis
/// configuration options, please see the [corresponding section of the axis
/// documentation](https://vega.github.io/vega-lite/docs/axis.html#config).
///
/// Specific axis config for axes with "band" scales.
///
/// Specific axis config for x-axis along the bottom edge of the chart.
///
/// Specific axis config for y-axis along the left edge of the chart.
///
/// Specific axis config for y-axis along the right edge of the chart.
///
/// Specific axis config for x-axis along the top edge of the chart.
///
/// X-axis specific config.
///
/// Y-axis specific config.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct AxisConfig {
    /// An interpolation fraction indicating where, for `band` scales, axis ticks should be
    /// positioned. A value of `0` places ticks at the left edge of their bands. A value of `0.5`
    /// places ticks in the middle of their bands.
    ///
    /// __Default value:__ `0.5`
    #[serde(rename = "bandPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub band_position: Option<f64>,
    /// A boolean flag indicating if the domain (the axis baseline) should be included as part of
    /// the axis.
    ///
    /// __Default value:__ `true`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub domain: Option<bool>,
    /// Color of axis domain line.
    ///
    /// __Default value:__ `"gray"`.
    #[serde(rename = "domainColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub domain_color: Option<String>,
    /// An array of alternating [stroke, space] lengths for dashed domain lines.
    #[serde(rename = "domainDash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub domain_dash: Option<Vec<f64>>,
    /// The pixel offset at which to start drawing with the domain dash array.
    #[serde(rename = "domainDashOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub domain_dash_offset: Option<f64>,
    /// Opacity of the axis domain line.
    #[serde(rename = "domainOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub domain_opacity: Option<f64>,
    /// Stroke width of axis domain line
    ///
    /// __Default value:__ `1`
    #[serde(rename = "domainWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub domain_width: Option<f64>,
    /// A boolean flag indicating if grid lines should be included as part of the axis
    ///
    /// __Default value:__ `true` for [continuous
    /// scales](https://vega.github.io/vega-lite/docs/scale.html#continuous) that are not binned;
    /// otherwise, `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub grid: Option<bool>,
    #[serde(rename = "gridColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub grid_color: Option<Color>,
    #[serde(rename = "gridDash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub grid_dash: Option<Dash>,
    #[serde(rename = "gridDashOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub grid_dash_offset: Option<GridDashOffset>,
    #[serde(rename = "gridOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub grid_opacity: Option<GridOpacity>,
    #[serde(rename = "gridWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub grid_width: Option<GridWidth>,
    #[serde(rename = "labelAlign")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_align: Option<LabelAlign>,
    /// The rotation angle of the axis labels.
    ///
    /// __Default value:__ `-90` for nominal and ordinal fields; `0` otherwise.
    #[serde(rename = "labelAngle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_angle: Option<f64>,
    #[serde(rename = "labelBaseline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_baseline: Option<TextBaseline>,
    /// Indicates if labels should be hidden if they exceed the axis range. If `false` (the
    /// default) no bounds overlap analysis is performed. If `true`, labels will be hidden if
    /// they exceed the axis range by more than 1 pixel. If this property is a number, it
    /// specifies the pixel tolerance: the maximum amount by which a label bounding box may
    /// exceed the axis range.
    ///
    /// __Default value:__ `false`.
    #[serde(rename = "labelBound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_bound: Option<Label>,
    #[serde(rename = "labelColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_color: Option<Color>,
    /// Indicates if the first and last axis labels should be aligned flush with the scale range.
    /// Flush alignment for a horizontal axis will left-align the first label and right-align the
    /// last label. For vertical axes, bottom and top text baselines are applied instead. If this
    /// property is a number, it also indicates the number of pixels by which to offset the first
    /// and last labels; for example, a value of 2 will flush-align the first and last labels and
    /// also push them 2 pixels outward from the center of the axis. The additional adjustment
    /// can sometimes help the labels better visually group with corresponding axis ticks.
    ///
    /// __Default value:__ `true` for axis of a continuous x-scale. Otherwise, `false`.
    #[serde(rename = "labelFlush")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_flush: Option<Label>,
    /// Indicates the number of pixels by which to offset flush-adjusted labels. For example, a
    /// value of `2` will push flush-adjusted labels 2 pixels outward from the center of the
    /// axis. Offsets can help the labels better visually group with corresponding axis ticks.
    ///
    /// __Default value:__ `0`.
    #[serde(rename = "labelFlushOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_flush_offset: Option<f64>,
    #[serde(rename = "labelFont")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_font: Option<LabelFont>,
    #[serde(rename = "labelFontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_font_size: Option<GridWidth>,
    #[serde(rename = "labelFontStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_font_style: Option<LabelFontStyle>,
    #[serde(rename = "labelFontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_font_weight: Option<LabelFontWeightUnion>,
    /// Maximum allowed pixel width of axis tick labels.
    ///
    /// __Default value:__ `180`
    #[serde(rename = "labelLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_limit: Option<f64>,
    #[serde(rename = "labelOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_opacity: Option<GridDashOffset>,
    /// The strategy to use for resolving overlap of axis labels. If `false` (the default), no
    /// overlap reduction is attempted. If set to `true` or `"parity"`, a strategy of removing
    /// every other label is used (this works well for standard linear axes). If set to
    /// `"greedy"`, a linear scan of the labels is performed, removing any labels that overlaps
    /// with the last visible label (this often works better for log-scaled axes).
    ///
    /// __Default value:__ `true` for non-nominal fields with non-log scales; `"greedy"` for log
    /// scales; otherwise `false`.
    #[serde(rename = "labelOverlap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_overlap: Option<LabelOverlap>,
    /// The padding, in pixels, between axis and text labels.
    ///
    /// __Default value:__ `2`
    #[serde(rename = "labelPadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_padding: Option<f64>,
    /// A boolean flag indicating if labels should be included as part of the axis.
    ///
    /// __Default value:__ `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub labels: Option<bool>,
    /// The minimum separation that must be between label bounding boxes for them to be
    /// considered non-overlapping (default `0`). This property is ignored if *labelOverlap*
    /// resolution is not enabled.
    #[serde(rename = "labelSeparation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_separation: Option<f64>,
    /// The maximum extent in pixels that axis ticks and labels should use. This determines a
    /// maximum offset value for axis titles.
    ///
    /// __Default value:__ `undefined`.
    #[serde(rename = "maxExtent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub max_extent: Option<f64>,
    /// The minimum extent in pixels that axis ticks and labels should use. This determines a
    /// minimum offset value for axis titles.
    ///
    /// __Default value:__ `30` for y-axis; `undefined` for x-axis.
    #[serde(rename = "minExtent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub min_extent: Option<f64>,
    /// The orientation of the axis. One of `"top"`, `"bottom"`, `"left"` or `"right"`. The
    /// orientation can be used to further specialize the axis type (e.g., a y-axis oriented
    /// towards the right edge of the chart).
    ///
    /// __Default value:__ `"bottom"` for x-axes and `"left"` for y-axes.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub orient: Option<Orient>,
    /// For band scales, indicates if ticks and grid lines should be placed at the center of a
    /// band (default) or at the band extents to indicate intervals.
    #[serde(rename = "tickBand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tick_band: Option<TickBand>,
    #[serde(rename = "tickColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tick_color: Option<Color>,
    #[serde(rename = "tickDash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tick_dash: Option<Dash>,
    #[serde(rename = "tickDashOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tick_dash_offset: Option<GridDashOffset>,
    /// Boolean flag indicating if an extra axis tick should be added for the initial position of
    /// the axis. This flag is useful for styling axes for `band` scales such that ticks are
    /// placed on band boundaries rather in the middle of a band. Use in conjunction with
    /// `"bandPosition": 1` and an axis `"padding"` value of `0`.
    #[serde(rename = "tickExtra")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tick_extra: Option<bool>,
    /// Position offset in pixels to apply to ticks, labels, and gridlines.
    #[serde(rename = "tickOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tick_offset: Option<f64>,
    #[serde(rename = "tickOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tick_opacity: Option<GridDashOffset>,
    /// Boolean flag indicating if pixel position values should be rounded to the nearest
    /// integer.
    ///
    /// __Default value:__ `true`
    #[serde(rename = "tickRound")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tick_round: Option<bool>,
    /// Boolean value that determines whether the axis should include ticks.
    ///
    /// __Default value:__ `true`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ticks: Option<bool>,
    /// The size in pixels of axis ticks.
    ///
    /// __Default value:__ `5`
    #[serde(rename = "tickSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tick_size: Option<f64>,
    #[serde(rename = "tickWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tick_width: Option<GridWidth>,
    /// Set to null to disable title for the axis, legend, or header.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub title: RemovableValue<serde_json::Value>,
    /// Horizontal text alignment of axis titles.
    #[serde(rename = "titleAlign")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_align: Option<Align>,
    /// Text anchor position for placing axis titles.
    #[serde(rename = "titleAnchor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_anchor: Option<TitleAnchorEnum>,
    /// Angle in degrees of axis titles.
    #[serde(rename = "titleAngle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_angle: Option<f64>,
    /// Vertical text baseline for axis titles.
    #[serde(rename = "titleBaseline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_baseline: Option<Baseline>,
    /// Color of the title, can be in hex color code or regular color name.
    #[serde(rename = "titleColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_color: Option<String>,
    /// Font of the title. (e.g., `"Helvetica Neue"`).
    #[serde(rename = "titleFont")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_font: Option<String>,
    /// Font size of the title.
    #[serde(rename = "titleFontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_font_size: Option<f64>,
    /// Font style of the title.
    #[serde(rename = "titleFontStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_font_style: Option<String>,
    /// Font weight of the title.
    /// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
    /// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
    #[serde(rename = "titleFontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_font_weight: Option<FontWeight>,
    /// Maximum allowed pixel width of axis titles.
    #[serde(rename = "titleLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_limit: Option<f64>,
    /// Line height in pixels for multi-line title text.
    #[serde(rename = "titleLineHeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_line_height: Option<f64>,
    /// Opacity of the axis title.
    #[serde(rename = "titleOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_opacity: Option<f64>,
    /// The padding, in pixels, between title and axis.
    #[serde(rename = "titlePadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_padding: Option<f64>,
    /// X-coordinate of the axis title relative to the axis group.
    #[serde(rename = "titleX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_x: Option<f64>,
    /// Y-coordinate of the axis title relative to the axis group.
    #[serde(rename = "titleY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_y: Option<f64>,
    /// Translation offset in pixels applied to the axis group mark x and y. If specified,
    /// overrides the default behavior of a 0.5 offset to pixel-align stroked lines.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub translate: Option<f64>,
}

/// Bar-Specific Config
///
/// Image-specific Config
///
/// Rect-Specific Config
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct RectConfig {
    /// The horizontal alignment of the text or ranged marks (area, bar, image, rect, rule). One
    /// of `"left"`, `"right"`, `"center"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub align: Option<Align>,
    /// The rotation angle of the text, in degrees.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub angle: Option<f64>,
    /// Whether to keep aspect ratio of image marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aspect: Option<bool>,
    /// The vertical alignment of the text or ranged marks (area, bar, image, rect, rule). One of
    /// `"top"`, `"middle"`, `"bottom"`.
    ///
    /// __Default value:__ `"middle"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub baseline: Option<Baseline>,
    /// Offset between bars for binned field. The ideal value for this is either 0 (preferred by
    /// statisticians) or 1 (Vega-Lite default, D3 example style).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "binSpacing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bin_spacing: Option<f64>,
    /// Default color.
    ///
    /// __Default value:__ <span style="color: #4682b4;">&#9632;</span> `"#4682b4"`
    ///
    /// __Note:__
    /// - This property cannot be used in a [style
    /// config](https://vega.github.io/vega-lite/docs/mark.html#style-config).
    /// - The `fill` and `stroke` properties have higher precedence than `color` and will
    /// override `color`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub color: Option<ColorUnion>,
    /// The default size of the bars on continuous scales.
    ///
    /// __Default value:__ `5`
    #[serde(rename = "continuousBandSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub continuous_band_size: Option<f64>,
    /// The radius in pixels of rounded rectangle corners.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadius")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius: Option<f64>,
    /// The radius in pixels of rounded rectangle bottom left corner.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadiusBottomLeft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius_bottom_left: Option<f64>,
    /// The radius in pixels of rounded rectangle bottom right corner.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadiusBottomRight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius_bottom_right: Option<f64>,
    /// The radius in pixels of rounded rectangle top right corner.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadiusTopLeft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius_top_left: Option<f64>,
    /// The radius in pixels of rounded rectangle top left corner.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadiusTopRight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius_top_right: Option<f64>,
    /// The mouse cursor used over the mark. Any valid [CSS cursor
    /// type](https://developer.mozilla.org/en-US/docs/Web/CSS/cursor#Values) can be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub cursor: Option<Cursor>,
    /// The direction of the text. One of `"ltr"` (left-to-right) or `"rtl"` (right-to-left).
    /// This property determines on which side is truncated in response to the limit parameter.
    ///
    /// __Default value:__ `"ltr"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dir: Option<Dir>,
    /// The default size of the bars with discrete dimensions. If unspecified, the default size
    /// is  `step-2`, which provides 2 pixel offset between bars.
    #[serde(rename = "discreteBandSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub discrete_band_size: Option<f64>,
    /// The horizontal offset, in pixels, between the text label and its anchor point. The offset
    /// is applied after rotation by the _angle_ property.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dx: Option<f64>,
    /// The vertical offset, in pixels, between the text label and its anchor point. The offset
    /// is applied after rotation by the _angle_ property.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dy: Option<f64>,
    /// The ellipsis string for text truncated in response to the limit parameter.
    ///
    /// __Default value:__ `"…"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ellipsis: Option<String>,
    /// Default Fill Color. This has higher precedence than `config.color`.
    ///
    /// __Default value:__ (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fill: Option<FillUnion>,
    /// Whether the mark's color should be used as fill color instead of stroke color.
    ///
    /// __Default value:__ `false` for all `point`, `line`, and `rule` marks as well as
    /// `geoshape` marks for
    /// [`graticule`](https://vega.github.io/vega-lite/docs/data.html#graticule) data sources;
    /// otherwise, `true`.
    ///
    /// __Note:__ This property cannot be used in a [style
    /// config](https://vega.github.io/vega-lite/docs/mark.html#style-config).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub filled: Option<bool>,
    /// The fill opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "fillOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fill_opacity: Option<f64>,
    /// The typeface to set the text in (e.g., `"Helvetica Neue"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font: Option<String>,
    /// The font size, in pixels.
    #[serde(rename = "fontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font_size: Option<f64>,
    /// The font style (e.g., `"italic"`).
    #[serde(rename = "fontStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font_style: Option<String>,
    /// The font weight.
    /// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
    /// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
    #[serde(rename = "fontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font_weight: Option<FontWeight>,
    /// Height of the marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub height: Option<f64>,
    /// A URL to load upon mouse click. If defined, the mark acts as a hyperlink.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub href: Option<String>,
    /// The line interpolation method to use for line and area marks. One of the following:
    /// - `"linear"`: piecewise linear segments, as in a polyline.
    /// - `"linear-closed"`: close the linear segments to form a polygon.
    /// - `"step"`: alternate between horizontal and vertical segments, as in a step function.
    /// - `"step-before"`: alternate between vertical and horizontal segments, as in a step
    /// function.
    /// - `"step-after"`: alternate between horizontal and vertical segments, as in a step
    /// function.
    /// - `"basis"`: a B-spline, with control point duplication on the ends.
    /// - `"basis-open"`: an open B-spline; may not intersect the start or end.
    /// - `"basis-closed"`: a closed B-spline, as in a loop.
    /// - `"cardinal"`: a Cardinal spline, with control point duplication on the ends.
    /// - `"cardinal-open"`: an open Cardinal spline; may not intersect the start or end, but
    /// will intersect other control points.
    /// - `"cardinal-closed"`: a closed Cardinal spline, as in a loop.
    /// - `"bundle"`: equivalent to basis, except the tension parameter is used to straighten the
    /// spline.
    /// - `"monotone"`: cubic interpolation that preserves monotonicity in y.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub interpolate: Option<Interpolate>,
    /// Defines how Vega-Lite should handle marks for invalid values (`null` and `NaN`).
    /// - If set to `"filter"` (default), all data items with null values will be skipped (for
    /// line, trail, and area marks) or filtered (for other marks).
    /// - If `null`, all data items are included. In this case, invalid values will be
    /// interpreted as zeroes.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub invalid: RemovableValue<Invalid>,
    /// The maximum length of the text mark in pixels. The text value will be automatically
    /// truncated if the rendered size exceeds the limit.
    ///
    /// __Default value:__ `0`, indicating no limit
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub limit: Option<f64>,
    /// A delimiter, such as a newline character, upon which to break text strings into multiple
    /// lines. This property will be ignored if the text property is array-valued.
    #[serde(rename = "lineBreak")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub line_break: Option<String>,
    /// The height, in pixels, of each line of text in a multi-line text mark.
    #[serde(rename = "lineHeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub line_height: Option<f64>,
    /// The overall opacity (value between [0,1]).
    ///
    /// __Default value:__ `0.7` for non-aggregate plots with `point`, `tick`, `circle`, or
    /// `square` marks or layered `bar` charts and `1` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub opacity: Option<f64>,
    /// For line and trail marks, this `order` property can be set to `null` or `false` to make
    /// the lines use the original order in the data sources.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub order: Option<bool>,
    /// The orientation of a non-stacked bar, tick, area, and line charts.
    /// The value is either horizontal (default) or vertical.
    /// - For bar, rule and tick, this determines whether the size of the bar and tick
    /// should be applied to x or y dimension.
    /// - For area, this property determines the orient property of the Vega output.
    /// - For line and trail marks, this property determines the sort order of the points in the
    /// line
    /// if `config.sortLineBy` is not specified.
    /// For stacked charts, this is always determined by the orientation of the stack;
    /// therefore explicitly specified value will be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub orient: Option<Orientation>,
    /// Polar coordinate radial offset, in pixels, of the text label from the origin determined
    /// by the `x` and `y` properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub radius: Option<f64>,
    /// Shape of the point marks. Supported values include:
    /// - plotting shapes: `"circle"`, `"square"`, `"cross"`, `"diamond"`, `"triangle-up"`,
    /// `"triangle-down"`, `"triangle-right"`, or `"triangle-left"`.
    /// - the line symbol `"stroke"`
    /// - centered directional shapes `"arrow"`, `"wedge"`, or `"triangle"`
    /// - a custom [SVG path
    /// string](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths) (For correct
    /// sizing, custom shape paths should be defined within a square bounding box with
    /// coordinates ranging from -1 to 1 along both the x and y dimensions.)
    ///
    /// __Default value:__ `"circle"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub shape: Option<String>,
    /// Default size for marks.
    /// - For `point`/`circle`/`square`, this represents the pixel area of the marks. For
    /// example: in the case of circles, the radius is determined in part by the square root of
    /// the size value.
    /// - For `bar`, this represents the band size of the bar, in pixels.
    /// - For `text`, this represents the font size, in pixels.
    ///
    /// __Default value:__
    /// - `30` for point, circle, square marks; width/height's `step`
    /// - `2` for bar marks with discrete dimensions;
    /// - `5` for bar marks with continuous dimensions;
    /// - `11` for text marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub size: Option<f64>,
    /// Default Stroke Color. This has higher precedence than `config.color`.
    ///
    /// __Default value:__ (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke: Option<FillUnion>,
    /// The stroke cap for line ending style. One of `"butt"`, `"round"`, or `"square"`.
    ///
    /// __Default value:__ `"square"`
    #[serde(rename = "strokeCap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_cap: Option<StrokeCap>,
    /// An array of alternating stroke, space lengths for creating dashed or dotted lines.
    #[serde(rename = "strokeDash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_dash: Option<Vec<f64>>,
    /// The offset (in pixels) into which to begin drawing with the stroke dash array.
    #[serde(rename = "strokeDashOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_dash_offset: Option<f64>,
    /// The stroke line join method. One of `"miter"`, `"round"` or `"bevel"`.
    ///
    /// __Default value:__ `"miter"`
    #[serde(rename = "strokeJoin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_join: Option<StrokeJoin>,
    /// The miter limit at which to bevel a line join.
    #[serde(rename = "strokeMiterLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_miter_limit: Option<f64>,
    /// The stroke opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "strokeOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_opacity: Option<f64>,
    /// The stroke width, in pixels.
    #[serde(rename = "strokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_width: Option<f64>,
    /// Depending on the interpolation type, sets the tension parameter (for line and area marks).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tension: Option<f64>,
    /// Placeholder text if the `text` channel is not specified
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub text: Option<PurpleText>,
    /// Polar coordinate angle, in radians, of the text label from the origin determined by the
    /// `x` and `y` properties. Values for `theta` follow the same convention of `arc` mark
    /// `startAngle` and `endAngle` properties: angles are measured in radians, with `0`
    /// indicating "north".
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub theta: Option<f64>,
    /// Default relative band size for a time unit. If set to `1`, the bandwidth of the marks
    /// will be equal to the time unit band step.
    /// If set to `0.5`, bandwidth of the marks will be half of the time unit band step.
    #[serde(rename = "timeUnitBand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit_band: Option<f64>,
    /// Default relative band position for a time unit. If set to `0`, the marks will be
    /// positioned at the beginning of the time unit band step.
    /// If set to `0.5`, the marks will be positioned in the middle of the time unit band step.
    #[serde(rename = "timeUnitBandPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit_band_position: Option<f64>,
    /// The tooltip text string to show upon mouse hover or an object defining which fields
    /// should the tooltip be derived from.
    ///
    /// - If `tooltip` is `true` or `{"content": "encoding"}`, then all fields from `encoding`
    /// will be used.
    /// - If `tooltip` is `{"content": "data"}`, then all fields that appear in the highlighted
    /// data point will be used.
    /// - If set to `null` or `false`, then no tooltip will be used.
    ///
    /// See the [`tooltip`](https://vega.github.io/vega-lite/docs/tooltip.html) documentation for
    /// a detailed discussion about tooltip  in Vega-Lite.
    ///
    /// __Default value:__ `null`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tooltip: Option<Value>,
    /// Width of the marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub width: Option<f64>,
    /// X coordinates of the marks, or width of horizontal `"bar"` and `"area"` without specified
    /// `x2` or `width`.
    ///
    /// The `value` of this channel can be a number or a string `"width"` for the width of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub x: Option<XUnion>,
    /// X2 coordinates for ranged `"area"`, `"bar"`, `"rect"`, and  `"rule"`.
    ///
    /// The `value` of this channel can be a number or a string `"width"` for the width of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub x2: Option<XUnion>,
    /// Y coordinates of the marks, or height of vertical `"bar"` and `"area"` without specified
    /// `y2` or `height`.
    ///
    /// The `value` of this channel can be a number or a string `"height"` for the height of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub y: Option<YUnion>,
    /// Y2 coordinates for ranged `"area"`, `"bar"`, `"rect"`, and  `"rule"`.
    ///
    /// The `value` of this channel can be a number or a string `"height"` for the height of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub y2: Option<YUnion>,
}

/// Box Config
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct BoxPlotConfig {
    #[serde(rename = "box")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub box_plot_config_box: Option<DefBox>,
    /// The extent of the whiskers. Available options include:
    /// - `"min-max"`: min and max are the lower and upper whiskers respectively.
    /// - A number representing multiple of the interquartile range. This number will be
    /// multiplied by the IQR to determine whisker boundary, which spans from the smallest data
    /// to the largest data within the range _[Q1 - k * IQR, Q3 + k * IQR]_ where _Q1_ and _Q3_
    /// are the first and third quartiles while _IQR_ is the interquartile range (_Q3-Q1_).
    ///
    /// __Default value:__ `1.5`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub extent: Option<BoxplotExtent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub median: Option<DefBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub outliers: Option<DefBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub rule: Option<DefBox>,
    /// Size of the box and median tick of a box plot
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ticks: Option<DefBox>,
}

/// Default configuration for all concatenation view composition operators (`concat`,
/// `hconcat`, and `vconcat`)
///
/// Default configuration for the `facet` view composition operator
///
/// Default configuration for the `repeat` view composition operator
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct CompositionConfig {
    /// The number of columns to include in the view composition layout.
    ///
    /// __Default value__: `undefined` -- An infinite number of columns (a single row) will be
    /// assumed. This is equivalent to
    /// `hconcat` (for `concat`) and to using the `column` channel (for `facet` and `repeat`).
    ///
    /// __Note__:
    ///
    /// 1) This property is only for:
    /// - the general (wrappable) `concat` operator (not `hconcat`/`vconcat`)
    /// - the `facet` and `repeat` operator with one field/repetition definition (without
    /// row/column nesting)
    ///
    /// 2) Setting the `columns` to `1` is equivalent to `vconcat` (for `concat`) and to using
    /// the `row` channel (for `facet` and `repeat`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub columns: Option<f64>,
    /// The default spacing in pixels between composed sub-views.
    ///
    /// __Default value__: `20`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub spacing: Option<f64>,
}

/// ErrorBand Config
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ErrorBandConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub band: Option<DefBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub borders: Option<DefBox>,
    /// The extent of the band. Available options include:
    /// - `"ci"`: Extend the band to the confidence interval of the mean.
    /// - `"stderr"`: The size of band are set to the value of standard error, extending from the
    /// mean.
    /// - `"stdev"`: The size of band are set to the value of standard deviation, extending from
    /// the mean.
    /// - `"iqr"`: Extend the band to the q1 and q3.
    ///
    /// __Default value:__ `"stderr"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub extent: Option<ErrorbandExtent>,
    /// The line interpolation method for the error band. One of the following:
    /// - `"linear"`: piecewise linear segments, as in a polyline.
    /// - `"linear-closed"`: close the linear segments to form a polygon.
    /// - `"step"`: a piecewise constant function (a step function) consisting of alternating
    /// horizontal and vertical lines. The y-value changes at the midpoint of each pair of
    /// adjacent x-values.
    /// - `"step-before"`: a piecewise constant function (a step function) consisting of
    /// alternating horizontal and vertical lines. The y-value changes before the x-value.
    /// - `"step-after"`: a piecewise constant function (a step function) consisting of
    /// alternating horizontal and vertical lines. The y-value changes after the x-value.
    /// - `"basis"`: a B-spline, with control point duplication on the ends.
    /// - `"basis-open"`: an open B-spline; may not intersect the start or end.
    /// - `"basis-closed"`: a closed B-spline, as in a loop.
    /// - `"cardinal"`: a Cardinal spline, with control point duplication on the ends.
    /// - `"cardinal-open"`: an open Cardinal spline; may not intersect the start or end, but
    /// will intersect other control points.
    /// - `"cardinal-closed"`: a closed Cardinal spline, as in a loop.
    /// - `"bundle"`: equivalent to basis, except the tension parameter is used to straighten the
    /// spline.
    /// - `"monotone"`: cubic interpolation that preserves monotonicity in y.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub interpolate: Option<Interpolate>,
    /// The tension parameter for the interpolation type of the error band.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tension: Option<f64>,
}

/// ErrorBar Config
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ErrorBarConfig {
    /// The extent of the rule. Available options include:
    /// - `"ci"`: Extend the rule to the confidence interval of the mean.
    /// - `"stderr"`: The size of rule are set to the value of standard error, extending from the
    /// mean.
    /// - `"stdev"`: The size of rule are set to the value of standard deviation, extending from
    /// the mean.
    /// - `"iqr"`: Extend the rule to the q1 and q3.
    ///
    /// __Default value:__ `"stderr"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub extent: Option<ErrorbandExtent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub rule: Option<DefBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ticks: Option<DefBox>,
}

/// Header configuration, which determines default properties for all
/// [headers](https://vega.github.io/vega-lite/docs/header.html).
///
/// For a full list of header configuration options, please see the [corresponding section of
/// in the header documentation](https://vega.github.io/vega-lite/docs/header.html#config).
///
/// Header configuration, which determines default properties for column
/// [headers](https://vega.github.io/vega-lite/docs/header.html).
///
/// For a full list of header configuration options, please see the [corresponding section of
/// in the header documentation](https://vega.github.io/vega-lite/docs/header.html#config).
///
/// Header configuration, which determines default properties for non-row/column facet
/// [headers](https://vega.github.io/vega-lite/docs/header.html).
///
/// For a full list of header configuration options, please see the [corresponding section of
/// in the header documentation](https://vega.github.io/vega-lite/docs/header.html#config).
///
/// Header configuration, which determines default properties for row
/// [headers](https://vega.github.io/vega-lite/docs/header.html).
///
/// For a full list of header configuration options, please see the [corresponding section of
/// in the header documentation](https://vega.github.io/vega-lite/docs/header.html#config).
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct HeaderConfig {
    /// The text formatting pattern for labels of guides (axes, legends, headers) and text
    /// marks.
    ///
    /// - If the format type is `"number"` (e.g., for quantitative fields), this is D3's [number
    /// format pattern](https://github.com/d3/d3-format#locale_format).
    /// - If the format type is `"time"` (e.g., for temporal fields), this is D3's [time format
    /// pattern](https://github.com/d3/d3-time-format#locale_format).
    ///
    /// See the [format documentation](https://vega.github.io/vega-lite/docs/format.html) for
    /// more examples.
    ///
    /// __Default value:__  Derived from
    /// [numberFormat](https://vega.github.io/vega-lite/docs/config.html#format) config for
    /// number format and from
    /// [timeFormat](https://vega.github.io/vega-lite/docs/config.html#format) config for time
    /// format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub format: Option<String>,
    /// The format type for labels (`"number"` or `"time"`).
    ///
    /// __Default value:__
    /// - `"time"` for temporal fields and ordinal and nomimal fields with `timeUnit`.
    /// - `"number"` for quantitative fields as well as ordinal and nomimal fields without
    /// `timeUnit`.
    #[serde(rename = "formatType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub format_type: Option<FormatType>,
    /// Horizontal text alignment of header labels. One of `"left"`, `"center"`, or `"right"`.
    #[serde(rename = "labelAlign")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_align: Option<Align>,
    /// The anchor position for placing the labels. One of `"start"`, `"middle"`, or `"end"`. For
    /// example, with a label orientation of top these anchor positions map to a left-, center-,
    /// or right-aligned label.
    #[serde(rename = "labelAnchor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_anchor: Option<TitleAnchorEnum>,
    /// The rotation angle of the header labels.
    ///
    /// __Default value:__ `0` for column header, `-90` for row header.
    #[serde(rename = "labelAngle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_angle: Option<f64>,
    /// The color of the header label, can be in hex color code or regular color name.
    #[serde(rename = "labelColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_color: Option<String>,
    /// [Vega expression](https://vega.github.io/vega/docs/expressions/) for customizing labels.
    ///
    /// __Note:__ The label text and value can be assessed via the `label` and `value` properties
    /// of the header's backing `datum` object.
    #[serde(rename = "labelExpr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_expr: Option<String>,
    /// The font of the header label.
    #[serde(rename = "labelFont")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_font: Option<String>,
    /// The font size of the header label, in pixels.
    #[serde(rename = "labelFontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_font_size: Option<f64>,
    /// The font style of the header label.
    #[serde(rename = "labelFontStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_font_style: Option<String>,
    /// The maximum length of the header label in pixels. The text value will be automatically
    /// truncated if the rendered size exceeds the limit.
    ///
    /// __Default value:__ `0`, indicating no limit
    #[serde(rename = "labelLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_limit: Option<f64>,
    /// The orientation of the header label. One of `"top"`, `"bottom"`, `"left"` or `"right"`.
    #[serde(rename = "labelOrient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_orient: Option<Orient>,
    /// The padding, in pixel, between facet header's label and the plot.
    ///
    /// __Default value:__ `10`
    #[serde(rename = "labelPadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_padding: Option<f64>,
    /// A boolean flag indicating if labels should be included as part of the header.
    ///
    /// __Default value:__ `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub labels: Option<bool>,
    /// Set to null to disable title for the axis, legend, or header.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub title: RemovableValue<serde_json::Value>,
    /// Horizontal text alignment (to the anchor) of header titles.
    #[serde(rename = "titleAlign")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_align: Option<Align>,
    /// The anchor position for placing the title. One of `"start"`, `"middle"`, or `"end"`. For
    /// example, with an orientation of top these anchor positions map to a left-, center-, or
    /// right-aligned title.
    #[serde(rename = "titleAnchor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_anchor: Option<TitleAnchorEnum>,
    /// The rotation angle of the header title.
    ///
    /// __Default value:__ `0`.
    #[serde(rename = "titleAngle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_angle: Option<f64>,
    /// Vertical text baseline for the header title. One of `"top"`, `"bottom"`, `"middle"`.
    ///
    /// __Default value:__ `"middle"`
    #[serde(rename = "titleBaseline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_baseline: Option<Baseline>,
    /// Color of the header title, can be in hex color code or regular color name.
    #[serde(rename = "titleColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_color: Option<String>,
    /// Font of the header title. (e.g., `"Helvetica Neue"`).
    #[serde(rename = "titleFont")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_font: Option<String>,
    /// Font size of the header title.
    #[serde(rename = "titleFontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_font_size: Option<f64>,
    /// The font style of the header title.
    #[serde(rename = "titleFontStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_font_style: Option<String>,
    /// Font weight of the header title.
    /// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
    /// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
    #[serde(rename = "titleFontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_font_weight: Option<FontWeight>,
    /// The maximum length of the header title in pixels. The text value will be automatically
    /// truncated if the rendered size exceeds the limit.
    ///
    /// __Default value:__ `0`, indicating no limit
    #[serde(rename = "titleLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_limit: Option<f64>,
    /// Line height in pixels for multi-line title text.
    #[serde(rename = "titleLineHeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_line_height: Option<f64>,
    /// The orientation of the header title. One of `"top"`, `"bottom"`, `"left"` or `"right"`.
    #[serde(rename = "titleOrient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_orient: Option<Orient>,
    /// The padding, in pixel, between facet header's title and the label.
    ///
    /// __Default value:__ `10`
    #[serde(rename = "titlePadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_padding: Option<f64>,
}

/// Legend configuration, which determines default properties for all
/// [legends](https://vega.github.io/vega-lite/docs/legend.html). For a full list of legend
/// configuration options, please see the [corresponding section of in the legend
/// documentation](https://vega.github.io/vega-lite/docs/legend.html#config).
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct LegendConfig {
    /// The height in pixels to clip symbol legend entries and limit their size.
    #[serde(rename = "clipHeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub clip_height: Option<f64>,
    /// The horizontal padding in pixels between symbol legend entries.
    ///
    /// __Default value:__ `10`.
    #[serde(rename = "columnPadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub column_padding: Option<f64>,
    /// The number of columns in which to arrange symbol legend entries. A value of `0` or lower
    /// indicates a single row with one column per entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub columns: Option<f64>,
    /// Corner radius for the full legend.
    #[serde(rename = "cornerRadius")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius: Option<f64>,
    /// Background fill color for the full legend.
    #[serde(rename = "fillColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fill_color: Option<String>,
    /// The default direction (`"horizontal"` or `"vertical"`) for gradient legends.
    ///
    /// __Default value:__ `"vertical"`.
    #[serde(rename = "gradientDirection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub gradient_direction: Option<Orientation>,
    /// Max legend length for a horizontal gradient when `config.legend.gradientLength` is
    /// undefined.
    ///
    /// __Default value:__ `200`
    #[serde(rename = "gradientHorizontalMaxLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub gradient_horizontal_max_length: Option<f64>,
    /// Min legend length for a horizontal gradient when `config.legend.gradientLength` is
    /// undefined.
    ///
    /// __Default value:__ `100`
    #[serde(rename = "gradientHorizontalMinLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub gradient_horizontal_min_length: Option<f64>,
    /// The maximum allowed length in pixels of color ramp gradient labels.
    #[serde(rename = "gradientLabelLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub gradient_label_limit: Option<f64>,
    /// Vertical offset in pixels for color ramp gradient labels.
    ///
    /// __Default value:__ `2`.
    #[serde(rename = "gradientLabelOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub gradient_label_offset: Option<f64>,
    /// The length in pixels of the primary axis of a color gradient. This value corresponds to
    /// the height of a vertical gradient or the width of a horizontal gradient.
    ///
    /// __Default value:__ `200`.
    #[serde(rename = "gradientLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub gradient_length: Option<f64>,
    /// Opacity of the color gradient.
    #[serde(rename = "gradientOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub gradient_opacity: Option<f64>,
    /// The color of the gradient stroke, can be in hex color code or regular color name.
    ///
    /// __Default value:__ `"lightGray"`.
    #[serde(rename = "gradientStrokeColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub gradient_stroke_color: Option<String>,
    /// The width of the gradient stroke, in pixels.
    ///
    /// __Default value:__ `0`.
    #[serde(rename = "gradientStrokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub gradient_stroke_width: Option<f64>,
    /// The thickness in pixels of the color gradient. This value corresponds to the width of a
    /// vertical gradient or the height of a horizontal gradient.
    ///
    /// __Default value:__ `16`.
    #[serde(rename = "gradientThickness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub gradient_thickness: Option<f64>,
    /// Max legend length for a vertical gradient when `config.legend.gradientLength` is
    /// undefined.
    ///
    /// __Default value:__ `200`
    #[serde(rename = "gradientVerticalMaxLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub gradient_vertical_max_length: Option<f64>,
    /// Min legend length for a vertical gradient when `config.legend.gradientLength` is
    /// undefined.
    ///
    /// __Default value:__ `100`
    #[serde(rename = "gradientVerticalMinLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub gradient_vertical_min_length: Option<f64>,
    /// The alignment to apply to symbol legends rows and columns. The supported string values
    /// are `"all"`, `"each"` (the default), and `none`. For more information, see the [grid
    /// layout documentation](https://vega.github.io/vega/docs/layout).
    ///
    /// __Default value:__ `"each"`.
    #[serde(rename = "gridAlign")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub grid_align: Option<LayoutAlign>,
    /// The alignment of the legend label, can be left, center, or right.
    #[serde(rename = "labelAlign")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_align: Option<Align>,
    /// The position of the baseline of legend label, can be `"top"`, `"middle"`, `"bottom"`, or
    /// `"alphabetic"`.
    ///
    /// __Default value:__ `"middle"`.
    #[serde(rename = "labelBaseline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_baseline: Option<Baseline>,
    /// The color of the legend label, can be in hex color code or regular color name.
    #[serde(rename = "labelColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_color: Option<String>,
    /// The font of the legend label.
    #[serde(rename = "labelFont")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_font: Option<String>,
    /// The font size of legend label.
    ///
    /// __Default value:__ `10`.
    #[serde(rename = "labelFontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_font_size: Option<f64>,
    /// The font style of legend label.
    #[serde(rename = "labelFontStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_font_style: Option<String>,
    /// The font weight of legend label.
    #[serde(rename = "labelFontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_font_weight: Option<FontWeight>,
    /// Maximum allowed pixel width of legend tick labels.
    ///
    /// __Default value:__ `160`.
    #[serde(rename = "labelLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_limit: Option<f64>,
    /// The offset of the legend label.
    #[serde(rename = "labelOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_offset: Option<f64>,
    /// Opacity of labels.
    #[serde(rename = "labelOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_opacity: Option<f64>,
    /// The strategy to use for resolving overlap of labels in gradient legends. If `false`, no
    /// overlap reduction is attempted. If set to `true` or `"parity"`, a strategy of removing
    /// every other label is used. If set to `"greedy"`, a linear scan of the labels is
    /// performed, removing any label that overlaps with the last visible label (this often works
    /// better for log-scaled axes).
    ///
    /// __Default value:__ `"greedy"` for `log scales otherwise `true`.
    #[serde(rename = "labelOverlap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_overlap: Option<LabelOverlap>,
    /// Padding in pixels between the legend and legend labels.
    #[serde(rename = "labelPadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_padding: Option<f64>,
    /// The minimum separation that must be between label bounding boxes for them to be
    /// considered non-overlapping (default `0`). This property is ignored if *labelOverlap*
    /// resolution is not enabled.
    #[serde(rename = "labelSeparation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub label_separation: Option<f64>,
    /// Legend orient group layout parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub layout: Option<LegendLayout>,
    /// Custom x-position for legend with orient "none".
    #[serde(rename = "legendX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub legend_x: Option<f64>,
    /// Custom y-position for legend with orient "none".
    #[serde(rename = "legendY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub legend_y: Option<f64>,
    /// The offset in pixels by which to displace the legend from the data rectangle and axes.
    ///
    /// __Default value:__ `18`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub offset: Option<f64>,
    /// The orientation of the legend, which determines how the legend is positioned within the
    /// scene. One of "left", "right", "top-left", "top-right", "bottom-left", "bottom-right",
    /// "none".
    ///
    /// __Default value:__ `"right"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub orient: Option<LegendOrient>,
    /// The padding between the border and content of the legend group.
    ///
    /// __Default value:__ `0`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub padding: Option<f64>,
    /// The vertical padding in pixels between symbol legend entries.
    ///
    /// __Default value:__ `2`.
    #[serde(rename = "rowPadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub row_padding: Option<f64>,
    /// Border stroke color for the full legend.
    #[serde(rename = "strokeColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_color: Option<String>,
    /// Border stroke dash pattern for the full legend.
    #[serde(rename = "strokeDash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_dash: Option<Vec<f64>>,
    /// Border stroke width for the full legend.
    #[serde(rename = "strokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_width: Option<f64>,
    /// Default fill color for legend symbols. Only applied if there is no `"fill"` scale color
    /// encoding for the legend.
    ///
    /// __Default value:__ `"transparent"`.
    #[serde(rename = "symbolBaseFillColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub symbol_base_fill_color: Option<String>,
    /// Default stroke color for legend symbols. Only applied if there is no `"fill"` scale color
    /// encoding for the legend.
    ///
    /// __Default value:__ `"gray"`.
    #[serde(rename = "symbolBaseStrokeColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub symbol_base_stroke_color: Option<String>,
    /// An array of alternating [stroke, space] lengths for dashed symbol strokes.
    #[serde(rename = "symbolDash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub symbol_dash: Option<Vec<f64>>,
    /// The pixel offset at which to start drawing with the symbol stroke dash array.
    #[serde(rename = "symbolDashOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub symbol_dash_offset: Option<f64>,
    /// The default direction (`"horizontal"` or `"vertical"`) for symbol legends.
    ///
    /// __Default value:__ `"vertical"`.
    #[serde(rename = "symbolDirection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub symbol_direction: Option<Orientation>,
    /// The color of the legend symbol,
    #[serde(rename = "symbolFillColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub symbol_fill_color: Option<String>,
    /// The maximum number of allowed entries for a symbol legend. Additional entries will be
    /// dropped.
    #[serde(rename = "symbolLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub symbol_limit: Option<f64>,
    /// Horizontal pixel offset for legend symbols.
    ///
    /// __Default value:__ `0`.
    #[serde(rename = "symbolOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub symbol_offset: Option<f64>,
    /// Opacity of the legend symbols.
    #[serde(rename = "symbolOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub symbol_opacity: Option<f64>,
    /// The size of the legend symbol, in pixels.
    ///
    /// __Default value:__ `100`.
    #[serde(rename = "symbolSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub symbol_size: Option<f64>,
    /// Stroke color for legend symbols.
    #[serde(rename = "symbolStrokeColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub symbol_stroke_color: Option<String>,
    /// The width of the symbol's stroke.
    ///
    /// __Default value:__ `1.5`.
    #[serde(rename = "symbolStrokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub symbol_stroke_width: Option<f64>,
    /// The symbol shape. One of the plotting shapes `circle` (default), `square`, `cross`,
    /// `diamond`, `triangle-up`, `triangle-down`, `triangle-right`, or `triangle-left`, the line
    /// symbol `stroke`, or one of the centered directional shapes `arrow`, `wedge`, or
    /// `triangle`. Alternatively, a custom [SVG path
    /// string](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths) can be provided.
    /// For correct sizing, custom shape paths should be defined within a square bounding box
    /// with coordinates ranging from -1 to 1 along both the x and y dimensions.
    ///
    /// __Default value:__ `"circle"`.
    #[serde(rename = "symbolType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub symbol_type: Option<String>,
    /// The desired number of tick values for quantitative legends.
    #[serde(rename = "tickCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tick_count: Option<TickCount>,
    /// Set to null to disable title for the axis, legend, or header.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub title: RemovableValue<serde_json::Value>,
    /// Horizontal text alignment for legend titles.
    ///
    /// __Default value:__ `"left"`.
    #[serde(rename = "titleAlign")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_align: Option<Align>,
    /// Text anchor position for placing legend titles.
    #[serde(rename = "titleAnchor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_anchor: Option<TitleAnchorEnum>,
    /// Vertical text baseline for legend titles.
    ///
    /// __Default value:__ `"top"`.
    #[serde(rename = "titleBaseline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_baseline: Option<Baseline>,
    /// The color of the legend title, can be in hex color code or regular color name.
    #[serde(rename = "titleColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_color: Option<String>,
    /// The font of the legend title.
    #[serde(rename = "titleFont")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_font: Option<String>,
    /// The font size of the legend title.
    #[serde(rename = "titleFontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_font_size: Option<f64>,
    /// The font style of the legend title.
    #[serde(rename = "titleFontStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_font_style: Option<String>,
    /// The font weight of the legend title.
    /// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
    /// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
    #[serde(rename = "titleFontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_font_weight: Option<FontWeight>,
    /// Maximum allowed pixel width of legend titles.
    ///
    /// __Default value:__ `180`.
    #[serde(rename = "titleLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_limit: Option<f64>,
    /// Line height in pixels for multi-line title text.
    #[serde(rename = "titleLineHeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_line_height: Option<f64>,
    /// Opacity of the legend title.
    #[serde(rename = "titleOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_opacity: Option<f64>,
    /// Orientation of the legend title.
    #[serde(rename = "titleOrient")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_orient: Option<Orient>,
    /// The padding, in pixels, between title and legend.
    ///
    /// __Default value:__ `5`.
    #[serde(rename = "titlePadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub title_padding: Option<f64>,
    /// The opacity of unselected legend entries.
    ///
    /// __Default value:__ 0.35.
    #[serde(rename = "unselectedOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub unselected_opacity: Option<f64>,
}

/// Legend orient group layout parameters.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct LegendLayout {
    /// The anchor point for legend orient group layout.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub anchor: Option<AnchorUnion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bottom: Option<BaseLegendLayout>,
    #[serde(rename = "bottom-left")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bottom_left: Option<BaseLegendLayout>,
    #[serde(rename = "bottom-right")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bottom_right: Option<BaseLegendLayout>,
    /// The bounds calculation to use for legend orient group layout.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bounds: Option<LayoutBounds>,
    /// A flag to center legends within a shared orient group.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub center: Option<BottomCenter>,
    /// The layout direction for legend orient group layout.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub direction: Option<Direction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub left: Option<BaseLegendLayout>,
    /// The pixel margin between legends within a orient group.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub margin: Option<MarginUnion>,
    /// The pixel offset from the chart body for a legend orient group.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub offset: Option<MarginUnion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub right: Option<BaseLegendLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub top: Option<BaseLegendLayout>,
    #[serde(rename = "top-left")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub top_left: Option<BaseLegendLayout>,
    #[serde(rename = "top-right")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub top_right: Option<BaseLegendLayout>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct PurpleSignalRef {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub signal: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct BaseLegendLayout {
    /// The anchor point for legend orient group layout.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub anchor: Option<AnchorUnion>,
    /// The bounds calculation to use for legend orient group layout.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bounds: Option<LayoutBounds>,
    /// A flag to center legends within a shared orient group.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub center: Option<BottomCenter>,
    /// The layout direction for legend orient group layout.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub direction: Option<Direction>,
    /// The pixel margin between legends within a orient group.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub margin: Option<MarginUnion>,
    /// The pixel offset from the chart body for a legend orient group.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub offset: Option<MarginUnion>,
}

/// Line-Specific Config
///
/// Trail-Specific Config
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct LineConfig {
    /// The horizontal alignment of the text or ranged marks (area, bar, image, rect, rule). One
    /// of `"left"`, `"right"`, `"center"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub align: Option<Align>,
    /// The rotation angle of the text, in degrees.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub angle: Option<f64>,
    /// Whether to keep aspect ratio of image marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aspect: Option<bool>,
    /// The vertical alignment of the text or ranged marks (area, bar, image, rect, rule). One of
    /// `"top"`, `"middle"`, `"bottom"`.
    ///
    /// __Default value:__ `"middle"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub baseline: Option<Baseline>,
    /// Default color.
    ///
    /// __Default value:__ <span style="color: #4682b4;">&#9632;</span> `"#4682b4"`
    ///
    /// __Note:__
    /// - This property cannot be used in a [style
    /// config](https://vega.github.io/vega-lite/docs/mark.html#style-config).
    /// - The `fill` and `stroke` properties have higher precedence than `color` and will
    /// override `color`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub color: Option<ColorUnion>,
    /// The radius in pixels of rounded rectangle corners.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadius")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius: Option<f64>,
    /// The radius in pixels of rounded rectangle bottom left corner.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadiusBottomLeft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius_bottom_left: Option<f64>,
    /// The radius in pixels of rounded rectangle bottom right corner.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadiusBottomRight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius_bottom_right: Option<f64>,
    /// The radius in pixels of rounded rectangle top right corner.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadiusTopLeft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius_top_left: Option<f64>,
    /// The radius in pixels of rounded rectangle top left corner.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadiusTopRight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius_top_right: Option<f64>,
    /// The mouse cursor used over the mark. Any valid [CSS cursor
    /// type](https://developer.mozilla.org/en-US/docs/Web/CSS/cursor#Values) can be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub cursor: Option<Cursor>,
    /// The direction of the text. One of `"ltr"` (left-to-right) or `"rtl"` (right-to-left).
    /// This property determines on which side is truncated in response to the limit parameter.
    ///
    /// __Default value:__ `"ltr"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dir: Option<Dir>,
    /// The horizontal offset, in pixels, between the text label and its anchor point. The offset
    /// is applied after rotation by the _angle_ property.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dx: Option<f64>,
    /// The vertical offset, in pixels, between the text label and its anchor point. The offset
    /// is applied after rotation by the _angle_ property.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dy: Option<f64>,
    /// The ellipsis string for text truncated in response to the limit parameter.
    ///
    /// __Default value:__ `"…"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ellipsis: Option<String>,
    /// Default Fill Color. This has higher precedence than `config.color`.
    ///
    /// __Default value:__ (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fill: Option<FillUnion>,
    /// Whether the mark's color should be used as fill color instead of stroke color.
    ///
    /// __Default value:__ `false` for all `point`, `line`, and `rule` marks as well as
    /// `geoshape` marks for
    /// [`graticule`](https://vega.github.io/vega-lite/docs/data.html#graticule) data sources;
    /// otherwise, `true`.
    ///
    /// __Note:__ This property cannot be used in a [style
    /// config](https://vega.github.io/vega-lite/docs/mark.html#style-config).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub filled: Option<bool>,
    /// The fill opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "fillOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fill_opacity: Option<f64>,
    /// The typeface to set the text in (e.g., `"Helvetica Neue"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font: Option<String>,
    /// The font size, in pixels.
    #[serde(rename = "fontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font_size: Option<f64>,
    /// The font style (e.g., `"italic"`).
    #[serde(rename = "fontStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font_style: Option<String>,
    /// The font weight.
    /// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
    /// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
    #[serde(rename = "fontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font_weight: Option<FontWeight>,
    /// Height of the marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub height: Option<f64>,
    /// A URL to load upon mouse click. If defined, the mark acts as a hyperlink.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub href: Option<String>,
    /// The line interpolation method to use for line and area marks. One of the following:
    /// - `"linear"`: piecewise linear segments, as in a polyline.
    /// - `"linear-closed"`: close the linear segments to form a polygon.
    /// - `"step"`: alternate between horizontal and vertical segments, as in a step function.
    /// - `"step-before"`: alternate between vertical and horizontal segments, as in a step
    /// function.
    /// - `"step-after"`: alternate between horizontal and vertical segments, as in a step
    /// function.
    /// - `"basis"`: a B-spline, with control point duplication on the ends.
    /// - `"basis-open"`: an open B-spline; may not intersect the start or end.
    /// - `"basis-closed"`: a closed B-spline, as in a loop.
    /// - `"cardinal"`: a Cardinal spline, with control point duplication on the ends.
    /// - `"cardinal-open"`: an open Cardinal spline; may not intersect the start or end, but
    /// will intersect other control points.
    /// - `"cardinal-closed"`: a closed Cardinal spline, as in a loop.
    /// - `"bundle"`: equivalent to basis, except the tension parameter is used to straighten the
    /// spline.
    /// - `"monotone"`: cubic interpolation that preserves monotonicity in y.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub interpolate: Option<Interpolate>,
    /// Defines how Vega-Lite should handle marks for invalid values (`null` and `NaN`).
    /// - If set to `"filter"` (default), all data items with null values will be skipped (for
    /// line, trail, and area marks) or filtered (for other marks).
    /// - If `null`, all data items are included. In this case, invalid values will be
    /// interpreted as zeroes.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub invalid: RemovableValue<Invalid>,
    /// The maximum length of the text mark in pixels. The text value will be automatically
    /// truncated if the rendered size exceeds the limit.
    ///
    /// __Default value:__ `0`, indicating no limit
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub limit: Option<f64>,
    /// A delimiter, such as a newline character, upon which to break text strings into multiple
    /// lines. This property will be ignored if the text property is array-valued.
    #[serde(rename = "lineBreak")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub line_break: Option<String>,
    /// The height, in pixels, of each line of text in a multi-line text mark.
    #[serde(rename = "lineHeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub line_height: Option<f64>,
    /// The overall opacity (value between [0,1]).
    ///
    /// __Default value:__ `0.7` for non-aggregate plots with `point`, `tick`, `circle`, or
    /// `square` marks or layered `bar` charts and `1` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub opacity: Option<f64>,
    /// For line and trail marks, this `order` property can be set to `null` or `false` to make
    /// the lines use the original order in the data sources.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub order: Option<bool>,
    /// The orientation of a non-stacked bar, tick, area, and line charts.
    /// The value is either horizontal (default) or vertical.
    /// - For bar, rule and tick, this determines whether the size of the bar and tick
    /// should be applied to x or y dimension.
    /// - For area, this property determines the orient property of the Vega output.
    /// - For line and trail marks, this property determines the sort order of the points in the
    /// line
    /// if `config.sortLineBy` is not specified.
    /// For stacked charts, this is always determined by the orientation of the stack;
    /// therefore explicitly specified value will be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub orient: Option<Orientation>,
    /// A flag for overlaying points on top of line or area marks, or an object defining the
    /// properties of the overlayed points.
    ///
    /// - If this property is `"transparent"`, transparent points will be used (for enhancing
    /// tooltips and selections).
    ///
    /// - If this property is an empty object (`{}`) or `true`, filled points with default
    /// properties will be used.
    ///
    /// - If this property is `false`, no points would be automatically added to line or area
    /// marks.
    ///
    /// __Default value:__ `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub point: Option<PointUnion>,
    /// Polar coordinate radial offset, in pixels, of the text label from the origin determined
    /// by the `x` and `y` properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub radius: Option<f64>,
    /// Shape of the point marks. Supported values include:
    /// - plotting shapes: `"circle"`, `"square"`, `"cross"`, `"diamond"`, `"triangle-up"`,
    /// `"triangle-down"`, `"triangle-right"`, or `"triangle-left"`.
    /// - the line symbol `"stroke"`
    /// - centered directional shapes `"arrow"`, `"wedge"`, or `"triangle"`
    /// - a custom [SVG path
    /// string](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths) (For correct
    /// sizing, custom shape paths should be defined within a square bounding box with
    /// coordinates ranging from -1 to 1 along both the x and y dimensions.)
    ///
    /// __Default value:__ `"circle"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub shape: Option<String>,
    /// Default size for marks.
    /// - For `point`/`circle`/`square`, this represents the pixel area of the marks. For
    /// example: in the case of circles, the radius is determined in part by the square root of
    /// the size value.
    /// - For `bar`, this represents the band size of the bar, in pixels.
    /// - For `text`, this represents the font size, in pixels.
    ///
    /// __Default value:__
    /// - `30` for point, circle, square marks; width/height's `step`
    /// - `2` for bar marks with discrete dimensions;
    /// - `5` for bar marks with continuous dimensions;
    /// - `11` for text marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub size: Option<f64>,
    /// Default Stroke Color. This has higher precedence than `config.color`.
    ///
    /// __Default value:__ (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke: Option<FillUnion>,
    /// The stroke cap for line ending style. One of `"butt"`, `"round"`, or `"square"`.
    ///
    /// __Default value:__ `"square"`
    #[serde(rename = "strokeCap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_cap: Option<StrokeCap>,
    /// An array of alternating stroke, space lengths for creating dashed or dotted lines.
    #[serde(rename = "strokeDash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_dash: Option<Vec<f64>>,
    /// The offset (in pixels) into which to begin drawing with the stroke dash array.
    #[serde(rename = "strokeDashOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_dash_offset: Option<f64>,
    /// The stroke line join method. One of `"miter"`, `"round"` or `"bevel"`.
    ///
    /// __Default value:__ `"miter"`
    #[serde(rename = "strokeJoin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_join: Option<StrokeJoin>,
    /// The miter limit at which to bevel a line join.
    #[serde(rename = "strokeMiterLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_miter_limit: Option<f64>,
    /// The stroke opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "strokeOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_opacity: Option<f64>,
    /// The stroke width, in pixels.
    #[serde(rename = "strokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_width: Option<f64>,
    /// Depending on the interpolation type, sets the tension parameter (for line and area marks).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tension: Option<f64>,
    /// Placeholder text if the `text` channel is not specified
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub text: Option<PurpleText>,
    /// Polar coordinate angle, in radians, of the text label from the origin determined by the
    /// `x` and `y` properties. Values for `theta` follow the same convention of `arc` mark
    /// `startAngle` and `endAngle` properties: angles are measured in radians, with `0`
    /// indicating "north".
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub theta: Option<f64>,
    /// Default relative band size for a time unit. If set to `1`, the bandwidth of the marks
    /// will be equal to the time unit band step.
    /// If set to `0.5`, bandwidth of the marks will be half of the time unit band step.
    #[serde(rename = "timeUnitBand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit_band: Option<f64>,
    /// Default relative band position for a time unit. If set to `0`, the marks will be
    /// positioned at the beginning of the time unit band step.
    /// If set to `0.5`, the marks will be positioned in the middle of the time unit band step.
    #[serde(rename = "timeUnitBandPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit_band_position: Option<f64>,
    /// The tooltip text string to show upon mouse hover or an object defining which fields
    /// should the tooltip be derived from.
    ///
    /// - If `tooltip` is `true` or `{"content": "encoding"}`, then all fields from `encoding`
    /// will be used.
    /// - If `tooltip` is `{"content": "data"}`, then all fields that appear in the highlighted
    /// data point will be used.
    /// - If set to `null` or `false`, then no tooltip will be used.
    ///
    /// See the [`tooltip`](https://vega.github.io/vega-lite/docs/tooltip.html) documentation for
    /// a detailed discussion about tooltip  in Vega-Lite.
    ///
    /// __Default value:__ `null`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tooltip: Option<Value>,
    /// Width of the marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub width: Option<f64>,
    /// X coordinates of the marks, or width of horizontal `"bar"` and `"area"` without specified
    /// `x2` or `width`.
    ///
    /// The `value` of this channel can be a number or a string `"width"` for the width of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub x: Option<XUnion>,
    /// X2 coordinates for ranged `"area"`, `"bar"`, `"rect"`, and  `"rule"`.
    ///
    /// The `value` of this channel can be a number or a string `"width"` for the width of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub x2: Option<XUnion>,
    /// Y coordinates of the marks, or height of vertical `"bar"` and `"area"` without specified
    /// `y2` or `height`.
    ///
    /// The `value` of this channel can be a number or a string `"height"` for the height of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub y: Option<YUnion>,
    /// Y2 coordinates for ranged `"area"`, `"bar"`, `"rect"`, and  `"rule"`.
    ///
    /// The `value` of this channel can be a number or a string `"height"` for the height of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub y2: Option<YUnion>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct PaddingClass {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bottom: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub left: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub right: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub top: Option<f64>,
}

/// An object hash that defines default range arrays or schemes for using with scales.
/// For a full list of scale range configuration options, please see the [corresponding
/// section of the scale
/// documentation](https://vega.github.io/vega-lite/docs/scale.html#config).
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct RangeConfig {
    /// Default [color scheme](https://vega.github.io/vega/docs/schemes/) for categorical data.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub category: Option<CategoryUnion>,
    /// Default [color scheme](https://vega.github.io/vega/docs/schemes/) for diverging
    /// quantitative ramps.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub diverging: Option<DivergingUnion>,
    /// Default [color scheme](https://vega.github.io/vega/docs/schemes/) for quantitative
    /// heatmaps.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub heatmap: Option<HeatmapUnion>,
    /// Default [color scheme](https://vega.github.io/vega/docs/schemes/) for rank-ordered data.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ordinal: Option<OrdinalUnion>,
    /// Default [color scheme](https://vega.github.io/vega/docs/schemes/) for sequential
    /// quantitative ramps.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ramp: Option<RampUnion>,
    /// Array of [symbol](https://vega.github.io/vega/docs/marks/symbol/) names or paths for the
    /// default shape palette.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub symbol: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct CategorySignalRef {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub signal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub count: Option<MarginUnion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub extent: Option<SignalRefExtent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub scheme: Option<ColorScheme>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct DivergingSignalRef {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub signal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub count: Option<MarginUnion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub extent: Option<SignalRefExtent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub scheme: Option<ColorScheme>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct HeatmapSignalRef {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub signal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub count: Option<MarginUnion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub extent: Option<SignalRefExtent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub scheme: Option<ColorScheme>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct OrdinalSignalRef {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub signal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub count: Option<MarginUnion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub extent: Option<SignalRefExtent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub scheme: Option<ColorScheme>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct RampSignalRef {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub signal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub count: Option<MarginUnion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub extent: Option<SignalRefExtent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub scheme: Option<ColorScheme>,
}

/// Scale configuration determines default properties for all
/// [scales](https://vega.github.io/vega-lite/docs/scale.html). For a full list of scale
/// configuration options, please see the [corresponding section of the scale
/// documentation](https://vega.github.io/vega-lite/docs/scale.html#config).
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ScaleConfig {
    /// Default inner padding for `x` and `y` band-ordinal scales.
    ///
    /// __Default value:__
    /// - `barBandPaddingInner` for bar marks (`0.1` by default)
    /// - `rectBandPaddingInner` for rect and other marks (`0` by default)
    #[serde(rename = "bandPaddingInner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub band_padding_inner: Option<f64>,
    /// Default outer padding for `x` and `y` band-ordinal scales.
    ///
    /// __Default value:__ `paddingInner/2` (which makes _width/height = number of unique values
    /// * step_)
    #[serde(rename = "bandPaddingOuter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub band_padding_outer: Option<f64>,
    /// Default inner padding for `x` and `y` band-ordinal scales of `"bar"` marks.
    ///
    /// __Default value:__ `0.1`
    #[serde(rename = "barBandPaddingInner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bar_band_padding_inner: Option<f64>,
    /// If true, values that exceed the data domain are clamped to either the minimum or maximum
    /// range value
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub clamp: Option<bool>,
    /// Default padding for continuous scales.
    ///
    /// __Default:__ `5` for continuous x-scale of a vertical bar and continuous y-scale of a
    /// horizontal bar.; `0` otherwise.
    #[serde(rename = "continuousPadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub continuous_padding: Option<f64>,
    /// The default max value for mapping quantitative fields to bar's size/bandSize.
    ///
    /// If undefined (default), we will use the axis's size (width or height) - 1.
    #[serde(rename = "maxBandSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub max_band_size: Option<f64>,
    /// The default max value for mapping quantitative fields to text's size/fontSize.
    ///
    /// __Default value:__ `40`
    #[serde(rename = "maxFontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub max_font_size: Option<f64>,
    /// Default max opacity for mapping a field to opacity.
    ///
    /// __Default value:__ `0.8`
    #[serde(rename = "maxOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub max_opacity: Option<f64>,
    /// Default max value for point size scale.
    #[serde(rename = "maxSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub max_size: Option<f64>,
    /// Default max strokeWidth for the scale of strokeWidth for rule and line marks and of size
    /// for trail marks.
    ///
    /// __Default value:__ `4`
    #[serde(rename = "maxStrokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub max_stroke_width: Option<f64>,
    /// The default min value for mapping quantitative fields to bar and tick's size/bandSize
    /// scale with zero=false.
    ///
    /// __Default value:__ `2`
    #[serde(rename = "minBandSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub min_band_size: Option<f64>,
    /// The default min value for mapping quantitative fields to tick's size/fontSize scale with
    /// zero=false
    ///
    /// __Default value:__ `8`
    #[serde(rename = "minFontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub min_font_size: Option<f64>,
    /// Default minimum opacity for mapping a field to opacity.
    ///
    /// __Default value:__ `0.3`
    #[serde(rename = "minOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub min_opacity: Option<f64>,
    /// Default minimum value for point size scale with zero=false.
    ///
    /// __Default value:__ `9`
    #[serde(rename = "minSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub min_size: Option<f64>,
    /// Default minimum strokeWidth for the scale of strokeWidth for rule and line marks and of
    /// size for trail marks with zero=false.
    ///
    /// __Default value:__ `1`
    #[serde(rename = "minStrokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub min_stroke_width: Option<f64>,
    /// Default outer padding for `x` and `y` point-ordinal scales.
    ///
    /// __Default value:__ `0.5` (which makes _width/height = number of unique values * step_)
    #[serde(rename = "pointPadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub point_padding: Option<f64>,
    /// Default range cardinality for
    /// [`quantile`](https://vega.github.io/vega-lite/docs/scale.html#quantile) scale.
    ///
    /// __Default value:__ `4`
    #[serde(rename = "quantileCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub quantile_count: Option<f64>,
    /// Default range cardinality for
    /// [`quantize`](https://vega.github.io/vega-lite/docs/scale.html#quantize) scale.
    ///
    /// __Default value:__ `4`
    #[serde(rename = "quantizeCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub quantize_count: Option<f64>,
    /// Default inner padding for `x` and `y` band-ordinal scales of `"rect"` marks.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "rectBandPaddingInner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub rect_band_padding_inner: Option<f64>,
    /// If true, rounds numeric output values to integers.
    /// This can be helpful for snapping to the pixel grid.
    /// (Only available for `x`, `y`, and `size` scales.)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub round: Option<bool>,
    /// Use the source data range before aggregation as scale domain instead of aggregated data
    /// for aggregate axis.
    ///
    /// This is equivalent to setting `domain` to `"unaggregate"` for aggregated _quantitative_
    /// fields by default.
    ///
    /// This property only works with aggregate functions that produce values within the raw data
    /// domain (`"mean"`, `"average"`, `"median"`, `"q1"`, `"q3"`, `"min"`, `"max"`). For other
    /// aggregations that produce values outside of the raw data domain (e.g. `"count"`,
    /// `"sum"`), this property is ignored.
    ///
    /// __Default value:__ `false`
    #[serde(rename = "useUnaggregatedDomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub use_unaggregated_domain: Option<bool>,
}

/// An object hash for defining default properties for each type of selections.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct SelectionConfig {
    /// The default definition for an
    /// [`interval`](https://vega.github.io/vega-lite/docs/selection.html#type) selection. All
    /// properties and transformations
    /// for an interval selection definition (except `type`) may be specified here.
    ///
    /// For instance, setting `interval` to `{"translate": false}` disables the ability to move
    /// interval selections by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub interval: Option<IntervalSelectionConfig>,
    /// The default definition for a
    /// [`multi`](https://vega.github.io/vega-lite/docs/selection.html#type) selection. All
    /// properties and transformations
    /// for a multi selection definition (except `type`) may be specified here.
    ///
    /// For instance, setting `multi` to `{"toggle": "event.altKey"}` adds additional values to
    /// multi selections when clicking with the alt-key pressed by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub multi: Option<MultiSelectionConfig>,
    /// The default definition for a
    /// [`single`](https://vega.github.io/vega-lite/docs/selection.html#type) selection. All
    /// properties and transformations
    /// for a single selection definition (except `type`) may be specified here.
    ///
    /// For instance, setting `single` to `{"on": "dblclick"}` populates single selections on
    /// double-click by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub single: Option<SingleSelectionConfig>,
}

/// The default definition for an
/// [`interval`](https://vega.github.io/vega-lite/docs/selection.html#type) selection. All
/// properties and transformations
/// for an interval selection definition (except `type`) may be specified here.
///
/// For instance, setting `interval` to `{"translate": false}` disables the ability to move
/// interval selections by default.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct IntervalSelectionConfig {
    /// Establishes a two-way binding between the interval selection and the scales
    /// used within the same view. This allows a user to interactively pan and
    /// zoom the view.
    ///
    /// __See also:__ [`bind`](https://vega.github.io/vega-lite/docs/bind.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bind: Option<Bind>,
    /// Clears the selection, emptying it of all values. Can be a
    /// [Event Stream](https://vega.github.io/vega/docs/event-streams/) or `false` to disable.
    ///
    /// __Default value:__ `dblclick`.
    ///
    /// __See also:__ [`clear`](https://vega.github.io/vega-lite/docs/clear.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub clear: Option<ClearUnion>,
    /// By default, `all` data values are considered to lie within an empty selection.
    /// When set to `none`, empty selections contain no data values.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub empty: Option<Empty>,
    /// An array of encoding channels. The corresponding data field values
    /// must match for a data tuple to fall within the selection.
    ///
    /// __See also:__ [`encodings`](https://vega.github.io/vega-lite/docs/project.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub encodings: Option<Vec<SingleDefUnitChannel>>,
    /// An array of field names whose values must match for a data tuple to
    /// fall within the selection.
    ///
    /// __See also:__ [`fields`](https://vega.github.io/vega-lite/docs/project.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fields: Option<Vec<String>>,
    /// Initialize the selection with a mapping between [projected channels or field
    /// names](https://vega.github.io/vega-lite/docs/project.html) and arrays of
    /// initial values.
    ///
    /// __See also:__ [`init`](https://vega.github.io/vega-lite/docs/init.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub init: Option<HashMap<String, Vec<Equal>>>,
    /// An interval selection also adds a rectangle mark to depict the
    /// extents of the interval. The `mark` property can be used to customize the
    /// appearance of the mark.
    ///
    /// __See also:__ [`mark`](https://vega.github.io/vega-lite/docs/selection-mark.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub mark: Option<BrushConfig>,
    /// A [Vega event stream](https://vega.github.io/vega/docs/event-streams/) (object or
    /// selector) that triggers the selection.
    /// For interval selections, the event stream must specify a [start and
    /// end](https://vega.github.io/vega/docs/event-streams/#between-filters).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub on: Option<OnUnion>,
    /// With layered and multi-view displays, a strategy that determines how
    /// selections' data queries are resolved when applied in a filter transform,
    /// conditional encoding rule, or scale domain.
    ///
    /// __See also:__ [`resolve`](https://vega.github.io/vega-lite/docs/selection-resolve.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub resolve: Option<SelectionResolution>,
    /// When truthy, allows a user to interactively move an interval selection
    /// back-and-forth. Can be `true`, `false` (to disable panning), or a
    /// [Vega event stream definition](https://vega.github.io/vega/docs/event-streams/)
    /// which must include a start and end event to trigger continuous panning.
    ///
    /// __Default value:__ `true`, which corresponds to
    /// `[mousedown, window:mouseup] > window:mousemove!` which corresponds to
    /// clicks and dragging within an interval selection to reposition it.
    ///
    /// __See also:__ [`translate`](https://vega.github.io/vega-lite/docs/translate.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub translate: Option<Translate>,
    /// When truthy, allows a user to interactively resize an interval selection.
    /// Can be `true`, `false` (to disable zooming), or a [Vega event stream
    /// definition](https://vega.github.io/vega/docs/event-streams/). Currently,
    /// only `wheel` events are supported.
    ///
    /// __Default value:__ `true`, which corresponds to `wheel!`.
    ///
    /// __See also:__ [`zoom`](https://vega.github.io/vega-lite/docs/zoom.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub zoom: Option<Translate>,
}

/// The default definition for a
/// [`multi`](https://vega.github.io/vega-lite/docs/selection.html#type) selection. All
/// properties and transformations
/// for a multi selection definition (except `type`) may be specified here.
///
/// For instance, setting `multi` to `{"toggle": "event.altKey"}` adds additional values to
/// multi selections when clicking with the alt-key pressed by default.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct MultiSelectionConfig {
    /// When set, a selection is populated by interacting with the corresponding legend. Direct
    /// manipulation interaction is disabled by default;
    /// to re-enable it, set the selection's
    /// [`on`](https://vega.github.io/vega-lite/docs/selection.html#common-selection-properties)
    /// property.
    ///
    /// Legend bindings are restricted to selections that only specify a single field or encoding.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bind: Option<LegendBinding>,
    /// Clears the selection, emptying it of all values. Can be a
    /// [Event Stream](https://vega.github.io/vega/docs/event-streams/) or `false` to disable.
    ///
    /// __Default value:__ `dblclick`.
    ///
    /// __See also:__ [`clear`](https://vega.github.io/vega-lite/docs/clear.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub clear: Option<ClearUnion>,
    /// By default, `all` data values are considered to lie within an empty selection.
    /// When set to `none`, empty selections contain no data values.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub empty: Option<Empty>,
    /// An array of encoding channels. The corresponding data field values
    /// must match for a data tuple to fall within the selection.
    ///
    /// __See also:__ [`encodings`](https://vega.github.io/vega-lite/docs/project.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub encodings: Option<Vec<SingleDefUnitChannel>>,
    /// An array of field names whose values must match for a data tuple to
    /// fall within the selection.
    ///
    /// __See also:__ [`fields`](https://vega.github.io/vega-lite/docs/project.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fields: Option<Vec<String>>,
    /// Initialize the selection with a mapping between [projected channels or field
    /// names](https://vega.github.io/vega-lite/docs/project.html) and an initial
    /// value (or array of values).
    ///
    /// __See also:__ [`init`](https://vega.github.io/vega-lite/docs/init.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub init: Option<Vec<HashMap<String, Option<SelectionInit>>>>,
    /// When true, an invisible voronoi diagram is computed to accelerate discrete
    /// selection. The data value _nearest_ the mouse cursor is added to the selection.
    ///
    /// __See also:__ [`nearest`](https://vega.github.io/vega-lite/docs/nearest.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub nearest: Option<bool>,
    /// A [Vega event stream](https://vega.github.io/vega/docs/event-streams/) (object or
    /// selector) that triggers the selection.
    /// For interval selections, the event stream must specify a [start and
    /// end](https://vega.github.io/vega/docs/event-streams/#between-filters).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub on: Option<OnUnion>,
    /// With layered and multi-view displays, a strategy that determines how
    /// selections' data queries are resolved when applied in a filter transform,
    /// conditional encoding rule, or scale domain.
    ///
    /// __See also:__ [`resolve`](https://vega.github.io/vega-lite/docs/selection-resolve.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub resolve: Option<SelectionResolution>,
    /// Controls whether data values should be toggled or only ever inserted into
    /// multi selections. Can be `true`, `false` (for insertion only), or a
    /// [Vega expression](https://vega.github.io/vega/docs/expressions/).
    ///
    /// __Default value:__ `true`, which corresponds to `event.shiftKey` (i.e.,
    /// data values are toggled when a user interacts with the shift-key pressed).
    ///
    /// __See also:__ [`toggle`](https://vega.github.io/vega-lite/docs/toggle.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub toggle: Option<Translate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct LegendStreamBinding {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub legend: Option<LegendUnion>,
}

/// The default definition for a
/// [`single`](https://vega.github.io/vega-lite/docs/selection.html#type) selection. All
/// properties and transformations
/// for a single selection definition (except `type`) may be specified here.
///
/// For instance, setting `single` to `{"on": "dblclick"}` populates single selections on
/// double-click by default.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct SingleSelectionConfig {
    /// When set, a selection is populated by input elements (also known as dynamic query
    /// widgets)
    /// or by interacting with the corresponding legend. Direct manipulation interaction is
    /// disabled by default;
    /// to re-enable it, set the selection's
    /// [`on`](https://vega.github.io/vega-lite/docs/selection.html#common-selection-properties)
    /// property.
    ///
    /// Legend bindings are restricted to selections that only specify a single field or
    /// encoding.
    ///
    /// Query widget binding takes the form of Vega's [input element binding
    /// definition](https://vega.github.io/vega/docs/signals/#bind)
    /// or can be a mapping between projected field/encodings and binding definitions.
    ///
    /// __See also:__ [`bind`](https://vega.github.io/vega-lite/docs/bind.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub bind: Option<Binding>,
    /// Clears the selection, emptying it of all values. Can be a
    /// [Event Stream](https://vega.github.io/vega/docs/event-streams/) or `false` to disable.
    ///
    /// __Default value:__ `dblclick`.
    ///
    /// __See also:__ [`clear`](https://vega.github.io/vega-lite/docs/clear.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub clear: Option<ClearUnion>,
    /// By default, `all` data values are considered to lie within an empty selection.
    /// When set to `none`, empty selections contain no data values.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub empty: Option<Empty>,
    /// An array of encoding channels. The corresponding data field values
    /// must match for a data tuple to fall within the selection.
    ///
    /// __See also:__ [`encodings`](https://vega.github.io/vega-lite/docs/project.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub encodings: Option<Vec<SingleDefUnitChannel>>,
    /// An array of field names whose values must match for a data tuple to
    /// fall within the selection.
    ///
    /// __See also:__ [`fields`](https://vega.github.io/vega-lite/docs/project.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fields: Option<Vec<String>>,
    /// Initialize the selection with a mapping between [projected channels or field
    /// names](https://vega.github.io/vega-lite/docs/project.html) and initial values.
    ///
    /// __See also:__ [`init`](https://vega.github.io/vega-lite/docs/init.html) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub init: Option<HashMap<String, Option<SelectionInit>>>,
    /// When true, an invisible voronoi diagram is computed to accelerate discrete
    /// selection. The data value _nearest_ the mouse cursor is added to the selection.
    ///
    /// __See also:__ [`nearest`](https://vega.github.io/vega-lite/docs/nearest.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub nearest: Option<bool>,
    /// A [Vega event stream](https://vega.github.io/vega/docs/event-streams/) (object or
    /// selector) that triggers the selection.
    /// For interval selections, the event stream must specify a [start and
    /// end](https://vega.github.io/vega/docs/event-streams/#between-filters).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub on: Option<OnUnion>,
    /// With layered and multi-view displays, a strategy that determines how
    /// selections' data queries are resolved when applied in a filter transform,
    /// conditional encoding rule, or scale domain.
    ///
    /// __See also:__ [`resolve`](https://vega.github.io/vega-lite/docs/selection-resolve.html)
    /// documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub resolve: Option<SelectionResolution>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct FluffyBinding {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub debounce: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub element: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub input: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub binding_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub labels: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub options: Option<Vec<Option<serde_json::Value>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub step: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub autocomplete: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub between: Option<Vec<Stream>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub consume: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub filter: Option<PurpleText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub markname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub marktype: Option<MarkType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub source: Option<Source>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub throttle: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stream: Option<Stream>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub merge: Option<Vec<Stream>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct BaseMarkConfig {
    /// The horizontal alignment of the text or ranged marks (area, bar, image, rect, rule). One
    /// of `"left"`, `"right"`, `"center"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub align: Option<Align>,
    /// The rotation angle of the text, in degrees.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub angle: Option<f64>,
    /// Whether to keep aspect ratio of image marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aspect: Option<bool>,
    /// The vertical alignment of the text or ranged marks (area, bar, image, rect, rule). One of
    /// `"top"`, `"middle"`, `"bottom"`.
    ///
    /// __Default value:__ `"middle"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub baseline: Option<Baseline>,
    /// The radius in pixels of rounded rectangle corners.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadius")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius: Option<f64>,
    /// The radius in pixels of rounded rectangle bottom left corner.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadiusBottomLeft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius_bottom_left: Option<f64>,
    /// The radius in pixels of rounded rectangle bottom right corner.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadiusBottomRight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius_bottom_right: Option<f64>,
    /// The radius in pixels of rounded rectangle top right corner.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadiusTopLeft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius_top_left: Option<f64>,
    /// The radius in pixels of rounded rectangle top left corner.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadiusTopRight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius_top_right: Option<f64>,
    /// The mouse cursor used over the mark. Any valid [CSS cursor
    /// type](https://developer.mozilla.org/en-US/docs/Web/CSS/cursor#Values) can be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub cursor: Option<Cursor>,
    /// The direction of the text. One of `"ltr"` (left-to-right) or `"rtl"` (right-to-left).
    /// This property determines on which side is truncated in response to the limit parameter.
    ///
    /// __Default value:__ `"ltr"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dir: Option<Dir>,
    /// The horizontal offset, in pixels, between the text label and its anchor point. The offset
    /// is applied after rotation by the _angle_ property.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dx: Option<f64>,
    /// The vertical offset, in pixels, between the text label and its anchor point. The offset
    /// is applied after rotation by the _angle_ property.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dy: Option<f64>,
    /// The ellipsis string for text truncated in response to the limit parameter.
    ///
    /// __Default value:__ `"…"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ellipsis: Option<String>,
    /// Default Fill Color. This has higher precedence than `config.color`.
    ///
    /// __Default value:__ (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fill: Option<FillUnion>,
    /// The fill opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "fillOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fill_opacity: Option<f64>,
    /// The typeface to set the text in (e.g., `"Helvetica Neue"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font: Option<String>,
    /// The font size, in pixels.
    #[serde(rename = "fontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font_size: Option<f64>,
    /// The font style (e.g., `"italic"`).
    #[serde(rename = "fontStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font_style: Option<String>,
    /// The font weight.
    /// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
    /// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
    #[serde(rename = "fontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font_weight: Option<FontWeight>,
    /// Height of the marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub height: Option<f64>,
    /// A URL to load upon mouse click. If defined, the mark acts as a hyperlink.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub href: Option<String>,
    /// The line interpolation method to use for line and area marks. One of the following:
    /// - `"linear"`: piecewise linear segments, as in a polyline.
    /// - `"linear-closed"`: close the linear segments to form a polygon.
    /// - `"step"`: alternate between horizontal and vertical segments, as in a step function.
    /// - `"step-before"`: alternate between vertical and horizontal segments, as in a step
    /// function.
    /// - `"step-after"`: alternate between horizontal and vertical segments, as in a step
    /// function.
    /// - `"basis"`: a B-spline, with control point duplication on the ends.
    /// - `"basis-open"`: an open B-spline; may not intersect the start or end.
    /// - `"basis-closed"`: a closed B-spline, as in a loop.
    /// - `"cardinal"`: a Cardinal spline, with control point duplication on the ends.
    /// - `"cardinal-open"`: an open Cardinal spline; may not intersect the start or end, but
    /// will intersect other control points.
    /// - `"cardinal-closed"`: a closed Cardinal spline, as in a loop.
    /// - `"bundle"`: equivalent to basis, except the tension parameter is used to straighten the
    /// spline.
    /// - `"monotone"`: cubic interpolation that preserves monotonicity in y.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub interpolate: Option<Interpolate>,
    /// The maximum length of the text mark in pixels. The text value will be automatically
    /// truncated if the rendered size exceeds the limit.
    ///
    /// __Default value:__ `0`, indicating no limit
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub limit: Option<f64>,
    /// A delimiter, such as a newline character, upon which to break text strings into multiple
    /// lines. This property will be ignored if the text property is array-valued.
    #[serde(rename = "lineBreak")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub line_break: Option<String>,
    /// The height, in pixels, of each line of text in a multi-line text mark.
    #[serde(rename = "lineHeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub line_height: Option<f64>,
    /// The overall opacity (value between [0,1]).
    ///
    /// __Default value:__ `0.7` for non-aggregate plots with `point`, `tick`, `circle`, or
    /// `square` marks or layered `bar` charts and `1` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub opacity: Option<f64>,
    /// The orientation of a non-stacked bar, tick, area, and line charts.
    /// The value is either horizontal (default) or vertical.
    /// - For bar, rule and tick, this determines whether the size of the bar and tick
    /// should be applied to x or y dimension.
    /// - For area, this property determines the orient property of the Vega output.
    /// - For line and trail marks, this property determines the sort order of the points in the
    /// line
    /// if `config.sortLineBy` is not specified.
    /// For stacked charts, this is always determined by the orientation of the stack;
    /// therefore explicitly specified value will be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub orient: Option<Orientation>,
    /// Polar coordinate radial offset, in pixels, of the text label from the origin determined
    /// by the `x` and `y` properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub radius: Option<f64>,
    /// Shape of the point marks. Supported values include:
    /// - plotting shapes: `"circle"`, `"square"`, `"cross"`, `"diamond"`, `"triangle-up"`,
    /// `"triangle-down"`, `"triangle-right"`, or `"triangle-left"`.
    /// - the line symbol `"stroke"`
    /// - centered directional shapes `"arrow"`, `"wedge"`, or `"triangle"`
    /// - a custom [SVG path
    /// string](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths) (For correct
    /// sizing, custom shape paths should be defined within a square bounding box with
    /// coordinates ranging from -1 to 1 along both the x and y dimensions.)
    ///
    /// __Default value:__ `"circle"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub shape: Option<String>,
    /// The pixel area each the point/circle/square.
    /// For example: in the case of circles, the radius is determined in part by the square root
    /// of the size value.
    ///
    /// __Default value:__ `30`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub size: Option<f64>,
    /// Default Stroke Color. This has higher precedence than `config.color`.
    ///
    /// __Default value:__ (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke: Option<FillUnion>,
    /// The stroke cap for line ending style. One of `"butt"`, `"round"`, or `"square"`.
    ///
    /// __Default value:__ `"square"`
    #[serde(rename = "strokeCap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_cap: Option<StrokeCap>,
    /// An array of alternating stroke, space lengths for creating dashed or dotted lines.
    #[serde(rename = "strokeDash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_dash: Option<Vec<f64>>,
    /// The offset (in pixels) into which to begin drawing with the stroke dash array.
    #[serde(rename = "strokeDashOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_dash_offset: Option<f64>,
    /// The stroke line join method. One of `"miter"`, `"round"` or `"bevel"`.
    ///
    /// __Default value:__ `"miter"`
    #[serde(rename = "strokeJoin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_join: Option<StrokeJoin>,
    /// The miter limit at which to bevel a line join.
    #[serde(rename = "strokeMiterLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_miter_limit: Option<f64>,
    /// The stroke opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "strokeOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_opacity: Option<f64>,
    /// The stroke width, in pixels.
    #[serde(rename = "strokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_width: Option<f64>,
    /// Depending on the interpolation type, sets the tension parameter (for line and area marks).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tension: Option<f64>,
    /// Placeholder text if the `text` channel is not specified
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub text: Option<PurpleText>,
    /// Polar coordinate angle, in radians, of the text label from the origin determined by the
    /// `x` and `y` properties. Values for `theta` follow the same convention of `arc` mark
    /// `startAngle` and `endAngle` properties: angles are measured in radians, with `0`
    /// indicating "north".
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub theta: Option<f64>,
    /// The tooltip text to show upon mouse hover.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub tooltip: RemovableValue<serde_json::Value>,
    /// Width of the marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub width: Option<f64>,
    /// X coordinates of the marks, or width of horizontal `"bar"` and `"area"` without specified
    /// `x2` or `width`.
    ///
    /// The `value` of this channel can be a number or a string `"width"` for the width of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub x: Option<XUnion>,
    /// X2 coordinates for ranged `"area"`, `"bar"`, `"rect"`, and  `"rule"`.
    ///
    /// The `value` of this channel can be a number or a string `"width"` for the width of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub x2: Option<XUnion>,
    /// Y coordinates of the marks, or height of vertical `"bar"` and `"area"` without specified
    /// `y2` or `height`.
    ///
    /// The `value` of this channel can be a number or a string `"height"` for the height of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub y: Option<YUnion>,
    /// Y2 coordinates for ranged `"area"`, `"bar"`, `"rect"`, and  `"rule"`.
    ///
    /// The `value` of this channel can be a number or a string `"height"` for the height of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub y2: Option<YUnion>,
}

/// Tick-Specific Config
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct TickConfig {
    /// The horizontal alignment of the text or ranged marks (area, bar, image, rect, rule). One
    /// of `"left"`, `"right"`, `"center"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub align: Option<Align>,
    /// The rotation angle of the text, in degrees.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub angle: Option<f64>,
    /// Whether to keep aspect ratio of image marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub aspect: Option<bool>,
    /// The width of the ticks.
    ///
    /// __Default value:__  3/4 of step (width step for horizontal ticks and height step for
    /// vertical ticks).
    #[serde(rename = "bandSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub band_size: Option<f64>,
    /// The vertical alignment of the text or ranged marks (area, bar, image, rect, rule). One of
    /// `"top"`, `"middle"`, `"bottom"`.
    ///
    /// __Default value:__ `"middle"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub baseline: Option<Baseline>,
    /// Default color.
    ///
    /// __Default value:__ <span style="color: #4682b4;">&#9632;</span> `"#4682b4"`
    ///
    /// __Note:__
    /// - This property cannot be used in a [style
    /// config](https://vega.github.io/vega-lite/docs/mark.html#style-config).
    /// - The `fill` and `stroke` properties have higher precedence than `color` and will
    /// override `color`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub color: Option<ColorUnion>,
    /// The radius in pixels of rounded rectangle corners.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadius")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius: Option<f64>,
    /// The radius in pixels of rounded rectangle bottom left corner.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadiusBottomLeft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius_bottom_left: Option<f64>,
    /// The radius in pixels of rounded rectangle bottom right corner.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadiusBottomRight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius_bottom_right: Option<f64>,
    /// The radius in pixels of rounded rectangle top right corner.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadiusTopLeft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius_top_left: Option<f64>,
    /// The radius in pixels of rounded rectangle top left corner.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadiusTopRight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius_top_right: Option<f64>,
    /// The mouse cursor used over the mark. Any valid [CSS cursor
    /// type](https://developer.mozilla.org/en-US/docs/Web/CSS/cursor#Values) can be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub cursor: Option<Cursor>,
    /// The direction of the text. One of `"ltr"` (left-to-right) or `"rtl"` (right-to-left).
    /// This property determines on which side is truncated in response to the limit parameter.
    ///
    /// __Default value:__ `"ltr"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dir: Option<Dir>,
    /// The horizontal offset, in pixels, between the text label and its anchor point. The offset
    /// is applied after rotation by the _angle_ property.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dx: Option<f64>,
    /// The vertical offset, in pixels, between the text label and its anchor point. The offset
    /// is applied after rotation by the _angle_ property.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dy: Option<f64>,
    /// The ellipsis string for text truncated in response to the limit parameter.
    ///
    /// __Default value:__ `"…"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub ellipsis: Option<String>,
    /// Default Fill Color. This has higher precedence than `config.color`.
    ///
    /// __Default value:__ (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fill: Option<FillUnion>,
    /// Whether the mark's color should be used as fill color instead of stroke color.
    ///
    /// __Default value:__ `false` for all `point`, `line`, and `rule` marks as well as
    /// `geoshape` marks for
    /// [`graticule`](https://vega.github.io/vega-lite/docs/data.html#graticule) data sources;
    /// otherwise, `true`.
    ///
    /// __Note:__ This property cannot be used in a [style
    /// config](https://vega.github.io/vega-lite/docs/mark.html#style-config).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub filled: Option<bool>,
    /// The fill opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "fillOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fill_opacity: Option<f64>,
    /// The typeface to set the text in (e.g., `"Helvetica Neue"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font: Option<String>,
    /// The font size, in pixels.
    #[serde(rename = "fontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font_size: Option<f64>,
    /// The font style (e.g., `"italic"`).
    #[serde(rename = "fontStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font_style: Option<String>,
    /// The font weight.
    /// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
    /// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
    #[serde(rename = "fontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font_weight: Option<FontWeight>,
    /// Height of the marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub height: Option<f64>,
    /// A URL to load upon mouse click. If defined, the mark acts as a hyperlink.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub href: Option<String>,
    /// The line interpolation method to use for line and area marks. One of the following:
    /// - `"linear"`: piecewise linear segments, as in a polyline.
    /// - `"linear-closed"`: close the linear segments to form a polygon.
    /// - `"step"`: alternate between horizontal and vertical segments, as in a step function.
    /// - `"step-before"`: alternate between vertical and horizontal segments, as in a step
    /// function.
    /// - `"step-after"`: alternate between horizontal and vertical segments, as in a step
    /// function.
    /// - `"basis"`: a B-spline, with control point duplication on the ends.
    /// - `"basis-open"`: an open B-spline; may not intersect the start or end.
    /// - `"basis-closed"`: a closed B-spline, as in a loop.
    /// - `"cardinal"`: a Cardinal spline, with control point duplication on the ends.
    /// - `"cardinal-open"`: an open Cardinal spline; may not intersect the start or end, but
    /// will intersect other control points.
    /// - `"cardinal-closed"`: a closed Cardinal spline, as in a loop.
    /// - `"bundle"`: equivalent to basis, except the tension parameter is used to straighten the
    /// spline.
    /// - `"monotone"`: cubic interpolation that preserves monotonicity in y.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub interpolate: Option<Interpolate>,
    /// Defines how Vega-Lite should handle marks for invalid values (`null` and `NaN`).
    /// - If set to `"filter"` (default), all data items with null values will be skipped (for
    /// line, trail, and area marks) or filtered (for other marks).
    /// - If `null`, all data items are included. In this case, invalid values will be
    /// interpreted as zeroes.
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub invalid: RemovableValue<Invalid>,
    /// The maximum length of the text mark in pixels. The text value will be automatically
    /// truncated if the rendered size exceeds the limit.
    ///
    /// __Default value:__ `0`, indicating no limit
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub limit: Option<f64>,
    /// A delimiter, such as a newline character, upon which to break text strings into multiple
    /// lines. This property will be ignored if the text property is array-valued.
    #[serde(rename = "lineBreak")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub line_break: Option<String>,
    /// The height, in pixels, of each line of text in a multi-line text mark.
    #[serde(rename = "lineHeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub line_height: Option<f64>,
    /// The overall opacity (value between [0,1]).
    ///
    /// __Default value:__ `0.7` for non-aggregate plots with `point`, `tick`, `circle`, or
    /// `square` marks or layered `bar` charts and `1` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub opacity: Option<f64>,
    /// For line and trail marks, this `order` property can be set to `null` or `false` to make
    /// the lines use the original order in the data sources.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub order: Option<bool>,
    /// The orientation of a non-stacked bar, tick, area, and line charts.
    /// The value is either horizontal (default) or vertical.
    /// - For bar, rule and tick, this determines whether the size of the bar and tick
    /// should be applied to x or y dimension.
    /// - For area, this property determines the orient property of the Vega output.
    /// - For line and trail marks, this property determines the sort order of the points in the
    /// line
    /// if `config.sortLineBy` is not specified.
    /// For stacked charts, this is always determined by the orientation of the stack;
    /// therefore explicitly specified value will be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub orient: Option<Orientation>,
    /// Polar coordinate radial offset, in pixels, of the text label from the origin determined
    /// by the `x` and `y` properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub radius: Option<f64>,
    /// Shape of the point marks. Supported values include:
    /// - plotting shapes: `"circle"`, `"square"`, `"cross"`, `"diamond"`, `"triangle-up"`,
    /// `"triangle-down"`, `"triangle-right"`, or `"triangle-left"`.
    /// - the line symbol `"stroke"`
    /// - centered directional shapes `"arrow"`, `"wedge"`, or `"triangle"`
    /// - a custom [SVG path
    /// string](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths) (For correct
    /// sizing, custom shape paths should be defined within a square bounding box with
    /// coordinates ranging from -1 to 1 along both the x and y dimensions.)
    ///
    /// __Default value:__ `"circle"`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub shape: Option<String>,
    /// Default size for marks.
    /// - For `point`/`circle`/`square`, this represents the pixel area of the marks. For
    /// example: in the case of circles, the radius is determined in part by the square root of
    /// the size value.
    /// - For `bar`, this represents the band size of the bar, in pixels.
    /// - For `text`, this represents the font size, in pixels.
    ///
    /// __Default value:__
    /// - `30` for point, circle, square marks; width/height's `step`
    /// - `2` for bar marks with discrete dimensions;
    /// - `5` for bar marks with continuous dimensions;
    /// - `11` for text marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub size: Option<f64>,
    /// Default Stroke Color. This has higher precedence than `config.color`.
    ///
    /// __Default value:__ (None)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke: Option<FillUnion>,
    /// The stroke cap for line ending style. One of `"butt"`, `"round"`, or `"square"`.
    ///
    /// __Default value:__ `"square"`
    #[serde(rename = "strokeCap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_cap: Option<StrokeCap>,
    /// An array of alternating stroke, space lengths for creating dashed or dotted lines.
    #[serde(rename = "strokeDash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_dash: Option<Vec<f64>>,
    /// The offset (in pixels) into which to begin drawing with the stroke dash array.
    #[serde(rename = "strokeDashOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_dash_offset: Option<f64>,
    /// The stroke line join method. One of `"miter"`, `"round"` or `"bevel"`.
    ///
    /// __Default value:__ `"miter"`
    #[serde(rename = "strokeJoin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_join: Option<StrokeJoin>,
    /// The miter limit at which to bevel a line join.
    #[serde(rename = "strokeMiterLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_miter_limit: Option<f64>,
    /// The stroke opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "strokeOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_opacity: Option<f64>,
    /// The stroke width, in pixels.
    #[serde(rename = "strokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_width: Option<f64>,
    /// Depending on the interpolation type, sets the tension parameter (for line and area marks).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tension: Option<f64>,
    /// Placeholder text if the `text` channel is not specified
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub text: Option<PurpleText>,
    /// Polar coordinate angle, in radians, of the text label from the origin determined by the
    /// `x` and `y` properties. Values for `theta` follow the same convention of `arc` mark
    /// `startAngle` and `endAngle` properties: angles are measured in radians, with `0`
    /// indicating "north".
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub theta: Option<f64>,
    /// Thickness of the tick mark.
    ///
    /// __Default value:__  `1`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub thickness: Option<f64>,
    /// Default relative band size for a time unit. If set to `1`, the bandwidth of the marks
    /// will be equal to the time unit band step.
    /// If set to `0.5`, bandwidth of the marks will be half of the time unit band step.
    #[serde(rename = "timeUnitBand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit_band: Option<f64>,
    /// Default relative band position for a time unit. If set to `0`, the marks will be
    /// positioned at the beginning of the time unit band step.
    /// If set to `0.5`, the marks will be positioned in the middle of the time unit band step.
    #[serde(rename = "timeUnitBandPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_unit_band_position: Option<f64>,
    /// The tooltip text string to show upon mouse hover or an object defining which fields
    /// should the tooltip be derived from.
    ///
    /// - If `tooltip` is `true` or `{"content": "encoding"}`, then all fields from `encoding`
    /// will be used.
    /// - If `tooltip` is `{"content": "data"}`, then all fields that appear in the highlighted
    /// data point will be used.
    /// - If set to `null` or `false`, then no tooltip will be used.
    ///
    /// See the [`tooltip`](https://vega.github.io/vega-lite/docs/tooltip.html) documentation for
    /// a detailed discussion about tooltip  in Vega-Lite.
    ///
    /// __Default value:__ `null`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tooltip: Option<Value>,
    /// Width of the marks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub width: Option<f64>,
    /// X coordinates of the marks, or width of horizontal `"bar"` and `"area"` without specified
    /// `x2` or `width`.
    ///
    /// The `value` of this channel can be a number or a string `"width"` for the width of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub x: Option<XUnion>,
    /// X2 coordinates for ranged `"area"`, `"bar"`, `"rect"`, and  `"rule"`.
    ///
    /// The `value` of this channel can be a number or a string `"width"` for the width of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub x2: Option<XUnion>,
    /// Y coordinates of the marks, or height of vertical `"bar"` and `"area"` without specified
    /// `y2` or `height`.
    ///
    /// The `value` of this channel can be a number or a string `"height"` for the height of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub y: Option<YUnion>,
    /// Y2 coordinates for ranged `"area"`, `"bar"`, `"rect"`, and  `"rule"`.
    ///
    /// The `value` of this channel can be a number or a string `"height"` for the height of the
    /// plot.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub y2: Option<YUnion>,
}

/// Title configuration, which determines default properties for all
/// [titles](https://vega.github.io/vega-lite/docs/title.html). For a full list of title
/// configuration options, please see the [corresponding section of the title
/// documentation](https://vega.github.io/vega-lite/docs/title.html#config).
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ExcludeMappedValueRefBaseTitle {
    /// Horizontal text alignment for title text. One of `"left"`, `"center"`, or `"right"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub align: Option<Align>,
    /// The anchor position for placing the title and subtitle text. One of `"start"`,
    /// `"middle"`, or `"end"`. For example, with an orientation of top these anchor positions
    /// map to a left-, center-, or right-aligned title.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub anchor: Option<TitleAnchorEnum>,
    /// Angle in degrees of title and subtitle text.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub angle: Option<f64>,
    /// Vertical text baseline for title and subtitle text. One of `"top"`, `"middle"`,
    /// `"bottom"`, or `"alphabetic"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub baseline: Option<Baseline>,
    /// Text color for title text.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub color: Option<String>,
    /// Delta offset for title and subtitle text x-coordinate.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dx: Option<f64>,
    /// Delta offset for title and subtitle text y-coordinate.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub dy: Option<f64>,
    /// Font name for title text.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font: Option<String>,
    /// Font size in pixels for title text.
    #[serde(rename = "fontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font_size: Option<f64>,
    /// Font style for title text.
    #[serde(rename = "fontStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font_style: Option<String>,
    /// Font weight for title text.
    /// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
    /// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
    #[serde(rename = "fontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub font_weight: Option<FontWeight>,
    /// The reference frame for the anchor position, one of `"bounds"` (to anchor relative to the
    /// full bounding box) or `"group"` (to anchor relative to the group width or height).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub frame: Option<String>,
    /// The maximum allowed length in pixels of title and subtitle text.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub limit: Option<f64>,
    /// Line height in pixels for multi-line title text.
    #[serde(rename = "lineHeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub line_height: Option<f64>,
    /// The orthogonal offset in pixels by which to displace the title group from its position
    /// along the edge of the chart.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub offset: Option<f64>,
    /// Default title orientation (`"top"`, `"bottom"`, `"left"`, or `"right"`)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub orient: Option<TitleOrient>,
    /// Text color for subtitle text.
    #[serde(rename = "subtitleColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub subtitle_color: Option<String>,
    /// Font name for subtitle text.
    #[serde(rename = "subtitleFont")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub subtitle_font: Option<String>,
    /// Font size in pixels for subtitle text.
    #[serde(rename = "subtitleFontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub subtitle_font_size: Option<f64>,
    /// Font style for subtitle text.
    #[serde(rename = "subtitleFontStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub subtitle_font_style: Option<String>,
    /// Font weight for subtitle text.
    /// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
    /// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
    #[serde(rename = "subtitleFontWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub subtitle_font_weight: Option<FontWeight>,
    /// Line height in pixels for multi-line subtitle text.
    #[serde(rename = "subtitleLineHeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub subtitle_line_height: Option<f64>,
    /// The padding in pixels between title and subtitle text.
    #[serde(rename = "subtitlePadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub subtitle_padding: Option<f64>,
}

/// Default properties for [single view
/// plots](https://vega.github.io/vega-lite/docs/spec.html#single).
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct ViewConfig {
    /// Whether the view should be clipped.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub clip: Option<bool>,
    /// The default height when the plot has a continuous y-field.
    ///
    /// __Default value:__ `200`
    #[serde(rename = "continuousHeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub continuous_height: Option<f64>,
    /// The default width when the plot has a continuous x-field.
    ///
    /// __Default value:__ `200`
    #[serde(rename = "continuousWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub continuous_width: Option<f64>,
    /// The radius in pixels of rounded rectangle corners.
    ///
    /// __Default value:__ `0`
    #[serde(rename = "cornerRadius")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub corner_radius: Option<f64>,
    /// The default height when the plot has either a discrete y-field or no y-field.
    ///
    /// __Default value:__ a step size based on `config.view.step`.
    #[serde(rename = "discreteHeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub discrete_height: Option<DiscreteHeightUnion>,
    /// The default width when the plot has either a discrete x-field or no x-field.
    ///
    /// __Default value:__ a step size based on `config.view.step`.
    #[serde(rename = "discreteWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub discrete_width: Option<DiscreteWidthUnion>,
    /// The fill color.
    ///
    /// __Default value:__ `undefined`
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fill: Option<String>,
    /// The fill opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "fillOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fill_opacity: Option<f64>,
    /// Default height
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub height: Option<f64>,
    /// The overall opacity (value between [0,1]).
    ///
    /// __Default value:__ `0.7` for non-aggregate plots with `point`, `tick`, `circle`, or
    /// `square` marks or layered `bar` charts and `1` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub opacity: Option<f64>,
    /// Default step size for x-/y- discrete fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub step: Option<f64>,
    /// The stroke color.
    ///
    /// __Default value:__ `"#ddd"`
    #[serde(default, skip_serializing_if = "RemovableValue::is_default")]
    #[builder(default)]
    pub stroke: RemovableValue<String>,
    /// The stroke cap for line ending style. One of `"butt"`, `"round"`, or `"square"`.
    ///
    /// __Default value:__ `"square"`
    #[serde(rename = "strokeCap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_cap: Option<StrokeCap>,
    /// An array of alternating stroke, space lengths for creating dashed or dotted lines.
    #[serde(rename = "strokeDash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_dash: Option<Vec<f64>>,
    /// The offset (in pixels) into which to begin drawing with the stroke dash array.
    #[serde(rename = "strokeDashOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_dash_offset: Option<f64>,
    /// The stroke line join method. One of `"miter"`, `"round"` or `"bevel"`.
    ///
    /// __Default value:__ `"miter"`
    #[serde(rename = "strokeJoin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_join: Option<StrokeJoin>,
    /// The miter limit at which to bevel a line join.
    #[serde(rename = "strokeMiterLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_miter_limit: Option<f64>,
    /// The stroke opacity (value between [0,1]).
    ///
    /// __Default value:__ `1`
    #[serde(rename = "strokeOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_opacity: Option<f64>,
    /// The stroke width, in pixels.
    #[serde(rename = "strokeWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub stroke_width: Option<f64>,
    /// Default width
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub width: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct DiscreteHeightClass {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub step: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct DiscreteWidthClass {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub step: Option<f64>,
}

/// The alignment to apply to grid rows and columns.
/// The supported string values are `"all"`, `"each"`, and `"none"`.
///
/// - For `"none"`, a flow layout will be used, in which adjacent subviews are simply placed
/// one after the other.
/// - For `"each"`, subviews will be aligned into a clean grid structure, but each row or
/// column may be of variable size.
/// - For `"all"`, subviews will be aligned and each row or column will be sized identically
/// based on the maximum observed size. String values for this property will be applied to
/// both grid rows and columns.
///
/// Alternatively, an object value of the form `{"row": string, "column": string}` can be
/// used to supply different alignments for rows and columns.
///
/// __Default value:__ `"all"`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum AlignUnion {
    Enum(LayoutAlign),
    RowColLayoutAlign(RowColLayoutAlign),
}

/// How the visualization size should be determined. If a string, should be one of `"pad"`,
/// `"fit"` or `"none"`.
/// Object values can additionally specify parameters for content sizing and automatic
/// resizing.
///
/// __Default value__: `pad`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum Autosize {
    AutoSizeParams(AutoSizeParams),
    Enum(AutosizeType),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum VegaliteCenter {
    Bool(bool),
    RowColBoolean(RowColBoolean),
}

/// Generate graticule GeoJSON data for geographic reference lines.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum Graticule {
    Bool(bool),
    GraticuleParams(GraticuleParams),
}

/// Generate sphere GeoJSON data for the full globe.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum SphereUnion {
    Bool(bool),
    SphereClass(SphereClass),
}

/// The full data set, included inline. This can be an array of objects or primitive values,
/// an object, or a string.
/// Arrays of primitive values are ingested as objects with a `data` property. Strings are
/// parsed according to the specified format type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum UrlDataInlineDataset {
    AnythingMap(HashMap<String, Option<serde_json::Value>>),
    String(String),
    UnionArray(Vec<serde_json::value::Value>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
#[allow(unused)]
enum UnusedInlineDataset {
    AnythingMap(HashMap<String, Option<serde_json::Value>>),
    Bool(bool),
    Double(f64),
    String(String),
}

/// Aggregation function for the field
/// (e.g., `"mean"`, `"sum"`, `"median"`, `"min"`, `"max"`, `"count"`).
///
/// __Default value:__ `undefined` (None)
///
/// __See also:__ [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html)
/// documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum Aggregate {
    ArgmDef(ArgmDef),
    Enum(NonArgAggregateOp),
}

/// An object indicating bin properties, or simply `true` for using default bin parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum PurpleBin {
    BinParams(BinParams),
    Bool(bool),
}

/// A two-element (`[min, max]`) array indicating the range of desired bin values.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum BinExtent {
    BinExtentClass(BinExtentClass),
    DoubleArray(Vec<f64>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum ColorCondition {
    ConditionalPredicateValueDefGradientStringNullClass(
        ConditionalPredicateValueDefGradientStringNullClass,
    ),
    ConditionalValueDefGradientStringNullArray(Vec<ConditionalValueDefGradientStringNull>),
}

/// Filter using a selection name.
///
/// A [selection name](https://vega.github.io/vega-lite/docs/selection.html), or a series of
/// [composed selections](https://vega.github.io/vega-lite/docs/selection.html#compose).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum SelectionOperandElement {
    Selection(Selection),
    String(String),
}

/// Filter using a selection name.
///
/// A [selection name](https://vega.github.io/vega-lite/docs/selection.html), or a series of
/// [composed selections](https://vega.github.io/vega-lite/docs/selection.html#compose).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum PurpleSelectionOperand {
    Selection(Selection),
    String(String),
}

/// Predicate for triggering the condition
///
/// The `filter` property must be one of the predicate definitions:
///
/// 1) an [expression](https://vega.github.io/vega-lite/docs/types.html#expression) string,
/// where `datum` can be used to refer to the current data object
///
/// 2) one of the field predicates:
/// [`equal`](https://vega.github.io/vega-lite/docs/filter.html#equal-predicate),
/// [`lt`](https://vega.github.io/vega-lite/docs/filter.html#lt-predicate),
/// [`lte`](https://vega.github.io/vega-lite/docs/filter.html#lte-predicate),
/// [`gt`](https://vega.github.io/vega-lite/docs/filter.html#gt-predicate),
/// [`gte`](https://vega.github.io/vega-lite/docs/filter.html#gte-predicate),
/// [`range`](https://vega.github.io/vega-lite/docs/filter.html#range-predicate),
/// [`oneOf`](https://vega.github.io/vega-lite/docs/filter.html#one-of-predicate),
/// or [`valid`](https://vega.github.io/vega-lite/docs/filter.html#valid-predicate),
///
/// 3) a [selection
/// predicate](https://vega.github.io/vega-lite/docs/filter.html#selection-predicate)
///
/// 4) a logical operand that combines (1), (2), or (3).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum LogicalOperandPredicateElement {
    Predicate(Box<Predicate>),
    String(String),
}

/// Predicate for triggering the condition
///
/// The `filter` property must be one of the predicate definitions:
///
/// 1) an [expression](https://vega.github.io/vega-lite/docs/types.html#expression) string,
/// where `datum` can be used to refer to the current data object
///
/// 2) one of the field predicates:
/// [`equal`](https://vega.github.io/vega-lite/docs/filter.html#equal-predicate),
/// [`lt`](https://vega.github.io/vega-lite/docs/filter.html#lt-predicate),
/// [`lte`](https://vega.github.io/vega-lite/docs/filter.html#lte-predicate),
/// [`gt`](https://vega.github.io/vega-lite/docs/filter.html#gt-predicate),
/// [`gte`](https://vega.github.io/vega-lite/docs/filter.html#gte-predicate),
/// [`range`](https://vega.github.io/vega-lite/docs/filter.html#range-predicate),
/// [`oneOf`](https://vega.github.io/vega-lite/docs/filter.html#one-of-predicate),
/// or [`valid`](https://vega.github.io/vega-lite/docs/filter.html#valid-predicate),
///
/// 3) a [selection
/// predicate](https://vega.github.io/vega-lite/docs/filter.html#selection-predicate)
///
/// 4) a logical operand that combines (1), (2), or (3).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum PurpleLogicalOperandPredicate {
    Predicate(Box<Predicate>),
    String(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum SelectionInit {
    Bool(bool),
    DateTime(DateTime),
    Double(f64),
    String(String),
}

/// Value representing the day of a week. This can be one of:
/// (1) integer value -- `1` represents Monday;
/// (2) case-insensitive day name (e.g., `"Monday"`);
/// (3) case-insensitive, 3-character short day name (e.g., `"Mon"`).
///
/// **Warning:** A DateTime definition object with `day`** should not be combined with
/// `year`, `quarter`, `month`, or `date`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum Day {
    Double(f64),
    String(String),
}

/// One of:
/// (1) integer value representing the month from `1`-`12`. `1` represents January;
/// (2) case-insensitive month name (e.g., `"January"`);
/// (3) case-insensitive, 3-character short month name (e.g., `"Jan"`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum Month {
    Double(f64),
    String(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum Lt {
    DateTime(DateTime),
    Double(f64),
    String(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum Equal {
    Bool(bool),
    DateTime(DateTime),
    Double(f64),
    String(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum PurpleRange {
    DateTime(DateTime),
    Double(f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum ValueUnion {
    String(String),
    ValueLinearGradient(ValueLinearGradient),
}

/// __Required.__ A string defining the name of the field from which to pull a data value
/// or an object defining iterated values from the
/// [`repeat`](https://vega.github.io/vega-lite/docs/repeat.html) operator.
///
/// __See also:__ [`field`](https://vega.github.io/vega-lite/docs/field.html) documentation.
///
/// __Notes:__
/// 1)  Dots (`.`) and brackets (`[` and `]`) can be used to access nested objects (e.g.,
/// `"field": "foo.bar"` and `"field": "foo['bar']"`).
/// If field names contain dots or brackets but are not nested, you can use `\\` to escape
/// dots and brackets (e.g., `"a\\.b"` and `"a\\[0\\]"`).
/// See more details about escaping in the [field
/// documentation](https://vega.github.io/vega-lite/docs/field.html).
/// 2) `field` is not required if `aggregate` is `count`.
///
/// The data [field](https://vega.github.io/vega-lite/docs/field.html) to sort by.
///
/// __Default value:__ If unspecified, defaults to the field specified in the outer data
/// reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum Field {
    RepeatRef(RepeatRef),
    String(String),
}

/// The font weight.
/// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
/// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
///
/// Font weight of axis tick labels.
///
/// Font weight of the title.
/// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
/// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
///
/// Font weight of the header title.
/// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
/// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
///
/// The font weight of legend label.
///
/// The font weight of the legend title.
/// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
/// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
///
/// Font weight for title text.
/// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
/// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
///
/// Font weight for subtitle text.
/// This can be either a string (e.g `"bold"`, `"normal"`) or a number (`100`, `200`, `300`,
/// ..., `900` where `"normal"` = `400` and `"bold"` = `700`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum FontWeight {
    Double(f64),
    Enum(FontWeightEnum),
}

/// The strategy to use for resolving overlap of axis labels. If `false` (the default), no
/// overlap reduction is attempted. If set to `true` or `"parity"`, a strategy of removing
/// every other label is used (this works well for standard linear axes). If set to
/// `"greedy"`, a linear scan of the labels is performed, removing any labels that overlaps
/// with the last visible label (this often works better for log-scaled axes).
///
/// __Default value:__ `true` for non-nominal fields with non-log scales; `"greedy"` for log
/// scales; otherwise `false`.
///
/// The strategy to use for resolving overlap of labels in gradient legends. If `false`, no
/// overlap reduction is attempted. If set to `true` or `"parity"`, a strategy of removing
/// every other label is used. If set to `"greedy"`, a linear scan of the labels is
/// performed, removing any label that overlaps with the last visible label (this often works
/// better for log-scaled axes).
///
/// __Default value:__ `"greedy"` for `log scales otherwise `true`.
///
/// The strategy to use for resolving overlap of labels in gradient legends. If `false`, no
/// overlap reduction is attempted. If set to `true` (default) or `"parity"`, a strategy of
/// removing every other label is used. If set to `"greedy"`, a linear scan of the labels is
/// performed, removing any label that overlaps with the last visible label (this often works
/// better for log-scaled axes).
///
/// __Default value:__ `true`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum LabelOverlap {
    Bool(bool),
    Enum(LabelOverlapEnum),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum TickCount {
    Double(f64),
    Enum(TimeInterval),
}

/// A string or array of strings indicating the name of custom styles to apply to the mark. A
/// style is a named collection of mark property defaults defined within the [style
/// configuration](https://vega.github.io/vega-lite/docs/mark.html#style-config). If style is
/// an array, later styles will override earlier styles. Any [mark
/// properties](https://vega.github.io/vega-lite/docs/encoding.html#mark-prop) explicitly
/// defined within the `encoding` will override a style default.
///
/// __Default value:__ The mark's name. For example, a bar mark will have style `"bar"` by
/// default.
/// __Note:__ Any specified style will augment the default style. For example, a bar mark
/// with `"style": "foo"` will receive from `config.style.bar` and `config.style.foo` (the
/// specified style `"foo"` has higher precedence).
///
/// Placeholder text if the `text` channel is not specified
///
/// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
/// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
/// between `0` to `1` for opacity).
///
/// The subtitle Text.
///
/// The title text.
///
/// A [mark style property](https://vega.github.io/vega-lite/docs/config.html#style) to apply
/// to the title text mark.
///
/// __Default value:__ `"group-title"`.
///
/// Output field names. This can be either a string or an array of strings with two elements
/// denoting the name for the fields for stack start and stack end respectively.
/// If a single string(e.g., `"val"`) is provided, the end field will be `"val_end"`.
///
/// A string or array of strings indicating the name of custom styles to apply to the view
/// background. A style is a named collection of mark property defaults defined within the
/// [style configuration](https://vega.github.io/vega-lite/docs/mark.html#style-config). If
/// style is an array, later styles will override earlier styles.
///
/// __Default value:__ `"cell"`
/// __Note:__ Any specified view background properties will augment the default style.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum PurpleText {
    String(String),
    StringArray(Vec<String>),
}

/// Customized domain values.
///
/// For _quantitative_ fields, `domain` can take the form of a two-element array with minimum
/// and maximum values. [Piecewise
/// scales](https://vega.github.io/vega-lite/docs/scale.html#piecewise) can be created by
/// providing a `domain` with more than two entries.
/// If the input field is aggregated, `domain` can also be a string value `"unaggregated"`,
/// indicating that the domain should include the raw data values prior to the aggregation.
///
/// For _temporal_ fields, `domain` can be a two-element array minimum and maximum values, in
/// the form of either timestamps or the [DateTime definition
/// objects](https://vega.github.io/vega-lite/docs/types.html#datetime).
///
/// For _ordinal_ and _nominal_ fields, `domain` can be an array that lists valid input
/// values.
///
/// The `selection` property can be used to [interactively
/// determine](https://vega.github.io/vega-lite/docs/selection.html#scale-domains) the scale
/// domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum DomainUnion {
    DomainClass(DomainClass),
    Enum(Domain),
    UnionArray(Vec<Equal>),
}

/// The interpolation method for range values. By default, a general interpolator for
/// numbers, dates, strings and colors (in HCL space) is used. For color ranges, this
/// property allows interpolation in alternative color spaces. Legal values include `rgb`,
/// `hsl`, `hsl-long`, `lab`, `hcl`, `hcl-long`, `cubehelix` and `cubehelix-long` ('-long'
/// variants use longer paths in polar coordinate spaces). If object-valued, this property
/// accepts an object with a string-valued _type_ property and an optional numeric _gamma_
/// property applicable to rgb and cubehelix interpolators. For more, see the [d3-interpolate
/// documentation](https://github.com/d3/d3-interpolate).
///
/// * __Default value:__ `hcl`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum InterpolateUnion {
    Enum(ScaleInterpolate),
    ScaleInterpolateParams(ScaleInterpolateParams),
}

/// Extending the domain so that it starts and ends on nice round values. This method
/// typically modifies the scale’s domain, and may only extend the bounds to the nearest
/// round value. Nicing is useful if the domain is computed from data and may be irregular.
/// For example, for a domain of _[0.201479…, 0.996679…]_, a nice domain might be _[0.2,
/// 1.0]_.
///
/// For quantitative scales such as linear, `nice` can be either a boolean flag or a number.
/// If `nice` is a number, it will represent a desired tick count. This allows greater
/// control over the step size used to extend the bounds, guaranteeing that the returned
/// ticks will exactly cover the domain.
///
/// For temporal fields with time and utc scales, the `nice` value can be a string indicating
/// the desired time interval. Legal values are `"millisecond"`, `"second"`, `"minute"`,
/// `"hour"`, `"day"`, `"week"`, `"month"`, and `"year"`. Alternatively, `time` and `utc`
/// scales can accept an object-valued interval specifier of the form `{"interval": "month",
/// "step": 3}`, which includes a desired number of interval steps. Here, the domain would
/// snap to quarter (Jan, Apr, Jul, Oct) boundaries.
///
/// __Default value:__ `true` for unbinned _quantitative_ fields; `false` otherwise.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum NiceUnion {
    Bool(bool),
    Double(f64),
    Enum(NiceTime),
    NiceClass(NiceClass),
}

/// The range of the scale. One of:
///
/// - A string indicating a [pre-defined named scale
/// range](https://vega.github.io/vega-lite/docs/scale.html#range-config) (e.g., example,
/// `"symbol"`, or `"diverging"`).
///
/// - For [continuous scales](https://vega.github.io/vega-lite/docs/scale.html#continuous),
/// two-element array indicating  minimum and maximum values, or an array with more than two
/// entries for specifying a [piecewise
/// scale](https://vega.github.io/vega-lite/docs/scale.html#piecewise).
///
/// - For [discrete](https://vega.github.io/vega-lite/docs/scale.html#discrete) and
/// [discretizing](https://vega.github.io/vega-lite/docs/scale.html#discretizing) scales, an
/// array of desired output values.
///
/// __Notes:__
///
/// 1) For color scales you can also specify a color
/// [`scheme`](https://vega.github.io/vega-lite/docs/scale.html#scheme) instead of `range`.
///
/// 2) Any directly specified `range` for `x` and `y` channels will be ignored. Range can be
/// customized via the view's corresponding
/// [size](https://vega.github.io/vega-lite/docs/size.html) (`width` and `height`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum ScaleRange {
    Enum(RangeEnum),
    UnionArray(Vec<RangeRange>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum RangeRange {
    Double(f64),
    String(String),
}

/// A string indicating a color
/// [scheme](https://vega.github.io/vega-lite/docs/scale.html#scheme) name (e.g.,
/// `"category10"` or `"blues"`) or a [scheme parameter
/// object](https://vega.github.io/vega-lite/docs/scale.html#scheme-params).
///
/// Discrete color schemes may be used with
/// [discrete](https://vega.github.io/vega-lite/docs/scale.html#discrete) or
/// [discretizing](https://vega.github.io/vega-lite/docs/scale.html#discretizing) scales.
/// Continuous color schemes are intended for use with color scales.
///
/// For the full list of supported schemes, please refer to the [Vega
/// Scheme](https://vega.github.io/vega/docs/schemes/#reference) reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum Scheme {
    SchemeParams(SchemeParams),
    String(String),
}

/// Sort order for the encoded field.
///
/// For continuous fields (quantitative or temporal), `sort` can be either `"ascending"` or
/// `"descending"`.
///
/// For discrete fields, `sort` can be one of the following:
/// - `"ascending"` or `"descending"` -- for sorting by the values' natural order in
/// JavaScript.
/// - [A string indicating an encoding channel name to sort
/// by](https://vega.github.io/vega-lite/docs/sort.html#sort-by-encoding) (e.g., `"x"` or
/// `"y"`) with an optional minus prefix for descending sort (e.g., `"-x"` to sort by
/// x-field, descending). This channel string is short-form of [a sort-by-encoding
/// definition](https://vega.github.io/vega-lite/docs/sort.html#sort-by-encoding). For
/// example, `"sort": "-x"` is equivalent to `"sort": {"encoding": "x", "order":
/// "descending"}`.
/// - [A sort field definition](https://vega.github.io/vega-lite/docs/sort.html#sort-field)
/// for sorting by another field.
/// - [An array specifying the field values in preferred
/// order](https://vega.github.io/vega-lite/docs/sort.html#sort-array). In this case, the
/// sort order will obey the values in the array, followed by any unspecified values in their
/// original order. For discrete time field, values in the sort array can be [date-time
/// definition objects](struct.DateTime.html). In addition, for time units `"month"` and `"day"`,
/// the values can be the month or day names (case insensitive) or their 3-letter initials
/// (e.g., `"Mon"`, `"Tue"`).
/// - `null` indicating no sort.
///
/// __Default value:__ `"ascending"`
///
/// __Note:__ `null` and sorting by another channel is not supported for `row` and `column`.
///
/// __See also:__ [`sort`](https://vega.github.io/vega-lite/docs/sort.html) documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum SortUnion {
    EncodingSortField(EncodingSortField),
    Enum(Sort),
    UnionArray(Vec<Equal>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum SortArray {
    Enum(SortOrder),
    SortEncodingSortField(SortEncodingSortField),
    UnionArray(Vec<Equal>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum Detail {
    TypedFieldDef(TypedFieldDef),
    TypedFieldDefArray(Vec<TypedFieldDef>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum FluffyBin {
    BinParams(BinParams),
    Bool(bool),
    Enum(BinEnum),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum Spacing {
    Double(f64),
    RowColNumber(RowColNumber),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum ConditionUnion {
    ConditionalDef(ConditionalDef),
    ConditionalNumberValueDefArray(Vec<ConditionalNumberValueDef>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum HrefCondition {
    ConditionElementArray(Vec<ConditionElement>),
    ConditionalPredicateValueDefStringClass(ConditionalPredicateValueDefStringClass),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum Order {
    OrderFieldDefArray(Vec<OrderFieldDef>),
    OrderFieldDefClass(OrderFieldDefClass),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum ShapeCondition {
    ConditionalPredicateMarkPropFieldDefTypeForShapeClass(
        ConditionalPredicateMarkPropFieldDefTypeForShapeClass,
    ),
    ConditionalStringValueDefArray(Vec<ConditionalStringValueDef>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum TextCondition {
    ConditionalPredicateValueDefTextClass(ConditionalPredicateValueDefTextClass),
    ConditionalValueDefTextArray(Vec<ConditionalValueDefText>),
}

/// A string or array of strings indicating the name of custom styles to apply to the mark. A
/// style is a named collection of mark property defaults defined within the [style
/// configuration](https://vega.github.io/vega-lite/docs/mark.html#style-config). If style is
/// an array, later styles will override earlier styles. Any [mark
/// properties](https://vega.github.io/vega-lite/docs/encoding.html#mark-prop) explicitly
/// defined within the `encoding` will override a style default.
///
/// __Default value:__ The mark's name. For example, a bar mark will have style `"bar"` by
/// default.
/// __Note:__ Any specified style will augment the default style. For example, a bar mark
/// with `"style": "foo"` will receive from `config.style.bar` and `config.style.foo` (the
/// specified style `"foo"` has higher precedence).
///
/// Placeholder text if the `text` channel is not specified
///
/// A constant value in visual domain (e.g., `"red"` / `"#0099ff"` / [gradient
/// definition](https://vega.github.io/vega-lite/docs/types.html#gradient) for color, values
/// between `0` to `1` for opacity).
///
/// The subtitle Text.
///
/// The title text.
///
/// A [mark style property](https://vega.github.io/vega-lite/docs/config.html#style) to apply
/// to the title text mark.
///
/// __Default value:__ `"group-title"`.
///
/// Output field names. This can be either a string or an array of strings with two elements
/// denoting the name for the fields for stack start and stack end respectively.
/// If a single string(e.g., `"val"`) is provided, the end field will be `"val_end"`.
///
/// A string or array of strings indicating the name of custom styles to apply to the view
/// background. A style is a named collection of mark property defaults defined within the
/// [style configuration](https://vega.github.io/vega-lite/docs/mark.html#style-config). If
/// style is an array, later styles will override earlier styles.
///
/// __Default value:__ `"cell"`
/// __Note:__ Any specified view background properties will augment the default style.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum ConditionalValueDefTextText {
    String(String),
    StringArray(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum Tooltip {
    FieldDefWithConditionStringFieldDefString(FieldDefWithConditionStringFieldDefString),
    StringFieldDefArray(Vec<StringFieldDef>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum Color {
    ConditionalAxisPropertyColorNull(ConditionalAxisPropertyColorNull),
    String(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum ConditionalAxisPropertyColorNullCondition {
    ConditionalPredicateValueDefColorNull(ConditionalPredicateValueDefColorNull),
    ConditionalPredicateValueDefColorNullArray(Vec<ConditionalPredicateValueDefColorNull>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum Dash {
    ConditionalAxisPropertyNumberNull(ConditionalAxisPropertyNumberNull),
    DoubleArray(Vec<f64>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum ConditionalAxisPropertyNumberNullCondition {
    ConditionalPredicateValueDefNumberNull(ConditionalPredicateValueDefNumberNull),
    ConditionalPredicateValueDefNumberNullArray(Vec<ConditionalPredicateValueDefNumberNull>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum GridDashOffset {
    ConditionalAxisPropertyNumberNullClass(ConditionalAxisPropertyNumberNullClass),
    Double(f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum ConditionalAxisPropertyNumberNullConditionUnion {
    ConditionalPredicateValueDefNumberNullElement(ConditionalPredicateValueDefNumberNullElement),
    ConditionalPredicateValueDefNumberNullElementArray(
        Vec<ConditionalPredicateValueDefNumberNullElement>,
    ),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum GridOpacity {
    ConditionalAxisPropertyNumberNullClass(ConditionalAxisPropertyNumberNullClass),
    Double(f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum GridWidth {
    ConditionalAxisPropertyNumberNullClass(ConditionalAxisPropertyNumberNullClass),
    Double(f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum LabelAlign {
    ConditionalAxisPropertyNumberNullClass(ConditionalAxisPropertyNumberNullClass),
    Enum(Align),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum TextBaseline {
    ConditionalAxisPropertyTextBaselineNull(ConditionalAxisPropertyTextBaselineNull),
    Enum(Baseline),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum ConditionalAxisPropertyTextBaselineNullCondition {
    ConditionalPredicateValueDefTextBaselineNull(ConditionalPredicateValueDefTextBaselineNull),
    ConditionalPredicateValueDefTextBaselineNullArray(
        Vec<ConditionalPredicateValueDefTextBaselineNull>,
    ),
}

/// Indicates if labels should be hidden if they exceed the axis range. If `false` (the
/// default) no bounds overlap analysis is performed. If `true`, labels will be hidden if
/// they exceed the axis range by more than 1 pixel. If this property is a number, it
/// specifies the pixel tolerance: the maximum amount by which a label bounding box may
/// exceed the axis range.
///
/// __Default value:__ `false`.
///
/// Indicates if the first and last axis labels should be aligned flush with the scale range.
/// Flush alignment for a horizontal axis will left-align the first label and right-align the
/// last label. For vertical axes, bottom and top text baselines are applied instead. If this
/// property is a number, it also indicates the number of pixels by which to offset the first
/// and last labels; for example, a value of 2 will flush-align the first and last labels and
/// also push them 2 pixels outward from the center of the axis. The additional adjustment
/// can sometimes help the labels better visually group with corresponding axis ticks.
///
/// __Default value:__ `true` for axis of a continuous x-scale. Otherwise, `false`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum Label {
    Bool(bool),
    Double(f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum LabelFont {
    ConditionalAxisPropertyStringNull(ConditionalAxisPropertyStringNull),
    String(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum ConditionalAxisPropertyStringNullCondition {
    ConditionalPredicateStringValueDef(ConditionalPredicateStringValueDef),
    ConditionalPredicateStringValueDefArray(Vec<ConditionalPredicateStringValueDef>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum LabelFontStyle {
    ConditionalAxisPropertyFontStyleNull(ConditionalAxisPropertyFontStyleNull),
    String(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum ConditionalAxisPropertyFontStyleNullCondition {
    ConditionalPredicateValueDefFontStyleNull(ConditionalPredicateValueDefFontStyleNull),
    ConditionalPredicateValueDefFontStyleNullArray(Vec<ConditionalPredicateValueDefFontStyleNull>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum LabelFontWeightUnion {
    ConditionalAxisPropertyFontWeightNull(ConditionalAxisPropertyFontWeightNull),
    Double(f64),
    Enum(FontWeightEnum),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum ConditionalAxisPropertyFontWeightNullCondition {
    ConditionalPredicateValueDefFontWeightNull(ConditionalPredicateValueDefFontWeightNull),
    ConditionalPredicateValueDefFontWeightNullArray(
        Vec<ConditionalPredicateValueDefFontWeightNull>,
    ),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum Keyvals {
    AnythingArray(Vec<Option<serde_json::Value>>),
    ImputeSequence(ImputeSequence),
}

/// Type of stacking offset if the field should be stacked.
/// `stack` is only applicable for `x` and `y` channels with continuous domains.
/// For example, `stack` of `y` can be used to customize stacking for a vertical bar chart.
///
/// `stack` can be one of the following values:
/// - `"zero"` or `true`: stacking with baseline offset at zero value of the scale (for
/// creating typical stacked [bar](https://vega.github.io/vega-lite/docs/stack.html#bar) and
/// [area](https://vega.github.io/vega-lite/docs/stack.html#area) chart).
/// - `"normalize"` - stacking with normalized domain (for creating [normalized stacked bar
/// and area charts](https://vega.github.io/vega-lite/docs/stack.html#normalized). <br/>
/// -`"center"` - stacking with center baseline (for
/// [streamgraph](https://vega.github.io/vega-lite/docs/stack.html#streamgraph)).
/// - `null` or `false` - No-stacking. This will produce layered
/// [bar](https://vega.github.io/vega-lite/docs/stack.html#layered-bar-chart) and area
/// chart.
///
/// __Default value:__ `zero` for plots with all of the following conditions are true:
/// (1) the mark is `bar` or `area`;
/// (2) the stacked measure channel (x or y) has a linear scale;
/// (3) At least one of non-position channels mapped to an unaggregated field that is
/// different from x and y. Otherwise, `null` by default.
///
/// __See also:__ [`stack`](https://vega.github.io/vega-lite/docs/stack.html) documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum Stack {
    Bool(bool),
    Enum(StackOffset),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum XUnion {
    Double(f64),
    Enum(PurpleValue),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum YUnion {
    Double(f64),
    Enum(FluffyValue),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum HeightUnion {
    Double(f64),
    Enum(HeightEnum),
    Step(Step),
}

/// A string describing the mark type (one of `"bar"`, `"circle"`, `"square"`, `"tick"`,
/// `"line"`,
/// `"area"`, `"point"`, `"rule"`, `"geoshape"`, and `"text"`) or a [mark definition
/// object](https://vega.github.io/vega-lite/docs/mark.html#mark-def).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum AnyMark {
    MarkDefClass(MarkDefClass),
    Enum(Mark),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum DefBox {
    Bool(bool),
    MarkConfig(MarkConfig),
}

/// Default color.
///
/// __Default value:__ <span style="color: #4682b4;">&#9632;</span> `"#4682b4"`
///
/// __Note:__
/// - This property cannot be used in a [style
/// config](https://vega.github.io/vega-lite/docs/mark.html#style-config).
/// - The `fill` and `stroke` properties have higher precedence than `color` and will
/// override `color`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum ColorUnion {
    ColorLinearGradient(ColorLinearGradient),
    String(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum FillUnion {
    FillLinearGradient(FillLinearGradient),
    String(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum Value {
    Bool(bool),
    Double(f64),
    String(String),
    TooltipContent(TooltipContent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum MarkDefExtent {
    Double(f64),
    Enum(ExtentExtent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum Line {
    Bool(bool),
    OverlayMarkDef(OverlayMarkDef),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum PointUnion {
    Bool(bool),
    Enum(PointEnum),
    OverlayMarkDef(OverlayMarkDef),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum BindUnion {
    Enum(PurpleLegendBinding),
    UnionMap(HashMap<String, PurpleStream>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum PurpleStream {
    AnythingArray(Vec<Option<serde_json::Value>>),
    Double(f64),
    PurpleBinding(PurpleBinding),
    String(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum ClearUnion {
    Bool(bool),
    ClearDerivedStream(ClearDerivedStream),
    String(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum Init {
    UnionMap(HashMap<String, Option<InitValue>>),
    UnionMapArray(Vec<HashMap<String, Option<SelectionInit>>>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum InitValue {
    Bool(bool),
    DateTime(DateTime),
    Double(f64),
    String(String),
    UnionArray(Vec<Equal>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum OnUnion {
    OnDerivedStream(OnDerivedStream),
    String(String),
}

/// When truthy, allows a user to interactively move an interval selection
/// back-and-forth. Can be `true`, `false` (to disable panning), or a
/// [Vega event stream definition](https://vega.github.io/vega/docs/event-streams/)
/// which must include a start and end event to trigger continuous panning.
///
/// __Default value:__ `true`, which corresponds to
/// `[mousedown, window:mouseup] > window:mousemove!` which corresponds to
/// clicks and dragging within an interval selection to reposition it.
///
/// __See also:__ [`translate`](https://vega.github.io/vega-lite/docs/translate.html)
/// documentation.
///
/// When truthy, allows a user to interactively resize an interval selection.
/// Can be `true`, `false` (to disable zooming), or a [Vega event stream
/// definition](https://vega.github.io/vega/docs/event-streams/). Currently,
/// only `wheel` events are supported.
///
/// __Default value:__ `true`, which corresponds to `wheel!`.
///
/// __See also:__ [`zoom`](https://vega.github.io/vega-lite/docs/zoom.html) documentation.
///
/// Controls whether data values should be toggled or only ever inserted into
/// multi selections. Can be `true`, `false` (for insertion only), or a
/// [Vega expression](https://vega.github.io/vega/docs/expressions/).
///
/// __Default value:__ `true`, which corresponds to `event.shiftKey` (i.e.,
/// data values are toggled when a user interacts with the shift-key pressed).
///
/// __See also:__ [`toggle`](https://vega.github.io/vega-lite/docs/toggle.html) documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum Translate {
    Bool(bool),
    String(String),
}

/// Title for the plot.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum Text {
    String(String),
    StringArray(Vec<String>),
    TitleParams(TitleParams),
}

/// Definition for fields to be repeated. One of:
/// 1) An array of fields to be repeated. If `"repeat"` is an array, the field can be
/// referred using `{"repeat": "repeat"}`
/// 2) An object that mapped `"row"` and/or `"column"` to the listed of fields to be repeated
/// along the particular orientations. The objects `{"repeat": "row"}` and `{"repeat":
/// "column"}` can be used to refer to the repeated field respectively.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum RepeatUnion {
    RepeatMapping(RepeatMapping),
    StringArray(Vec<String>),
}

/// The extent of the whiskers. Available options include:
/// - `"min-max"`: min and max are the lower and upper whiskers respectively.
/// - A number representing multiple of the interquartile range. This number will be
/// multiplied by the IQR to determine whisker boundary, which spans from the smallest data
/// to the largest data within the range _[Q1 - k * IQR, Q3 + k * IQR]_ where _Q1_ and _Q3_
/// are the first and third quartiles while _IQR_ is the interquartile range (_Q3-Q1_).
///
/// __Default value:__ `1.5`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum BoxplotExtent {
    Double(f64),
    Enum(ExtentEnum),
}

/// The anchor point for legend orient group layout.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum AnchorUnion {
    Enum(TitleAnchorEnum),
    PurpleSignalRef(PurpleSignalRef),
}

/// The bounds calculation to use for legend orient group layout.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum LayoutBounds {
    Enum(BoundsEnum),
    PurpleSignalRef(PurpleSignalRef),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum BottomCenter {
    Bool(bool),
    PurpleSignalRef(PurpleSignalRef),
}

/// The layout direction for legend orient group layout.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum Direction {
    Enum(Orientation),
    PurpleSignalRef(PurpleSignalRef),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum MarginUnion {
    Double(f64),
    PurpleSignalRef(PurpleSignalRef),
}

/// The default visualization padding, in pixels, from the edge of the visualization canvas
/// to the data rectangle. If a number, specifies padding for all sides.
/// If an object, the value should have the format `{"left": 5, "top": 5, "right": 5,
/// "bottom": 5}` to specify padding for each side of the visualization.
///
/// __Default value__: `5`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum Padding {
    Double(f64),
    PaddingClass(PaddingClass),
}

/// Default [color scheme](https://vega.github.io/vega/docs/schemes/) for categorical data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum CategoryUnion {
    CategorySignalRef(CategorySignalRef),
    Enum(RangeEnum),
    UnionArray(Vec<Option<RangeRaw>>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum RangeRaw {
    Bool(bool),
    Double(f64),
    PurpleSignalRef(PurpleSignalRef),
    String(String),
    UnionArray(Vec<RangeRawArrayElement>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum RangeRawArrayElement {
    Double(f64),
    PurpleSignalRef(PurpleSignalRef),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum SignalRefExtent {
    PurpleSignalRef(PurpleSignalRef),
    UnionArray(Vec<RangeRawArrayElement>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum ColorScheme {
    PurpleSignalRef(PurpleSignalRef),
    String(String),
    StringArray(Vec<String>),
}

/// Default [color scheme](https://vega.github.io/vega/docs/schemes/) for diverging
/// quantitative ramps.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum DivergingUnion {
    DivergingSignalRef(DivergingSignalRef),
    Enum(RangeEnum),
    UnionArray(Vec<Option<RangeRaw>>),
}

/// Default [color scheme](https://vega.github.io/vega/docs/schemes/) for quantitative
/// heatmaps.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum HeatmapUnion {
    Enum(RangeEnum),
    HeatmapSignalRef(HeatmapSignalRef),
    UnionArray(Vec<Option<RangeRaw>>),
}

/// Default [color scheme](https://vega.github.io/vega/docs/schemes/) for rank-ordered data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum OrdinalUnion {
    Enum(RangeEnum),
    OrdinalSignalRef(OrdinalSignalRef),
    UnionArray(Vec<Option<RangeRaw>>),
}

/// Default [color scheme](https://vega.github.io/vega/docs/schemes/) for sequential
/// quantitative ramps.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum RampUnion {
    Enum(RangeEnum),
    RampSignalRef(RampSignalRef),
    UnionArray(Vec<Option<RangeRaw>>),
}

/// When set, a selection is populated by interacting with the corresponding legend. Direct
/// manipulation interaction is disabled by default;
/// to re-enable it, set the selection's
/// [`on`](https://vega.github.io/vega-lite/docs/selection.html#common-selection-properties)
/// property.
///
/// Legend bindings are restricted to selections that only specify a single field or encoding.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum LegendBinding {
    Enum(LegendBindingEnum),
    LegendStreamBinding(LegendStreamBinding),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum LegendUnion {
    OnDerivedStream(OnDerivedStream),
    String(String),
}

/// When set, a selection is populated by input elements (also known as dynamic query
/// widgets)
/// or by interacting with the corresponding legend. Direct manipulation interaction is
/// disabled by default;
/// to re-enable it, set the selection's
/// [`on`](https://vega.github.io/vega-lite/docs/selection.html#common-selection-properties)
/// property.
///
/// Legend bindings are restricted to selections that only specify a single field or
/// encoding.
///
/// Query widget binding takes the form of Vega's [input element binding
/// definition](https://vega.github.io/vega/docs/signals/#bind)
/// or can be a mapping between projected field/encodings and binding definitions.
///
/// __See also:__ [`bind`](https://vega.github.io/vega-lite/docs/bind.html) documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum Binding {
    Enum(LegendBindingEnum),
    UnionMap(HashMap<String, FluffyStream>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum FluffyStream {
    AnythingArray(Vec<Option<serde_json::Value>>),
    Double(f64),
    FluffyBinding(FluffyBinding),
    String(String),
}

/// The default height when the plot has either a discrete y-field or no y-field.
///
/// __Default value:__ a step size based on `config.view.step`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum DiscreteHeightUnion {
    DiscreteHeightClass(DiscreteHeightClass),
    Double(f64),
}

/// The default width when the plot has either a discrete x-field or no x-field.
///
/// __Default value:__ a step size based on `config.view.step`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum DiscreteWidthUnion {
    DiscreteWidthClass(DiscreteWidthClass),
    Double(f64),
}

/// The full data set, included inline. This can be an array of objects or primitive values,
/// an object, or a string.
/// Arrays of primitive values are ingested as objects with a `data` property. Strings are
/// parsed according to the specified format type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[derive(From)]
pub enum InlineDatasetValue {
    AnythingMap(HashMap<String, Option<serde_json::Value>>),
    String(String),
    UnionArray(Vec<serde_json::value::Value>),
}

/// The alignment to apply to symbol legends rows and columns. The supported string values
/// are `"all"`, `"each"` (the default), and `none`. For more information, see the [grid
/// layout documentation](https://vega.github.io/vega/docs/layout).
///
/// __Default value:__ `"each"`.
///
/// The alignment to apply to row/column facet's subplot.
/// The supported string values are `"all"`, `"each"`, and `"none"`.
///
/// - For `"none"`, a flow layout will be used, in which adjacent subviews are simply placed
/// one after the other.
/// - For `"each"`, subviews will be aligned into a clean grid structure, but each row or
/// column may be of variable size.
/// - For `"all"`, subviews will be aligned and each row or column will be sized identically
/// based on the maximum observed size. String values for this property will be applied to
/// both grid rows and columns.
///
/// __Default value:__ `"all"`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LayoutAlign {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "each")]
    Each,
    #[serde(rename = "none")]
    None,
}

/// The sizing format type. One of `"pad"`, `"fit"`, `"fit-x"`, `"fit-y"`,  or `"none"`. See
/// the [autosize type](https://vega.github.io/vega-lite/docs/size.html#autosize)
/// documentation for descriptions of each.
///
/// __Default value__: `"pad"`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutosizeType {
    #[serde(rename = "fit")]
    Fit,
    #[serde(rename = "fit-x")]
    FitX,
    #[serde(rename = "fit-y")]
    FitY,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "pad")]
    Pad,
}

/// Determines how size calculation should be performed, one of `"content"` or `"padding"`.
/// The default setting (`"content"`) interprets the width and height settings as the data
/// rectangle (plotting) dimensions, to which padding is then added. In contrast, the
/// `"padding"` setting includes the padding within the view size calculations, such that the
/// width and height settings indicate the **total** intended size of the view.
///
/// __Default value__: `"content"`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Contains {
    #[serde(rename = "content")]
    Content,
    #[serde(rename = "padding")]
    Padding,
}

/// The bounds calculation method to use for determining the extent of a sub-plot. One of
/// `full` (the default) or `flush`.
///
/// - If set to `full`, the entire calculated bounds (including axes, title, and legend) will
/// be used.
/// - If set to `flush`, only the specified width and height values for the sub-view will be
/// used. The `flush` setting can be useful when attempting to place sub-plots without axes
/// or legends into a uniform grid structure.
///
/// __Default value:__ `"full"`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BoundsEnum {
    #[serde(rename = "flush")]
    Flush,
    #[serde(rename = "full")]
    Full,
}

/// Type of input data: `"json"`, `"csv"`, `"tsv"`, `"dsv"`.
///
/// __Default value:__  The default format type is determined by the extension of the file
/// URL.
/// If no extension is detected, `"json"` will be used by default.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataFormatType {
    #[serde(rename = "csv")]
    Csv,
    #[serde(rename = "dsv")]
    Dsv,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "topojson")]
    Topojson,
    #[serde(rename = "tsv")]
    Tsv,
}

/// An [aggregate operation](https://vega.github.io/vega-lite/docs/aggregate.html#ops) to
/// perform on the field prior to sorting (e.g., `"count"`, `"mean"` and `"median"`).
/// An aggregation is required when there are multiple values of the sort field for each
/// encoded data field.
/// The input data objects will be aggregated, grouped by the encoded data field.
///
/// For a full list of operations, please see the documentation for
/// [aggregate](https://vega.github.io/vega-lite/docs/aggregate.html#ops).
///
/// __Default value:__ `"sum"` for stacked plots. Otherwise, `"mean"`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NonArgAggregateOp {
    #[serde(rename = "average")]
    Average,
    #[serde(rename = "ci0")]
    Ci0,
    #[serde(rename = "ci1")]
    Ci1,
    #[serde(rename = "count")]
    Count,
    #[serde(rename = "distinct")]
    Distinct,
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "mean")]
    Mean,
    #[serde(rename = "median")]
    Median,
    #[serde(rename = "min")]
    Min,
    #[serde(rename = "missing")]
    Missing,
    #[serde(rename = "q1")]
    Q1,
    #[serde(rename = "q3")]
    Q3,
    #[serde(rename = "stderr")]
    Stderr,
    #[serde(rename = "stdev")]
    Stdev,
    #[serde(rename = "stdevp")]
    Stdevp,
    #[serde(rename = "sum")]
    Sum,
    #[serde(rename = "valid")]
    Valid,
    #[serde(rename = "values")]
    Values,
    #[serde(rename = "variance")]
    Variance,
    #[serde(rename = "variancep")]
    Variancep,
}

/// The encoding channel to extract selected values for, when a selection is
/// [projected](https://vega.github.io/vega-lite/docs/project.html)
/// over multiple fields or encodings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SingleDefUnitChannel {
    #[serde(rename = "color")]
    Color,
    #[serde(rename = "fill")]
    Fill,
    #[serde(rename = "fillOpacity")]
    FillOpacity,
    #[serde(rename = "href")]
    Href,
    #[serde(rename = "key")]
    Key,
    #[serde(rename = "latitude")]
    Latitude,
    #[serde(rename = "latitude2")]
    Latitude2,
    #[serde(rename = "longitude")]
    Longitude,
    #[serde(rename = "longitude2")]
    Longitude2,
    #[serde(rename = "opacity")]
    Opacity,
    #[serde(rename = "shape")]
    Shape,
    #[serde(rename = "size")]
    Size,
    #[serde(rename = "stroke")]
    Stroke,
    #[serde(rename = "strokeOpacity")]
    StrokeOpacity,
    #[serde(rename = "strokeWidth")]
    StrokeWidth,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "x")]
    X,
    #[serde(rename = "x2")]
    X2,
    #[serde(rename = "y")]
    Y,
    #[serde(rename = "y2")]
    Y2,
}

/// Time unit for the field to be filtered.
///
/// Time unit (e.g., `year`, `yearmonth`, `month`, `hours`) for a temporal field.
/// or [a temporal field that gets casted as
/// ordinal](https://vega.github.io/vega-lite/docs/type.html#cast).
///
/// __Default value:__ `undefined` (None)
///
/// __See also:__ [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html)
/// documentation.
///
/// The timeUnit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeUnit {
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "hours")]
    Hours,
    #[serde(rename = "hoursminutes")]
    Hoursminutes,
    #[serde(rename = "hoursminutesseconds")]
    Hoursminutesseconds,
    #[serde(rename = "milliseconds")]
    Milliseconds,
    #[serde(rename = "minutes")]
    Minutes,
    #[serde(rename = "minutesseconds")]
    Minutesseconds,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "monthdate")]
    Monthdate,
    #[serde(rename = "monthdatehours")]
    Monthdatehours,
    #[serde(rename = "quarter")]
    Quarter,
    #[serde(rename = "quartermonth")]
    Quartermonth,
    #[serde(rename = "seconds")]
    Seconds,
    #[serde(rename = "secondsmilliseconds")]
    Secondsmilliseconds,
    #[serde(rename = "utcdate")]
    Utcdate,
    #[serde(rename = "utcday")]
    Utcday,
    #[serde(rename = "utchours")]
    Utchours,
    #[serde(rename = "utchoursminutes")]
    Utchoursminutes,
    #[serde(rename = "utchoursminutesseconds")]
    Utchoursminutesseconds,
    #[serde(rename = "utcmilliseconds")]
    Utcmilliseconds,
    #[serde(rename = "utcminutes")]
    Utcminutes,
    #[serde(rename = "utcminutesseconds")]
    Utcminutesseconds,
    #[serde(rename = "utcmonth")]
    Utcmonth,
    #[serde(rename = "utcmonthdate")]
    Utcmonthdate,
    #[serde(rename = "utcmonthdatehours")]
    Utcmonthdatehours,
    #[serde(rename = "utcquarter")]
    Utcquarter,
    #[serde(rename = "utcquartermonth")]
    Utcquartermonth,
    #[serde(rename = "utcseconds")]
    Utcseconds,
    #[serde(rename = "utcsecondsmilliseconds")]
    Utcsecondsmilliseconds,
    #[serde(rename = "utcyear")]
    Utcyear,
    #[serde(rename = "utcyearmonth")]
    Utcyearmonth,
    #[serde(rename = "utcyearmonthdate")]
    Utcyearmonthdate,
    #[serde(rename = "utcyearmonthdatehours")]
    Utcyearmonthdatehours,
    #[serde(rename = "utcyearmonthdatehoursminutes")]
    Utcyearmonthdatehoursminutes,
    #[serde(rename = "utcyearmonthdatehoursminutesseconds")]
    Utcyearmonthdatehoursminutesseconds,
    #[serde(rename = "utcyearquarter")]
    Utcyearquarter,
    #[serde(rename = "utcyearquartermonth")]
    Utcyearquartermonth,
    #[serde(rename = "year")]
    Year,
    #[serde(rename = "yearmonth")]
    Yearmonth,
    #[serde(rename = "yearmonthdate")]
    Yearmonthdate,
    #[serde(rename = "yearmonthdatehours")]
    Yearmonthdatehours,
    #[serde(rename = "yearmonthdatehoursminutes")]
    Yearmonthdatehoursminutes,
    #[serde(rename = "yearmonthdatehoursminutesseconds")]
    Yearmonthdatehoursminutesseconds,
    #[serde(rename = "yearquarter")]
    Yearquarter,
    #[serde(rename = "yearquartermonth")]
    Yearquartermonth,
}

/// The type of gradient. Use `"linear"` for a linear gradient.
///
/// The type of gradient. Use `"radial"` for a radial gradient.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Gradient {
    #[serde(rename = "linear")]
    Linear,
    #[serde(rename = "radial")]
    Radial,
}

/// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
/// `"nominal"`).
/// It can also be a `"geojson"` type for encoding
/// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
///
///
/// __Note:__
///
/// - Data values for a temporal field can be either a date-time string (e.g., `"2015-03-07
/// 12:32:17"`, `"17:01"`, `"2015-03-16"`. `"2015"`) or a timestamp number (e.g.,
/// `1552199579097`).
/// - Data `type` describes the semantics of the data rather than the primitive data types
/// (number, string, etc.). The same primitive data type can have different types of
/// measurement. For example, numeric data can represent quantitative, ordinal, or nominal
/// data.
/// - When using with [`bin`](https://vega.github.io/vega-lite/docs/bin.html), the `type`
/// property can be either `"quantitative"` (for using a linear bin scale) or [`"ordinal"`
/// (for using an ordinal bin
/// scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
/// - When using with [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html), the
/// `type` property can be either `"temporal"` (for using a temporal scale) or [`"ordinal"`
/// (for using an ordinal scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
/// - When using with [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html),
/// the `type` property refers to the post-aggregation data type. For example, we can
/// calculate count `distinct` of a categorical field `"cat"` using `{"aggregate":
/// "distinct", "field": "cat", "type": "quantitative"}`. The `"type"` of the aggregate
/// output is `"quantitative"`.
/// - Secondary channels (e.g., `x2`, `y2`, `xError`, `yError`) do not have `type` as they
/// have exactly the same type as their primary channels (e.g., `x`, `y`).
///
/// __See also:__ [`type`](https://vega.github.io/vega-lite/docs/type.html) documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StandardType {
    #[serde(rename = "nominal")]
    Nominal,
    #[serde(rename = "ordinal")]
    Ordinal,
    #[serde(rename = "quantitative")]
    Quantitative,
    #[serde(rename = "temporal")]
    Temporal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RepeatEnum {
    #[serde(rename = "column")]
    Column,
    #[serde(rename = "repeat")]
    Repeat,
    #[serde(rename = "row")]
    Row,
}

/// The orientation of a non-stacked bar, tick, area, and line charts.
/// The value is either horizontal (default) or vertical.
/// - For bar, rule and tick, this determines whether the size of the bar and tick
/// should be applied to x or y dimension.
/// - For area, this property determines the orient property of the Vega output.
/// - For line and trail marks, this property determines the sort order of the points in the
/// line
/// if `config.sortLineBy` is not specified.
/// For stacked charts, this is always determined by the orientation of the stack;
/// therefore explicitly specified value will be ignored.
///
/// The default direction (`"horizontal"` or `"vertical"`) for gradient legends.
///
/// __Default value:__ `"vertical"`.
///
/// The default direction (`"horizontal"` or `"vertical"`) for symbol legends.
///
/// __Default value:__ `"vertical"`.
///
/// The direction of the legend, one of `"vertical"` or `"horizontal"`.
///
/// __Default value:__
/// - For top-/bottom-`orient`ed legends, `"horizontal"`
/// - For left-/right-`orient`ed legends, `"vertical"`
/// - For top/bottom-left/right-`orient`ed legends, `"horizontal"` for gradient legends and
/// `"vertical"` for symbol legends.
///
/// Orientation of the box plot. This is normally automatically determined based on types of
/// fields on x and y channels. However, an explicit `orient` be specified when the
/// orientation is ambiguous.
///
/// __Default value:__ `"vertical"`.
///
/// Orientation of the error bar. This is normally automatically determined, but can be
/// specified when the orientation is ambiguous and cannot be automatically determined.
///
/// Orientation of the error band. This is normally automatically determined, but can be
/// specified when the orientation is ambiguous and cannot be automatically determined.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Orientation {
    #[serde(rename = "horizontal")]
    Horizontal,
    #[serde(rename = "vertical")]
    Vertical,
}

/// The format type for labels (`"number"` or `"time"`).
///
/// __Default value:__
/// - `"time"` for temporal fields and ordinal and nomimal fields with `timeUnit`.
/// - `"number"` for quantitative fields as well as ordinal and nomimal fields without
/// `timeUnit`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FormatType {
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "time")]
    Time,
}

/// The horizontal alignment of the text or ranged marks (area, bar, image, rect, rule). One
/// of `"left"`, `"right"`, `"center"`.
///
/// Horizontal text alignment of axis tick labels, overriding the default setting for the
/// current axis orientation.
///
/// Horizontal text alignment of axis titles.
///
/// Horizontal text alignment of header labels. One of `"left"`, `"center"`, or `"right"`.
///
/// Horizontal text alignment (to the anchor) of header titles.
///
/// The alignment of the legend label, can be left, center, or right.
///
/// Horizontal text alignment for legend titles.
///
/// __Default value:__ `"left"`.
///
/// Horizontal text alignment for title text. One of `"left"`, `"center"`, or `"right"`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Align {
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "right")]
    Right,
}

/// The vertical alignment of the text or ranged marks (area, bar, image, rect, rule). One of
/// `"top"`, `"middle"`, `"bottom"`.
///
/// __Default value:__ `"middle"`
///
/// Vertical text baseline of axis tick labels, overriding the default setting for the
/// current axis orientation. Can be `"top"`, `"middle"`, `"bottom"`, or `"alphabetic"`.
///
/// Vertical text baseline for axis titles.
///
/// Vertical text baseline for the header title. One of `"top"`, `"bottom"`, `"middle"`.
///
/// __Default value:__ `"middle"`
///
/// The position of the baseline of legend label, can be `"top"`, `"middle"`, `"bottom"`, or
/// `"alphabetic"`.
///
/// __Default value:__ `"middle"`.
///
/// Vertical text baseline for legend titles.
///
/// __Default value:__ `"top"`.
///
/// Vertical text baseline for title and subtitle text. One of `"top"`, `"middle"`,
/// `"bottom"`, or `"alphabetic"`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Baseline {
    #[serde(rename = "alphabetic")]
    Alphabetic,
    #[serde(rename = "bottom")]
    Bottom,
    #[serde(rename = "middle")]
    Middle,
    #[serde(rename = "top")]
    Top,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FontWeightEnum {
    #[serde(rename = "bold")]
    Bold,
    #[serde(rename = "bolder")]
    Bolder,
    #[serde(rename = "lighter")]
    Lighter,
    #[serde(rename = "normal")]
    Normal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LabelOverlapEnum {
    #[serde(rename = "greedy")]
    Greedy,
    #[serde(rename = "parity")]
    Parity,
}

/// The type of the legend. Use `"symbol"` to create a discrete legend and `"gradient"` for a
/// continuous color gradient.
///
/// __Default value:__ `"gradient"` for non-binned quantitative fields and temporal fields;
/// `"symbol"` otherwise.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LegendType {
    #[serde(rename = "gradient")]
    Gradient,
    #[serde(rename = "symbol")]
    Symbol,
}

/// The orientation of the legend, which determines how the legend is positioned within the
/// scene. One of "left", "right", "top-left", "top-right", "bottom-left", "bottom-right",
/// "none".
///
/// __Default value:__ `"right"`
///
/// The orientation of the legend, which determines how the legend is positioned within the
/// scene. One of `"left"`, `"right"`, `"top"`, `"bottom"`, `"top-left"`, `"top-right"`,
/// `"bottom-left"`, `"bottom-right"`, `"none"`.
///
/// __Default value:__ `"right"`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LegendOrient {
    #[serde(rename = "bottom")]
    Bottom,
    #[serde(rename = "bottom-left")]
    BottomLeft,
    #[serde(rename = "bottom-right")]
    BottomRight,
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "top")]
    Top,
    #[serde(rename = "top-left")]
    TopLeft,
    #[serde(rename = "top-right")]
    TopRight,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeInterval {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "hour")]
    Hour,
    #[serde(rename = "millisecond")]
    Millisecond,
    #[serde(rename = "minute")]
    Minute,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "second")]
    Second,
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "year")]
    Year,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TitleAnchorEnum {
    #[serde(rename = "end")]
    End,
    #[serde(rename = "middle")]
    Middle,
    #[serde(rename = "start")]
    Start,
}

/// The orientation of the header label. One of `"top"`, `"bottom"`, `"left"` or `"right"`.
///
/// The orientation of the header title. One of `"top"`, `"bottom"`, `"left"` or `"right"`.
///
/// Orientation of the legend title.
///
/// The orientation of the axis. One of `"top"`, `"bottom"`, `"left"` or `"right"`. The
/// orientation can be used to further specialize the axis type (e.g., a y-axis oriented
/// towards the right edge of the chart).
///
/// __Default value:__ `"bottom"` for x-axes and `"left"` for y-axes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Orient {
    #[serde(rename = "bottom")]
    Bottom,
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "top")]
    Top,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Domain {
    #[serde(rename = "unaggregated")]
    Unaggregated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScaleInterpolateParamsType {
    #[serde(rename = "cubehelix")]
    Cubehelix,
    #[serde(rename = "cubehelix-long")]
    CubehelixLong,
    #[serde(rename = "rgb")]
    Rgb,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScaleInterpolate {
    #[serde(rename = "cubehelix")]
    Cubehelix,
    #[serde(rename = "cubehelix-long")]
    CubehelixLong,
    #[serde(rename = "hcl")]
    Hcl,
    #[serde(rename = "hcl-long")]
    HclLong,
    #[serde(rename = "hsl")]
    Hsl,
    #[serde(rename = "hsl-long")]
    HslLong,
    #[serde(rename = "lab")]
    Lab,
    #[serde(rename = "rgb")]
    Rgb,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NiceTime {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "hour")]
    Hour,
    #[serde(rename = "minute")]
    Minute,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "second")]
    Second,
    #[serde(rename = "week")]
    Week,
    #[serde(rename = "year")]
    Year,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RangeEnum {
    #[serde(rename = "category")]
    Category,
    #[serde(rename = "diverging")]
    Diverging,
    #[serde(rename = "heatmap")]
    Heatmap,
    #[serde(rename = "height")]
    Height,
    #[serde(rename = "ordinal")]
    Ordinal,
    #[serde(rename = "ramp")]
    Ramp,
    #[serde(rename = "symbol")]
    Symbol,
    #[serde(rename = "width")]
    Width,
}

/// The type of scale. Vega-Lite supports the following categories of scale types:
///
/// 1) [**Continuous Scales**](https://vega.github.io/vega-lite/docs/scale.html#continuous)
/// -- mapping continuous domains to continuous output ranges
/// ([`"linear"`](https://vega.github.io/vega-lite/docs/scale.html#linear),
/// [`"pow"`](https://vega.github.io/vega-lite/docs/scale.html#pow),
/// [`"sqrt"`](https://vega.github.io/vega-lite/docs/scale.html#sqrt),
/// [`"symlog"`](https://vega.github.io/vega-lite/docs/scale.html#symlog),
/// [`"log"`](https://vega.github.io/vega-lite/docs/scale.html#log),
/// [`"time"`](https://vega.github.io/vega-lite/docs/scale.html#time),
/// [`"utc"`](https://vega.github.io/vega-lite/docs/scale.html#utc).
///
/// 2) [**Discrete Scales**](https://vega.github.io/vega-lite/docs/scale.html#discrete) --
/// mapping discrete domains to discrete
/// ([`"ordinal"`](https://vega.github.io/vega-lite/docs/scale.html#ordinal)) or continuous
/// ([`"band"`](https://vega.github.io/vega-lite/docs/scale.html#band) and
/// [`"point"`](https://vega.github.io/vega-lite/docs/scale.html#point)) output ranges.
///
/// 3) [**Discretizing
/// Scales**](https://vega.github.io/vega-lite/docs/scale.html#discretizing) -- mapping
/// continuous domains to discrete output ranges
/// [`"bin-ordinal"`](https://vega.github.io/vega-lite/docs/scale.html#bin-ordinal),
/// [`"quantile"`](https://vega.github.io/vega-lite/docs/scale.html#quantile),
/// [`"quantize"`](https://vega.github.io/vega-lite/docs/scale.html#quantize) and
/// [`"threshold"`](https://vega.github.io/vega-lite/docs/scale.html#threshold).
///
/// __Default value:__ please see the [scale type
/// table](https://vega.github.io/vega-lite/docs/scale.html#type).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScaleType {
    #[serde(rename = "band")]
    Band,
    #[serde(rename = "bin-ordinal")]
    BinOrdinal,
    #[serde(rename = "linear")]
    Linear,
    #[serde(rename = "log")]
    Log,
    #[serde(rename = "ordinal")]
    Ordinal,
    #[serde(rename = "point")]
    Point,
    #[serde(rename = "pow")]
    Pow,
    #[serde(rename = "quantile")]
    Quantile,
    #[serde(rename = "quantize")]
    Quantize,
    #[serde(rename = "sqrt")]
    Sqrt,
    #[serde(rename = "symlog")]
    Symlog,
    #[serde(rename = "threshold")]
    Threshold,
    #[serde(rename = "time")]
    Time,
    #[serde(rename = "utc")]
    Utc,
}

/// The [encoding channel](https://vega.github.io/vega-lite/docs/encoding.html#channels) to
/// sort by (e.g., `"x"`, `"y"`)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortByChannel {
    #[serde(rename = "color")]
    Color,
    #[serde(rename = "fill")]
    Fill,
    #[serde(rename = "fillOpacity")]
    FillOpacity,
    #[serde(rename = "opacity")]
    Opacity,
    #[serde(rename = "shape")]
    Shape,
    #[serde(rename = "size")]
    Size,
    #[serde(rename = "stroke")]
    Stroke,
    #[serde(rename = "strokeOpacity")]
    StrokeOpacity,
    #[serde(rename = "strokeWidth")]
    StrokeWidth,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "x")]
    X,
    #[serde(rename = "y")]
    Y,
}

/// The sort order. One of `"ascending"` (default) or `"descending"`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortOrder {
    #[serde(rename = "ascending")]
    Ascending,
    #[serde(rename = "descending")]
    Descending,
}

/// The sort order. One of `"ascending"` (default) or `"descending"`.
///
/// The [encoding channel](https://vega.github.io/vega-lite/docs/encoding.html#channels) to
/// sort by (e.g., `"x"`, `"y"`)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Sort {
    #[serde(rename = "ascending")]
    Ascending,
    #[serde(rename = "color")]
    Color,
    #[serde(rename = "descending")]
    Descending,
    #[serde(rename = "fill")]
    Fill,
    #[serde(rename = "fillOpacity")]
    FillOpacity,
    #[serde(rename = "opacity")]
    Opacity,
    #[serde(rename = "shape")]
    Shape,
    #[serde(rename = "size")]
    Size,
    #[serde(rename = "-color")]
    SortColor,
    #[serde(rename = "-fill")]
    SortFill,
    #[serde(rename = "-fillOpacity")]
    SortFillOpacity,
    #[serde(rename = "-opacity")]
    SortOpacity,
    #[serde(rename = "-shape")]
    SortShape,
    #[serde(rename = "-size")]
    SortSize,
    #[serde(rename = "-stroke")]
    SortStroke,
    #[serde(rename = "-strokeOpacity")]
    SortStrokeOpacity,
    #[serde(rename = "-strokeWidth")]
    SortStrokeWidth,
    #[serde(rename = "-text")]
    SortText,
    #[serde(rename = "-x")]
    SortX,
    #[serde(rename = "-y")]
    SortY,
    #[serde(rename = "stroke")]
    Stroke,
    #[serde(rename = "strokeOpacity")]
    StrokeOpacity,
    #[serde(rename = "strokeWidth")]
    StrokeWidth,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "x")]
    X,
    #[serde(rename = "y")]
    Y,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BinEnum {
    #[serde(rename = "binned")]
    Binned,
}

/// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
/// `"nominal"`).
/// It can also be a `"geojson"` type for encoding
/// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
///
///
/// __Note:__
///
/// - Data values for a temporal field can be either a date-time string (e.g., `"2015-03-07
/// 12:32:17"`, `"17:01"`, `"2015-03-16"`. `"2015"`) or a timestamp number (e.g.,
/// `1552199579097`).
/// - Data `type` describes the semantics of the data rather than the primitive data types
/// (number, string, etc.). The same primitive data type can have different types of
/// measurement. For example, numeric data can represent quantitative, ordinal, or nominal
/// data.
/// - When using with [`bin`](https://vega.github.io/vega-lite/docs/bin.html), the `type`
/// property can be either `"quantitative"` (for using a linear bin scale) or [`"ordinal"`
/// (for using an ordinal bin
/// scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
/// - When using with [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html), the
/// `type` property can be either `"temporal"` (for using a temporal scale) or [`"ordinal"`
/// (for using an ordinal scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
/// - When using with [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html),
/// the `type` property refers to the post-aggregation data type. For example, we can
/// calculate count `distinct` of a categorical field `"cat"` using `{"aggregate":
/// "distinct", "field": "cat", "type": "quantitative"}`. The `"type"` of the aggregate
/// output is `"quantitative"`.
/// - Secondary channels (e.g., `x2`, `y2`, `xError`, `yError`) do not have `type` as they
/// have exactly the same type as their primary channels (e.g., `x`, `y`).
///
/// __See also:__ [`type`](https://vega.github.io/vega-lite/docs/type.html) documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LatitudeType {
    #[serde(rename = "quantitative")]
    Quantitative,
}

/// The encoded field's type of measurement (`"quantitative"`, `"temporal"`, `"ordinal"`, or
/// `"nominal"`).
/// It can also be a `"geojson"` type for encoding
/// ['geoshape'](https://vega.github.io/vega-lite/docs/geoshape.html).
///
///
/// __Note:__
///
/// - Data values for a temporal field can be either a date-time string (e.g., `"2015-03-07
/// 12:32:17"`, `"17:01"`, `"2015-03-16"`. `"2015"`) or a timestamp number (e.g.,
/// `1552199579097`).
/// - Data `type` describes the semantics of the data rather than the primitive data types
/// (number, string, etc.). The same primitive data type can have different types of
/// measurement. For example, numeric data can represent quantitative, ordinal, or nominal
/// data.
/// - When using with [`bin`](https://vega.github.io/vega-lite/docs/bin.html), the `type`
/// property can be either `"quantitative"` (for using a linear bin scale) or [`"ordinal"`
/// (for using an ordinal bin
/// scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
/// - When using with [`timeUnit`](https://vega.github.io/vega-lite/docs/timeunit.html), the
/// `type` property can be either `"temporal"` (for using a temporal scale) or [`"ordinal"`
/// (for using an ordinal scale)](https://vega.github.io/vega-lite/docs/type.html#cast-bin).
/// - When using with [`aggregate`](https://vega.github.io/vega-lite/docs/aggregate.html),
/// the `type` property refers to the post-aggregation data type. For example, we can
/// calculate count `distinct` of a categorical field `"cat"` using `{"aggregate":
/// "distinct", "field": "cat", "type": "quantitative"}`. The `"type"` of the aggregate
/// output is `"quantitative"`.
/// - Secondary channels (e.g., `x2`, `y2`, `xError`, `yError`) do not have `type` as they
/// have exactly the same type as their primary channels (e.g., `x`, `y`).
///
/// __See also:__ [`type`](https://vega.github.io/vega-lite/docs/type.html) documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeForShape {
    #[serde(rename = "geojson")]
    Geojson,
    #[serde(rename = "nominal")]
    Nominal,
    #[serde(rename = "ordinal")]
    Ordinal,
}

/// For band scales, indicates if ticks and grid lines should be placed at the center of a
/// band (default) or at the band extents to indicate intervals.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TickBand {
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "extent")]
    Extent,
}

/// The imputation method to use for the field value of imputed data objects.
/// One of `"value"`, `"mean"`, `"median"`, `"max"` or `"min"`.
///
/// __Default value:__  `"value"`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImputeParamsMethod {
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "mean")]
    Mean,
    #[serde(rename = "median")]
    Median,
    #[serde(rename = "min")]
    Min,
    #[serde(rename = "value")]
    Value,
}

/// Mode for stacking marks. One of `"zero"` (default), `"center"`, or `"normalize"`.
/// The `"zero"` offset will stack starting at `0`. The `"center"` offset will center the
/// stacks. The `"normalize"` offset will compute percentage values for each stack point,
/// with output values in the range `[0,1]`.
///
/// __Default value:__ `"zero"`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StackOffset {
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "normalize")]
    Normalize,
    #[serde(rename = "zero")]
    Zero,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PurpleValue {
    #[serde(rename = "width")]
    Width,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FluffyValue {
    #[serde(rename = "height")]
    Height,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HeightEnum {
    #[serde(rename = "container")]
    Container,
}

/// The mouse cursor used over the mark. Any valid [CSS cursor
/// type](https://developer.mozilla.org/en-US/docs/Web/CSS/cursor#Values) can be used.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Cursor {
    #[serde(rename = "alias")]
    Alias,
    #[serde(rename = "all-scroll")]
    AllScroll,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "cell")]
    Cell,
    #[serde(rename = "col-resize")]
    ColResize,
    #[serde(rename = "context-menu")]
    ContextMenu,
    #[serde(rename = "copy")]
    Copy,
    #[serde(rename = "crosshair")]
    Crosshair,
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "e-resize")]
    EResize,
    #[serde(rename = "ew-resize")]
    EwResize,
    #[serde(rename = "grab")]
    Grab,
    #[serde(rename = "grabbing")]
    Grabbing,
    #[serde(rename = "help")]
    Help,
    #[serde(rename = "move")]
    Move,
    #[serde(rename = "n-resize")]
    NResize,
    #[serde(rename = "ne-resize")]
    NeResize,
    #[serde(rename = "nesw-resize")]
    NeswResize,
    #[serde(rename = "no-drop")]
    NoDrop,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "not-allowed")]
    NotAllowed,
    #[serde(rename = "ns-resize")]
    NsResize,
    #[serde(rename = "nw-resize")]
    NwResize,
    #[serde(rename = "nwse-resize")]
    NwseResize,
    #[serde(rename = "pointer")]
    Pointer,
    #[serde(rename = "progress")]
    Progress,
    #[serde(rename = "row-resize")]
    RowResize,
    #[serde(rename = "s-resize")]
    SResize,
    #[serde(rename = "se-resize")]
    SeResize,
    #[serde(rename = "sw-resize")]
    SwResize,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "vertical-text")]
    VerticalText,
    #[serde(rename = "w-resize")]
    WResize,
    #[serde(rename = "wait")]
    Wait,
    #[serde(rename = "zoom-in")]
    ZoomIn,
    #[serde(rename = "zoom-out")]
    ZoomOut,
}

/// The direction of the text. One of `"ltr"` (left-to-right) or `"rtl"` (right-to-left).
/// This property determines on which side is truncated in response to the limit parameter.
///
/// __Default value:__ `"ltr"`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Dir {
    #[serde(rename = "ltr")]
    Ltr,
    #[serde(rename = "rtl")]
    Rtl,
}

/// The line interpolation method to use for line and area marks. One of the following:
/// - `"linear"`: piecewise linear segments, as in a polyline.
/// - `"linear-closed"`: close the linear segments to form a polygon.
/// - `"step"`: alternate between horizontal and vertical segments, as in a step function.
/// - `"step-before"`: alternate between vertical and horizontal segments, as in a step
/// function.
/// - `"step-after"`: alternate between horizontal and vertical segments, as in a step
/// function.
/// - `"basis"`: a B-spline, with control point duplication on the ends.
/// - `"basis-open"`: an open B-spline; may not intersect the start or end.
/// - `"basis-closed"`: a closed B-spline, as in a loop.
/// - `"cardinal"`: a Cardinal spline, with control point duplication on the ends.
/// - `"cardinal-open"`: an open Cardinal spline; may not intersect the start or end, but
/// will intersect other control points.
/// - `"cardinal-closed"`: a closed Cardinal spline, as in a loop.
/// - `"bundle"`: equivalent to basis, except the tension parameter is used to straighten the
/// spline.
/// - `"monotone"`: cubic interpolation that preserves monotonicity in y.
///
/// The line interpolation method for the error band. One of the following:
/// - `"linear"`: piecewise linear segments, as in a polyline.
/// - `"linear-closed"`: close the linear segments to form a polygon.
/// - `"step"`: a piecewise constant function (a step function) consisting of alternating
/// horizontal and vertical lines. The y-value changes at the midpoint of each pair of
/// adjacent x-values.
/// - `"step-before"`: a piecewise constant function (a step function) consisting of
/// alternating horizontal and vertical lines. The y-value changes before the x-value.
/// - `"step-after"`: a piecewise constant function (a step function) consisting of
/// alternating horizontal and vertical lines. The y-value changes after the x-value.
/// - `"basis"`: a B-spline, with control point duplication on the ends.
/// - `"basis-open"`: an open B-spline; may not intersect the start or end.
/// - `"basis-closed"`: a closed B-spline, as in a loop.
/// - `"cardinal"`: a Cardinal spline, with control point duplication on the ends.
/// - `"cardinal-open"`: an open Cardinal spline; may not intersect the start or end, but
/// will intersect other control points.
/// - `"cardinal-closed"`: a closed Cardinal spline, as in a loop.
/// - `"bundle"`: equivalent to basis, except the tension parameter is used to straighten the
/// spline.
/// - `"monotone"`: cubic interpolation that preserves monotonicity in y.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Interpolate {
    #[serde(rename = "basis")]
    Basis,
    #[serde(rename = "basis-closed")]
    BasisClosed,
    #[serde(rename = "basis-open")]
    BasisOpen,
    #[serde(rename = "bundle")]
    Bundle,
    #[serde(rename = "cardinal")]
    Cardinal,
    #[serde(rename = "cardinal-closed")]
    CardinalClosed,
    #[serde(rename = "cardinal-open")]
    CardinalOpen,
    #[serde(rename = "catmull-rom")]
    CatmullRom,
    #[serde(rename = "linear")]
    Linear,
    #[serde(rename = "linear-closed")]
    LinearClosed,
    #[serde(rename = "monotone")]
    Monotone,
    #[serde(rename = "natural")]
    Natural,
    #[serde(rename = "step")]
    Step,
    #[serde(rename = "step-after")]
    StepAfter,
    #[serde(rename = "step-before")]
    StepBefore,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Invalid {
    #[serde(rename = "filter")]
    Filter,
}

/// The stroke cap for line ending style. One of `"butt"`, `"round"`, or `"square"`.
///
/// __Default value:__ `"square"`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StrokeCap {
    #[serde(rename = "butt")]
    Butt,
    #[serde(rename = "round")]
    Round,
    #[serde(rename = "square")]
    Square,
}

/// The stroke line join method. One of `"miter"`, `"round"` or `"bevel"`.
///
/// __Default value:__ `"miter"`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StrokeJoin {
    #[serde(rename = "bevel")]
    Bevel,
    #[serde(rename = "miter")]
    Miter,
    #[serde(rename = "round")]
    Round,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Content {
    #[serde(rename = "data")]
    Data,
    #[serde(rename = "encoding")]
    Encoding,
}

/// The mark type. This could a primitive mark type
/// (one of `"bar"`, `"circle"`, `"square"`, `"tick"`, `"line"`,
/// `"area"`, `"point"`, `"geoshape"`, `"rule"`, and `"text"`)
/// or a composite mark type (`"boxplot"`, `"errorband"`, `"errorbar"`).
///
/// All types of primitive marks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Mark {
    #[serde(rename = "area")]
    Area,
    #[serde(rename = "bar")]
    Bar,
    #[serde(rename = "boxplot")]
    Boxplot,
    #[serde(rename = "circle")]
    Circle,
    #[serde(rename = "errorband")]
    Errorband,
    #[serde(rename = "errorbar")]
    Errorbar,
    #[serde(rename = "geoshape")]
    Geoshape,
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "line")]
    Line,
    #[serde(rename = "point")]
    Point,
    #[serde(rename = "rect")]
    Rect,
    #[serde(rename = "rule")]
    Rule,
    #[serde(rename = "square")]
    Square,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "tick")]
    Tick,
    #[serde(rename = "trail")]
    Trail,
}

/// The extent of the band. Available options include:
/// - `"ci"`: Extend the band to the confidence interval of the mean.
/// - `"stderr"`: The size of band are set to the value of standard error, extending from the
/// mean.
/// - `"stdev"`: The size of band are set to the value of standard deviation, extending from
/// the mean.
/// - `"iqr"`: Extend the band to the q1 and q3.
///
/// __Default value:__ `"stderr"`.
///
/// The extent of the rule. Available options include:
/// - `"ci"`: Extend the rule to the confidence interval of the mean.
/// - `"stderr"`: The size of rule are set to the value of standard error, extending from the
/// mean.
/// - `"stdev"`: The size of rule are set to the value of standard deviation, extending from
/// the mean.
/// - `"iqr"`: Extend the rule to the q1 and q3.
///
/// __Default value:__ `"stderr"`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExtentExtent {
    #[serde(rename = "ci")]
    Ci,
    #[serde(rename = "iqr")]
    Iqr,
    #[serde(rename = "min-max")]
    MinMax,
    #[serde(rename = "stderr")]
    Stderr,
    #[serde(rename = "stdev")]
    Stdev,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PointEnum {
    #[serde(rename = "transparent")]
    Transparent,
}

/// The cartographic projection to use. This value is case-insensitive, for example
/// `"albers"` and `"Albers"` indicate the same projection type. You can find all valid
/// projection types [in the
/// documentation](https://vega.github.io/vega-lite/docs/projection.html#projection-types).
///
/// __Default value:__ `mercator`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectionType {
    #[serde(rename = "albers")]
    Albers,
    #[serde(rename = "albersUsa")]
    AlbersUsa,
    #[serde(rename = "azimuthalEqualArea")]
    AzimuthalEqualArea,
    #[serde(rename = "azimuthalEquidistant")]
    AzimuthalEquidistant,
    #[serde(rename = "conicConformal")]
    ConicConformal,
    #[serde(rename = "conicEqualArea")]
    ConicEqualArea,
    #[serde(rename = "conicEquidistant")]
    ConicEquidistant,
    #[serde(rename = "equalEarth")]
    EqualEarth,
    #[serde(rename = "equirectangular")]
    Equirectangular,
    #[serde(rename = "gnomonic")]
    Gnomonic,
    #[serde(rename = "identity")]
    Identity,
    #[serde(rename = "mercator")]
    Mercator,
    #[serde(rename = "naturalEarth1")]
    NaturalEarth1,
    #[serde(rename = "orthographic")]
    Orthographic,
    #[serde(rename = "stereographic")]
    Stereographic,
    #[serde(rename = "transverseMercator")]
    TransverseMercator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResolveMode {
    #[serde(rename = "independent")]
    Independent,
    #[serde(rename = "shared")]
    Shared,
}

/// Establishes a two-way binding between the interval selection and the scales
/// used within the same view. This allows a user to interactively pan and
/// zoom the view.
///
/// __See also:__ [`bind`](https://vega.github.io/vega-lite/docs/bind.html) documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PurpleLegendBinding {
    #[serde(rename = "legend")]
    Legend,
    #[serde(rename = "scales")]
    Scales,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarkType {
    #[serde(rename = "arc")]
    Arc,
    #[serde(rename = "area")]
    Area,
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "line")]
    Line,
    #[serde(rename = "path")]
    Path,
    #[serde(rename = "rect")]
    Rect,
    #[serde(rename = "rule")]
    Rule,
    #[serde(rename = "shape")]
    Shape,
    #[serde(rename = "symbol")]
    Symbol,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "trail")]
    Trail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Source {
    #[serde(rename = "scope")]
    Scope,
    #[serde(rename = "view")]
    View,
    #[serde(rename = "window")]
    Window,
}

/// By default, `all` data values are considered to lie within an empty selection.
/// When set to `none`, empty selections contain no data values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Empty {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "none")]
    None,
}

/// With layered and multi-view displays, a strategy that determines how
/// selections' data queries are resolved when applied in a filter transform,
/// conditional encoding rule, or scale domain.
///
/// __See also:__ [`resolve`](https://vega.github.io/vega-lite/docs/selection-resolve.html)
/// documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SelectionResolution {
    #[serde(rename = "global")]
    Global,
    #[serde(rename = "intersect")]
    Intersect,
    #[serde(rename = "union")]
    Union,
}

/// Determines the default event processing and data query for the selection. Vega-Lite
/// currently supports three selection types:
///
/// - `"single"` -- to select a single discrete data value on `click`.
/// - `"multi"` -- to select multiple discrete data value; the first value is selected on
/// `click` and additional values toggled on shift-`click`.
/// - `"interval"` -- to select a continuous range of data values on `drag`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SelectionDefType {
    #[serde(rename = "interval")]
    Interval,
    #[serde(rename = "multi")]
    Multi,
    #[serde(rename = "single")]
    Single,
}

/// Default title orientation (`"top"`, `"bottom"`, `"left"`, or `"right"`)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TitleOrient {
    #[serde(rename = "bottom")]
    Bottom,
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "top")]
    Top,
}

/// The aggregation operation to apply to the fields (e.g., `"sum"`, `"average"`, or
/// `"count"`).
/// See the [full list of supported aggregation
/// operations](https://vega.github.io/vega-lite/docs/aggregate.html#ops)
/// for more information.
///
/// The aggregation operation to apply (e.g., `"sum"`, `"average"` or `"count"`). See the
/// list of all supported operations
/// [here](https://vega.github.io/vega-lite/docs/aggregate.html#ops).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AggregateOp {
    #[serde(rename = "argmax")]
    Argmax,
    #[serde(rename = "argmin")]
    Argmin,
    #[serde(rename = "average")]
    Average,
    #[serde(rename = "ci0")]
    Ci0,
    #[serde(rename = "ci1")]
    Ci1,
    #[serde(rename = "count")]
    Count,
    #[serde(rename = "distinct")]
    Distinct,
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "mean")]
    Mean,
    #[serde(rename = "median")]
    Median,
    #[serde(rename = "min")]
    Min,
    #[serde(rename = "missing")]
    Missing,
    #[serde(rename = "q1")]
    Q1,
    #[serde(rename = "q3")]
    Q3,
    #[serde(rename = "stderr")]
    Stderr,
    #[serde(rename = "stdev")]
    Stdev,
    #[serde(rename = "stdevp")]
    Stdevp,
    #[serde(rename = "sum")]
    Sum,
    #[serde(rename = "valid")]
    Valid,
    #[serde(rename = "values")]
    Values,
    #[serde(rename = "variance")]
    Variance,
    #[serde(rename = "variancep")]
    Variancep,
}

/// The imputation method to use for the field value of imputed data objects.
/// One of `"value"`, `"mean"`, `"median"`, `"max"` or `"min"`.
///
/// __Default value:__  `"value"`
///
/// The functional form of the regression model. One of `"linear"`, `"log"`, `"exp"`,
/// `"pow"`, `"quad"`, or `"poly"`.
///
/// __Default value:__ `"linear"`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransformMethod {
    #[serde(rename = "exp")]
    Exp,
    #[serde(rename = "linear")]
    Linear,
    #[serde(rename = "log")]
    Log,
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "mean")]
    Mean,
    #[serde(rename = "median")]
    Median,
    #[serde(rename = "min")]
    Min,
    #[serde(rename = "poly")]
    Poly,
    #[serde(rename = "pow")]
    Pow,
    #[serde(rename = "quad")]
    Quad,
    #[serde(rename = "value")]
    Value,
}

/// The window or aggregation operation to apply within a window (e.g., `"rank"`, `"lead"`,
/// `"sum"`, `"average"` or `"count"`). See the list of all supported operations
/// [here](https://vega.github.io/vega-lite/docs/window.html#ops).
///
/// The aggregation operation to apply to the fields (e.g., `"sum"`, `"average"`, or
/// `"count"`).
/// See the [full list of supported aggregation
/// operations](https://vega.github.io/vega-lite/docs/aggregate.html#ops)
/// for more information.
///
/// The aggregation operation to apply (e.g., `"sum"`, `"average"` or `"count"`). See the
/// list of all supported operations
/// [here](https://vega.github.io/vega-lite/docs/aggregate.html#ops).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Op {
    #[serde(rename = "argmax")]
    Argmax,
    #[serde(rename = "argmin")]
    Argmin,
    #[serde(rename = "average")]
    Average,
    #[serde(rename = "ci0")]
    Ci0,
    #[serde(rename = "ci1")]
    Ci1,
    #[serde(rename = "count")]
    Count,
    #[serde(rename = "cume_dist")]
    CumeDist,
    #[serde(rename = "dense_rank")]
    DenseRank,
    #[serde(rename = "distinct")]
    Distinct,
    #[serde(rename = "first_value")]
    FirstValue,
    #[serde(rename = "lag")]
    Lag,
    #[serde(rename = "last_value")]
    LastValue,
    #[serde(rename = "lead")]
    Lead,
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "mean")]
    Mean,
    #[serde(rename = "median")]
    Median,
    #[serde(rename = "min")]
    Min,
    #[serde(rename = "missing")]
    Missing,
    #[serde(rename = "nth_value")]
    NthValue,
    #[serde(rename = "ntile")]
    Ntile,
    #[serde(rename = "percent_rank")]
    PercentRank,
    #[serde(rename = "q1")]
    Q1,
    #[serde(rename = "q3")]
    Q3,
    #[serde(rename = "rank")]
    Rank,
    #[serde(rename = "row_number")]
    RowNumber,
    #[serde(rename = "stderr")]
    Stderr,
    #[serde(rename = "stdev")]
    Stdev,
    #[serde(rename = "stdevp")]
    Stdevp,
    #[serde(rename = "sum")]
    Sum,
    #[serde(rename = "valid")]
    Valid,
    #[serde(rename = "values")]
    Values,
    #[serde(rename = "variance")]
    Variance,
    #[serde(rename = "variancep")]
    Variancep,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExtentEnum {
    #[serde(rename = "min-max")]
    MinMax,
}

/// The extent of the band. Available options include:
/// - `"ci"`: Extend the band to the confidence interval of the mean.
/// - `"stderr"`: The size of band are set to the value of standard error, extending from the
/// mean.
/// - `"stdev"`: The size of band are set to the value of standard deviation, extending from
/// the mean.
/// - `"iqr"`: Extend the band to the q1 and q3.
///
/// __Default value:__ `"stderr"`.
///
/// The extent of the rule. Available options include:
/// - `"ci"`: Extend the rule to the confidence interval of the mean.
/// - `"stderr"`: The size of rule are set to the value of standard error, extending from the
/// mean.
/// - `"stdev"`: The size of rule are set to the value of standard deviation, extending from
/// the mean.
/// - `"iqr"`: Extend the rule to the q1 and q3.
///
/// __Default value:__ `"stderr"`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorbandExtent {
    #[serde(rename = "ci")]
    Ci,
    #[serde(rename = "iqr")]
    Iqr,
    #[serde(rename = "stderr")]
    Stderr,
    #[serde(rename = "stdev")]
    Stdev,
}

/// Defines how Vega-Lite generates title for fields. There are three possible styles:
/// - `"verbal"` (Default) - displays function in a verbal style (e.g., "Sum of field",
/// "Year-month of date", "field (binned)").
/// - `"function"` - displays function using parentheses and capitalized texts (e.g.,
/// "SUM(field)", "YEARMONTH(date)", "BIN(field)").
/// - `"plain"` - displays only the field name without functions (e.g., "field", "date",
/// "field").
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldTitle {
    #[serde(rename = "functional")]
    Functional,
    #[serde(rename = "plain")]
    Plain,
    #[serde(rename = "verbal")]
    Verbal,
}

/// Establishes a two-way binding between the interval selection and the scales
/// used within the same view. This allows a user to interactively pan and
/// zoom the view.
///
/// __See also:__ [`bind`](https://vega.github.io/vega-lite/docs/bind.html) documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Bind {
    #[serde(rename = "scales")]
    Scales,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LegendBindingEnum {
    #[serde(rename = "legend")]
    Legend,
}
