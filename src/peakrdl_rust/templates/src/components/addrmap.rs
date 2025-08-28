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
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct {{ctx.type_name}} {
    ptr: *mut u8,
}

unsafe impl Send for {{ctx.type_name}} {}
unsafe impl Sync for {{ctx.type_name}} {}

impl {{ctx.type_name}} {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as *mut u8 }
    }

    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as *mut ()
    }

{% for reg in ctx.registers %}
    {{reg.comment | indent()}}
    #[inline(always)]
    {% if not reg.is_array %}
    pub const fn {{reg.inst_name}}(&self) -> crate::reg::Reg<{{reg.type_name}}, crate::reg::{{reg.access}}> {
        unsafe { crate::reg::Reg::from_ptr(self.ptr.add(0x{{"%x" % reg.addr_offset}}) as _) }
    }
    {% else %}
    {{TODO}}
    {% endif %}

{% endfor %}

{% for node in ctx.submaps %}
    {{node.comment | indent()}}
    #[inline(always)]
    {%- if not node.is_array %}
    pub const fn {{node.inst_name}}(&self) -> {{node.type_name}} {
        unsafe { node.type_name::from_ptr(self.ptr.add(0x{{"%x" % node.addr_offset}}) as _) }
    }
    {% else %}
    {{TODO}}
    {% endif %}

{% endfor %}
}
