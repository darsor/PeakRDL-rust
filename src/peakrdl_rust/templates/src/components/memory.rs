{% import 'src/components/macros.jinja2' as macros %}
//! {{ctx.module_comment}}

{{macros.includes(ctx)}}

{{ctx.comment}}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct {{ctx.type_name|kw_filter}} {
    ptr: *mut {{ctx.primitive}},
}

unsafe impl Send for {{ctx.type_name|kw_filter}} {}
unsafe impl Sync for {{ctx.type_name|kw_filter}} {}

impl crate::mem::Memory for {{ctx.type_name|kw_filter}} {
    type Memwidth = {{ctx.primitive}};
    type Access = crate::access::{{ctx.access}};

    fn first_entry_ptr(&self) -> *mut Self::Memwidth {
        self.ptr
    }

    fn num_entries(&self) -> usize {
        {{ctx.mementries}}
    }

    fn width(&self) -> usize {
        {{ctx.memwidth}}
    }
}

impl {{ctx.type_name|kw_filter}} {
    /// Size in bytes of the memory
    pub const SIZE: usize = {{"0x{:_X}".format(ctx.size)}};

    /// # Safety
    ///
    /// The caller must guarantee that the provided address points to a
    /// hardware memory implementing this interface.
    #[inline(always)]
    #[must_use]
    pub const unsafe fn from_ptr(ptr: *mut {{ctx.primitive}}) -> Self {
        Self { ptr }
    }

    #[inline(always)]
    #[must_use]
    pub const fn as_ptr(&self) -> *mut {{ctx.primitive}} {
        self.ptr
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
}
