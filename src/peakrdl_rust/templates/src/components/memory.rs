{% import 'src/components/macros.jinja2' as macros %}
//! {{ctx.module_comment}}

{{macros.includes(ctx)}}

{{ctx.comment}}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct {{ctx.type_name}} {
    ptr: *mut {{ctx.primitive}},
}

unsafe impl Send for {{ctx.type_name}} {}
unsafe impl Sync for {{ctx.type_name}} {}

impl crate::mem::Memory for {{ctx.type_name}} {
    type Memwidth = {{ctx.primitive}};

    fn first_entry_ptr(&self) -> *mut Self::Memwidth {
        self.ptr
    }

    fn len(&self) -> usize {
        {{ctx.mementries}}
    }

    fn width(&self) -> usize {
        {{ctx.memwidth}}
    }
}

impl {{ctx.type_name}} {
    /// Size in bytes of the memory
    pub const SIZE: usize = 0x{{"%x" % ctx.size}};

    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut {{ctx.primitive}}) -> Self {
        Self { ptr }
    }

    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut {{ctx.primitive}} {
        self.ptr
    }

{% for reg in ctx.registers %}
    {{reg.comment | indent()}}
    #[inline(always)]
    {% if reg.array is none %}
    pub const fn {{reg.inst_name}}(&self) -> crate::reg::Reg<{{reg.type_name}}, crate::reg::{{reg.access}}> {
        unsafe { crate::reg::Reg::from_ptr(self.ptr.byte_add(0x{{"%x" % reg.addr_offset}}) as _) }
    }
    {% else %}
    pub const fn {{reg.inst_name}}(&self) -> {{reg.array.type.format("crate::reg::Reg<" ~ reg.type_name ~ ", crate::reg::" ~ reg.access ~ ">")}} {
        // SAFETY: We will initialize every element before using the array
        let mut array = {{reg.array.type.format("core::mem::MaybeUninit::uninit()")}};

        {% set expr = "unsafe { crate::reg::Reg::<" ~ reg.type_name ~ ", crate::reg::" ~ reg.access ~ ">::from_ptr(self.ptr.byte_add(" ~ reg.array.addr_offset ~ ") as _) }"  %}
        {{ macros.loop(0, reg.array.dims, expr) | indent(8) }}

        // SAFETY: All elements have been initialized above
        unsafe { core::mem::transmute(array) }
    }
    {% endif %}

{% endfor %}
}
