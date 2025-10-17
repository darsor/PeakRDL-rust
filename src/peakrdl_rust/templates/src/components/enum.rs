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
    /// Generate an instance of this field from a bit pattern. If the bit
    /// pattern is not a valid variant, return None.
    pub const fn from_bits(bits: {{ctx.primitive}}) -> Option<Self> {
        match bits {
            {% for variant in ctx.variants %}
            {{variant.value}} => Some(Self::{{variant.name|kw_filter}}),
            {% endfor %}
            _ => None,
        }
    }

    pub const fn bits(&self) -> u8 {
        *self as {{ctx.primitive}}
    }
}
