use crate::{
    parser::grammar::{description, directive, name, ty, value},
    Parser, SyntaxKind, TokenKind, S, T,
};

/// See: https://spec.graphql.org/October2021/#InputObjectTypeDefinition
///
/// *InputObjectTypeDefinition*:
///     Description? **input** Name Directives? InputFieldsDefinition?
/// ```
pub(crate) fn input_object_type_definition(p: &mut Parser) {
    let _g = p.start_node(SyntaxKind::INPUT_OBJECT_TYPE_DEFINITION);

    if let Some(TokenKind::StringValue) = p.peek() {
        description::description(p);
    }

    if let Some("input") = p.peek_data().as_deref() {
        p.bump(SyntaxKind::input_KW);
    }

    match p.peek() {
        Some(TokenKind::Name) => name::name(p),
        _ => p.err("expected a Name"),
    }

    if let Some(T![@]) = p.peek() {
        directive::directives(p);
    }

    if let Some(T!['{']) = p.peek() {
        input_fields_definition(p);
    }
}

/// See: https://spec.graphql.org/October2021/#InputObjectTypeExtension
///
/// *InputObjectTypeExtension*:
///     **extend** **input** Name Directives? InputFieldsDefinition
///     **extend** **input** Name Directives
pub(crate) fn input_object_type_extension(p: &mut Parser) {
    let _g = p.start_node(SyntaxKind::INPUT_OBJECT_TYPE_EXTENSION);
    p.bump(SyntaxKind::extend_KW);
    p.bump(SyntaxKind::input_KW);

    let mut meets_requirements = false;

    match p.peek() {
        Some(TokenKind::Name) => name::name(p),
        _ => p.err("expected a Name"),
    }

    if let Some(T![@]) = p.peek() {
        meets_requirements = true;
        directive::directives(p);
    }

    if let Some(T!['{']) = p.peek() {
        meets_requirements = true;
        input_fields_definition(p);
    }

    if !meets_requirements {
        p.err("expected Directives or an Input Fields Definition");
    }
}

/// See: https://spec.graphql.org/October2021/#InputFieldsDefinition
///
/// *InputFieldsDefinition*:
///     **{** InputValueDefinition* **}**
pub(crate) fn input_fields_definition(p: &mut Parser) {
    let _g = p.start_node(SyntaxKind::INPUT_FIELDS_DEFINITION);
    p.bump(S!['{']);
    input_value_definition(p, false);
    p.expect(T!['}'], S!['}']);
}

/// See: https://spec.graphql.org/October2021/#InputValueDefinition
///
/// *InputValueDefinition*:
///     Description? Name **:** Type DefaultValue? Directives?
pub(crate) fn input_value_definition(p: &mut Parser, is_input: bool) -> Node {
    let input_value = InputValue::new(
        limited_string(20, u),
        ty::ty(u, builder).try_into_type().unwrap(),
    );

    input_value
}
