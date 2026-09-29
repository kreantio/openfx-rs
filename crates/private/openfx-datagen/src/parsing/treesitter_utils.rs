use treesitter_types_c::{
    Declaration, DeclarationDeclarator, Declarator, Enumerator, FieldDeclaration, FieldDeclarator,
    FromNode, FunctionDeclaratorDeclarator, ParameterListChildren, ParenthesizedDeclaratorChildren,
    PointerDeclaratorDeclarator, Span, Spanned, TypeDeclarator, TypeDefinition, TypeSpecifier,
};

use crate::parsing::{
    RootItem, TypedefEnumCValueExpr, TypedefEnumVariant, TypedefFunctionParameter,
    TypedefFunctionParameterType, TypedefFunctionParameterTypeSimpleCName,
    TypedefFunctionParameterTypeSimpleNonCName, TypedefFunctionReturnType, TypedefPrimitiveCType,
    TypedefStructField, TypedefStructFieldType, TypedefStructFieldTypeSimpleCName,
    TypedefStructFieldTypeSimpleNonCName,
    utils::{find_line_before, is_identifier},
};

pub fn extract_name_from_declaration(declaration: &Declaration) -> Result<Span, ()> {
    if declaration.declarator.len() != 1 {
        return Err(());
    }
    let Some(declarator) = declaration.declarator.first() else {
        return Err(());
    };
    extract_name_from_declaration_declarator(declarator)
}

fn extract_name_from_declaration_declarator(
    declarator: &DeclarationDeclarator,
) -> Result<Span, ()> {
    match declarator {
        DeclarationDeclarator::FunctionDeclarator(function_declarator) => {
            match &function_declarator.declarator {
                treesitter_types_c::FunctionDeclaratorDeclarator::Declarator(declarator) => {
                    extract_name_from_declarator(declarator)
                }
                _ => Err(()),
            }
        }
        DeclarationDeclarator::Identifier(identifier) => Ok(identifier.span),
        DeclarationDeclarator::PointerDeclarator(pointer_declarator) => {
            match &pointer_declarator.declarator {
                treesitter_types_c::PointerDeclaratorDeclarator::Declarator(declarator) => {
                    extract_name_from_declarator(declarator)
                }
                _ => Err(()),
            }
        }
        _ => Err(()),
    }
}

fn extract_name_from_declarator(declarator: &Declarator) -> Result<Span, ()> {
    match declarator {
        Declarator::FunctionDeclarator(function_declarator) => {
            match &function_declarator.declarator {
                treesitter_types_c::FunctionDeclaratorDeclarator::Declarator(declarator) => {
                    extract_name_from_declarator(declarator)
                }
                _ => Err(()),
            }
        }
        Declarator::Identifier(identifier) => Ok(identifier.span),
        Declarator::PointerDeclarator(pointer_declarator) => match &pointer_declarator.declarator {
            treesitter_types_c::PointerDeclaratorDeclarator::Declarator(declarator) => {
                extract_name_from_declarator(declarator)
            }
            _ => Err(()),
        },
        _ => Err(()),
    }
}

pub fn parse_type_definition(
    raw_node: &tree_sitter::Node,
    code: &str,
    type_definition: &TypeDefinition,
) -> Result<RootItem, ()> {
    macro_rules! text_from_span {
        ($span:expr) => {
            code[$span.start_byte..$span.end_byte].to_owned()
        };
    }

    // e.g., `typedef <c_type> <name>`.
    if let TypeSpecifier::PrimitiveType(specifier) = &type_definition.r#type
        && type_definition.declarator.len() == 1
        && let Some(TypeDeclarator::TypeIdentifier(declarator)) =
            &type_definition.declarator.first()
    {
        return Ok(RootItem::TypedefPrimitive {
            name: text_from_span!(declarator.span).trim().to_owned(),
            c_type: TypedefPrimitiveCType::try_from(text_from_span!(specifier.span).trim())?,
        });
    }

    // e.g., `typedef struct <pointee_struct_name> *<name>`.
    if let TypeSpecifier::StructSpecifier(specifier) = &type_definition.r#type
        && let Some(specifier_name) = &specifier.name
        && type_definition.declarator.len() == 1
        && let Some(TypeDeclarator::PointerDeclarator(declarator)) =
            &type_definition.declarator.first()
        && let PointerDeclaratorDeclarator::TypeDeclarator(declarator) = &declarator.declarator
        && let TypeDeclarator::TypeIdentifier(declarator) = &**declarator
    {
        return Ok(RootItem::TypedefOpaquePointer {
            name: text_from_span!(declarator.span).trim().to_owned(),
            pointee_struct_name: text_from_span!(specifier_name.span).trim().to_owned(),
        });
    }

    // e.g., `typedef <return_type> (<name>)(<parameters>)`.
    if type_definition.declarator.len() == 1
        && let Some(TypeDeclarator::FunctionDeclarator(declarator)) =
            type_definition.declarator.first()
        && let parameters_node = &declarator.parameters
        && let FunctionDeclaratorDeclarator::Declarator(declarator) = &declarator.declarator
        && let Declarator::ParenthesizedDeclarator(declarator) = &**declarator
        && declarator.children.len() == 1
        && let Some(ParenthesizedDeclaratorChildren::TypeDeclarator(delearator)) =
            declarator.children.first()
        && let TypeDeclarator::TypeIdentifier(delearator) = &**delearator
    {
        let Some(raw_parameters_node) = raw_node.descendant_for_byte_range(
            parameters_node.span.start_byte,
            parameters_node.span.end_byte,
        ) else {
            return Err(());
        };
        assert!(raw_parameters_node.kind() == "parameter_list");

        let name = text_from_span!(delearator.span).trim().to_owned();

        let ret_ty = match &type_definition.r#type {
            TypeSpecifier::PrimitiveType(specifier)
                if text_from_span!(specifier.span).trim() == "void" =>
            {
                TypedefFunctionReturnType::Void
            }
            TypeSpecifier::TypeIdentifier(specifier)
                if text_from_span!(specifier.span).trim() == "OfxStatus" =>
            {
                TypedefFunctionReturnType::OfxStatus
            }
            _ => return Err(()),
        };

        let mut cursor = raw_parameters_node.walk();

        let mut parameters: Vec<TypedefFunctionParameter> = Vec::new();

        for raw_parameter_node in raw_parameters_node.children(&mut cursor) {
            if !raw_parameter_node.is_named() {
                continue;
            }
            if raw_parameter_node.kind() == "comment" {
                return Err(());
            }

            let Ok(_parameter_node) =
                ParameterListChildren::from_node(raw_parameter_node, code.as_bytes())
            else {
                return Err(());
            };
            let Ok(parameter_str) = raw_parameter_node.utf8_text(code.as_bytes()) else {
                return Err(());
            };
            let parameter_str = parameter_str.trim();

            // TODO: parse it properly.
            if let Some(maybe_ident) = parameter_str.strip_prefix("const char *") {
                let maybe_ident = maybe_ident.trim();
                if !is_identifier(maybe_ident) {
                    return Err(());
                }
                parameters.push(TypedefFunctionParameter {
                    name: maybe_ident.to_owned(),
                    r#type: TypedefFunctionParameterType::ConstCharPtr,
                });
            } else if let Some(maybe_ident) = parameter_str.strip_prefix("const void *") {
                let maybe_ident = maybe_ident.trim();
                if !is_identifier(maybe_ident) {
                    return Err(());
                }
                parameters.push(TypedefFunctionParameter {
                    name: maybe_ident.to_owned(),
                    r#type: TypedefFunctionParameterType::ConstVoidPtr,
                });
            } else if let Some(maybe_ident) = parameter_str.strip_prefix("void *") {
                let maybe_ident = maybe_ident.trim();
                if !is_identifier(maybe_ident) {
                    return Err(());
                }
                parameters.push(TypedefFunctionParameter {
                    name: maybe_ident.to_owned(),
                    r#type: TypedefFunctionParameterType::VoidPtr,
                });
            } else if let split = parameter_str.split_whitespace().collect::<Vec<_>>()
                && split.len() >= 2
            {
                let simple_ty = split[..split.len() - 1].join(" ");
                let maybe_ident = *split.last().unwrap();
                if !is_identifier(maybe_ident) {
                    return Err(());
                }

                let ty = if let Ok(name) =
                    TypedefFunctionParameterTypeSimpleCName::try_from(&simple_ty)
                {
                    TypedefFunctionParameterType::SimpleC { name }
                } else if let Ok(name) =
                    TypedefFunctionParameterTypeSimpleNonCName::try_from(&simple_ty)
                {
                    TypedefFunctionParameterType::SimpleNonC { name }
                } else {
                    return Err(());
                };

                parameters.push(TypedefFunctionParameter {
                    name: maybe_ident.to_owned(),
                    r#type: ty,
                });
            } else {
                return Err(());
            }
        }

        return Ok(RootItem::TypedefFunction {
            name,
            return_type: ret_ty,
            parameters,
        });
    }

    // e.g., `typedef struct <name> { … }`
    if let TypeSpecifier::StructSpecifier(specifier) = &type_definition.r#type
        && let Some(specifier_name) = &specifier.name
        && type_definition.declarator.len() == 1
        && let Some(TypeDeclarator::TypeIdentifier(declarator)) =
            &type_definition.declarator.first()
    {
        let name = text_from_span!(specifier_name.span).trim().to_owned();
        if name != text_from_span!(declarator.span).trim() {
            return Err(());
        }

        let Some(body) = &specifier.body else {
            return Err(());
        };
        let Some(raw_body) =
            raw_node.descendant_for_byte_range(body.span.start_byte, body.span.end_byte)
        else {
            return Err(());
        };
        assert_eq!(raw_body.kind(), "field_declaration_list");

        let mut cursor = raw_body.walk();

        let mut items: Vec<TypedefStructField> = Vec::new();
        let mut last_comment: Option<String> = None;

        for raw_item_node in raw_body.children(&mut cursor) {
            if !raw_item_node.is_named() {
                continue;
            }

            if raw_item_node.kind() == "comment" {
                // We assume that comments appear on lines by themselves, before
                // the field declaration.
                if !find_line_before(code, raw_item_node.start_byte())
                    .trim()
                    .is_empty()
                {
                    return Err(());
                }
                if last_comment.is_some() {
                    return Err(());
                }

                last_comment = Some(
                    raw_item_node
                        .utf8_text(code.as_bytes())
                        .unwrap()
                        .trim()
                        .to_owned(),
                );

                continue;
            }

            let comment_above = last_comment.take();

            let Ok(item_node) = FieldDeclaration::from_node(raw_item_node, code.as_bytes()) else {
                return Err(());
            };

            let identifier_declarators: Vec<_> = item_node
                .declarator
                .iter()
                .filter_map(|d| {
                    if let FieldDeclarator::FieldIdentifier(declarator) = d {
                        Some(declarator)
                    } else {
                        None
                    }
                })
                .collect();
            if identifier_declarators.len() == item_node.declarator.len() {
                let type_str = text_from_span!(item_node.r#type.span()).trim().to_owned();
                let ty = if let Ok(name) = TypedefStructFieldTypeSimpleCName::try_from(&type_str) {
                    TypedefStructFieldType::SimpleC { name: name.clone() }
                } else if let Ok(name) = TypedefStructFieldTypeSimpleNonCName::try_from(&type_str) {
                    TypedefStructFieldType::SimpleNonC { name: name.clone() }
                } else {
                    return Err(());
                };

                for declarator in identifier_declarators {
                    items.push(TypedefStructField {
                        comment_above: comment_above.clone(),
                        name: text_from_span!(declarator.span).trim().to_owned(),
                        r#type: ty.clone(),
                    });
                }
                continue;
            }

            let Ok(field_str) = raw_item_node.utf8_text(code.as_bytes()) else {
                return Err(());
            };
            let field_str = field_str.trim();

            // TODO: parse it properly.
            if let Some(maybe_ident) = field_str.strip_prefix("const unsigned char *") {
                let Some(maybe_ident) = maybe_ident.trim().strip_suffix(";") else {
                    return Err(());
                };
                let maybe_ident = maybe_ident.trim();
                if !is_identifier(maybe_ident) {
                    return Err(());
                }
                items.push(TypedefStructField {
                    comment_above: comment_above.clone(),
                    name: text_from_span!(declarator.span).trim().to_owned(),
                    r#type: TypedefStructFieldType::ConstUnsignedCharPtr,
                });
                continue;
            }

            return Ok(RootItem::Todo {
                kind: "TypedefStruct".to_owned(),
                code: text_from_span!(type_definition.span),
            });
        }

        return Ok(RootItem::TypedefStruct {
            name,
            fields: items,
        });
    }

    // e.g., `typedef enum <name> { … } <name>`.
    if let TypeSpecifier::EnumSpecifier(specifier) = &type_definition.r#type
        && let Some(specifier_name) = &specifier.name
        && type_definition.declarator.len() == 1
        && let Some(TypeDeclarator::TypeIdentifier(declarator)) =
            &type_definition.declarator.first()
    {
        let name = text_from_span!(specifier_name.span).trim().to_owned();
        if name != text_from_span!(declarator.span).trim() {
            return Err(());
        }

        let Some(body) = &specifier.body else {
            return Err(());
        };
        let Some(raw_body) =
            raw_node.descendant_for_byte_range(body.span.start_byte, body.span.end_byte)
        else {
            return Err(());
        };
        assert_eq!(raw_body.kind(), "enumerator_list");

        let mut cursor = raw_body.walk();

        let mut items: Vec<TypedefEnumVariant> = Vec::new();

        for raw_item_node in raw_body.children(&mut cursor) {
            if !raw_item_node.is_named() {
                continue;
            }

            if raw_item_node.kind() == "comment" {
                // We assume that comments come after the items they belong to,
                // on the same line.
                if find_line_before(code, raw_item_node.start_byte())
                    .trim()
                    .is_empty()
                {
                    return Err(());
                }

                let Some(current_item) = &mut items.last_mut() else {
                    return Err(());
                };
                if std::mem::take(&mut current_item.comment).is_some() {
                    return Err(());
                }

                continue;
            }

            let Ok(item_node) = Enumerator::from_node(raw_item_node, code.as_bytes()) else {
                return Err(());
            };

            let name = text_from_span!(item_node.name.span).trim().to_owned();
            let c_value_expr = item_node
                .value
                .map(|expr| TypedefEnumCValueExpr::try_from(text_from_span!(expr.span()).trim()))
                .transpose()?;

            items.push(TypedefEnumVariant {
                name,
                c_value_expr,
                comment: None,
            });
        }

        return Ok(RootItem::TypedefEnum {
            name,
            variants: items,
        });
    }

    Err(())
}
