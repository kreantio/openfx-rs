use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use crate::parsing::{BindingsUnprocessed, RootItemWithCommentAbove};

#[derive(Debug, snafu::Snafu)]
pub enum Error {
    CircularInclusion,
    MissingInclusion { file_names: HashSet<String> },
    ProcessingErrors { errors: Vec<ProcessingError> },
}

#[derive(Debug, snafu::Snafu)]
pub enum ProcessingError {
    DuplicateType {
        type_name: String,
        file_names: (String, String),
    },
    DuplicateConst {
        const_name: String,
        file_names: (String, String),
    },
    UnresolvedIdentifier {
        file_name: String,
        identifier_name: String,
    },
    AmbiguousIdentifier {
        identifier_name: String,
    },
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct Bindings {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub copyright_comments: Vec<String>,

    /// key: header file stem name
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub used_types: BTreeMap<String, BTreeSet<String>>,
    /// key: header file stem name
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub used_consts: BTreeMap<String, BTreeSet<String>>,

    pub items: Vec<RootItemWithCommentAbove>,
}

/// Keys should be header file names with `.h`.
pub fn process(
    input: BTreeMap<String, BindingsUnprocessed>,
) -> Result<HashMap<String, Bindings>, Error> {
    let input = topological_sort(input)?;
    let mut output: HashMap<String, Bindings> = HashMap::new();

    let mut type_to_file: HashMap<String, String> = HashMap::new();
    let mut const_to_file: HashMap<String, String> = HashMap::new();

    let mut errors: Vec<ProcessingError> = vec![];

    for (file_name, bindings) in input {
        let mut processed = Bindings {
            copyright_comments: bindings.copyright_comments,
            items: bindings.items,
            ..Default::default()
        };

        let mut missing_idents = HashSet::<String>::new();
        for ident in bindings.info.referred_identifiers {
            if let Some(file_name) = type_to_file.get(&ident) {
                processed
                    .used_types
                    .entry(file_name.clone())
                    .or_default()
                    .insert(ident.clone());
            } else if let Some(file_name) = const_to_file.get(&ident) {
                processed
                    .used_consts
                    .entry(file_name.clone())
                    .or_default()
                    .insert(ident.clone());
            } else if !bindings.info.declared_types.contains(&ident)
                && !bindings.info.defined_consts.contains(&ident)
            {
                missing_idents.insert(ident);
            }
        }
        if !missing_idents.is_empty() {
            errors.extend(missing_idents.into_iter().map(|identifier| {
                ProcessingError::UnresolvedIdentifier {
                    file_name: file_name.clone(),
                    identifier_name: identifier,
                }
            }));
        }

        for ident in bindings.info.declared_types {
            if let Some(another_file_name) = type_to_file.insert(ident.clone(), file_name.clone()) {
                errors.push(ProcessingError::DuplicateType {
                    type_name: ident,
                    file_names: (file_name.clone(), another_file_name),
                });
            }
        }
        for ident in bindings.info.defined_consts {
            if let Some(another_file_name) = const_to_file.insert(ident.clone(), file_name.clone())
            {
                errors.push(ProcessingError::DuplicateConst {
                    const_name: ident,
                    file_names: (file_name.clone(), another_file_name),
                });
            }
        }

        output.insert(file_name, processed);
    }

    let types: HashSet<_> = type_to_file.keys().collect();
    let consts: HashSet<_> = const_to_file.keys().collect();
    let ambiguous_idents: HashSet<_> = types.intersection(&consts).cloned().collect();
    if !ambiguous_idents.is_empty() {
        errors.extend(ambiguous_idents.into_iter().map(|identifier_name| {
            ProcessingError::AmbiguousIdentifier {
                identifier_name: identifier_name.to_owned(),
            }
        }));
    }

    if !errors.is_empty() {
        return Err(Error::ProcessingErrors { errors });
    }

    Ok(output)
}

fn topological_sort(
    input: BTreeMap<String, BindingsUnprocessed>,
) -> Result<Vec<(String, BindingsUnprocessed)>, Error> {
    let all_file_names: HashSet<_> = input.keys().cloned().collect();
    let all_referred_file_names: HashSet<_> = input
        .values()
        .flat_map(|bindings| &bindings.info.includes)
        .cloned()
        .collect();
    let missing_file_names: HashSet<_> = all_referred_file_names
        .difference(&all_file_names)
        .cloned()
        .collect();
    if !missing_file_names.is_empty() {
        return Err(Error::MissingInclusion {
            file_names: missing_file_names,
        });
    }

    let mut unordered: Vec<(String, BindingsUnprocessed)> = input.into_iter().collect();
    // Keep `ofxCore.h` at the beginning, since some files (currently
    // `ofxMemory.h`, `ofxProgress.h`, and `ofxTimeLine.h`) rely on `OfxStatus`
    // but don't include `ofxCore.h` themselves.
    let mut ordered: Vec<(String, BindingsUnprocessed)> = unordered
        .extract_if(.., |(name, _)| name == "ofxCore.h")
        .collect();

    let mut last_unordered_len: Option<usize> = None;
    loop {
        if let Some(last_unordered_len) = last_unordered_len
            && last_unordered_len == unordered.len()
        {
            return Err(Error::CircularInclusion);
        }
        last_unordered_len = Some(unordered.len());

        let satisfied: Vec<_> = unordered
            .extract_if(.., |(_name, bindings)| {
                bindings
                    .info
                    .includes
                    .iter()
                    .all(|include| ordered.iter().any(|(other_name, _)| other_name == include))
            })
            .collect();

        ordered.extend(satisfied);
        if unordered.is_empty() {
            break;
        }
    }

    Ok(ordered)
}
