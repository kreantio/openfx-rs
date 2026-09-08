//! Authors:
//! - OpenCode / Omen Alpha (Default)

use crate::{CodegenConfigObjectMappingEntry, CodegenConfigObjectParameterSetMappingEntry};

#[expect(non_snake_case)]
pub fn deserialize_CodegenConfigObjectMappingEntry<'de, D>(
    deserializer: D,
) -> Result<CodegenConfigObjectMappingEntry, D::Error>
where
    D: serde::Deserializer<'de>,
{
    const FIELDS: &[&str] = &["is", "set", "omit"];

    struct Visitor;

    impl<'de> serde::de::Visitor<'de> for Visitor {
        type Value = CodegenConfigObjectMappingEntry;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter
                .write_str("a type name string or a table with `is` and optional `set` and `omit`")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Self::Value {
                is: v.to_owned(),
                set: false,
                omit: false,
            })
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
        {
            let mut is: Option<String> = None;
            let mut set = false;
            let mut omit = false;
            while let Some(key) = map.next_key::<String>()? {
                match key.as_str() {
                    "is" => is = Some(map.next_value()?),
                    "set" => set = map.next_value()?,
                    "omit" => omit = map.next_value()?,
                    _ => return Err(serde::de::Error::unknown_field(&key, FIELDS)),
                }
            }

            Ok(Self::Value {
                is: is.ok_or_else(|| serde::de::Error::missing_field("is"))?,
                set,
                omit,
            })
        }
    }

    deserializer.deserialize_any(Visitor)
}

/// Author: OpenCode / Omen Alpha (Default)
#[expect(non_snake_case)]
pub fn deserialize_CodegenConfigObjectParameterSetMappingEntry<'de, D>(
    deserializer: D,
) -> Result<CodegenConfigObjectParameterSetMappingEntry, D::Error>
where
    D: serde::Deserializer<'de>,
{
    const FIELDS: &[&str] = &["is", "set", "d", "ty"];

    struct Visitor;

    impl<'de> serde::de::Visitor<'de> for Visitor {
        type Value = CodegenConfigObjectParameterSetMappingEntry;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("a table with `is`, `set`, `d` and `ty`")
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
        {
            let mut is: Option<String> = None;
            let mut set: Option<String> = None;
            let mut d: Option<u32> = None;
            let mut ty: Option<String> = None;
            while let Some(key) = map.next_key::<String>()? {
                match key.as_str() {
                    "is" => is = Some(map.next_value()?),
                    "set" => set = Some(map.next_value()?),
                    "d" => d = Some(map.next_value()?),
                    "ty" => ty = Some(map.next_value()?),
                    _ => return Err(serde::de::Error::unknown_field(&key, FIELDS)),
                }
            }

            let is = is.ok_or_else(|| serde::de::Error::missing_field("is"))?;
            let set = set.ok_or_else(|| serde::de::Error::missing_field("set"))?;
            let d: u32 = d.ok_or_else(|| serde::de::Error::missing_field("d"))?;
            let ty = ty.ok_or_else(|| serde::de::Error::missing_field("ty"))?;

            if is != "OfxParamHandle" {
                return Err(serde::de::Error::custom(format_args!(
                    "`is` must be \"OfxParamHandle\", found \"{is}\"",
                )));
            }

            if (d == 0) != ty.is_empty() {
                return Err(serde::de::Error::custom(format_args!(
                    "`d = 0` must be paired with `ty = \"\"` and vice versa, found `d = {d}` and `ty = \"{ty}\"`",
                )));
            }

            Ok(Self::Value {
                is,
                set,
                ty_d: if d == 0 { None } else { Some((ty, d)) },
            })
        }
    }

    deserializer.deserialize_map(Visitor)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{CodegenConfigObjectMappingEntry, CodegenConfigObjectParameterSetMappingEntry};

    fn parse_object_mapping(
        toml_str: &str,
    ) -> Result<HashMap<String, CodegenConfigObjectMappingEntry>, toml::de::Error> {
        toml::from_str(toml_str)
    }

    fn parse_parameter_set_mapping(
        toml_str: &str,
    ) -> Result<HashMap<String, CodegenConfigObjectParameterSetMappingEntry>, toml::de::Error> {
        toml::from_str(toml_str)
    }

    fn parse_err<T>(toml_str: &str) -> String
    where
        T: serde::de::DeserializeOwned,
    {
        let Err(err) = toml::from_str::<T>(toml_str) else {
            panic!("expected an error, parsed: {toml_str}");
        };
        err.to_string()
    }

    #[test]
    fn object_mapping_accepts_string_value() {
        let mapping = parse_object_mapping(r#"EffectDescriptor = "OfxImageEffectHandle""#).unwrap();
        assert_eq!(mapping["EffectDescriptor"].is, "OfxImageEffectHandle");
        assert!(!mapping["EffectDescriptor"].set);
        assert!(!mapping["EffectDescriptor"].omit);
    }

    #[test]
    fn object_mapping_accepts_table_value() {
        let mapping = parse_object_mapping(
            r#"
EffectDescriptor = { is = "OfxImageEffectHandle", set = true }
ImageEffectHost = { is = "OfxPropertySetHandle", omit = true }
DrawContext = { is = "OfxDrawContextHandle" }
"#,
        )
        .unwrap();
        assert_eq!(mapping["EffectDescriptor"].is, "OfxImageEffectHandle");
        assert!(mapping["EffectDescriptor"].set);
        assert!(!mapping["EffectDescriptor"].omit);
        assert_eq!(mapping["ImageEffectHost"].is, "OfxPropertySetHandle");
        assert!(!mapping["ImageEffectHost"].set);
        assert!(mapping["ImageEffectHost"].omit);
        assert_eq!(mapping["DrawContext"].is, "OfxDrawContextHandle");
        assert!(!mapping["DrawContext"].set);
        assert!(!mapping["DrawContext"].omit);
    }

    #[test]
    fn object_mapping_rejects_unknown_and_missing_fields() {
        let err = parse_err::<HashMap<String, CodegenConfigObjectMappingEntry>>(
            r#"X = { is = "A", bogus = true }"#,
        );
        assert!(err.contains("unknown field `bogus`"), "{err}");
        let err =
            parse_err::<HashMap<String, CodegenConfigObjectMappingEntry>>(r#"X = { omit = true }"#);
        assert!(err.contains("missing field `is`"), "{err}");
    }

    #[test]
    fn parameter_set_mapping_accepts_d_nonzero() {
        let mapping = parse_parameter_set_mapping(
            r#"
ParamInteger = { is = "OfxParamHandle", set = "ParamsByte", d = 1, ty = "Int" }
ParamRGBA = { is = "OfxParamHandle", set = "ParamDouble2D3D", d = 4, ty = "Double" }
"#,
        )
        .unwrap();
        assert_eq!(mapping["ParamInteger"].is, "OfxParamHandle");
        assert_eq!(mapping["ParamInteger"].set, "ParamsByte");
        assert_eq!(mapping["ParamInteger"].ty_d, Some(("Int".to_owned(), 1)));
        assert_eq!(mapping["ParamRGBA"].ty_d, Some(("Double".to_owned(), 4)));
    }

    #[test]
    fn parameter_set_mapping_accepts_d_zero() {
        let mapping = parse_parameter_set_mapping(
            r#"ParamGroup = { is = "OfxParamHandle", set = "ParamsGroup", d = 0, ty = "" }"#,
        )
        .unwrap();
        assert_eq!(mapping["ParamGroup"].is, "OfxParamHandle");
        assert_eq!(mapping["ParamGroup"].set, "ParamsGroup");
        assert_eq!(mapping["ParamGroup"].ty_d, None);
    }

    #[test]
    fn parameter_set_mapping_rejects_wrong_is() {
        let err = parse_err::<HashMap<String, CodegenConfigObjectParameterSetMappingEntry>>(
            r#"X = { is = "OfxPropertySetHandle", set = "S", d = 1, ty = "Int" }"#,
        );
        assert!(err.contains("`is` must be \"OfxParamHandle\""), "{err}");
    }

    #[test]
    fn parameter_set_mapping_rejects_d_ty_mismatch() {
        for (d, ty) in [(0, "Int"), (1, "")] {
            let err = parse_err::<HashMap<String, CodegenConfigObjectParameterSetMappingEntry>>(
                &format!(r#"X = {{ is = "OfxParamHandle", set = "S", d = {d}, ty = "{ty}" }}"#,),
            );
            assert!(err.contains("must be paired"), "{err}");
        }
    }

    #[test]
    fn parameter_set_mapping_rejects_unknown_and_missing_fields() {
        let err = parse_err::<HashMap<String, CodegenConfigObjectParameterSetMappingEntry>>(
            r#"X = { is = "OfxParamHandle", set = "S", d = 1, ty = "Int", bogus = true }"#,
        );
        assert!(err.contains("unknown field `bogus`"), "{err}");
        let err = parse_err::<HashMap<String, CodegenConfigObjectParameterSetMappingEntry>>(
            r#"X = { is = "OfxParamHandle", set = "S", d = 1 }"#,
        );
        assert!(err.contains("missing field `ty`"), "{err}");
    }
}
