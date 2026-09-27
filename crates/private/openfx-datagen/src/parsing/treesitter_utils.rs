use treesitter_types_c::{Declaration, DeclarationDeclarator, Declarator, Span};

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
