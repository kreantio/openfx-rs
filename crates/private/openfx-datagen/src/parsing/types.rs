use std::{
    collections::{BTreeSet, HashMap},
    sync::LazyLock,
};

use regex::Regex;

/// ## TODO
///
/// Remove `serde::Serialize`, `serde::Deserialize`, and `schemars::JsonSchema`.
/// Because [`Bindings`] is the one that will be serialized and deserialized.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct BindingsUnprocessed {
    #[serde(skip_serializing_if = "BTreeSet::is_empty")]
    pub unprocessed_includes: BTreeSet<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub copyright_comments: Vec<String>,
    pub items: Vec<RootItemWithCommentAbove>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct Bindings {
    pub copyright_comments: Vec<String>,

    /// key: header file stem name
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub used_types: HashMap<String, String>,
    /// key: header file stem name
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub used_values: HashMap<String, String>,

    pub items: Vec<RootItemWithCommentAbove>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(tag = "$type")]
pub enum RootItemWithCommentAbove {
    Item {
        #[serde(skip_serializing_if = "Option::is_none")]
        comment_above: Option<String>,
        item: RootItem,
    },
    StandaloneComment {
        comment: String,
    },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(tag = "$type")]
pub enum RootItem {
    Define {
        name: String,
        value: DefineValue,
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
    },
    TypedefPrimitive {
        name: String,
        c_type: TypedefPrimitiveCType,
    },
    TypedefOpaquePointer {
        name: String,
        pointee_struct_name: String,
    },
    TypedefFunction {
        name: String,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        parameters: Vec<FunctionParameter>,
        return_type: TypedefFunctionReturnType,
    },
    TypedefStruct {
        name: String,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        fields: Vec<TypedefStructField>,
    },
    TypedefEnum {
        name: String,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        variants: Vec<TypedefEnumVariant>,
    },

    Todo {
        kind: String,
        code: String,
    },
}

impl RootItem {
    pub fn name(&self) -> &str {
        match self {
            RootItem::Define { name, .. } => name,
            _ => todo!(),
        }
    }
}

/// The value of a `#define` directive that appears in the C headers of the
/// OpenFX standard.
#[derive(
    Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(tag = "$type")]
pub enum DefineValue {
    /// String literal inside the quotes. Its contents are guaranteed to be
    /// unescaped by panicking if the string contains `\` characters.
    StringLiteral { value: String },
    /// Integer literal.
    ///
    /// Currently, in the source code:
    /// - values can have the `0x` & `0X` prefix (but not the `0` prefix);
    /// - values are always not negative;
    /// - values can always be held by a `u32`.
    IntegerLiteral { value: u32 },
    /// `"false"` or `"true"`.
    BooleanLiteral { value: bool },
    /// Things like `#define kOfxStatFailed  ((int)1)` and
    /// `#define kOfxStatGPUOutOfMemory  ((int) 1001)`
    ///
    /// Currently, in the source code:
    /// - values do not use a prefix like `0x`;
    /// - values are always not negative;
    /// - values can always be held by a `u32`.
    TypedIntegerLiteral {
        c_type: TypedIntegerLiteralCType,
        value: u32,
    },
    /// e.g., `#define kOfxActionDescribeInteract kOfxActionDescribe`.
    Symbol { value: String },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct FunctionParameter {
    pub name: String,
    pub r#type: FunctionParameterType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(tag = "$type")]
pub enum FunctionParameterType {
    Ptr { pointee: Box<FunctionParameterType> },
    ConstPtr { pointee: Box<FunctionParameterType> },
    SimpleC { name: TypeSimpleCName },
    SimpleNonC { name: TypeSimpleNonCName },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(tag = "$type")]
pub enum TypedefFunctionReturnType {
    OfxStatus,
    Void,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(tag = "$type")]
pub enum TypedefStructField {
    Item {
        #[serde(skip_serializing_if = "Option::is_none")]
        comment_above: Option<String>,
        item: TypedefStructFieldItem,
    },
    StandaloneComment {
        comment: String,
    },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct TypedefStructFieldItem {
    pub name: String,
    pub r#type: TypedefStructFieldType,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(tag = "$type")]
pub enum TypedefStructFieldType {
    ConstUnsignedCharPtr,
    ConstCharPtr,

    /// `OfxPluginEntryPoint *`
    ///
    /// TODO: This is horrible. Refactor to another representation before the PR
    /// is merged.
    OfxPluginEntryPointPtr,

    SimpleC {
        name: TypeSimpleCName,
    },
    SimpleNonC {
        name: TypeSimpleNonCName,
    },

    FunctionPointer {
        #[serde(skip_serializing_if = "Vec::is_empty")]
        parameters: Vec<FunctionParameter>,
        return_type: TypedefStructFieldTypeFunctionPointerReturnType,
    },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(tag = "$type")]
pub enum TypedefStructFieldTypeFunctionPointerReturnType {
    ConstVoidPtr,
    OfxStatus,

    SimpleC { name: TypeSimpleCName },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct TypedefEnumVariant {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_value_expr: Option<TypedefEnumCValueExpr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

macro_rules! define_string_guarded_by_regex {
    ($name:ident, regex($regex_name:ident) = $regex:expr) => {
        #[derive(
            Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
        )]
        #[serde(transparent)]
        pub struct $name(#[schemars(regex(pattern = $regex_name))] String);
        static $regex_name: LazyLock<Regex> = LazyLock::new(|| Regex::new($regex).unwrap());
        impl $name {
            pub(crate) fn try_from(code: &str) -> Result<Self, ()> {
                if $regex_name.is_match(code) {
                    return Ok(Self(code.to_owned()));
                }
                Err(())
            }
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

define_string_guarded_by_regex!(
    TypedIntegerLiteralCType,
    regex(TYPED_INTEGER_LITERAL_CTYPE_REGEX) = r#"^int$"#
);
define_string_guarded_by_regex!(
    TypedefPrimitiveCType,
    regex(TYPEDEF_PRIMITIVE_CTYPE_REGEX) = r#"^(int|double)$"#
);
define_string_guarded_by_regex!(
    TypeSimpleCName,
    regex(TYPEDEF_STRUCT_FIELD_TYPE_SIMPLE_C_NAME_REGEX) =
        r#"^(void|char|int|float|double|size_t|unsigned (char|short|int))$"#
);
define_string_guarded_by_regex!(
    TypeSimpleNonCName,
    regex(TYPE_SIMPLE_NON_C_NAME_REGEX) = r#"^[_a-zA-Z][_a-zA-Z0-9]*Handle$"#
);
define_string_guarded_by_regex!(
    TypedefEnumCValueExpr,
    regex(TYPEDEF_ENUM_C_VALUE_EXPR_REGEX) =
        r#"^(0x[0-9a-fA-F]+|\(\s*[_a-zA-Z][_a-zA-Z0-9]*\s*\|\s*[_a-zA-Z][_a-zA-Z0-9]*\s*\))$"#
);
