use crate::parser::{GaParser, Rule};
use pest::Parser;
use std::collections::BTreeMap;

#[test]
pub fn all_input_should_pass() {
    let mut file_mappings = BTreeMap::new();
    file_mappings.insert("chromosome_test_*.txt", Rule::ChromosomeSection);
    file_mappings.insert("evaluation_test_*.txt", Rule::EvaluationSection);
    file_mappings.insert("ga_test_*.txt", Rule::GrammarFile);
    for (patt, rule) in file_mappings {
        let paths = glob::glob(&format!("src/parser/test/{}", patt)).unwrap();
        for path in paths {
            let path = path.unwrap();
            let input = std::fs::read_to_string(&path).unwrap();
            let parsed = GaParser::parse(rule, &input).map_err(|e| e.to_string());
            assert_eq!(None, parsed.err(), "Failed to parse file: {:?}", path);
            println!("Parsed file: {:?}", path);
        }
    }
}
