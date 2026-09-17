use std::collections::HashMap;

pub fn parse_docs(c_code: &str) -> Result<CHeaderDocParseOutput, Error> {
    crate::vibe_zone::doc_parsing::parse_docs(c_code)
}

pub use crate::vibe_zone::doc_parsing::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CHeaderDocParseOutput {
    pub misc_docs: Vec<MiscDoc>,
    pub file_doc: Option<String>,
    /// name -> entry
    pub entries: HashMap<String, DocEntry>,
    /// group name -> group
    pub group_docs: HashMap<String, DocGroup>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MiscDoc {
    MainPage(String),
    Page(String),
    /// e.g., `@propset …`
    OfxPropSet(String),
    /// e.g., `@propsetdef …`
    OfxPropSetDef(String),
    Unclassified {
        kind: String,
        content: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocEntry {
    pub group: Option<String>,
    pub name: String,
    pub content: DocContent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocContent {
    /// e.g., `#define …`
    Define(String),
    /// e.g., `typedef …`
    SimpleType(String),
    /// e.g., `typedef struct …`
    StructType {
        self_doc: String,
        field_docs: HashMap<String, String>,
    },
    /// e.g., `OfxExport …`
    EnumType {
        self_doc: String,
        variant_docs: HashMap<String, String>,
    },
    Fn(String),
    /// e.g., `typedef enum …`
    Unclassified {
        /// in case it is a nested entity.
        parent: Option<String>,
        kind: String,
        content: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocGroup {
    pub parent: Option<String>,
    /// group name
    pub name: String,
    pub content: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_defines() -> Result<(), Error> {
        assert_eq!(
            parse_docs(
                "
/** @brief Description of the plug-in to a user.

This is a string giving a potentially verbose description of the effect.
    
    - Valid Values - UTF8 string
    @propdef
    type: string
    dimension: 1
*/
#define kOfxPropPluginDescription \"OfxPropPluginDescription\"

/** @brief User visible name of an object.

The label is what a user sees on any interface in place of the object's name.

Note that resetting this will also reset ::kOfxPropShortLabel and ::kOfxPropLongLabel.
    
    @propdef
    type: string
    dimension: 1
*/
#define kOfxPropLabel \"OfxPropLabel\"
            "
                .trim()
            )?,
            CHeaderDocParseOutput {
                misc_docs: vec![],
                file_doc: None,
                entries: HashMap::from([
                    (
                        "kOfxPropPluginDescription".to_owned(),
                        DocEntry {
                            group: None,
                            name: "kOfxPropPluginDescription".to_owned(),
                            content: DocContent::Define(
                                [
                                    "@brief Description of the plug-in to a user.",
                                    "",
                                    "This is a string giving a potentially verbose description of \
                                     the effect.",
                                    "",
                                    "    - Valid Values - UTF8 string",
                                    "    @propdef",
                                    "    type: string",
                                    "    dimension: 1"
                                ]
                                .join("\n")
                            )
                        }
                    ),
                    (
                        "kOfxPropLabel".to_owned(),
                        DocEntry {
                            group: None,
                            name: "kOfxPropLabel".to_owned(),
                            content: DocContent::Define(
                                [
                                    "@brief User visible name of an object.",
                                    "",
                                    "The label is what a user sees on any interface in place of \
                                     the object's name.",
                                    "",
                                    "Note that resetting this will also reset \
                                     ::kOfxPropShortLabel and ::kOfxPropLongLabel.",
                                    "",
                                    "    @propdef",
                                    "    type: string",
                                    "    dimension: 1"
                                ]
                                .join("\n")
                            )
                        }
                    ),
                ]),
                group_docs: Default::default(),
            }
        );

        Ok(())
    }

    #[test]
    fn test_parse_define_conditional() -> Result<(), Error> {
        assert_eq!(
            parse_docs(
                "
/** @brief Platform independent export macro.
 *
 * This macro is to be used before any symbol that is to be
 * exported from a plug-in. This is OS/compiler dependent.
 */
#if defined(_WIN32)
	#define OfxExport extern __declspec(dllexport)
#else
	#define OfxExport extern
#endif
            "
                .trim()
            )?,
            CHeaderDocParseOutput {
                misc_docs: vec![],
                file_doc: None,
                entries: HashMap::from([(
                    "OfxExport".to_owned(),
                    DocEntry {
                        group: None,
                        name: "OfxExport".to_owned(),
                        content: DocContent::Define(
                            [
                                "@brief Platform independent export macro.",
                                "",
                                "This macro is to be used before any symbol that is to be",
                                "exported from a plug-in. This is OS/compiler dependent.",
                            ]
                            .join("\n")
                        )
                    }
                ),]),
                group_docs: Default::default(),
            }
        );

        Ok(())
    }

    #[test]
    fn test_parse_defgroup() -> Result<(), Error> {
        assert_eq!(
            parse_docs(
                "
/**
   \\defgroup PropertiesAll Ofx Properties

These strings are used to identify properties within OFX, they are broken up by the host suite or \
                 API they relate to.
*/
/*@{*/

/**
   \\defgroup PropertiesGeneral General Properties

These properties are general properties and  apply to may objects across OFX
*/
/*@{*/

/** @brief Property on the host descriptor, saying what API version of the API is being implemented

This is a version string that will specify which version of the API is being implemented by a \
                 host. It
can have multiple values. For example \"1.0\", \"1.2.4\" etc.....

If this is not present, it is safe to assume that the version of the API is \"1.0\".
    
    @propdef
    type: int
    dimension: N
*/
#define kOfxPropAPIVersion \"OfxPropAPIVersion\"

/** @brief General property used to get/set the time of something.
    
    @propdef
    type: double
    dimension: 1
*/
#define kOfxPropTime \"OfxPropTime\"

/*@}*/

/*@}*/
            "
                .trim()
            )?,
            CHeaderDocParseOutput {
                misc_docs: vec![],
                file_doc: None,
                entries: HashMap::from([
                    (
                        "kOfxPropAPIVersion".to_owned(),
                        DocEntry {
                            group: Some("PropertiesGeneral".to_owned()),
                            name: "kOfxPropAPIVersion".to_owned(),
                            content: DocContent::Define(
                                [
                                    "@brief Property on the host descriptor, saying what API \
                                     version of the API is being implemented",
                                    "",
                                    "This is a version string that will specify which version of \
                                     the API is being implemented by a host. It",
                                    "can have multiple values. For example \"1.0\", \"1.2.4\" \
                                     etc.....",
                                    "",
                                    "If this is not present, it is safe to assume that the \
                                     version of the API is \"1.0\".",
                                    "",
                                    "    @propdef",
                                    "    type: int",
                                    "    dimension: N",
                                ]
                                .join("\n")
                            )
                        }
                    ),
                    (
                        "kOfxPropTime".to_owned(),
                        DocEntry {
                            group: Some("PropertiesGeneral".to_owned()),
                            name: "kOfxPropTime".to_owned(),
                            content: DocContent::Define(
                                [
                                    "@brief General property used to get/set the time of \
                                     something.",
                                    "",
                                    "    @propdef",
                                    "    type: double",
                                    "    dimension: 1",
                                ]
                                .join("\n")
                            )
                        }
                    ),
                ]),
                group_docs: HashMap::from([
                    (
                        "PropertiesAll".to_owned(),
                        DocGroup {
                            parent: None,
                            name: "PropertiesAll".to_owned(),
                            content: [
                                "   \\defgroup PropertiesAll Ofx Properties",
                                "These strings are used to identify properties within OFX, they \
                                 are broken up by the host suite or API they relate to."
                            ]
                            .join("\n")
                        }
                    ),
                    (
                        "PropertiesGeneral".to_owned(),
                        DocGroup {
                            parent: Some("PropertiesAll".to_owned()),
                            name: "PropertiesGeneral".to_owned(),
                            content: [
                                "   \\defgroup PropertiesGeneral General Properties",
                                "",
                                "These properties are general properties and  apply to may \
                                 objects across OFX"
                            ]
                            .join("\n")
                        }
                    )
                ]),
            }
        );

        Ok(())
    }

    #[test]
    fn test_parse_addtogroup() -> Result<(), Error> {
        assert_eq!(
            parse_docs(
                "
/**
   \\addtogroup PropertiesGeneral
*/
/*@{*/

/** @brief Property used to indicate which a key on the keyboard or a button on a button device \
                 has been pressed

This property represents a raw key press, it does not represent the 'character value' of the key. 

This property is associated with a ::kOfxPropKeyString property, which encodes the UTF8
value for the keypress/button press. Some keys (for example arrow keys) have no UTF8 equivalent.

Some keys, especially on non-english language systems, may have a UTF8 value, but \\em not a \
                 keysym values, in these
cases, the keysym will have a value of kOfxKey_Unknown, but the ::kOfxPropKeyString property will \
                 still be set with
the UTF8 value.
  
  - Valid Values - one of any specified by #defines in the file ofxKeySyms.h.
  @propdef
  type: int
  dimension: 1
  cname: kOfxPropKeySym
 */
#define kOfxPropKeySym \"kOfxPropKeySym\"

/*@}*/
            "
                .trim()
            )?,
            CHeaderDocParseOutput {
                misc_docs: vec![],
                file_doc: None,
                entries: HashMap::from([(
                    "kOfxPropKeySym".to_owned(),
                    DocEntry {
                        group: Some("PropertiesGeneral".to_owned()),
                        name: "kOfxPropKeySym".to_owned(),
                        content: DocContent::Define(
                            [
                                "@brief Property used to indicate which a key on the keyboard or \
                                 a button on a button device has been pressed",
                                "",
                                "This property represents a raw key press, it does not represent \
                                 the 'character value' of the key.",
                                "",
                                "This property is associated with a ::kOfxPropKeyString property, \
                                 which encodes the UTF8",
                                "value for the keypress/button press. Some keys (for example \
                                 arrow keys) have no UTF8 equivalent.",
                                "",
                                "Some keys, especially on non-english language systems, may have \
                                 a UTF8 value, but \\em not a keysym values, in these",
                                "cases, the keysym will have a value of kOfxKey_Unknown, but the \
                                 ::kOfxPropKeyString property will still be set with",
                                "the UTF8 value.",
                                "",
                                "  - Valid Values - one of any specified by #defines in the file \
                                 ofxKeySyms.h.",
                                "  @propdef",
                                "  type: int",
                                "  dimension: 1",
                                "  cname: kOfxPropKeySym",
                            ]
                            .join("\n")
                        )
                    }
                ),]),
                group_docs: Default::default(),
            }
        );

        Ok(())
    }

    #[test]
    fn test_parse_defgroup_in_addtogroup() -> Result<(), Error> {
        assert_eq!(
            parse_docs(
                "
/**
   \\addtogroup ActionsAll
*/
/*@{*/
/**
   \\defgroup ImageEffectActions Image Effect Actions

These are the list of actions passed to an image effect plugin's main function. For more details \
                 on how to deal with actions, see \ref ImageEffectActions.
*/
/*@{*/

/*@}*/

/*@}*/
            "
                .trim()
            )?,
            CHeaderDocParseOutput {
                misc_docs: vec![],
                file_doc: None,
                entries: Default::default(),
                group_docs: HashMap::from([(
                    "ImageEffectActions".to_owned(),
                    DocGroup {
                        parent: Some("ActionsAll".to_owned()),
                        name: "ImageEffectActions".to_owned(),
                        content: [
                            "   \\defgroup ImageEffectActions Image Effect Actions",
                            "",
                            "These are the list of actions passed to an image effect plugin's \
                             main function. For more details on how to deal with actions, see \
                             \ref ImageEffectActions."
                        ]
                        .join("\n")
                    }
                )])
            }
        );

        Ok(())
    }

    #[test]
    fn test_parse_typedef_primitive_types() -> Result<(), Error> {
        assert_eq!(
            parse_docs(
                "
/** @brief How time is specified within the OFX API */
typedef double OfxTime;
            "
                .trim()
            )?,
            CHeaderDocParseOutput {
                misc_docs: vec![],
                file_doc: None,
                entries: HashMap::from([(
                    "OfxTime".to_owned(),
                    DocEntry {
                        group: None,
                        name: "OfxTime".to_owned(),
                        content: DocContent::SimpleType(
                            ["@brief How time is specified within the OFX API",].join("\n")
                        ),
                    }
                )]),
                group_docs: Default::default(),
            }
        );

        Ok(())
    }

    #[test]
    fn test_parse_typedef_opaque_pointer_types() -> Result<(), Error> {
        assert_eq!(
            parse_docs(
                "
/** @brief Blind declaration of an OFX image effect
*/
typedef struct OfxImageEffectStruct *OfxImageEffectHandle;
            "
                .trim()
            )?,
            CHeaderDocParseOutput {
                misc_docs: vec![],
                file_doc: None,
                entries: HashMap::from([(
                    "OfxImageEffectHandle".to_owned(),
                    DocEntry {
                        group: None,
                        name: "OfxImageEffectHandle".to_owned(),
                        content: DocContent::SimpleType(
                            ["@brief Blind declaration of an OFX image effect",].join("\n")
                        ),
                    }
                )]),
                group_docs: Default::default(),
            }
        );

        Ok(())
    }

    #[test]
    fn test_parse_typedef_struct_types_simple() -> Result<(), Error> {
        assert_eq!(
            parse_docs(
                "
/** @brief Defines one dimensional integer bounds */
typedef struct OfxRangeI {
  int min, max;
} OfxRangeI;

/** @brief Defines one dimensional double bounds */
typedef struct OfxRangeD {
  double min, max;
} OfxRangeD;
            "
                .trim()
            )?,
            CHeaderDocParseOutput {
                misc_docs: vec![],
                file_doc: None,
                entries: HashMap::from([
                    (
                        "OfxRangeI".to_owned(),
                        DocEntry {
                            group: None,
                            name: "OfxRangeI".to_owned(),
                            content: DocContent::StructType {
                                self_doc: ["@brief Defines one dimensional integer bounds",]
                                    .join("\n"),
                                field_docs: Default::default(),
                            },
                        }
                    ),
                    (
                        "OfxRangeD".to_owned(),
                        DocEntry {
                            group: None,
                            name: "OfxRangeD".to_owned(),
                            content: DocContent::StructType {
                                self_doc: ["@brief Defines one dimensional double bounds",]
                                    .join("\n"),
                                field_docs: Default::default(),
                            },
                        }
                    )
                ]),
                group_docs: Default::default(),
            }
        );

        Ok(())
    }

    #[test]
    fn test_parse_typedef_struct_types() -> Result<(), Error> {
        assert_eq!(
            parse_docs(
                "
/** @brief OFX suite that allows an effect to interact with an openGL window so as to provide \
                 custom interfaces.

*/
typedef struct OfxInteractSuiteV1 {	
  /** @brief Requests an openGL buffer swap on the interact instance */
  OfxStatus (*interactSwapBuffers)(OfxInteractHandle interactInstance);

  /** @brief Requests a redraw of the interact instance */
  OfxStatus (*interactRedraw)(OfxInteractHandle interactInstance);

  /** @brief Gets the property set handle for this interact handle */
  OfxStatus (*interactGetPropertySet)(OfxInteractHandle interactInstance,
				      OfxPropertySetHandle *property);
} OfxInteractSuiteV1;
            "
                .trim()
            )?,
            CHeaderDocParseOutput {
                misc_docs: vec![],
                file_doc: None,
                entries: HashMap::from([(
                    "OfxInteractSuiteV1".to_owned(),
                    DocEntry {
                        group: None,
                        name: "OfxInteractSuiteV1".to_owned(),
                        content: DocContent::StructType {
                            self_doc: ["@brief OFX suite that allows an effect to interact with \
                                        an openGL window so as to provide custom interfaces.",]
                            .join("\n"),
                            field_docs: HashMap::from([
                                (
                                    "interactSwapBuffers".to_owned(),
                                    "@brief Requests an openGL buffer swap on the interact \
                                     instance"
                                        .to_owned(),
                                ),
                                (
                                    "interactRedraw".to_owned(),
                                    "@brief Requests a redraw of the interact instance".to_owned(),
                                ),
                                (
                                    "interactGetPropertySet".to_owned(),
                                    "@brief Gets the property set handle for this interact handle"
                                        .to_owned(),
                                ),
                            ]),
                        }
                    }
                ),]),
                group_docs: Default::default(),
            }
        );

        Ok(())
    }

    #[test]
    fn test_parse_typedef_enum_types() -> Result<(), Error> {
        assert_eq!(
            parse_docs(
                "
/** @brief Defines valid values for OfxDrawSuiteV1::setLineStipple */
typedef enum OfxDrawLineStipplePattern
{
	kOfxDrawLineStipplePatternSolid,	// -----
	kOfxDrawLineStipplePatternDot,		// .....
	kOfxDrawLineStipplePatternDash,		// - - -
	kOfxDrawLineStipplePatternAltDash,	//  - - -
	kOfxDrawLineStipplePatternDotDash	// .-.-.-
} OfxDrawLineStipplePattern;

/** @brief Defines valid values for OfxDrawSuiteV1::draw */

typedef enum OfxDrawPrimitive
{
	kOfxDrawPrimitiveLines,
	kOfxDrawPrimitiveLineStrip,
	kOfxDrawPrimitiveLineLoop,
	kOfxDrawPrimitiveRectangle,
	kOfxDrawPrimitivePolygon,
	kOfxDrawPrimitiveEllipse
} OfxDrawPrimitive;
            "
                .trim()
            )?,
            CHeaderDocParseOutput {
                misc_docs: vec![],
                file_doc: None,
                entries: HashMap::from([
                    (
                        "OfxDrawLineStipplePattern".to_owned(),
                        DocEntry {
                            group: None,
                            name: "OfxDrawLineStipplePattern".to_owned(),
                            content: DocContent::EnumType {
                                self_doc: ["@brief Defines valid values for \
                                            OfxDrawSuiteV1::setLineStipple"]
                                .join("\n"),
                                variant_docs: HashMap::from([
                                    (
                                        "kOfxDrawLineStipplePatternSolid".to_owned(),
                                        "-----".to_owned(),
                                    ),
                                    (
                                        "kOfxDrawLineStipplePatternDot".to_owned(),
                                        ".....".to_owned(),
                                    ),
                                    (
                                        "kOfxDrawLineStipplePatternDash".to_owned(),
                                        "- - -".to_owned(),
                                    ),
                                    (
                                        "kOfxDrawLineStipplePatternAltDash".to_owned(),
                                        " - - -".to_owned(),
                                    ),
                                    (
                                        "kOfxDrawLineStipplePatternDotDash".to_owned(),
                                        ".-.-.-".to_owned(),
                                    ),
                                ]),
                            }
                        }
                    ),
                    (
                        "OfxDrawPrimitive".to_owned(),
                        DocEntry {
                            group: None,
                            name: "OfxDrawPrimitive".to_owned(),
                            content: DocContent::EnumType {
                                self_doc: ["@brief Defines valid values for OfxDrawSuiteV1::draw"]
                                    .join("\n"),
                                variant_docs: Default::default(),
                            }
                        }
                    ),
                ]),
                group_docs: Default::default(),
            }
        );

        Ok(())
    }

    #[test]
    fn test_parse_fns() -> Result<(), Error> {
        assert_eq!(
            parse_docs(
                "
/** @brief Returns the 'nth' plug-in implemented inside a binary
 *
 * Returns a pointer to the 'nth' plug-in implemented in the binary. A function of this type
 * must be implemented in and exported from each plug-in binary.
 */

OfxExport OfxPlugin *OfxGetPlugin(int nth);

/** @brief Defines the number of plug-ins implemented inside a binary
 *
 * A host calls this to determine how many plug-ins there are inside
 * a binary it has loaded. A function of this type
 * must be implemented in and exported from each plug-in binary.
 */
OfxExport int OfxGetNumberOfPlugins(void);
            "
                .trim()
            )?,
            CHeaderDocParseOutput {
                misc_docs: vec![],
                file_doc: None,
                entries: HashMap::from([
                    (
                        "OfxGetPlugin".to_owned(),
                        DocEntry {
                            group: None,
                            name: "OfxGetPlugin".to_owned(),
                            content: DocContent::Fn(
                                [
                                    "@brief Returns the 'nth' plug-in implemented inside a binary",
                                    "",
                                    "Returns a pointer to the 'nth' plug-in implemented in the \
                                     binary. A function of this type",
                                    "must be implemented in and exported from each plug-in binary.",
                                ]
                                .join("\n")
                                .to_owned()
                            )
                        }
                    ),
                    (
                        "OfxGetNumberOfPlugins".to_owned(),
                        DocEntry {
                            group: None,
                            name: "OfxGetNumberOfPlugins".to_owned(),
                            content: DocContent::Fn(
                                [
                                    "@brief Defines the number of plug-ins implemented inside a \
                                     binary",
                                    "",
                                    "A host calls this to determine how many plug-ins there are \
                                     inside",
                                    "a binary it has loaded. A function of this type",
                                    "must be implemented in and exported from each plug-in binary.",
                                ]
                                .join("\n")
                                .to_owned()
                            )
                        }
                    )
                ]),
                group_docs: Default::default(),
            }
        );

        Ok(())
    }

    #[test]
    fn test_parse_misc_main_page() -> Result<(), Error> {
        assert_eq!(
            parse_docs(
                "
/** @mainpage OFX : Open Plug-Ins For Special Effects

This page represents the automatically \
                 extracted HTML documentation of the source headers for the OFX Image Effect API.
The \
                 documentation was extracted by doxygen (http://www.doxygen.org).
A more complete reference \
                 manual is https://openfx.readthedocs.io .

*/
            "
                .trim()
            )?,
            CHeaderDocParseOutput {
                misc_docs: vec![MiscDoc::MainPage(
                    [
                        "@mainpage OFX : Open Plug-Ins For Special Effects",
                        "",
                        "This page represents the automatically extracted HTML documentation of \
                         the source headers for the OFX Image Effect API.",
                        "The documentation was extracted by doxygen (http://www.doxygen.org).",
                        "A more complete reference manual is https://openfx.readthedocs.io .",
                    ]
                    .join("\n")
                )],
                file_doc: None,
                entries: Default::default(),
                group_docs: Default::default(),
            }
        );

        Ok(())
    }

    #[test]
    fn test_parse_misc_page() -> Result<(), Error> {
        assert_eq!(
            parse_docs(
                "
/** @page ofxOpenGLRender OpenGL Acceleration of Rendering

@section ofxOpenGLRenderIntro Introduction

The OfxOpenGLRenderSuite allows image effects to use OpenGL commands
(hopefully backed by a GPU) to accelerate rendering
of their outputs. The basic scheme is simple....
  TLDR
*/
            "
                .trim()
            )?,
            CHeaderDocParseOutput {
                misc_docs: vec![MiscDoc::Page(
                    [
                        "@page ofxOpenGLRender OpenGL Acceleration of Rendering",
                        "",
                        "@section ofxOpenGLRenderIntro Introduction",
                        "",
                        "The OfxOpenGLRenderSuite allows image effects to use OpenGL commands",
                        "(hopefully backed by a GPU) to accelerate rendering",
                        "of their outputs. The basic scheme is simple....",
                        "  TLDR",
                    ]
                    .join("\n")
                )],
                file_doc: None,
                entries: Default::default(),
                group_docs: Default::default(),
            }
        );

        Ok(())
    }

    #[test]
    fn test_parse_misc_ofx_stuff() -> Result<(), Error> {
        assert_eq!(
            parse_docs(
                "
/** @propset ParameterSet
    write: plugin
    props:
      - OfxPropParamSetNeedsSyncing
      - OfxPluginPropParamPageOrder
*/

/** @propsetdef ParamsNumeric
    - OfxParamPropMin
    - OfxParamPropMax
    - OfxParamPropDisplayMin
    - OfxParamPropDisplayMax
*/

/** @propsetdef ParamsDouble
    - OfxParamPropIncrement
    - OfxParamPropDigits
*/

/** @propset ParamsGroup
    write: plugin
    props:
      - OfxParamPropGroupOpen
      - ParamsCommon_REF
*/
            "
                .trim()
            )?,
            CHeaderDocParseOutput {
                misc_docs: vec![
                    MiscDoc::OfxPropSet(
                        [
                            "@propset ParameterSet",
                            "    write: plugin",
                            "    props:",
                            "      - OfxPropParamSetNeedsSyncing",
                            "      - OfxPluginPropParamPageOrder",
                            "",
                        ]
                        .join("\n")
                    ),
                    MiscDoc::OfxPropSetDef(
                        [
                            "@propsetdef ParamsNumeric",
                            "    - OfxParamPropMin",
                            "    - OfxParamPropMax",
                            "    - OfxParamPropDisplayMin",
                            "    - OfxParamPropDisplayMax",
                        ]
                        .join("\n")
                    ),
                    MiscDoc::OfxPropSetDef(
                        [
                            "@propsetdef ParamsDouble",
                            "    - OfxParamPropIncrement",
                            "    - OfxParamPropDigits",
                        ]
                        .join("\n")
                    ),
                    MiscDoc::OfxPropSet(
                        [
                            "@propset ParamsGroup",
                            "    write: plugin",
                            "    props:",
                            "      - OfxParamPropGroupOpen",
                            "      - ParamsCommon_REF",
                        ]
                        .join("\n")
                    ),
                ],
                file_doc: None,
                entries: Default::default(),
                group_docs: Default::default(),
            }
        );

        Ok(())
    }

    #[test]
    fn test_parse_file_doc() -> Result<(), Error> {
        assert_eq!(
            parse_docs(
                "
/** @file ofxParam.h
 
  This header contains the suite definition to manipulate host side parameters.

  For more details go see @ref ParametersPage
*/
            "
                .trim()
            )?,
            CHeaderDocParseOutput {
                misc_docs: vec![],
                file_doc: Some(
                    [
                        "@file ofxParam.h",
                        "",
                        "  This header contains the suite definition to manipulate host side \
                         parameters.",
                        "",
                        "  For more details go see @ref ParametersPage",
                    ]
                    .join("\n")
                ),
                entries: Default::default(),
                group_docs: Default::default(),
            }
        );

        Ok(())
    }

    macro_rules! make_test_real_file {
        ($fn_name: ident, $name:ident) => {
            #[test]
            #[expect(non_snake_case)]
            fn $fn_name() -> Result<(), Error> {
                let code = include_str!(concat!(
                    "../../../openfx/vendor/openfx/include/",
                    stringify!($name),
                    ".h"
                ));

                parse_docs(code)?;
                Ok(())
            }
        };
    }

    make_test_real_file!(test_real_file_ofxColour, ofxColour);
    make_test_real_file!(test_real_file_ofxCore, ofxCore);
    make_test_real_file!(test_real_file_ofxDialog, ofxDialog);
    make_test_real_file!(test_real_file_ofxDrawSuite, ofxDrawSuite);
    make_test_real_file!(test_real_file_ofxGPURender, ofxGPURender);
    make_test_real_file!(test_real_file_ofxImageEffect, ofxImageEffect);
    make_test_real_file!(test_real_file_ofxInteract, ofxInteract);
    make_test_real_file!(test_real_file_ofxKeySyms, ofxKeySyms);
    make_test_real_file!(test_real_file_ofxMemory, ofxMemory);
    make_test_real_file!(test_real_file_ofxMessage, ofxMessage);
    make_test_real_file!(test_real_file_ofxMultiThread, ofxMultiThread);
    make_test_real_file!(test_real_file_ofxOld, ofxOld);
    make_test_real_file!(test_real_file_ofxOpenGLRender, ofxOpenGLRender);
    make_test_real_file!(test_real_file_ofxParam, ofxParam);
    make_test_real_file!(test_real_file_ofxParametricParam, ofxParametricParam);
    make_test_real_file!(test_real_file_ofxPixels, ofxPixels);
    make_test_real_file!(test_real_file_ofxProgress, ofxProgress);
    make_test_real_file!(test_real_file_ofxProperty, ofxProperty);
    make_test_real_file!(test_real_file_ofxTimeLine, ofxTimeLine);
}
