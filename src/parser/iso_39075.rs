use pest::Parser;
use pest_derive::Parser;

use crate::ast::GqlProgram;


#[derive(Parser)]
#[grammar = "ISO_IEC_39075_2024_GQL.pest"]
pub struct GqlParser;

pub fn parse(query: &str) -> Result<GqlProgram, pest::error::Error<Rule>> {
    let _ = GqlParser::parse(Rule::GqlProgram, query)?;
    
    todo!()
}