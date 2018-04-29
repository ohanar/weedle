use types::*;
use Parse;
use common::*;
use attribute::*;

/// Parses a list of argument. Ex: `double v1, double v2, double v3, optional double alpha`
pub type ArgumentList = Punctuated<Argument, term!(,)>;

/// Parses an argument. Ex: `double v1|double... v1s`
#[derive(Debug, PartialEq)]
pub enum Argument {
    Single(SingleArgument),
    Variadic(VariadicArgument)
}

impl Parse for Argument {
    named!(parse -> Self, alt!(
        weedle!(SingleArgument) => {|inner| Argument::Single(inner)} |
        weedle!(VariadicArgument) => {|inner| Argument::Variadic(inner)}
    ));
}

/// Parses `[attributes]? optional? type identifier ( = default )?`
///
/// Note: `= default` is only allowed if `optional` is present
#[derive(Debug, PartialEq)]
pub struct SingleArgument {
    pub attributes: Option<ExtendedAttributeList>,
    pub optional: Option<term!(optional)>,
    pub type_: Type,
    pub identifier: Identifier,
    pub default: Option<Default>
}

impl Parse for SingleArgument {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        optional: weedle!(Option<term!(optional)>) >>
        type_: weedle!(Type) >>
        identifier: weedle!(Identifier) >>
        default: opt_flat!(cond_reduce!(optional.is_some(), weedle!(Option<Default>))) >>
        (SingleArgument { attributes, optional, type_, identifier, default })
    ));
}

/// Parses `[attributes]? type... identifier`
#[derive(Debug, PartialEq)]
pub struct VariadicArgument {
    pub attributes: Option<ExtendedAttributeList>,
    pub type_: Type,
    pub ellipsis: term!(...),
    pub identifier: Identifier
}

impl Parse for VariadicArgument {
    named!(parse -> Self, do_parse!(
        attributes: weedle!(Option<ExtendedAttributeList>) >>
        type_: weedle!(Type) >>
        ellipsis: weedle!(term!(...)) >>
        identifier: weedle!(Identifier) >>
        (VariadicArgument { attributes, type_, ellipsis, identifier })
    ));
}

#[cfg(test)]
mod test {
    use super::*;

    test!(should_parse_single_argument { "optional short a" =>
        "";
        SingleArgument;
        attributes.is_none();
        optional.is_some();
        identifier.name == "a";
        default.is_none();
    });

    test!(should_parse_variadic_argument { "short... a" =>
        "";
        VariadicArgument;
        attributes.is_none();
        identifier.name == "a";
    });
}
