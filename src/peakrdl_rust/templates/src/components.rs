//! SystemRDL component definitions

{% for component in ctx.components %}
pub mod {{component}};
{% endfor %}
