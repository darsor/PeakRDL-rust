{% for component in components %}
pub mod {{component}};
{% endfor %}
