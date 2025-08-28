{% for use in ctx.use_statements %}
{{use}}
{% endfor %}

{% if ctx.anon_instances|length > 0 %}
// anonymous component instances
{% endif %}
{% for mod in ctx.anon_instances %}
pub mod {{mod}};
{% endfor %}

{% if ctx.named_type_declarations|length > 0 %}
// named component type declarations
pub mod named_types {
    {% for mod in ctx.named_type_declarations %}
    pub mod {{mod}};
    {% endfor %}
}
{% endif %}

{% if ctx.named_type_instances|length > 0 %}
// instances of named component types
{% endif %}
{% for (inst_name, module) in ctx.named_type_instances %}
pub use {{module}} as {{inst_name}};
{% endfor %}

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
