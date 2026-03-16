{% import 'components/macros.jinja2' as macros %}
//! {{ctx.module_comment}}

{{macros.includes(ctx)}}

{{ctx.comment}}
#[derive(Eq, PartialEq)]
{% set struct_name = ctx.type_name|kw_filter %}
pub struct {{struct_name}}<'io, IO = peakrdl_rust::io::PtrIO> {
    ptr: *mut {{ctx.primitive}},
    io: &'io IO,
}

unsafe impl<IO: Sync> Send for {{struct_name}}<'_, IO> {}
unsafe impl<IO: Sync> Sync for {{struct_name}}<'_, IO> {}

// manually implement Copy to ease generic bounds
// (IO does not need to be Copy)
impl<IO> Copy for {{struct_name}}<'_, IO> {}

// manually implement Clone to ease generic bounds
// (IO does not need to be Clone)
impl<IO> Clone for {{struct_name}}<'_, IO> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<IO> peakrdl_rust::mem::Memory for {{struct_name}}<'_, IO> {
    type Memwidth = {{ctx.primitive}};
    type Access = peakrdl_rust::access::{{ctx.access}};
    type Endian = peakrdl_rust::endian::{{ctx.endian}}Endian;

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

impl {{struct_name}}<'static> {
    /// # Safety
    ///
    /// The caller must guarantee that the provided address points to a
    /// hardware memory implementing this interface.
    #[inline(always)]
    #[must_use]
    pub const unsafe fn from_ptr(ptr: *mut {{ctx.primitive}}) -> Self {
        Self { ptr, io: &peakrdl_rust::io::PtrIO }
    }
}

impl<'io, IO> {{struct_name}}<'io, IO> {
    /// Size in bytes of the memory
    pub const SIZE: usize = {{"0x{:_X}".format(ctx.size)}};

    /// # Safety
    ///
    /// The caller must guarantee that the provided address points to a
    /// hardware memory implementing this interface.
    #[inline(always)]
    #[must_use]
    pub const unsafe fn from_ptr_with(ptr: *mut {{ctx.primitive}}, io: &'io IO) -> Self {
        Self { ptr, io }
    }

    #[inline(always)]
    #[must_use]
    pub const fn as_ptr(&self) -> *mut {{ctx.primitive}} {
        self.ptr
    }
}

{% if ctx.registers|length > 0 %}
// Virtual registers
impl<IO: peakrdl_rust::io::RegisterIO> {{struct_name}}<'_, IO> {
{% for reg in ctx.registers %}
    {% set reg_type_name = reg.type_name|kw_filter %}
    {{reg.comment | indent()}}
    #[inline(always)]
    #[must_use]
    {% if reg.array is none %}
    pub const fn {{reg.inst_name|kw_filter}}(&self) -> peakrdl_rust::reg::Reg<'_, {{reg_type_name}}, IO> {
        unsafe { peakrdl_rust::reg::Reg::from_ptr_with(self.ptr.wrapping_byte_add({{"0x{:_X}".format(reg.addr_offset)}}).cast(), self.io) }
    }
    {% else %}
    pub const fn {{reg.inst_name|kw_filter}}(&self) -> {{reg.array.type.format("peakrdl_rust::reg::Reg<'_, " ~ reg_type_name ~ ", IO>")}} {
        // SAFETY: We will initialize every element before using the array
        let mut array = {{reg.array.type.format("core::mem::MaybeUninit::uninit()")}};

        {% set expr = "unsafe { peakrdl_rust::reg::Reg::<'_, " ~ reg_type_name ~ ", IO>::from_ptr_with(self.ptr.wrapping_byte_add(" ~ reg.array.addr_offset ~ ").cast(), self.io) }"  %}
        {{ macros.loop(0, reg.array.dims, expr) | indent(8) }}

        // SAFETY: All elements have been initialized above
        unsafe { core::mem::transmute(array) }
    }
    {% endif %}

{% endfor %}
}
{% endif %}
