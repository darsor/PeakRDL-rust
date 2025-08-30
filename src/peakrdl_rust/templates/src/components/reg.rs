{% import 'src/components/macros.jinja2' as macros %}
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
    pub const fn {{field.inst_name}}(&self) -> {{field.primitive}} {
        let val = (self.0 >> {{field.bit_offset}}usize) & 0x{{"%x" % field.mask}};
        {% if field.primitive == "bool" %}
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
    pub const fn set_{{field.inst_name}}(&mut self, val: {{field.primitive}}) {
        self.0 = (self.0 & !(0x{{"%x" % field.mask}} << {{field.bit_offset}}usize)) | (((val as {{ctx.primitive}}) & 0x{{"%x" % field.mask}}) << {{field.bit_offset}}usize);
    }
    {% endif %}

{% endfor %}
}
