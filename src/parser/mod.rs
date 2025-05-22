#[cfg(test)]
mod test;

use pest::Parser;

pub struct GaAst {
    pub chromosome: ChromosomeAst,
    pub evaluation: EvaluationAst,
}

pub struct ChromosomeAst {}

pub struct EvaluationAst {}

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct GaParser {}

pub fn parse_input(input: &str) -> Result<GaAst, String> {
    let parsed = GaParser::parse(Rule::GrammarFile, input).map_err(|e| e.to_string())?;
    for part in parsed {
        for (i, inner_part) in part.into_inner().enumerate() {
            println!("[{i}] {:?} - {}", inner_part.as_rule(), inner_part.as_str());
        }
    }
    Ok(GaAst {
        chromosome: ChromosomeAst {},
        evaluation: EvaluationAst {},
    })
}
