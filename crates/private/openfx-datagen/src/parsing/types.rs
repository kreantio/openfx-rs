use std::{collections::BTreeSet, sync::LazyLock};

use regex::Regex;

#[derive(Debug, Default, Clone)]
pub struct BindingsUnprocessed {
    pub info: UnprocessedInfo,

    pub copyright_comments: Vec<String>,
    pub items: Vec<RootItemWithCommentAbove>,
}

#[derive(Debug, Default, Clone)]
pub struct UnprocessedInfo {
    pub includes: BTreeSet<String>,
    pub declared_types: BTreeSet<String>,
    pub defined_consts: BTreeSet<String>,
    pub referred_identifiers: BTreeSet<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(tag = "$type")]
pub enum RootItemWithCommentAbove {
    Item {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        comment_above: Option<String>,
        item: RootItem,
    },
    StandaloneComment {
        comment: String,
    },
}

impl RootItemWithCommentAbove {
    pub fn name(&self) -> Option<&str> {
        match self {
            RootItemWithCommentAbove::Item { item, .. } => Some(item.name()),
            RootItemWithCommentAbove::StandaloneComment { .. } => None,
        }
    }

    pub fn is_typedef(&self) -> bool {
        match self {
            RootItemWithCommentAbove::Item { item, .. } => item.is_typedef(),
            RootItemWithCommentAbove::StandaloneComment { .. } => false,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(tag = "$type")]
pub enum RootItem {
    Define {
        name: String,
        value: DefineValue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
    },
    TypedefPrimitive {
        name: String,
        c_type: CPrimitiveType,
    },
    TypedefOpaquePointer {
        name: String,
        pointee_struct_name: String,
    },
    TypedefFunction {
        name: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        parameters: Vec<FunctionParameter>,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        is_variadic: bool,
        return_type: TypeStraightforward,
    },
    TypedefStruct {
        name: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        fields: Vec<TypedefStructField>,
    },
    TypedefEnum {
        name: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        variants: Vec<TypedefEnumVariant>,
    },
}

impl RootItem {
    pub fn name(&self) -> &str {
        match self {
            RootItem::Define { name, .. } => name,
            RootItem::TypedefPrimitive { name, .. } => name,
            RootItem::TypedefOpaquePointer { name, .. } => name,
            RootItem::TypedefFunction { name, .. } => name,
            RootItem::TypedefStruct { name, .. } => name,
            RootItem::TypedefEnum { name, .. } => name,
        }
    }
    pub fn collect_referred_identifiers(&self, identifiers: &mut BTreeSet<String>) {
        match self {
            RootItem::Define { value, .. } => {
                if let Some(ty) = value.referred_identifier() {
                    identifiers.insert(ty.to_owned());
                }
            }
            RootItem::TypedefPrimitive { .. } => {}
            RootItem::TypedefOpaquePointer { .. } => {}
            RootItem::TypedefFunction {
                parameters,
                return_type,
                ..
            } => {
                for param in parameters {
                    if let Some(ty) = param.referred_identifier() {
                        identifiers.insert(ty.to_owned());
                    }
                }
                if let Some(ty) = return_type.referred_identifier() {
                    identifiers.insert(ty.to_owned());
                }
            }
            RootItem::TypedefStruct { fields, .. } => {
                for field in fields {
                    field.collect_referred_identifiers(identifiers);
                }
            }
            RootItem::TypedefEnum { variants, .. } => {
                for variant in variants {
                    variant.collect_referred_identifiers(identifiers);
                }
            }
        }
    }

    pub fn is_typedef(&self) -> bool {
        match self {
            RootItem::Define { .. } => false,
            RootItem::TypedefPrimitive { .. }
            | RootItem::TypedefOpaquePointer { .. }
            | RootItem::TypedefFunction { .. }
            | RootItem::TypedefStruct { .. }
            | RootItem::TypedefEnum { .. } => true,
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
    StringLiteral {
        value: String,
    },
    /// Integer literal.
    ///
    /// Currently, in the source code:
    /// - values can have the `0x` & `0X` prefix (but not the `0` prefix);
    /// - values are always not negative;
    /// - values can always be held by a `u32`.
    IntegerLiteral {
        value: u32,
    },
    /// `"false"` or `"true"`.
    BooleanLiteral {
        value: bool,
    },
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
    Identifier {
        value: String,
    },
    WellKnownIdentifier {
        value: WellKnownIdentifier,
    },
}

#[expect(non_camel_case_types)]
#[derive(
    Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub enum WellKnownIdentifier {
    INT_MAX,
    INT_MIN,
}

impl WellKnownIdentifier {
    pub fn try_from(value: &str) -> Option<Self> {
        match value {
            "INT_MAX" => Some(Self::INT_MAX),
            "INT_MIN" => Some(Self::INT_MIN),
            _ => None,
        }
    }
}

impl DefineValue {
    fn referred_identifier(&self) -> Option<&str> {
        match self {
            DefineValue::Identifier { value } => Some(value.as_str()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct FunctionParameter {
    pub name: String,
    pub r#type: TypeStraightforward,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl FunctionParameter {
    fn referred_identifier(&self) -> Option<&str> {
        self.r#type.referred_identifier()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(tag = "$type")]
pub enum TypeStraightforward {
    Ptr { pointee: Box<TypeStraightforward> },
    ConstPtr { pointee: Box<TypeStraightforward> },
    CPrimitive { is: CPrimitiveType },
    TypeIdentifier { is: TypeIdentifier },
}

impl TypeStraightforward {
    fn referred_identifier(&self) -> Option<&str> {
        match self {
            TypeStraightforward::TypeIdentifier { is } => Some(is.as_str()),
            TypeStraightforward::Ptr { pointee } => pointee.referred_identifier(),
            TypeStraightforward::ConstPtr { pointee } => pointee.referred_identifier(),
            _ => None,
        }
    }

    pub fn is_void(&self) -> bool {
        match self {
            TypeStraightforward::CPrimitive { is } => is.as_str() == "void",
            _ => false,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(tag = "$type")]
pub enum TypedefStructField {
    Item {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        comment_above: Option<String>,
        item: TypedefStructFieldItem,
    },
    StandaloneComment {
        comment: String,
    },
}

impl TypedefStructField {
    fn collect_referred_identifiers(&self, identifiers: &mut BTreeSet<String>) {
        match self {
            TypedefStructField::Item { item, .. } => item.collect_referred_identifiers(identifiers),
            TypedefStructField::StandaloneComment { .. } => {}
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct TypedefStructFieldItem {
    pub name: String,
    pub r#type: TypedefStructFieldType,
}

impl TypedefStructFieldItem {
    fn collect_referred_identifiers(&self, identifiers: &mut BTreeSet<String>) {
        self.r#type.collect_referred_identifiers(identifiers);
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(tag = "$type")]
pub enum TypedefStructFieldType {
    Straightforward {
        r#type: TypeStraightforward,
    },
    FunctionPointer {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        parameters: Vec<FunctionParameter>,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        is_variadic: bool,
        return_type: TypeStraightforward,
    },
}

impl TypedefStructFieldType {
    fn collect_referred_identifiers(&self, identifiers: &mut BTreeSet<String>) {
        match self {
            TypedefStructFieldType::Straightforward { r#type } => {
                if let Some(identifier) = r#type.referred_identifier() {
                    identifiers.insert(identifier.to_string());
                }
            }
            TypedefStructFieldType::FunctionPointer {
                parameters,
                return_type,
                ..
            } => {
                for param in parameters {
                    if let Some(identifier) = param.referred_identifier() {
                        identifiers.insert(identifier.to_string());
                    }
                }
                if let Some(identifier) = return_type.referred_identifier() {
                    identifiers.insert(identifier.to_string());
                }
            }
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct TypedefEnumVariant {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub c_value_expr: Option<TypedefEnumCValueExpr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl TypedefEnumVariant {
    /// TODO: parse `c_value_expr`?
    fn collect_referred_identifiers(&self, _identifiers: &mut BTreeSet<String>) {}
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
    regex(TYPED_INTEGER_LITERAL_C_TYPE_REGEX) = r#"^int$"#
);
define_string_guarded_by_regex!(
    CPrimitiveType,
    regex(C_PRIMITIVE_TYPE_REGEX) =
        r#"^(void|bool|char|int|float|double|size_t|unsigned (char|short|int))$"#
);
define_string_guarded_by_regex!(
    TypeIdentifier,
    regex(TYPE_IDENTIFIER_REGEX) = r#"^Ofx[A-Z][_a-zA-Z\d]*$"#
);
define_string_guarded_by_regex!(
    TypedefEnumCValueExpr,
    regex(TYPEDEF_ENUM_C_VALUE_EXPR_REGEX) =
        r#"^(0x[\da-fA-F]+|\(\s*[_a-zA-Z][_a-zA-Z\d]*\s*\|\s*[_a-zA-Z][_a-zA-Z\d]*\s*\))$"#
);
