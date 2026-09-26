use crate::parsing::utils::LinesEx;

/// Preprocess the input C code to make tree-sitter happy. Note that this is
/// not a C preprocessor.
///
/// 1. It removes `#ifdef __cplusplus` blocks from the input C code.
///    This is necessary because tree-sitter cannot correctly parse code (at
///    least in the playground) such as (`|` = line feed):
///
///    - `#ifdef __cplusplus` | `extern "C" {` | `#endif`, and
///    - `#ifdef __cplusplus` | `}` | `#endif`
/// 2. It removes `OfxExport`s that appear at the beginning of lines (for
///    function declarations of `OfxGetPlugin`, `OfxGetNumberOfPlugins` and
///    `OfxSetHost`), otherwise tree-sitter would be confused. To simplify
///    the implementation, we currently just find `OfxExport` at the beginning
///    of lines.
/// 3. It removes `#ifndef $name` | `#define $name` and the corresponding
///    `#endif`, so that the parse result can be flat.
pub fn preprocess_for_tree_sitter(code: &str) -> String {
    let mut chunks: Vec<&str> = Vec::new();

    let mut lines = LinesEx::new(code).peekable();

    let mut current_chunk: Option<std::ops::Range<usize>> = None;
    let mut has_ifndef_define = false;

    macro_rules! push_current_chunk {
        () => {
            if let Some(chunk) = current_chunk.take()
                && !chunk.is_empty()
            {
                chunks.push(&code[chunk]);
            }
        };
    }

    while let Some(line) = lines.next() {
        let line_content_start_trimmed = line.content.trim_start();

        if line_content_start_trimmed.starts_with("#ifdef __cplusplus") {
            push_current_chunk!();
            loop {
                let next_line = lines
                    .next()
                    .expect("There should be lines before `#endif`.");
                if next_line.content.trim_start().starts_with("#endif") {
                    break;
                }
            }
        } else if line_content_start_trimmed.starts_with("OfxExport") {
            push_current_chunk!();
            current_chunk =
                Some(line.start_offset + "OfxExport".len()..line.start_offset + line.content.len());
        } else if let Some(name) = line_content_start_trimmed.strip_prefix("#ifndef")
            && let Some(name_2) = lines
                .peek()
                .and_then(|l| l.content.trim_start().strip_prefix("#define"))
            && name.trim() == name_2.trim()
        {
            push_current_chunk!();
            has_ifndef_define = true;
            lines.next().expect("Peeked line should exist.");
        } else {
            if let Some(chunk) = current_chunk.as_mut() {
                chunk.end = line.start_offset + line.content.len();
            } else {
                current_chunk = Some(line.start_offset..line.start_offset + line.content.len());
            }
        }
    }
    push_current_chunk!();

    let result = chunks.join("\n");

    if has_ifndef_define {
        // inefficient but who cares
        let mut lines: Vec<&str> = result.lines().collect();
        let i = lines
            .iter()
            .rposition(|l| l.trim_start().starts_with("#endif"))
            .expect("`#endif` corresponding to `#ifndef` should exist.");
        lines.remove(i);
        lines.join("\n")
    } else {
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preprocess_for_tree_sitter() {
        let cases = [(
            r#"#ifndef _ofxCore_h_
#define _ofxCore_h_

// Copyright OpenFX and contributors to the OpenFX project.
// SPDX-License-Identifier: BSD-3-Clause


#include "stddef.h" // for size_t
#include <limits.h> // for INT_MIN & INT_MAX

#ifdef __cplusplus
extern "C" {
#endif

/** @file ofxCore.h
Contains the core OFX architectural struct and function definitions. For more details on the basic OFX architecture, see \ref Architecture.
*/


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

/** @brief Blind data structure to manipulate sets of properties through */
typedef struct OfxPropertySetStruct *OfxPropertySetHandle;

/** @brief OFX status return type */
typedef int OfxStatus;

/** @brief Generic host structure passed to OfxPlugin::setHost function

    This structure contains what is needed by a plug-in to bootstrap its connection
    to the host.
*/
typedef struct OfxHost {
  /** @brief Global handle to the host. Extract relevant host properties from this.
      This pointer will be valid while the binary containing the plug-in is loaded.
   */
  OfxPropertySetHandle host;

  /** @brief The function which the plug-in uses to fetch suites from the host.

      \arg \c host          the host the suite is being fetched from this \em must be the \e host member of the OfxHost struct containing fetchSuite.
      \arg \c suiteName     ASCII string labelling the host supplied API
      \arg \c suiteVersion  version of that suite to fetch

      Any API fetched will be valid while the binary containing the plug-in is loaded.

      Repeated calls to fetchSuite with the same parameters will return the same pointer.

      It is recommended that hosts should return the same host and suite pointers to all plugins
      in the same shared lib or bundle.

      returns
         - NULL if the API is unknown (either the api or the version requested),
	 - pointer to the relevant API if it was found
  */
  const void *(*fetchSuite)(OfxPropertySetHandle host, const char *suiteName, int suiteVersion);
} OfxHost;
 
/** @brief String used to label signed 32 bit floating point samples */
#define kOfxBitDepthFloat "OfxBitDepthFloat"

/**
   \defgroup StatusCodes Status Codes

These strings are used to identify error states within ofx, they are returned
by various host suite functions, as well as plug-in functions. The valid return codes
for each function are documented with that function.
*/
/*@{*/

/**
   \defgroup StatusCodesGeneral General Status Codes

General status codes start at 1 and continue until 999

*/
/*@{*/

/** @brief Status code indicating all was fine */
#define kOfxStatOK 0

/** @brief Status error code for a failed operation. */
#define kOfxStatFailed  ((int)1)

/*@}*/

/*@}*/

#ifdef __cplusplus
}
#endif

/** @mainpage OFX : Open Plug-Ins For Special Effects

This page represents the automatically extracted HTML documentation of the source headers for the OFX Image Effect API.
The documentation was extracted by doxygen (http://www.doxygen.org).
A more complete reference manual is https://openfx.readthedocs.io .

*/

#endif"#,
            r#"
// Copyright OpenFX and contributors to the OpenFX project.
// SPDX-License-Identifier: BSD-3-Clause


#include "stddef.h" // for size_t
#include <limits.h> // for INT_MIN & INT_MAX


/** @file ofxCore.h
Contains the core OFX architectural struct and function definitions. For more details on the basic OFX architecture, see \ref Architecture.
*/


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

/** @brief Blind data structure to manipulate sets of properties through */
typedef struct OfxPropertySetStruct *OfxPropertySetHandle;

/** @brief OFX status return type */
typedef int OfxStatus;

/** @brief Generic host structure passed to OfxPlugin::setHost function

    This structure contains what is needed by a plug-in to bootstrap its connection
    to the host.
*/
typedef struct OfxHost {
  /** @brief Global handle to the host. Extract relevant host properties from this.
      This pointer will be valid while the binary containing the plug-in is loaded.
   */
  OfxPropertySetHandle host;

  /** @brief The function which the plug-in uses to fetch suites from the host.

      \arg \c host          the host the suite is being fetched from this \em must be the \e host member of the OfxHost struct containing fetchSuite.
      \arg \c suiteName     ASCII string labelling the host supplied API
      \arg \c suiteVersion  version of that suite to fetch

      Any API fetched will be valid while the binary containing the plug-in is loaded.

      Repeated calls to fetchSuite with the same parameters will return the same pointer.

      It is recommended that hosts should return the same host and suite pointers to all plugins
      in the same shared lib or bundle.

      returns
         - NULL if the API is unknown (either the api or the version requested),
	 - pointer to the relevant API if it was found
  */
  const void *(*fetchSuite)(OfxPropertySetHandle host, const char *suiteName, int suiteVersion);
} OfxHost;
 
/** @brief String used to label signed 32 bit floating point samples */
#define kOfxBitDepthFloat "OfxBitDepthFloat"

/**
   \defgroup StatusCodes Status Codes

These strings are used to identify error states within ofx, they are returned
by various host suite functions, as well as plug-in functions. The valid return codes
for each function are documented with that function.
*/
/*@{*/

/**
   \defgroup StatusCodesGeneral General Status Codes

General status codes start at 1 and continue until 999

*/
/*@{*/

/** @brief Status code indicating all was fine */
#define kOfxStatOK 0

/** @brief Status error code for a failed operation. */
#define kOfxStatFailed  ((int)1)

/*@}*/

/*@}*/


/** @mainpage OFX : Open Plug-Ins For Special Effects

This page represents the automatically extracted HTML documentation of the source headers for the OFX Image Effect API.
The documentation was extracted by doxygen (http://www.doxygen.org).
A more complete reference manual is https://openfx.readthedocs.io .

*/
"#,
        )];

        for (input, expected) in cases {
            pretty_assertions::assert_eq!(preprocess_for_tree_sitter(input), expected);
        }
    }
}
