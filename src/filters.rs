use crate::context::Context;
use crate::error::{Error, ParseError, ParseErrorKind, Result};
use crate::expression_evaluator::CallParams;
use crate::value::Value;
use std::collections::HashMap;

pub enum Filter {
    Abs,
    Capitalize,
    Center,
    Default,
    Escape,
    First,
    Float,
    Int,
    Last,
    Length,
    Lower,
    Max,
    Min,
    Round,
    String,
    Sum,
    Title,
    Truncate,
    Upper,
    WordCount,
}
impl Filter {
    pub fn new(name: &str) -> Result<Self> {
        match name {
            "abs" => Ok(Filter::Abs),
            "capitalize" => Ok(Filter::Capitalize),
            "center" => Ok(Filter::Center),
            "default" | "d" => Ok(Filter::Default),
            "escape" | "e" => Ok(Filter::Escape),
            "first" => Ok(Filter::First),
            "float" => Ok(Filter::Float),
            "int" => Ok(Filter::Int),
            "last" => Ok(Filter::Last),
            "length" | "count" => Ok(Filter::Length),
            "lower" => Ok(Filter::Lower),
            "max" => Ok(Filter::Max),
            "min" => Ok(Filter::Min),
            "round" => Ok(Filter::Round),
            "string" => Ok(Filter::String),
            "sum" => Ok(Filter::Sum),
            "title" => Ok(Filter::Title),
            "truncate" => Ok(Filter::Truncate),
            "upper" => Ok(Filter::Upper),
            "wordcount" => Ok(Filter::WordCount),
            unknown => Err(Error::from(ParseError::new(
                ParseErrorKind::UnknownFilter(unknown.to_string()),
                None,
            ))),
        }
    }
    pub fn filter(
        &self,
        base_value: Value,
        params: &Option<CallParams<'_>>,
        context: Context<'_>,
    ) -> Result<Value> {
        match &self {
            Filter::Abs => base_value.abs(),
            Filter::Capitalize => base_value.capitalize(),
            Filter::Center => {
                let parameters = if params.is_some() {
                    params.as_ref().unwrap().parse(vec!["width"], context)?
                } else {
                    HashMap::default()
                };

                base_value.center(parameters)
            }
            Filter::Default => {
                let parameters = if params.is_some() {
                    params
                        .as_ref()
                        .unwrap()
                        .parse(vec!["default_value"], context)?
                } else {
                    HashMap::default()
                };

                base_value.default_filter(parameters)
            }
            Filter::Escape => base_value.escape(),
            Filter::First => base_value.first(),
            Filter::Int => {
                let parameters = if params.is_some() {
                    params
                        .as_ref()
                        .unwrap()
                        .parse(vec!["default", "base"], context)?
                } else {
                    HashMap::default()
                };
                Ok(Value::Integer(base_value.int(parameters)?))
            }
            Filter::Float => {
                let parameters = if params.is_some() {
                    params.as_ref().unwrap().parse(vec!["default"], context)?
                } else {
                    HashMap::default()
                };

                Ok(Value::Double(base_value.float(parameters)?))
            }
            Filter::Last => base_value.last(),
            Filter::Length => Ok(Value::Integer(base_value.len()? as i64)),
            Filter::Lower => base_value.lower(),
            Filter::Max => base_value.max(), // TODO Accept params
            Filter::Min => base_value.min(), // TODO Accept params
            Filter::Round => {
                let parameters = if params.is_some() {
                    params
                        .as_ref()
                        .unwrap()
                        .parse(vec!["precision", "method"], context)?
                } else {
                    HashMap::default()
                };
                base_value.round(parameters)
            }
            Filter::String => Ok(Value::String(base_value.to_string())),
            Filter::Sum => base_value.sum(), // TODO: ACcept params
            Filter::Title => base_value.title(),
            Filter::Truncate => {
                let parameters = if params.is_some() {
                    params
                        .as_ref()
                        .unwrap()
                        .parse(vec!["length", "end"], context)?
                } else {
                    HashMap::default()
                };

                base_value.truncate(parameters)
            } // TODO: Add additional parameters.,
            Filter::Upper => base_value.upper(),
            Filter::WordCount => base_value.wordcount(),
        }
    }
}

pub struct FilterExpression<'a> {
    filter: Filter,
    params: Option<CallParams<'a>>,
    parent: Option<Box<FilterExpression<'a>>>,
}

impl<'a> FilterExpression<'a> {
    pub fn new(identifier: &str, params: Option<CallParams<'a>>) -> Result<Self> {
        let filter = Filter::new(identifier)?;
        Ok(Self {
            filter,
            params,
            parent: None,
        })
    }
    pub fn set_parent_filter(&mut self, parent: FilterExpression<'a>) {
        self.parent = Some(Box::new(parent));
    }

    pub fn filter(&self, base_value: Value, context: Context<'_>) -> Result<Value> {
        if self.parent.is_some() {
            self.filter.filter(
                self.parent
                    .as_ref()
                    .unwrap()
                    .filter(base_value, context.clone())?,
                &self.params,
                context,
            )
        } else {
            self.filter.filter(base_value, &self.params, context)
        }
    }
}
