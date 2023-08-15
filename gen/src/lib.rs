mod filters;
mod language;
pub mod templates;

use liquid::Parser as LiquidParser;
use liquid_core::model::KString;
use liquid_core::{Object, Value, ValueView};
use seedle_parser::FlattenError;
use std::collections::BTreeMap;

pub struct Parser {
    context: Object,
    parser: LiquidParser,
}
impl Parser {
    pub fn build() -> liquid_core::Result<Parser> {
        let context = Object::new();
        liquid::ParserBuilder::with_stdlib()
            .filter(crate::filters::case::LowerCamelCase)
            .filter(crate::filters::case::UpperCamelCase)
            .filter(crate::filters::case::SnakeCase)
            .filter(crate::filters::case::ShoutySnakeCase)
            .filter(crate::filters::collect::Collect)
            .filter(crate::filters::field::Field)
            .build()
            .map(|parser| Parser { parser, context })
    }

    pub fn load_cddl<K: Into<KString>>(&mut self, key: K, cddl: &str) -> Result<(), FlattenError> {
        let nodes = seedle_parser::parse(cddl)?
            .into_iter()
            .map(|(k, v)| (k, Value::from(v)))
            .collect::<BTreeMap<_, Value>>();
        self.context.insert(key.into(), nodes.to_value());
        Ok(())
    }

    pub fn render(&self, text: &str) -> liquid_core::Result<String> {
        self.parser.parse(text)?.render(&self.context)
    }

    pub fn render_to(
        &self,
        writer: &mut dyn std::io::Write,
        text: &str,
    ) -> liquid_core::Result<()> {
        self.parser
            .parse(text)?
            .render_to(writer, &self.context)
    }
}
