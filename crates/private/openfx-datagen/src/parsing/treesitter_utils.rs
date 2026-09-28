use treesitter_types_c::{
    Declaration, DeclarationDeclarator, Declarator, Enumerator, FromNode,
    PointerDeclaratorDeclarator, Span, Spanned, TypeDeclarator, TypeDefinition, TypeSpecifier,
};

use crate::parsing::{
    RootItem, TypedefEnumCValueExpr, TypedefEnumVariant, TypedefPrimitiveCType,
    utils::find_line_before,
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

    // e.g., `typedef enum <name> { ... } <name>`.
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
        let mut current_item: Option<TypedefEnumVariant> = None;

        for raw_item_node in raw_body.children(&mut cursor) {
            if raw_item_node.kind() == "comment" {
                // We assume that comments come after the items they belong to,
                // on the same line.
                if find_line_before(code, raw_item_node.start_byte())
                    .trim()
                    .is_empty()
                {
                    return Err(());
                }

                let Some(current_item) = &mut current_item else {
                    return Err(());
                };
                if std::mem::take(&mut current_item.comment).is_some() {
                    return Err(());
                }

                continue;
            }

            if !raw_item_node.is_named() {
                continue;
            }

            let Ok(item_node) = Enumerator::from_node(raw_item_node, code.as_bytes()) else {
                return Err(());
            };

            if let Some(current_item) = current_item.take() {
                items.push(current_item);
            }

            let name = text_from_span!(item_node.name.span).trim().to_owned();
            let c_value_expr = item_node
                .value
                .map(|expr| TypedefEnumCValueExpr::try_from(text_from_span!(expr.span()).trim()))
                .transpose()?;

            current_item = Some(TypedefEnumVariant {
                name,
                c_value_expr,
                comment: None,
            });
        }

        if let Some(current_item) = current_item.take() {
            items.push(current_item);
        }

        return Ok(RootItem::TypedefEnum {
            name,
            variants: items,
        });
    }

    Ok(RootItem::Todo {
        kind: "TypeDefinition".to_owned(),
        code: text_from_span!(type_definition.span),
    })
}
