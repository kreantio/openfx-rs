use crate::doc_parsing::CHeaderDocParseOutput;

#[derive(Debug, snafu::Snafu)]
pub enum Error {}

pub fn parse_docs(c_code: &str) -> Result<CHeaderDocParseOutput, Error> {
    todo!()
}

/// additional tests
mod tests {}
