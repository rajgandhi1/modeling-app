//! Functions for generating docs for our stdlib functions.

#[cfg(test)]
mod gen_std_tests;
pub mod kcl_doc;

use std::{
    fmt::{self, Write},
    path::Path,
};

use anyhow::Result;
use kcl_doc::ModData;
use parse_display::Display;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tower_lsp::lsp_types::{
    CompletionItem, CompletionItemKind, CompletionItemLabelDetails, Documentation, InsertTextFormat, MarkupContent,
    MarkupKind, ParameterInformation, ParameterLabel, SignatureHelp, SignatureInformation,
};

use crate::execution::{types::NumericType, Sketch};

// These types are declared in (KCL) std.
const DECLARED_TYPES: [&str; 17] = [
    "any",
    "number",
    "string",
    "tag",
    "bool",
    "Sketch",
    "Solid",
    "Plane",
    "Helix",
    "Face",
    "Edge",
    "Point2d",
    "Point3d",
    "Axis2d",
    "Axis3d",
    "ImportedGeometry",
    "fn",
];

lazy_static::lazy_static! {
    static ref NUMERIC_TYPE_SCHEMA: schemars::schema::SchemaObject = {
        let mut settings = schemars::gen::SchemaSettings::openapi3();
        settings.inline_subschemas = true;
        let mut generator = schemars::gen::SchemaGenerator::new(settings);
        generator.root_schema_for::<NumericType>().schema
    };
}

/// The primitive types that can be used in a KCL file.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema, Display)]
#[serde(rename_all = "lowercase")]
#[display(style = "lowercase")]
enum Primitive {
    /// A boolean value.
    Bool,
    /// A number value.
    Number,
    /// A string value.
    String,
    /// A uuid value.
    Uuid,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema, ts_rs::TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct StdLibFnData {
    /// The name of the function.
    pub name: String,
    /// The summary of the function.
    pub summary: String,
    /// The description of the function.
    pub description: String,
    /// The tags of the function.
    pub tags: Vec<String>,
    /// The args of the function.
    pub args: Vec<StdLibFnArg>,
    /// The return value of the function.
    pub return_value: Option<StdLibFnArg>,
    /// If the function is unpublished.
    pub unpublished: bool,
    /// If the function is deprecated.
    pub deprecated: bool,
    /// Code examples. The bool is whether the example is `norun``
    /// These are tested and we know they compile and execute.
    pub examples: Vec<(String, bool)>,
}

/// This struct defines a single argument to a stdlib function.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, JsonSchema, ts_rs::TS)]
// There's a bug in ts_rs where this isn't correctly imported by StdLibFnData.
#[ts(export_to = "StdLibFnData.ts")]
#[serde(rename_all = "camelCase")]
pub struct StdLibFnArg {
    /// The name of the argument.
    pub name: String,
    /// The type of the argument.
    pub type_: String,
    /// The schema of the argument.
    #[ts(type = "any")]
    pub schema: schemars::schema::RootSchema,
    /// If the argument is required.
    pub required: bool,
    /// Include this in completion snippets?
    #[serde(default, skip_serializing_if = "is_false")]
    pub include_in_snippet: bool,
    /// Snippet should suggest this value for the argument.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snippet_value: Option<String>,
    /// Snippet should suggest this value for the argument.
    /// The suggested value should be an array, with these elements.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snippet_value_array: Option<Vec<String>>,
    /// Additional information that could be used instead of the type's description.
    /// This is helpful if the type is really basic, like "u32" -- that won't tell the user much about
    /// how this argument is meant to be used.
    /// Empty string means this has no docs.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    /// Even in functions that use keyword arguments, not every parameter requires a label (most do though).
    /// Some functions allow one unlabeled parameter, which has to be first in the
    /// argument list.
    ///
    /// This field is ignored for functions that still use positional arguments.
    /// Defaults to true.
    #[serde(default = "its_true")]
    pub label_required: bool,
}

impl fmt::Display for StdLibFnArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.label_required {
            f.write_char('@')?;
        }
        f.write_str(&self.name)?;
        if !self.required {
            f.write_char('?')?;
        }
        f.write_str(": ")?;
        f.write_str(&self.type_)
    }
}

fn its_true() -> bool {
    true
}

fn is_false(b: &bool) -> bool {
    !b
}

impl StdLibFnArg {
    /// If the argument is a primitive.
    pub fn is_primitive(&self) -> Result<bool> {
        is_primitive(&self.schema.schema.clone().into()).map(|r| r.is_some())
    }

    pub fn get_autocomplete_string(&self) -> Result<String> {
        get_autocomplete_string_from_schema(&self.schema.schema.clone().into())
    }

    pub fn get_autocomplete_snippet(&self, index: usize, in_keyword_fn: bool) -> Result<Option<(usize, String)>> {
        let label = if in_keyword_fn && self.label_required {
            &format!("{} = ", self.name)
        } else {
            ""
        };
        if let Some(vals) = &self.snippet_value_array {
            let mut snippet = label.to_owned();
            snippet.push('[');
            for (i, val) in vals.iter().enumerate() {
                snippet.push_str(&format!("${{{}:{}}}", index + i, val));
                if i != vals.len() - 1 {
                    snippet.push_str(", ");
                }
            }
            snippet.push(']');
            return Ok(Some((index + vals.len() - 1, snippet)));
        }
        if let Some(val) = &self.snippet_value {
            return Ok(Some((index, format!("{label}${{{}:{}}}", index, val))));
        }
        if (self.type_ == "Sketch"
            || self.type_ == "[Sketch]"
            || self.type_ == "Geometry"
            || self.type_ == "GeometryWithImportedGeometry"
            || self.type_ == "Solid"
            || self.type_ == "[Solid]"
            || self.type_ == "SketchSurface"
            || self.type_ == "SketchOrSurface"
            || self.type_ == "SolidOrImportedGeometry"
            || self.type_ == "SolidOrSketchOrImportedGeometry")
            && (self.required || self.include_in_snippet)
        {
            return Ok(Some((index, format!("{label}${{{}:{}}}", index, "%"))));
        } else if (self.type_ == "TagDeclarator" || self.type_ == "TagNode") && self.required {
            return Ok(Some((index, format!("{label}${{{}:{}}}", index, "$myTag"))));
        } else if self.type_ == "TagIdentifier" && self.required {
            // TODO: actually use the ast to populate this.
            return Ok(Some((index, format!("{label}${{{}:{}}}", index, "myTag"))));
        } else if self.type_ == "[KclValue]" && self.required {
            return Ok(Some((index, format!("{label}${{{}:{}}}", index, "[0..9]"))));
        } else if self.type_ == "KclValue" && self.required {
            return Ok(Some((index, format!("{label}${{{}:{}}}", index, "3"))));
        }
        self.get_autocomplete_snippet_from_schema(&self.schema.schema.clone().into(), index, in_keyword_fn, &self.name)
            .map(|maybe| maybe.map(|(index, snippet)| (index, format!("{label}{snippet}"))))
    }

    pub fn description(&self, kcl_std: Option<&ModData>) -> Option<String> {
        // Check if we explicitly gave this stdlib arg a description.
        if !self.description.is_empty() {
            assert!(!self.description.contains('\n'), "Arg docs will get truncated");
            return Some(self.description.clone());
        }

        if let Some(kcl_std) = kcl_std {
            if let Some(t) = docs_for_type(&self.type_, kcl_std) {
                return Some(t);
            }
        }

        // If not, then try to get something meaningful from the schema.
        get_description_string_from_schema(&self.schema.clone())
    }

    fn get_autocomplete_snippet_from_schema(
        &self,
        schema: &schemars::schema::Schema,
        index: usize,
        in_keyword_fn: bool,
        name: &str,
    ) -> Result<Option<(usize, String)>> {
        match schema {
            schemars::schema::Schema::Object(o) => {
                // Check if the schema is the same as a Sketch.
                let mut settings = schemars::gen::SchemaSettings::openapi3();
                // We set this so we can recurse them later.
                settings.inline_subschemas = true;
                let mut generator = schemars::gen::SchemaGenerator::new(settings);
                let sketch_schema = generator.root_schema_for::<Sketch>().schema;
                if sketch_schema.object == o.object {
                    return Ok(Some((index, format!("${{{}:sketch{}}}", index, "000"))));
                }

                if name == "color" {
                    let snippet = format!("${{{}:\"#ff0000\"}}", index);
                    return Ok(Some((index, snippet)));
                }
                if let Some(serde_json::Value::Bool(nullable)) = o.extensions.get("nullable") {
                    if (!in_keyword_fn && *nullable) || (in_keyword_fn && !self.include_in_snippet) {
                        return Ok(None);
                    }
                }
                if o.enum_values.is_some() {
                    let auto_str = get_autocomplete_string_from_schema(schema)?;
                    return Ok(Some((index, format!("${{{}:{}}}", index, auto_str))));
                }

                if let Some(format) = &o.format {
                    if format == "uuid" {
                        return Ok(Some((index, format!(r#"${{{}:"tag_or_edge_fn"}}"#, index))));
                    } else if format == "double" {
                        return Ok(Some((index, format!(r#"${{{}:3.14}}"#, index))));
                    } else if format == "uint"
                        || format == "int64"
                        || format == "uint32"
                        || format == "uint64"
                        || format == "uint8"
                    {
                        return Ok(Some((index, format!(r#"${{{}:10}}"#, index))));
                    } else {
                        anyhow::bail!("unknown format: {}", format);
                    }
                }

                if let Some(obj_val) = &o.object {
                    let mut fn_docs = String::new();
                    fn_docs.push_str("{\n");
                    // Let's print out the object's properties.
                    let mut i = index;
                    for (prop_name, prop) in obj_val.properties.iter() {
                        if prop_name.starts_with('_') {
                            continue;
                        }

                        // Tolerance is a an optional property that we don't want to show in the
                        // autocomplete, since it is mostly for advanced users.
                        if prop_name == "tolerance" {
                            continue;
                        }

                        if let Some((new_index, snippet)) =
                            self.get_autocomplete_snippet_from_schema(prop, i, false, name)?
                        {
                            fn_docs.push_str(&format!("\t{} = {},\n", prop_name, snippet));
                            i = new_index + 1;
                        }
                    }

                    fn_docs.push('}');

                    return Ok(Some((i - 1, fn_docs)));
                }

                if let Some(array_val) = &o.array {
                    if let Some(schemars::schema::SingleOrVec::Single(items)) = &array_val.items {
                        // Let's print out the object's properties.
                        match array_val.max_items {
                            Some(val) => {
                                return Ok(Some((
                                    index + (val as usize) - 1,
                                    format!(
                                        "[{}]",
                                        (0..val)
                                            .map(|v| self
                                                .get_autocomplete_snippet_from_schema(
                                                    items,
                                                    index + (v as usize),
                                                    in_keyword_fn,
                                                    name
                                                )
                                                .unwrap()
                                                .unwrap()
                                                .1)
                                            .collect::<Vec<_>>()
                                            .join(", ")
                                    ),
                                )));
                            }
                            None => {
                                return Ok(Some((
                                    index,
                                    format!(
                                        "[{}]",
                                        self.get_autocomplete_snippet_from_schema(items, index, in_keyword_fn, name)?
                                            .ok_or_else(|| anyhow::anyhow!("expected snippet"))?
                                            .1
                                    ),
                                )));
                            }
                        };
                    } else if let Some(items) = &array_val.contains {
                        return Ok(Some((
                            index,
                            format!(
                                "[{}]",
                                self.get_autocomplete_snippet_from_schema(items, index, in_keyword_fn, name)?
                                    .ok_or_else(|| anyhow::anyhow!("expected snippet"))?
                                    .1
                            ),
                        )));
                    }
                }

                if let Some(subschemas) = &o.subschemas {
                    let mut fn_docs = String::new();
                    let mut i = index;
                    if let Some(items) = &subschemas.one_of {
                        let mut had_enum_string = false;
                        let mut parsed_enum_values: Vec<String> = Vec::new();
                        for item in items {
                            if let schemars::schema::Schema::Object(o) = item {
                                if let Some(enum_values) = &o.enum_values {
                                    for enum_value in enum_values {
                                        if let serde_json::value::Value::String(enum_value) = enum_value {
                                            had_enum_string = true;
                                            parsed_enum_values.push(format!("\"{}\"", enum_value));
                                        } else {
                                            had_enum_string = false;
                                            break;
                                        }
                                    }
                                    if !had_enum_string {
                                        break;
                                    }
                                } else {
                                    had_enum_string = false;
                                    break;
                                }
                            } else {
                                had_enum_string = false;
                                break;
                            }
                        }

                        if had_enum_string && !parsed_enum_values.is_empty() {
                            return Ok(Some((index, parsed_enum_values[0].to_string())));
                        } else if let Some(item) = items.iter().next() {
                            if let Some((new_index, snippet)) =
                                self.get_autocomplete_snippet_from_schema(item, index, in_keyword_fn, name)?
                            {
                                i = new_index + 1;
                                fn_docs.push_str(&snippet);
                            }
                        }
                    } else if let Some(items) = &subschemas.any_of {
                        if let Some(item) = items.iter().next() {
                            if let Some((new_index, snippet)) =
                                self.get_autocomplete_snippet_from_schema(item, index, in_keyword_fn, name)?
                            {
                                i = new_index + 1;
                                fn_docs.push_str(&snippet);
                            }
                        }
                    } else {
                        anyhow::bail!("unknown subschemas: {:#?}", subschemas);
                    }

                    return Ok(Some((i - 1, fn_docs)));
                }

                if let Some(schemars::schema::SingleOrVec::Single(single)) = &o.instance_type {
                    if schemars::schema::InstanceType::Boolean == **single {
                        return Ok(Some((index, format!(r#"${{{}:false}}"#, index))));
                    } else if schemars::schema::InstanceType::String == **single {
                        return Ok(Some((index, format!(r#"${{{}:"string"}}"#, index))));
                    } else if schemars::schema::InstanceType::Null == **single {
                        return Ok(None);
                    }
                }

                anyhow::bail!("unknown type: {:#?}", o)
            }
            schemars::schema::Schema::Bool(_) => Ok(Some((index, format!(r#"${{{}:false}}"#, index)))),
        }
    }
}

impl From<StdLibFnArg> for ParameterInformation {
    fn from(arg: StdLibFnArg) -> Self {
        ParameterInformation {
            label: ParameterLabel::Simple(arg.name.to_string()),
            documentation: arg.description(None).map(|description| {
                Documentation::MarkupContent(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: description,
                })
            }),
        }
    }
}

fn docs_for_type(ty: &str, kcl_std: &ModData) -> Option<String> {
    let key = if ty.starts_with("number") { "number" } else { ty };

    if DECLARED_TYPES.contains(&key) {
        if let Some(data) = kcl_std.find_by_name(key) {
            return data.summary().cloned();
        }
    }

    None
}

/// This trait defines functions called upon stdlib functions to generate
/// documentation for them.
pub trait StdLibFn: std::fmt::Debug + Send + Sync {
    /// The name of the function.
    fn name(&self) -> String;

    /// The summary of the function.
    fn summary(&self) -> String;

    /// The description of the function.
    fn description(&self) -> String;

    /// The tags of the function.
    fn tags(&self) -> Vec<String>;

    /// The args of the function.
    fn args(&self, inline_subschemas: bool) -> Vec<StdLibFnArg>;

    /// The return value of the function.
    fn return_value(&self, inline_subschemas: bool) -> Option<StdLibFnArg>;

    /// If the function is unpublished.
    fn unpublished(&self) -> bool;

    /// If the function is deprecated.
    fn deprecated(&self) -> bool;

    /// If the function should appear in the feature tree.
    fn feature_tree_operation(&self) -> bool;

    /// Any example code blocks.
    fn examples(&self) -> Vec<(String, bool)>;

    /// The function itself.
    fn std_lib_fn(&self) -> crate::std::StdFn;

    /// Helper function to clone the boxed trait object.
    fn clone_box(&self) -> Box<dyn StdLibFn>;

    /// Return a JSON struct representing the function.
    fn to_json(&self) -> Result<StdLibFnData> {
        Ok(StdLibFnData {
            name: self.name(),
            summary: self.summary(),
            description: self.description(),
            tags: self.tags(),
            args: self.args(false),
            return_value: self.return_value(false),
            unpublished: self.unpublished(),
            deprecated: self.deprecated(),
            examples: self.examples(),
        })
    }

    fn fn_signature(&self, include_name: bool) -> String {
        let mut signature = String::new();
        if include_name {
            signature.push_str(&self.name());
        }

        let args = self.args(false);
        if args.is_empty() {
            signature.push_str("()");
        } else if args.len() == 1 {
            signature.push('(');
            signature.push_str(&args[0].to_string());
            signature.push(')');
        } else {
            signature.push('(');
            for a in args {
                signature.push_str("\n  ");
                signature.push_str(&a.to_string());
                signature.push(',');
            }
            signature.push('\n');
            signature.push(')');
        }
        if let Some(return_value) = self.return_value(false) {
            signature.push_str(&format!(": {}", return_value.type_));
        }

        signature
    }

    fn to_completion_item(&self) -> Result<CompletionItem> {
        Ok(CompletionItem {
            label: self.name(),
            label_details: Some(CompletionItemLabelDetails {
                detail: Some(self.fn_signature(false)),
                description: None,
            }),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: None,
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: if !self.description().is_empty() {
                    format!("{}\n\n{}", self.summary(), self.description())
                } else {
                    self.summary()
                },
            })),
            deprecated: Some(self.deprecated()),
            preselect: None,
            sort_text: None,
            filter_text: None,
            insert_text: Some(self.to_autocomplete_snippet()?),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            insert_text_mode: None,
            text_edit: None,
            additional_text_edits: None,
            command: None,
            commit_characters: None,
            data: None,
            tags: None,
        })
    }

    fn to_autocomplete_snippet(&self) -> Result<String> {
        if self.name() == "loft" {
            return Ok("loft([${0:sketch000}, ${1:sketch001}])".to_string());
        } else if self.name() == "union" {
            return Ok("union([${0:extrude001}, ${1:extrude002}])".to_string());
        } else if self.name() == "subtract" {
            return Ok("subtract([${0:extrude001}], tools = [${1:extrude002}])".to_string());
        } else if self.name() == "intersect" {
            return Ok("intersect([${0:extrude001}, ${1:extrude002}])".to_string());
        } else if self.name() == "subtract2D" {
            return Ok("subtract2d(${0:%}, tool = ${1:%})".to_string());
        }
        let in_keyword_fn = true;
        let mut args = Vec::new();
        let mut index = 0;
        for arg in self.args(true).iter() {
            if let Some((i, arg_str)) = arg.get_autocomplete_snippet(index, in_keyword_fn)? {
                index = i + 1;
                args.push(arg_str);
            }
        }
        Ok(format!("{}({})", self.name(), args.join(", ")))
    }

    fn to_signature_help(&self) -> SignatureHelp {
        // Fill this in based on the current position of the cursor.
        let active_parameter = None;

        SignatureHelp {
            signatures: vec![SignatureInformation {
                label: self.fn_signature(true),
                documentation: Some(Documentation::MarkupContent(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: format!(
                        r#"{}

{}"#,
                        self.summary(),
                        self.description()
                    )
                    .trim()
                    .to_string(),
                })),
                parameters: Some(self.args(true).into_iter().map(|arg| arg.into()).collect()),
                active_parameter,
            }],
            active_signature: Some(0),
            active_parameter,
        }
    }
}

impl JsonSchema for dyn StdLibFn {
    fn schema_name() -> String {
        "StdLibFn".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        gen.subschema_for::<StdLibFnData>()
    }
}

impl Serialize for Box<dyn StdLibFn> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_json().unwrap().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Box<dyn StdLibFn> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let data = StdLibFnData::deserialize(deserializer)?;
        let stdlib = crate::std::StdLib::new();
        let stdlib_fn = stdlib
            .get(&data.name)
            .ok_or_else(|| serde::de::Error::custom(format!("StdLibFn {} not found", data.name)))?;
        Ok(stdlib_fn)
    }
}

impl ts_rs::TS for dyn StdLibFn {
    type WithoutGenerics = Self;

    fn name() -> String {
        "StdLibFnData".to_string()
    }

    fn decl() -> String {
        StdLibFnData::decl()
    }

    fn decl_concrete() -> String {
        StdLibFnData::decl_concrete()
    }

    fn inline() -> String {
        StdLibFnData::inline()
    }

    fn inline_flattened() -> String {
        StdLibFnData::inline_flattened()
    }

    fn output_path() -> Option<&'static Path> {
        StdLibFnData::output_path()
    }
}

impl Clone for Box<dyn StdLibFn> {
    fn clone(&self) -> Box<dyn StdLibFn> {
        self.clone_box()
    }
}

pub fn get_description_string_from_schema(schema: &schemars::schema::RootSchema) -> Option<String> {
    if let Some(metadata) = &schema.schema.metadata {
        if let Some(description) = &metadata.description {
            return Some(description.to_string());
        }
    }

    if let Some(reference) = &schema.schema.reference {
        if let Some(definition) = schema.definitions.get(reference.split('/').next_back().unwrap_or("")) {
            let schemars::schema::Schema::Object(definition) = definition else {
                return None;
            };
            if let Some(metadata) = &definition.metadata {
                if let Some(description) = &metadata.description {
                    return Some(description.to_string());
                }
            }
        }
    }

    // If we have subschemas iterate over them and recursively create references.
    if let Some(subschema) = &schema.schema.subschemas {
        if let Some(one_of) = &subschema.one_of {
            if one_of.len() == 1 {
                return get_description_string_from_schema(&schemars::schema::RootSchema {
                    meta_schema: schema.meta_schema.clone(),
                    schema: one_of[0].clone().into(),
                    definitions: schema.definitions.clone(),
                });
            }
        }

        if let Some(all_of) = &subschema.all_of {
            if all_of.len() == 1 {
                return get_description_string_from_schema(&schemars::schema::RootSchema {
                    meta_schema: schema.meta_schema.clone(),
                    schema: all_of[0].clone().into(),
                    definitions: schema.definitions.clone(),
                });
            }
        }

        if let Some(any_of) = &subschema.any_of {
            if any_of.len() == 1 {
                return get_description_string_from_schema(&schemars::schema::RootSchema {
                    meta_schema: schema.meta_schema.clone(),
                    schema: any_of[0].clone().into(),
                    definitions: schema.definitions.clone(),
                });
            }
        }
    }

    None
}

fn is_primitive(schema: &schemars::schema::Schema) -> Result<Option<Primitive>> {
    match schema {
        schemars::schema::Schema::Object(o) => {
            if o.enum_values.is_some() {
                // It's an enum so it's not a primitive.
                return Ok(None);
            }

            // Check if there
            if let Some(format) = &o.format {
                if format == "uuid" {
                    return Ok(Some(Primitive::Uuid));
                } else if format == "double"
                    || format == "uint"
                    || format == "int32"
                    || format == "int64"
                    || format == "uint8"
                    || format == "uint32"
                    || format == "uint64"
                {
                    return Ok(Some(Primitive::Number));
                } else {
                    anyhow::bail!("unknown format: {}", format);
                }
            }

            if o.object.is_some() {
                // It's an object so it's not a primitive.
                return Ok(None);
            }

            if o.array.is_some() {
                return Ok(None);
            }

            if o.subschemas.is_some() {
                return Ok(None);
            }

            if let Some(schemars::schema::SingleOrVec::Single(single)) = &o.instance_type {
                if schemars::schema::InstanceType::Boolean == **single {
                    return Ok(Some(Primitive::Bool));
                } else if schemars::schema::InstanceType::String == **single
                    || schemars::schema::InstanceType::Null == **single
                {
                    return Ok(Some(Primitive::String));
                }
            }

            if o.reference.is_some() {
                return Ok(None);
            }

            anyhow::bail!("unknown type: {:#?}", o)
        }
        schemars::schema::Schema::Bool(_) => Ok(Some(Primitive::Bool)),
    }
}

fn get_autocomplete_string_from_schema(schema: &schemars::schema::Schema) -> Result<String> {
    match schema {
        schemars::schema::Schema::Object(o) => {
            if let Some(enum_values) = &o.enum_values {
                let mut parsed_enum_values: Vec<String> = Default::default();
                let mut had_enum_string = false;
                for enum_value in enum_values {
                    if let serde_json::value::Value::String(enum_value) = enum_value {
                        had_enum_string = true;
                        parsed_enum_values.push(enum_value.to_owned());
                    } else {
                        had_enum_string = false;
                        break;
                    }
                }

                if had_enum_string && !parsed_enum_values.is_empty() {
                    return Ok(parsed_enum_values[0].to_string());
                }
            }

            if let Some(format) = &o.format {
                if format == "uuid" {
                    return Ok(Primitive::Uuid.to_string());
                } else if format == "double"
                    || format == "uint"
                    || format == "uint8"
                    || format == "int64"
                    || format == "uint32"
                    || format == "uint64"
                {
                    return Ok(Primitive::Number.to_string());
                } else {
                    anyhow::bail!("unknown format: {}", format);
                }
            }

            if let Some(obj_val) = &o.object {
                let mut fn_docs = String::new();
                fn_docs.push_str("{\n");
                // Let's print out the object's properties.
                for (prop_name, prop) in obj_val.properties.iter() {
                    if prop_name.starts_with('_') {
                        continue;
                    }

                    fn_docs.push_str(&format!(
                        "\t{}: {},\n",
                        prop_name,
                        get_autocomplete_string_from_schema(prop)?,
                    ));
                }

                fn_docs.push('}');

                return Ok(fn_docs);
            }

            if let Some(array_val) = &o.array {
                if let Some(schemars::schema::SingleOrVec::Single(items)) = &array_val.items {
                    // Let's print out the object's properties.
                    match array_val.max_items {
                        Some(val) => {
                            return Ok(format!(
                                "[{}]",
                                (0..val).map(|_| "number").collect::<Vec<_>>().join(", ")
                            ));
                        }
                        None => {
                            return Ok(format!("[{}]", get_autocomplete_string_from_schema(items)?));
                        }
                    };
                } else if let Some(items) = &array_val.contains {
                    return Ok(format!("[{}]", get_autocomplete_string_from_schema(items)?));
                }
            }

            if let Some(subschemas) = &o.subschemas {
                let mut fn_docs = String::new();
                if let Some(items) = &subschemas.one_of {
                    let mut had_enum_string = false;
                    let mut parsed_enum_values: Vec<String> = Vec::new();
                    for item in items {
                        if let schemars::schema::Schema::Object(o) = item {
                            if let Some(enum_values) = &o.enum_values {
                                for enum_value in enum_values {
                                    if let serde_json::value::Value::String(enum_value) = enum_value {
                                        had_enum_string = true;
                                        parsed_enum_values.push(format!("\"{}\"", enum_value));
                                    } else {
                                        had_enum_string = false;
                                        break;
                                    }
                                }
                                if !had_enum_string {
                                    break;
                                }
                            } else {
                                had_enum_string = false;
                                break;
                            }
                        } else {
                            had_enum_string = false;
                            break;
                        }
                    }

                    if had_enum_string && !parsed_enum_values.is_empty() {
                        return Ok(parsed_enum_values[0].to_string());
                    } else if let Some(item) = items.iter().next() {
                        // Let's print out the object's properties.
                        fn_docs.push_str(&get_autocomplete_string_from_schema(item)?);
                    }
                } else if let Some(items) = &subschemas.any_of {
                    if let Some(item) = items.iter().next() {
                        // Let's print out the object's properties.
                        fn_docs.push_str(&get_autocomplete_string_from_schema(item)?);
                    }
                } else {
                    anyhow::bail!("unknown subschemas: {:#?}", subschemas);
                }

                return Ok(fn_docs);
            }

            if let Some(schemars::schema::SingleOrVec::Single(single)) = &o.instance_type {
                if schemars::schema::InstanceType::Boolean == **single {
                    return Ok(Primitive::Bool.to_string());
                } else if schemars::schema::InstanceType::String == **single
                    || schemars::schema::InstanceType::Null == **single
                {
                    return Ok(Primitive::String.to_string());
                }
            }

            anyhow::bail!("unknown type: {:#?}", o)
        }
        schemars::schema::Schema::Bool(_) => Ok(Primitive::Bool.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::StdLibFn;
    use crate::docs::kcl_doc::{self, DocData};

    #[test]
    fn test_serialize_function() {
        let some_function = crate::parsing::ast::types::Function::StdLib {
            func: Box::new(crate::std::sketch::Line),
        };
        let serialized = serde_json::to_string(&some_function).unwrap();
        assert!(serialized.contains(r#"{"type":"StdLib""#));
    }

    #[test]
    fn test_deserialize_function() {
        let some_function_string = r#"{"type":"StdLib","func":{"name":"line","keywordArguments":false,"summary":"","description":"","tags":[],"returnValue":{"type":"","required":false,"name":"","schema":{},"schemaDefinitions":{}},"args":[],"unpublished":false,"deprecated":false, "examples": []}}"#;
        let some_function: crate::parsing::ast::types::Function = serde_json::from_str(some_function_string).unwrap();

        assert_eq!(
            some_function,
            crate::parsing::ast::types::Function::StdLib {
                func: Box::new(crate::std::sketch::Line)
            }
        );
    }

    #[test]
    fn get_autocomplete_snippet_line() {
        let line_fn: Box<dyn StdLibFn> = Box::new(crate::std::sketch::Line);
        let snippet = line_fn.to_autocomplete_snippet().unwrap();
        assert_eq!(snippet, r#"line(${0:%}, end = [${1:3.14}, ${2:3.14}])"#);
    }

    #[test]
    fn get_autocomplete_snippet_extrude() {
        let extrude_fn: Box<dyn StdLibFn> = Box::new(crate::std::extrude::Extrude);
        let snippet = extrude_fn.to_autocomplete_snippet().unwrap();
        assert_eq!(snippet, r#"extrude(${0:%}, length = ${1:3.14})"#);
    }

    #[test]
    fn get_autocomplete_snippet_fillet() {
        let data = kcl_doc::walk_prelude();
        let DocData::Fn(fillet_fn) = data.find_by_name("fillet").unwrap() else {
            panic!();
        };
        let snippet = fillet_fn.to_autocomplete_snippet();
        assert_eq!(snippet, r#"fillet(radius = ${0:3.14}, tags = [${1:tag_or_edge_fn}])"#);
    }

    #[test]
    fn get_autocomplete_snippet_start_sketch_on() {
        let start_sketch_on_fn: Box<dyn StdLibFn> = Box::new(crate::std::sketch::StartSketchOn);
        let snippet = start_sketch_on_fn.to_autocomplete_snippet().unwrap();
        assert_eq!(snippet, r#"startSketchOn(${0:XY})"#);
    }

    #[test]
    fn get_autocomplete_snippet_start_profile() {
        let start_sketch_on_fn: Box<dyn StdLibFn> = Box::new(crate::std::sketch::StartProfile);
        let snippet = start_sketch_on_fn.to_autocomplete_snippet().unwrap();
        assert_eq!(snippet, r#"startProfile(${0:%}, at = [${1:0}, ${2:0}])"#);
    }

    #[test]
    fn get_autocomplete_snippet_pattern_circular_3d() {
        // We test this one specifically because it has ints and floats and strings.
        let pattern_fn: Box<dyn StdLibFn> = Box::new(crate::std::patterns::PatternCircular3D);
        let snippet = pattern_fn.to_autocomplete_snippet().unwrap();
        assert_eq!(
            snippet,
            r#"patternCircular3d(${0:%}, instances = ${1:10}, axis = [${2:1}, ${3:0}, ${4:0}], center = [${5:0}, ${6:0}, ${7:0}])"#
        );
    }

    #[test]
    fn get_autocomplete_snippet_revolve() {
        let data = kcl_doc::walk_prelude();
        let DocData::Fn(revolve_fn) = data.find_by_name("revolve").unwrap() else {
            panic!();
        };
        let snippet = revolve_fn.to_autocomplete_snippet();
        assert_eq!(snippet, r#"revolve(axis = ${0:X})"#);
    }

    #[test]
    fn get_autocomplete_snippet_circle() {
        let data = kcl_doc::walk_prelude();
        let DocData::Fn(circle_fn) = data.find_by_name("circle").unwrap() else {
            panic!();
        };
        let snippet = circle_fn.to_autocomplete_snippet();
        assert_eq!(
            snippet, r#"circle(center = [${0:0}, ${1:0}], diameter = ${2:3.14})"#,
            "actual = left, expected = right"
        );
    }

    #[test]
    fn get_autocomplete_snippet_arc() {
        let arc_fn: Box<dyn StdLibFn> = Box::new(crate::std::sketch::Arc);
        let snippet = arc_fn.to_autocomplete_snippet().unwrap();
        assert_eq!(
            snippet,
            r#"arc(${0:%}, angleStart = ${1:3.14}, angleEnd = ${2:3.14}, radius = ${3:3.14})"#
        );
    }

    #[test]
    fn get_autocomplete_snippet_map() {
        let data = kcl_doc::walk_prelude();
        let DocData::Fn(map_fn) = data.find_by_name("map").unwrap() else {
            panic!();
        };
        let snippet = map_fn.to_autocomplete_snippet();
        assert_eq!(snippet, r#"map()"#);
    }

    #[test]
    fn get_autocomplete_snippet_pattern_linear_2d() {
        let pattern_fn: Box<dyn StdLibFn> = Box::new(crate::std::patterns::PatternLinear2D);
        let snippet = pattern_fn.to_autocomplete_snippet().unwrap();
        assert_eq!(
            snippet,
            r#"patternLinear2d(${0:%}, instances = ${1:10}, distance = ${2:3.14}, axis = [${3:1}, ${4:0}])"#
        );
    }

    #[test]
    fn get_autocomplete_snippet_appearance() {
        let appearance_fn: Box<dyn StdLibFn> = Box::new(crate::std::appearance::Appearance);
        let snippet = appearance_fn.to_autocomplete_snippet().unwrap();
        assert_eq!(
            snippet,
            r#"appearance(${0:%}, color = ${1:"#.to_owned() + "\"#" + r#"ff0000"})"#
        );
    }

    #[test]
    fn get_autocomplete_snippet_loft() {
        let loft_fn: Box<dyn StdLibFn> = Box::new(crate::std::loft::Loft);
        let snippet = loft_fn.to_autocomplete_snippet().unwrap();
        assert_eq!(snippet, r#"loft([${0:sketch000}, ${1:sketch001}])"#);
    }

    #[test]
    fn get_autocomplete_snippet_sweep() {
        let sweep_fn: Box<dyn StdLibFn> = Box::new(crate::std::sweep::Sweep);
        let snippet = sweep_fn.to_autocomplete_snippet().unwrap();
        assert_eq!(snippet, r#"sweep(${0:%}, path = ${1:sketch000})"#);
    }

    #[test]
    fn get_autocomplete_snippet_hole() {
        let f: Box<dyn StdLibFn> = Box::new(crate::std::sketch::Subtract2D);
        let snippet = f.to_autocomplete_snippet().unwrap();
        assert_eq!(snippet, r#"subtract2d(${0:%}, tool = ${1:%})"#);
    }

    #[test]
    fn get_autocomplete_snippet_helix() {
        let data = kcl_doc::walk_prelude();
        let DocData::Fn(helix_fn) = data.find_by_name("helix").unwrap() else {
            panic!();
        };
        let snippet = helix_fn.to_autocomplete_snippet();
        assert_eq!(
            snippet,
            r#"helix(revolutions = ${0:3.14}, angleStart = ${1:3.14}, radius = ${2:3.14}, axis = ${3:X}, length = ${4:3.14})"#
        );
    }

    #[test]
    fn get_autocomplete_snippet_union() {
        let union_fn: Box<dyn StdLibFn> = Box::new(crate::std::csg::Union);
        let snippet = union_fn.to_autocomplete_snippet().unwrap();
        assert_eq!(snippet, r#"union([${0:extrude001}, ${1:extrude002}])"#);
    }

    #[test]
    fn get_autocomplete_snippet_subtract() {
        let subtract_fn: Box<dyn StdLibFn> = Box::new(crate::std::csg::Subtract);
        let snippet = subtract_fn.to_autocomplete_snippet().unwrap();
        assert_eq!(snippet, r#"subtract([${0:extrude001}], tools = [${1:extrude002}])"#);
    }

    #[test]
    fn get_autocomplete_snippet_intersect() {
        let intersect_fn: Box<dyn StdLibFn> = Box::new(crate::std::csg::Intersect);
        let snippet = intersect_fn.to_autocomplete_snippet().unwrap();
        assert_eq!(snippet, r#"intersect([${0:extrude001}, ${1:extrude002}])"#);
    }

    #[test]
    fn get_autocomplete_snippet_get_common_edge() {
        let get_common_edge_fn: Box<dyn StdLibFn> = Box::new(crate::std::edge::GetCommonEdge);
        let snippet = get_common_edge_fn.to_autocomplete_snippet().unwrap();
        assert_eq!(
            snippet,
            r#"getCommonEdge(faces = [{
	value = ${0:"string"},
}])"#
        );
    }

    #[test]
    fn get_autocomplete_snippet_scale() {
        let scale_fn: Box<dyn StdLibFn> = Box::new(crate::std::transform::Scale);
        let snippet = scale_fn.to_autocomplete_snippet().unwrap();
        assert_eq!(snippet, r#"scale(${0:%}, x = ${1:3.14}, y = ${2:3.14}, z = ${3:3.14})"#);
    }

    #[test]
    fn get_autocomplete_snippet_translate() {
        let translate_fn: Box<dyn StdLibFn> = Box::new(crate::std::transform::Translate);
        let snippet = translate_fn.to_autocomplete_snippet().unwrap();
        assert_eq!(
            snippet,
            r#"translate(${0:%}, x = ${1:3.14}, y = ${2:3.14}, z = ${3:3.14})"#
        );
    }

    #[test]
    fn get_autocomplete_snippet_rotate() {
        let rotate_fn: Box<dyn StdLibFn> = Box::new(crate::std::transform::Rotate);
        let snippet = rotate_fn.to_autocomplete_snippet().unwrap();
        assert_eq!(
            snippet,
            r#"rotate(${0:%}, roll = ${1:3.14}, pitch = ${2:3.14}, yaw = ${3:3.14})"#
        );
    }

    #[test]
    #[allow(clippy::literal_string_with_formatting_args)]
    fn get_autocomplete_snippet_clone() {
        let data = kcl_doc::walk_prelude();
        let DocData::Fn(clone_fn) = data.find_by_name("clone").unwrap() else {
            panic!();
        };
        let snippet = clone_fn.to_autocomplete_snippet();
        assert_eq!(snippet, r#"clone(${0:part001})"#);
    }

    #[test]
    fn get_autocomplete_snippet_offset_plane() {
        let data = kcl_doc::walk_prelude();
        let DocData::Fn(offset_plane_fn) = data.find_by_name("offsetPlane").unwrap() else {
            panic!();
        };
        let snippet = offset_plane_fn.to_autocomplete_snippet();
        assert_eq!(snippet, r#"offsetPlane(${0:XY}, offset = ${1:3.14})"#);
    }

    // We want to test the snippets we compile at lsp start.
    #[test]
    fn get_all_stdlib_autocomplete_snippets() {
        let stdlib = crate::std::StdLib::new();
        let kcl_std = crate::docs::kcl_doc::walk_prelude();
        crate::lsp::kcl::get_completions_from_stdlib(&stdlib, &kcl_std).unwrap();
    }

    // We want to test the signatures we compile at lsp start.
    #[test]
    fn get_all_stdlib_signatures() {
        let stdlib = crate::std::StdLib::new();
        let kcl_std = crate::docs::kcl_doc::walk_prelude();
        crate::lsp::kcl::get_signatures_from_stdlib(&stdlib, &kcl_std);
    }

    #[test]
    fn get_extrude_signature_help() {
        let extrude_fn: Box<dyn StdLibFn> = Box::new(crate::std::extrude::Extrude);
        let sh = extrude_fn.to_signature_help();
        assert_eq!(
            sh.signatures[0].label,
            r#"extrude(
  @sketches: [Sketch],
  length: number,
  symmetric?: bool,
  bidirectionalLength?: number,
  tagStart?: TagNode,
  tagEnd?: TagNode,
): [Solid]"#
        );
    }
}
