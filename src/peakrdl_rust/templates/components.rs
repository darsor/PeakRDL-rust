//! SystemRDL component definitions
#![allow(non_camel_case_types)] // needed for type normalization suffixes

{% for component in ctx.components %}
pub mod {{component|kw_filter}};
{% endfor %}
