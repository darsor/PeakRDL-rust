{% import 'src/components/macros.jinja2' as macros %}
//! {{ctx.module_comment}}

{{macros.includes(ctx)}}

{{ctx.comment}}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct {{ctx.type_name|kw_filter}} {
    ptr: *mut u8,
}

unsafe impl Send for {{ctx.type_name|kw_filter}} {}
unsafe impl Sync for {{ctx.type_name|kw_filter}} {}

impl {{ctx.type_name|kw_filter}} {
    /// Size in bytes of the underlying memory
    pub const SIZE: usize = {{"0x{:_X}".format(ctx.size)}};

    /// # Safety
    ///
    /// The caller must guarantee that the provided address points to a
    /// hardware register block implementing this interface.
    #[inline(always)]
    #[must_use]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr.cast::<u8>() }
    }

    #[inline(always)]
    #[must_use]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr.cast::<()>()
    }

{% for reg in ctx.registers %}
    {% set reg_type_name = reg.type_name|kw_filter %}
    {{reg.comment | indent()}}
    #[inline(always)]
    #[must_use]
    {% if reg.array is none %}
    pub const fn {{reg.inst_name|kw_filter}}(&self) -> crate::reg::Reg<{{reg_type_name}}, crate::access::{{reg.access}}> {
        unsafe { crate::reg::Reg::from_ptr(self.ptr.wrapping_byte_add({{"0x{:_X}".format(reg.addr_offset)}}).cast()) }
    }
    {% else %}
    pub const fn {{reg.inst_name|kw_filter}}(&self) -> {{reg.array.type.format("crate::reg::Reg<" ~ reg_type_name ~ ", crate::access::" ~ reg.access ~ ">")}} {
        // SAFETY: We will initialize every element before using the array
        let mut array = {{reg.array.type.format("core::mem::MaybeUninit::uninit()")}};

        {% set expr = "unsafe { crate::reg::Reg::<" ~ reg_type_name ~ ", crate::access::" ~ reg.access ~ ">::from_ptr(self.ptr.wrapping_byte_add(" ~ reg.array.addr_offset ~ ").cast()) }"  %}
        {{ macros.loop(0, reg.array.dims, expr) | indent(8) }}

        // SAFETY: All elements have been initialized above
        unsafe { core::mem::transmute(array) }
    }
    {% endif %}

{% endfor %}

{% for node in ctx.submaps %}
    {% set node_type_name = node.type_name|kw_filter %}
    {{node.comment | indent()}}
    #[inline(always)]
    #[must_use]
    {% if node.array is none %}
    pub const fn {{node.inst_name|kw_filter}}(&self) -> {{node_type_name}} {
        unsafe { {{node_type_name}}::from_ptr(self.ptr.wrapping_byte_add({{"0x{:_X}".format(node.addr_offset)}}).cast()) }
    }
    {% else %}
    pub const fn {{node.inst_name|kw_filter}}(&self) -> {{node.array.type.format(node_type_name)}} {
        // SAFETY: We will initialize every element before using the array
        let mut array = {{node.array.type.format("core::mem::MaybeUninit::uninit()")}};

        {% set expr = "unsafe { " ~ node_type_name ~ "::from_ptr(self.ptr.wrapping_byte_add(" ~ node.array.addr_offset ~ ").cast()) }"  %}
        {{ macros.loop(0, node.array.dims, expr) | indent(8) }}

        // SAFETY: All elements have been initialized above
        unsafe { core::mem::transmute(array) }
    }
    {% endif %}

{% endfor %}

{% for mem in ctx.memories %}
    {% set mem_type_name = mem.type_name|kw_filter %}
    {{mem.comment | indent()}}
    #[inline(always)]
    #[must_use]
    {% if mem.array is none %}
    pub const fn {{mem.inst_name|kw_filter}}(&self) -> {{mem_type_name}} {
        unsafe { {{mem_type_name}}::from_ptr(self.ptr.wrapping_byte_add({{"0x{:_X}".format(mem.addr_offset)}}).cast()) }
    }
    {% else %}
    pub const fn {{mem.inst_name|kw_filter}}(&self) -> {{mem.array.type.format(mem_type_name)}} {
        // SAFETY: We will initialize every element before using the array
        let mut array = {{mem.array.type.format("core::mem::MaybeUninit::uninit()")}};

        {% set expr = "unsafe { " ~ mem_type_name ~ "::from_ptr(self.ptr.wrapping_byte_add(" ~ mem.array.addr_offset ~ ").cast()) }"  %}
        {{ macros.loop(0, mem.array.dims, expr) | indent(8) }}

        // SAFETY: All elements have been initialized above
        unsafe { core::mem::transmute(array) }
    }
    {% endif %}

{% endfor %}
}
