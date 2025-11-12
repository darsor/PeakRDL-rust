{% import 'src/components/macros.jinja2' as macros %}
//! {{ctx.module_comment}}

{{macros.includes(ctx)}}

{{ctx.comment}}
#[repr({{ctx.primitive}})]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum {{ctx.type_name|kw_filter}} {
    {% for variant in ctx.variants %}
    {% if variant.comment != "" %}
    {{variant.comment | indent(4)}}
    {% endif %}
    {{variant.name|kw_filter}} = {{variant.value}},
    {% endfor %}
}

impl {{ctx.type_name|kw_filter}} {
    /// Decode a bit pattern into an encoded enum variant.
    ///
    /// # Errors
    /// Returns an error if the bit pattern does not match any encoded variants.
    pub const fn from_bits(bits: {{ctx.primitive}}) -> Result<Self, crate::encode::UnknownVariant<{{ctx.primitive}}>> {
        match bits {
            {% for variant in ctx.variants %}
            {{variant.value}} => Ok(Self::{{variant.name|kw_filter}}),
            {% endfor %}
            bits => Err(crate::encode::UnknownVariant::new(bits)),
        }
    }

    /// The bit pattern of the variant
    #[must_use]
    pub const fn bits(&self) -> {{ctx.primitive}} {
        *self as {{ctx.primitive}}
    }
}
