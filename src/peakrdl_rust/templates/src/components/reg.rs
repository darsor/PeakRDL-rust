{% import 'src/components/macros.jinja2' as macros %}
//! {{ctx.module_comment}}

{{macros.includes(ctx)}}

{{ctx.comment}}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct {{ctx.type_name}}({{ctx.primitive}});

impl core::default::Default for {{ctx.type_name}} {
    fn default() -> Self {
        Self(0x{{"%x" % ctx.reset_val}})
    }
}

impl {{ctx.type_name}} {
{% for field in ctx.fields %}
    {% if "R" in field.access %}
    {{field.comment | indent()}}
    #[inline(always)]
    {% set return_type = "Option<" ~ field.encoding ~ ">" if field.encoding else field.primitive %}
    pub const fn {{field.inst_name}}(&self) -> {{return_type}} {
        let val = (self.0 >> {{field.bit_offset}}usize) & 0x{{"%x" % field.mask}};
        {% if field.encoding is not none %}
        {{field.encoding}}::from_bits(val as {{field.primitive}})
        {% elif field.primitive == "bool" %}
        val != 0
        {% elif field.primitive != ctx.primitive %}
        val as {{field.primitive}}
        {% else %}
        val
        {% endif %}
    }
    {% endif %}

    {% if "W" in field.access %}
    {{field.comment | indent()}}
    #[inline(always)]
    {% set input_type = field.encoding if field.encoding else field.primitive %}
    pub const fn set_{{field.inst_name}}(&mut self, val: {{input_type}}) {
        {% if field.encoding %}
        let val = val.bits() as {{ctx.primitive}};
        {% else %}
        let val = val as {{ctx.primitive}};
        {% endif %}
        self.0 = (self.0 & !(0x{{"%x" % field.mask}} << {{field.bit_offset}}usize)) | ((val & 0x{{"%x" % field.mask}}) << {{field.bit_offset}}usize);
    }
    {% endif %}

{% endfor %}
}
