use treesitter_types_c::{
    Declaration, DeclarationDeclarator, Declarator, EnumSpecifier, Enumerator, FieldDeclaration,
    FieldDeclarator, FromNode, FunctionDeclarator, FunctionDeclaratorDeclarator,
    ParameterDeclaration, ParameterDeclarationChildren, ParameterDeclarationDeclarator,
    ParameterList, ParameterListChildren, ParenthesizedDeclaratorChildren, PointerDeclarator,
    PointerDeclaratorDeclarator, Span, Spanned, StructSpecifier, TypeDeclarator, TypeDefinition,
    TypeIdentifier, TypeSpecifier,
};

use crate::parsing::{
    FunctionParameter, FunctionParameterType, RootItem, TypeSimpleCName, TypeSimpleNonCName,
    TypedefEnumCValueExpr, TypedefEnumVariant, TypedefFunctionReturnType, TypedefPrimitiveCType,
    TypedefStructField, TypedefStructFieldItem, TypedefStructFieldType,
    TypedefStructFieldTypeFunctionPointerReturnType,
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

fn extract_function_declarator_from_field_declaration<'a>(
    declaration: &'a FieldDeclaration,
) -> Option<&'a FunctionDeclarator<'a>> {
    if declaration.declarator.len() != 1 {
        return None;
    }
    match &declaration.declarator.first().unwrap() {
        FieldDeclarator::FunctionDeclarator(function_declarator) => Some(function_declarator),
        FieldDeclarator::PointerDeclarator(pointer_declarator) => {
            extract_function_declarator_from_pointer_declarator(pointer_declarator)
        }
        _ => None,
    }
}

fn extract_function_declarator_from_pointer_declarator<'a>(
    declaration: &'a PointerDeclarator,
) -> Option<&'a FunctionDeclarator<'a>> {
    match &declaration.declarator {
        PointerDeclaratorDeclarator::Declarator(declarator) => match &**declarator {
            Declarator::FunctionDeclarator(function_declarator) => Some(function_declarator),
            Declarator::PointerDeclarator(pointer_declarator) => {
                extract_function_declarator_from_pointer_declarator(pointer_declarator)
            }
            _ => None,
        },
        _ => None,
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
        && let Some(ParenthesizedDeclaratorChildren::TypeDeclarator(declarator)) =
            declarator.children.first()
        && let TypeDeclarator::TypeIdentifier(declarator) = &**declarator
    {
        return parse_type_definition_function(
            raw_node,
            code,
            type_definition,
            parameters_node,
            declarator,
        );
    }

    // e.g., `typedef struct <name> { … } <name>`
    if let TypeSpecifier::StructSpecifier(specifier) = &type_definition.r#type
        && let Some(specifier_name) = &specifier.name
        && type_definition.declarator.len() == 1
        && let Some(TypeDeclarator::TypeIdentifier(declarator)) =
            &type_definition.declarator.first()
    {
        return parse_type_definition_struct(raw_node, code, specifier, specifier_name, declarator);
    }

    // e.g., `typedef enum <name> { … } <name>`.
    if let TypeSpecifier::EnumSpecifier(specifier) = &type_definition.r#type
        && let Some(specifier_name) = &specifier.name
        && type_definition.declarator.len() == 1
        && let Some(TypeDeclarator::TypeIdentifier(declarator)) =
            &type_definition.declarator.first()
    {
        return parse_type_definition_enum(raw_node, code, specifier, specifier_name, declarator);
    }

    Err(())
}

fn parse_type_definition_function(
    raw_node: &tree_sitter::Node,
    code: &str,
    type_definition: &TypeDefinition,
    parameters_node: &ParameterList,
    declarator: &TypeIdentifier,
) -> Result<RootItem, ()> {
    macro_rules! text_from_span {
        ($span:expr) => {
            code[$span.start_byte..$span.end_byte].to_owned()
        };
    }

    let name = text_from_span!(declarator.span).trim().to_owned();

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

    let (parameters, is_variadic) = parse_function_parameter_list(raw_node, code, parameters_node)?;

    Ok(RootItem::TypedefFunction {
        name,
        parameters,
        is_variadic,
        return_type: ret_ty,
    })
}

fn parse_type_definition_struct(
    raw_node: &tree_sitter::Node,
    code: &str,
    specifier: &StructSpecifier,
    specifier_name: &TypeIdentifier,
    declarator: &TypeIdentifier,
) -> Result<RootItem, ()> {
    macro_rules! text_from_span {
        ($span:expr) => {
            code[$span.start_byte..$span.end_byte].to_owned()
        };
    }

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

    let mut fields: Vec<TypedefStructField> = Vec::new();
    let mut last_comment: Option<String> = None;

    for raw_item_node in raw_body.children(&mut cursor) {
        if !raw_item_node.is_named() {
            continue;
        }

        if raw_item_node.kind() == "comment" {
            if last_comment.is_some() {
                fields.push(TypedefStructField::StandaloneComment {
                    comment: last_comment.take().unwrap(),
                });
            }

            let last_comment_ = Some(
                raw_item_node
                    .utf8_text(code.as_bytes())
                    .unwrap()
                    .trim()
                    .to_owned(),
            );

            if find_line_before(code, raw_item_node.start_byte())
                .trim()
                .is_empty()
            {
                // comments appear on lines by themselves: there should be
                // a field declaration following it, which it belongs to.
                last_comment = last_comment_;
            } else {
                // comments after some code in the same line: there should
                // be a function paramter before it at the same line, which
                // it belongs to.
                if let Some(current_item) = fields.last_mut()
                    && let TypedefStructField::Item {
                        item: current_item, ..
                    } = current_item
                    && let TypedefStructFieldType::FunctionPointer { parameters, .. } =
                        &mut current_item.r#type
                    && let Some(last_parameter) = parameters.last_mut()
                    && let FunctionParameter { comment, .. } = last_parameter
                    && comment.is_none()
                {
                    *comment = last_comment_;
                } else {
                    return Err(());
                }
            }

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
            let ty = if let Ok(name) = TypeSimpleCName::try_from(&type_str) {
                TypedefStructFieldType::SimpleC { name: name.clone() }
            } else if let Ok(name) = TypeSimpleNonCName::try_from(&type_str) {
                TypedefStructFieldType::SimpleNonC { name: name.clone() }
            } else {
                return Err(());
            };

            for declarator in identifier_declarators {
                fields.push(TypedefStructField::Item {
                    comment_above: comment_above.clone(),
                    item: TypedefStructFieldItem {
                        name: text_from_span!(declarator.span).trim().to_owned(),
                        r#type: ty.clone(),
                    },
                });
            }
            continue;
        }

        if let Some(declarator) = extract_function_declarator_from_field_declaration(&item_node)
            && let FunctionDeclaratorDeclarator::Declarator(inner_declarator) =
                &declarator.declarator
            && let Declarator::ParenthesizedDeclarator(inner_declarator) = &**inner_declarator
            && inner_declarator.children.len() == 1
            && let Some(ParenthesizedDeclaratorChildren::Declarator(inner_declarator)) =
                inner_declarator.children.first()
            && let Declarator::PointerDeclarator(inner_declarator) = &**inner_declarator
            && let PointerDeclaratorDeclarator::FieldDeclarator(inner_declarator) =
                &inner_declarator.declarator
            && let FieldDeclarator::FieldIdentifier(name) = &**inner_declarator
        {
            let name = text_from_span!(name.span()).trim().to_owned();

            let ret_ty_str = &code[item_node.span.start_byte..declarator.span.start_byte];
            let ret_ty_str = ret_ty_str.split_whitespace().collect::<Vec<_>>().join(" ");
            let ret_ty = if ret_ty_str == "const void *" {
                TypedefStructFieldTypeFunctionPointerReturnType::ConstVoidPtr
            } else if ret_ty_str == "OfxStatus" {
                TypedefStructFieldTypeFunctionPointerReturnType::OfxStatus
            } else if let Ok(name) = TypeSimpleCName::try_from(&ret_ty_str) {
                TypedefStructFieldTypeFunctionPointerReturnType::SimpleC { name }
            } else {
                return Err(());
            };

            let (parameters, is_variadic) =
                parse_function_parameter_list(raw_node, code, &declarator.parameters)?;

            fields.push(TypedefStructField::Item {
                comment_above,
                item: TypedefStructFieldItem {
                    name,
                    r#type: TypedefStructFieldType::FunctionPointer {
                        parameters,
                        is_variadic,
                        return_type: ret_ty,
                    },
                },
            });

            continue;
        }

        let Ok(field_str) = raw_item_node.utf8_text(code.as_bytes()) else {
            return Err(());
        };
        let field_str = field_str.split_whitespace().collect::<Vec<_>>().join(" ");

        // TODO: parse it properly.
        if let Some(maybe_ident) = field_str.strip_prefix("const unsigned char *") {
            let Some(maybe_ident) = maybe_ident.trim().strip_suffix(";") else {
                return Err(());
            };
            let maybe_ident = maybe_ident.trim();
            if !is_identifier(maybe_ident) {
                return Err(());
            }
            fields.push(TypedefStructField::Item {
                comment_above: comment_above.clone(),
                item: TypedefStructFieldItem {
                    name: text_from_span!(declarator.span).trim().to_owned(),
                    r#type: TypedefStructFieldType::ConstUnsignedCharPtr,
                },
            });
            continue;
        } else if let Some(maybe_ident) = field_str.strip_prefix("const char *") {
            let Some(maybe_ident) = maybe_ident.trim().strip_suffix(";") else {
                return Err(());
            };
            let maybe_ident = maybe_ident.trim();
            if !is_identifier(maybe_ident) {
                return Err(());
            }
            fields.push(TypedefStructField::Item {
                comment_above: comment_above.clone(),
                item: TypedefStructFieldItem {
                    name: text_from_span!(declarator.span).trim().to_owned(),
                    r#type: TypedefStructFieldType::ConstCharPtr,
                },
            });
            continue;
        } else if let Some(maybe_ident) = field_str.strip_prefix("OfxPluginEntryPoint *") {
            let Some(maybe_ident) = maybe_ident.trim().strip_suffix(";") else {
                return Err(());
            };
            let maybe_ident = maybe_ident.trim();
            if !is_identifier(maybe_ident) {
                return Err(());
            }
            fields.push(TypedefStructField::Item {
                comment_above: comment_above.clone(),
                item: TypedefStructFieldItem {
                    name: text_from_span!(declarator.span).trim().to_owned(),
                    r#type: TypedefStructFieldType::OfxPluginEntryPointPtr,
                },
            });
            continue;
        }

        return Err(());
    }

    if let Some(comment) = last_comment.take() {
        fields.push(TypedefStructField::StandaloneComment { comment });
    }

    Ok(RootItem::TypedefStruct { name, fields })
}

fn parse_type_definition_enum(
    raw_node: &tree_sitter::Node,
    code: &str,
    specifier: &EnumSpecifier,
    specifier_name: &TypeIdentifier,
    declarator: &TypeIdentifier,
) -> Result<RootItem, ()> {
    macro_rules! text_from_span {
        ($span:expr) => {
            code[$span.start_byte..$span.end_byte].to_owned()
        };
    }

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

    Ok(RootItem::TypedefEnum {
        name,
        variants: items,
    })
}

fn parse_function_parameter_list(
    raw_node: &tree_sitter::Node,
    code: &str,
    parameters_node: &ParameterList,
) -> Result<(Vec<FunctionParameter>, bool), ()> {
    let Some(raw_parameters_node) = raw_node.descendant_for_byte_range(
        parameters_node.span.start_byte,
        parameters_node.span.end_byte,
    ) else {
        return Err(());
    };
    assert!(raw_parameters_node.kind() == "parameter_list");

    let mut cursor = raw_parameters_node.walk();

    let mut parameters: Vec<FunctionParameter> = Vec::new();

    let mut has_void_parameter = false;
    let mut is_variadic = false;

    for raw_parameter_node in raw_parameters_node.children(&mut cursor) {
        if !raw_parameter_node.is_named() {
            continue;
        }
        if raw_parameter_node.kind() == "comment" {
            // We assume that comments come after the parameter they belong to,
            // on the same line.
            if find_line_before(code, raw_parameter_node.start_byte())
                .trim()
                .is_empty()
            {
                return Err(());
            }

            let Some(current_item) = &mut parameters.last_mut() else {
                return Err(());
            };
            if std::mem::take(&mut current_item.comment).is_some() {
                return Err(());
            }

            continue;
        }

        if is_variadic {
            return Err(());
        }

        let Ok(parameter_node) =
            ParameterListChildren::from_node(raw_parameter_node, code.as_bytes())
        else {
            return Err(());
        };

        match parameter_node {
            ParameterListChildren::ParameterDeclaration(parameter_declaration) => {
                let Some((name, r#type)) =
                    parse_parameter_declaration(code, &parameter_declaration)?
                else {
                    has_void_parameter = true;
                    continue;
                };

                parameters.push(FunctionParameter {
                    name,
                    r#type,
                    comment: None,
                });
            }
            ParameterListChildren::VariadicParameter(_variadic_parameter) => {
                is_variadic = true;
            }
            _ => return Err(()),
        }
    }

    if has_void_parameter && !parameters.is_empty() {
        return Err(());
    }

    Ok((parameters, is_variadic))
}

fn parse_parameter_declaration(
    code: &str,
    declaration: &ParameterDeclaration,
) -> Result<Option<(String, FunctionParameterType)>, ()> {
    macro_rules! text_from_span {
        ($span:expr) => {
            code[$span.start_byte..$span.end_byte].to_owned()
        };
    }

    if text_from_span!(declaration.span()).trim() == "void" {
        return Ok(None);
    }

    let mut has_const_type_qualifier = if declaration.children.is_empty() {
        false
    } else if declaration.children.len() == 1
        && let Some(ParameterDeclarationChildren::TypeQualifier(type_qualifier)) =
            declaration.children.first()
        && text_from_span!(type_qualifier.span) == "const"
    {
        true
    } else {
        return Err(());
    };

    let mut ty =
        if let Ok(name) = TypeSimpleCName::try_from(&text_from_span!(declaration.r#type.span())) {
            FunctionParameterType::SimpleC { name }
        } else if let Ok(name) =
            TypeSimpleNonCName::try_from(&text_from_span!(declaration.r#type.span()))
        {
            FunctionParameterType::SimpleNonC { name }
        } else {
            return Err(());
        };

    let Some(ParameterDeclarationDeclarator::Declarator(declarator)) = &declaration.declarator
    else {
        return Err(());
    };
    let mut declarator = declarator;
    let name = loop {
        match &**declarator {
            Declarator::Identifier(identifier) => {
                break text_from_span!(identifier.span).trim().to_owned();
            }
            Declarator::PointerDeclarator(pointer_declarator) => {
                if has_const_type_qualifier {
                    has_const_type_qualifier = false;
                    ty = FunctionParameterType::ConstPtr {
                        pointee: Box::new(ty),
                    };
                } else {
                    ty = FunctionParameterType::Ptr {
                        pointee: Box::new(ty),
                    }
                }
                let PointerDeclaratorDeclarator::Declarator(declarator_) =
                    &pointer_declarator.declarator
                else {
                    return Err(());
                };
                declarator = declarator_;
            }
            _ => return Err(()),
        }
    };

    Ok(Some((name, ty)))
}
