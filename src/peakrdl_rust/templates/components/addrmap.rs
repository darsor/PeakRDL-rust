{% import 'components/macros.jinja2' as macros %}
//! {{ctx.module_comment}}

{{macros.includes(ctx)}}

{{ctx.comment}}
#[derive(Eq, PartialEq)]
{% set struct_name = ctx.type_name|kw_filter %}
pub struct {{struct_name}}<'io, IO = peakrdl_rust::io::PtrIO> {
    ptr: *mut u8,
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

impl {{struct_name}}<'static> {
    /// # Safety
    ///
    /// The caller must guarantee that the provided address points to a
    /// hardware register block implementing this interface.
    #[inline(always)]
    #[must_use]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr.cast::<u8>(), io: &peakrdl_rust::io::PtrIO }
    }
}

impl<'io, IO> {{struct_name}}<'io, IO> {
    /// Size in bytes of the underlying memory
    pub const SIZE: usize = {{"0x{:_X}".format(ctx.size)}};

    /// # Safety
    ///
    /// The caller must guarantee that the provided address points to a
    /// hardware register block implementing this interface.
    #[inline(always)]
    #[must_use]
    pub const unsafe fn from_ptr_with(ptr: *mut (), io: &'io IO) -> Self {
        Self { ptr: ptr.cast::<u8>(), io }
    }

    #[inline(always)]
    #[must_use]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr.cast::<()>()
    }
}

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

{% for node in ctx.submaps %}
    {% set node_type_name = node.type_name|kw_filter %}
    {% set node_type_name_generics = node_type_name ~ "<'_, IO>" %}
    {{node.comment | indent()}}
    #[inline(always)]
    #[must_use]
    {% if node.array is none %}
    pub const fn {{node.inst_name|kw_filter}}(&self) -> {{node_type_name_generics}} {
        unsafe { {{node_type_name}}::from_ptr_with(self.ptr.wrapping_byte_add({{"0x{:_X}".format(node.addr_offset)}}).cast(), self.io) }
    }
    {% else %}
    pub const fn {{node.inst_name|kw_filter}}(&self) -> {{node.array.type.format(node_type_name_generics)}} {
        // SAFETY: We will initialize every element before using the array
        let mut array = {{node.array.type.format("core::mem::MaybeUninit::uninit()")}};

        {% set expr = "unsafe { " ~ node_type_name ~ "::<IO>::from_ptr_with(self.ptr.wrapping_byte_add(" ~ node.array.addr_offset ~ ").cast(), self.io) }"  %}
        {{ macros.loop(0, node.array.dims, expr) | indent(8) }}

        // SAFETY: All elements have been initialized above
        unsafe { core::mem::transmute(array) }
    }
    {% endif %}

{% endfor %}

{% for mem in ctx.memories %}
    {% set mem_type_name = mem.type_name|kw_filter %}
    {% set mem_type_name_generics = mem_type_name ~ "<'_, IO>" %}
    {{mem.comment | indent()}}
    #[inline(always)]
    #[must_use]
    {% if mem.array is none %}
    pub const fn {{mem.inst_name|kw_filter}}(&self) -> {{mem_type_name_generics}} {
        unsafe { {{mem_type_name}}::from_ptr_with(self.ptr.wrapping_byte_add({{"0x{:_X}".format(mem.addr_offset)}}).cast(), self.io) }
    }
    {% else %}
    pub const fn {{mem.inst_name|kw_filter}}(&self) -> {{mem.array.type.format(mem_type_name_generics)}} {
        // SAFETY: We will initialize every element before using the array
        let mut array = {{mem.array.type.format("core::mem::MaybeUninit::uninit()")}};

        {% set expr = "unsafe { " ~ mem_type_name ~ "::from_ptr_with(self.ptr.wrapping_byte_add(" ~ mem.array.addr_offset ~ ").cast(), self.io) }"  %}
        {{ macros.loop(0, mem.array.dims, expr) | indent(8) }}

        // SAFETY: All elements have been initialized above
        unsafe { core::mem::transmute(array) }
    }
    {% endif %}

{% endfor %}
}
