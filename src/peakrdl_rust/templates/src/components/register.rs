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
pub struct {{ctx.type_name}}(u{{ctx.regwidth}});

unsafe impl Send for {{ctx.type_name}} {}
unsafe impl Sync for {{ctx.type_name}} {}

impl core::default::Default for {{ctx.type_name}} {
    fn default() -> Self {
        Self(0x{{"%X" % ctx.reset_val}})
    }
}

impl crate::reg::Register for {{ctx.type_name}} {
    type Regwidth = u{{ctx.regwidth}};
    type Accesswidth = u{{ctx.accesswidth}};

    unsafe fn from_raw(val: Self::Regwidth) -> Self {
        Self(val)
    }

    fn to_raw(self) -> Self::Regwidth {
        self.0
    }
}

impl {{ctx.type_name}} {
{% for field in ctx.fields %}
    pub const {{field.inst_name|upper}}_OFFSET: usize = {{field.bit_offset}};
    pub const {{field.inst_name|upper}}_WIDTH: usize = {{field.width}};
    pub const {{field.inst_name|upper}}_MASK: u{{ctx.regwidth}} = 0x{{"%X" % field.mask}};
    {% if field.is_signed is not none %}
    pub const {{field.inst_name|upper}}_SIGNED: bool = {{ field.is_signed|lower }};
    {% endif %}
    {% if field.fracwidth is not none %}
    pub const {{field.inst_name|upper}}_INTWIDTH: isize = {{ field.intwidth }};
    pub const {{field.inst_name|upper}}_FRACWIDTH: isize = {{ field.fracwidth }};
    {% endif %}

    {# Field Getter #}
    {{field.comment | indent()}}
    #[inline(always)]
    {% set return_type = "Option<" ~ field.encoding ~ ">" if field.encoding else field.primitive %}
    {% if "R" in field.access and field.fracwidth is none %}pub {% endif -%}
    const fn {{field.inst_name}}
    {%- if field.fracwidth is not none %}_raw_{% endif -%}
    (&self) -> {{return_type}} {
        let val = (self.0 >> Self::{{field.inst_name|upper}}_OFFSET) & Self::{{field.inst_name|upper}}_MASK;
        {% if field.encoding is not none %}
        {{field.encoding}}::from_bits(val as {{field.primitive}})
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
    {% if "R" in field.access %}pub {% endif -%}
    fn {{field.inst_name}}(&self) -> {{field.type_name}}FixedPoint {
        {{field.type_name}}FixedPoint::from_bits(self.{{field.inst_name}}_raw_())
    }
    {% endif %}

    {# Field Setter #}
    {% if "W" in field.access %}
    {{field.comment | indent()}}
    #[inline(always)]
    {% set input_type = field.encoding if field.encoding else field.primitive %}
    {% if field.fracwidth is none %}pub {% endif -%}
    const fn set_{{field.inst_name}}
    {%- if field.fracwidth is not none %}_raw_{% endif -%}
    (&mut self, val: {{input_type}}) {
        {% if field.encoding %}
        let val = val.bits() as u{{ctx.regwidth}};
        {% else %}
        let val = val as u{{ctx.regwidth}};
        {% endif %}
        self.0 = (self.0 & !(Self::{{field.inst_name|upper}}_MASK << Self::{{field.inst_name|upper}}_OFFSET)) | ((val & Self::{{field.inst_name|upper}}_MASK) << Self::{{field.inst_name|upper}}_OFFSET);
    }
    {% endif %}

    {# Field Fixed-Point Setter #}
    {% if field.fracwidth is not none %}
    {{field.comment | indent()}}
    #[inline(always)]
    pub const fn set_{{field.inst_name}}(&mut self, val: {{field.type_name}}FixedPoint) {
        self.set_{{field.inst_name}}_raw_(val.to_bits());
    }
    {% endif %}

{% endfor %}
}

impl core::fmt::Debug for {{ctx.type_name}} {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("{{ctx.type_name}}")
            {% for field in ctx.fields %}
            .field("{{field.inst_name}}", &self.{{field.inst_name}}())
            {% endfor %}
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let reg = {{ctx.type_name}}::default();
        {% for field in ctx.fields %}
        assert_eq!(reg.{{field.inst_name}}(){% if field.fracwidth is not none %}.to_f64(){% endif %}, {{field.reset_val}});
        {% endfor %}
    }
}
