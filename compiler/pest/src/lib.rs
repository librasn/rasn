use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "asn1.pest"]
pub struct Asn1Parser;

#[cfg(test)]
mod tests {
    use pest::Parser;

    use super::*;

    #[test]
    fn basic_definition() {
        let input = include_str!("../../tests/basic.asn1");

        Asn1Parser::parse(Rule::ModuleDefinition, input).unwrap_or_else(|e| panic!("{}", e));
    }

    #[test]
    fn pkcs12() {
        let input = include_str!("../../asn1/pkcs12.asn1");

        Asn1Parser::parse(Rule::ModuleDefinition, input).unwrap_or_else(|e| panic!("{}", e));
    }

    #[test]
    fn pkcs12_header() {
        let input = include_str!("../../asn1/pkcs12.asn1");

        Asn1Parser::parse(Rule::ModuleHeaderOnly, input).unwrap_or_else(|e| panic!("{}", e));
    }
}
