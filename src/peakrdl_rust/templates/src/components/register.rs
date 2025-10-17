{% import 'src/components/macros.jinja2' as macros %}
//! {{ctx.module_comment}}

{{macros.includes(ctx)}}

{% for field in ctx.fields %}
    {% if field.fracwidth is not none %}
pub type {{field.type_name}}FixedPoint = crate::fixedpoint::FixedPoint<{{field.primitive}}, {{field.intwidth}}, {{field.fracwidth}}>;
    {% endif %}
{% endfor %}

{{ctx.comment}}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct {{ctx.type_name|kw_filter}}(u{{ctx.regwidth}});

unsafe impl Send for {{ctx.type_name|kw_filter}} {}
unsafe impl Sync for {{ctx.type_name|kw_filter}} {}

impl core::default::Default for {{ctx.type_name|kw_filter}} {
    fn default() -> Self {
        Self({{"0x{:_X}".format(ctx.reset_val)}})
    }
}

impl crate::reg::Register for {{ctx.type_name|kw_filter}} {
    type Regwidth = u{{ctx.regwidth}};
    type Accesswidth = u{{ctx.accesswidth}};

    unsafe fn from_raw(val: Self::Regwidth) -> Self {
        Self(val)
    }

    fn to_raw(self) -> Self::Regwidth {
        self.0
    }
}

impl {{ctx.type_name|kw_filter}} {
{% for field in ctx.fields %}
    pub const {{field.inst_name|upper}}_OFFSET: usize = {{field.bit_offset}};
    pub const {{field.inst_name|upper}}_WIDTH: usize = {{field.width}};
    pub const {{field.inst_name|upper}}_MASK: u{{ctx.regwidth}} = {{"0x{:_X}".format(field.mask)}};
    {% if field.is_signed is not none %}
    pub const {{field.inst_name|upper}}_SIGNED: bool = {{ field.is_signed|lower }};
    {% endif %}
    {% if field.fracwidth is not none %}
    pub const {{field.inst_name|upper}}_INTWIDTH: isize = {{ field.intwidth }};
    pub const {{field.inst_name|upper}}_FRACWIDTH: isize = {{ field.fracwidth }};
    {% endif %}

    {# Field Getter #}
    {% if "R" in field.access %}
    {{field.comment | indent()}}
    {% set return_type = field.encoding if field.encoding else field.primitive %}
    {% set return_type = "Result<" ~ return_type ~ ", crate::encode::UnknownVariant<" ~ field.primitive ~">>" if not field.exhaustive else return_type %}
    #[inline(always)]
    {% if return_type.startswith("Result<") %}
    #[allow(clippy::missing_errors_doc)]
    {% else %}
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    {% endif %}
    {% if field.fracwidth is not none %}
    fn {{field.inst_name}}_raw_(&self) -> {{return_type}} {
    {% else %}
    pub fn {{field.inst_name|kw_filter}}(&self) -> {{return_type}} {
    {% endif %}
        let val = (self.0 >> Self::{{field.inst_name|upper}}_OFFSET) & Self::{{field.inst_name|upper}}_MASK;
        {% if field.encoding is not none %}
        {{field.encoding}}::from_bits(val as {{field.primitive}})
            {%- if field.exhaustive %}.expect("All possible field values represented by enum"){% endif %}
        {% elif field.primitive == "bool" %}
        val != 0
        {% elif field.is_signed %}
            {% set primitive_width = field.primitive[1:]|int %}
            {% set num_extra_bits = primitive_width - field.width %}
            {% if num_extra_bits == 0 %}
        val as {{field.primitive}}
            {% else %}
        // sign extend
        (val as {{field.primitive}}).wrapping_shl({{num_extra_bits}}).wrapping_shr({{num_extra_bits}})
            {% endif %}
        {% elif field.primitive != "u" ~ ctx.regwidth %}
        val as {{field.primitive}}
        {% else %}
        val
        {% endif %}
    }

    {# Field Fixed-Point Getter #}
    {% if field.fracwidth is not none %}
    {{field.comment | indent()}}
    #[inline(always)]
    #[must_use]
    pub fn {{field.inst_name|kw_filter}}(&self) -> {{field.type_name}}FixedPoint {
        {{field.type_name}}FixedPoint::from_bits(self.{{field.inst_name}}_raw_())
    }
    {% endif %}
    {% endif %}

    {# Field Setter #}
    {% if "W" in field.access %}
    {{field.comment | indent()}}
    #[inline(always)]
    {% set input_type = field.encoding if field.encoding else field.primitive %}
    {% if field.fracwidth is none %}
    pub {% endif -%}
    fn set_{{field.inst_name}}
    {%- if field.fracwidth is not none %}_raw_{% endif -%}
    (&mut self, val: {{input_type}}) {
        {% if field.encoding %}
        let val = val.bits() as u{{ctx.regwidth}};
        {% else %}
        let val = val as u{{ctx.regwidth}};
        {% endif %}
        self.0 = (self.0 & !(Self::{{field.inst_name|upper}}_MASK << Self::{{field.inst_name|upper}}_OFFSET)) | ((val & Self::{{field.inst_name|upper}}_MASK) << Self::{{field.inst_name|upper}}_OFFSET);
    }

    {# Field Fixed-Point Setter #}
    {% if field.fracwidth is not none %}
    {{field.comment | indent()}}
    #[inline(always)]
    pub fn set_{{field.inst_name}}(&mut self, val: {{field.type_name}}FixedPoint) {
        self.set_{{field.inst_name}}_raw_(val.to_bits());
    }
    {% endif %}
    {% endif %}

{% endfor %}
}

impl core::fmt::Debug for {{ctx.type_name|kw_filter}} {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("{{ctx.type_name|kw_filter}}")
            {% for field in ctx.fields %}
            {% if "R" in field.access %}
            .field("{{field.inst_name|kw_filter}}", &self.{{field.inst_name|kw_filter}}())
            {% endif %}
            {% endfor %}
            .finish()
    }
}

{% if ctx.has_sw_readable %}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let reg = {{ctx.type_name|kw_filter}}::default();
        {% for field in ctx.fields %}
        {% if "R" in field.access %}
        assert_eq!(reg.{{field.inst_name|kw_filter}}(){% if field.fracwidth is not none %}.to_f64(){% endif %}, {{field.reset_val}});
        {% endif %}
        {% endfor %}
    }
}
{% endif %}
