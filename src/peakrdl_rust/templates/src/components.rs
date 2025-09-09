//! SystemRDL component definitions

{% for component in components %}
pub mod {{component}};
{% endfor %}
