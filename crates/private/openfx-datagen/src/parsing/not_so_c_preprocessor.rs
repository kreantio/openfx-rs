mod messy_chunks;

use std::collections::HashMap;

pub use messy_chunks::DefineValue;

use crate::parsing::not_so_c_preprocessor::messy_chunks::MessyChunkParseStream;

pub enum Chunk<'a> {
    /// Its content is the text inside the include directive. (`…` in
    /// `#include "…"` or `#include <…>`).
    Include(&'a str),
    Define {
        /// The name of the macro.
        name: &'a str,
        value: DefineValue<'a>,
        /// `/* … */` at the same line.
        comment: Option<&'a str>,
    },

    /// Consecutive single-line comments without empty lines.
    ///
    /// Both `//` and line breaks are included in the content.
    SingleLineComments(&'a str),
    /// A special form of [`Self::SingleLineComments`]. It looks like:
    /// ```c
    /// // Copyright OpenFX and contributors to the OpenFX project.
    /// // SPDX-License-Identifier: BSD-3-Clause
    /// ```
    CopyrightStatement(&'a str),
    /// Its content is the text between the `/*` or `/**` and `*/` in a multi-
    /// line comment.
    MultiLineCommentContent(&'a str),

    CCodeStruct {
        code: &'a str,
        field_multiple_line_comment_contents: HashMap<&'a str, &'a str>,
        fn_field_parameter_same_line_comment_contents: HashMap<&'a str, &'a str>,
    },
    CCodeEnum {
        code: &'a str,
        variant_same_line_comment_contents: HashMap<&'a str, &'a str>,
    },
    CCodeOther(&'a str),
}

struct ChunkParseStream<'a> {
    messy_chunk_parse_stream: MessyChunkParseStream<'a>,
}

impl<'a> ChunkParseStream<'a> {
    pub fn new(code: &'a str) -> Self {
        Self {
            messy_chunk_parse_stream: MessyChunkParseStream::new(code),
        }
    }
}
