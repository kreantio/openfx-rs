pub enum CodegenConfigStringMatcher {
    Exact(String),
    Regex(regex::Regex),
}

impl<'de> serde::Deserialize<'de> for CodegenConfigStringMatcher {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        #[serde(untagged)]
        enum OneOf {
            Exact(String),
            Regex { regex: String },
        }
        let one_of = OneOf::deserialize(deserializer)?;
        Ok(match one_of {
            OneOf::Exact(s) => CodegenConfigStringMatcher::Exact(s),
            OneOf::Regex { regex } => {
                CodegenConfigStringMatcher::Regex(regex::Regex::new(&regex).map_err(|e| {
                    serde::de::Error::custom(format!("Invalid regex `{regex}`: {e}"))
                })?)
            }
        })
    }
}

impl CodegenConfigStringMatcher {
    pub fn matches(&self, s: &str) -> bool {
        match self {
            CodegenConfigStringMatcher::Exact(exact) => exact == s,
            CodegenConfigStringMatcher::Regex(re) => re.is_match(s),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct CodegenConfigStringMatcherAnyOf(pub Vec<CodegenConfigStringMatcher>);

impl CodegenConfigStringMatcherAnyOf {
    pub fn matches(&self, s: &str) -> bool {
        self.0.iter().any(|matcher| matcher.matches(s))
    }
}
