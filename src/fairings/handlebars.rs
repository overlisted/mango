use super::prelude::*;

// https://github.com/sunng87/handlebars-rust/issues/43
pub fn array_length_helper<'r, 'reg, 'rc, 's, 't0>(
    h: &'r handlebars::Helper<'reg, 'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &'s mut handlebars::RenderContext<'reg, 'rc>,
    out: &'t0 mut (dyn handlebars::Output + 't0),
) -> Result<(), handlebars::RenderError> {
    let length = h
        .param(0)
        .as_ref()
        .and_then(|v| v.value().as_array())
        .map(|arr| arr.len())
        .ok_or(handlebars::RenderError::new(
            "Param 0 of type 'array' is required for array_length helper",
        ))?;

    out.write(length.to_string().as_ref())?;

    Ok(())
}

pub fn fairing() -> impl Fairing {
    Template::custom(|h| {
        h.handlebars
            .register_helper("array_length", Box::new(array_length_helper));
    })
}
