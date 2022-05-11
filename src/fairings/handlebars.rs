use super::prelude::*;
use handlebars::*;

// https://github.com/sunng87/handlebars-rust/issues/43
pub fn array_length_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
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
    rocket_dyn_templates::Template::custom(|h| {
        h.handlebars
            .register_helper("array_length", Box::new(array_length_helper));
    })
}
