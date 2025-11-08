# 🛡️ Security Vulnerability Report

{% for project in projects %}
## {{ project.project_type_emoji }} {{ project.project_type_name }}: {{ project.name }}

**Organization:** {{ project.organization }}  
**Target:** `{{ project.target_file }}`

### 📊 Vulnerability Summary

| Severity         | Count                                  |
|------------------|----------------------------------------|
| 🔴 Critical      | {{ project.summary.critical }}         |
| 🟠 High          | {{ project.summary.high }}             |
| 🟡 Medium        | {{ project.summary.medium }}           |
| 🟢 Low           | {{ project.summary.low }}              |
| **Total Unique** | **{{ project.summary.unique_count }}** |

---

### 🐛 Detailed Vulnerabilities

{% for vuln in project.vulnerabilities %}
#### {{ vuln.severity_emoji }} {{ vuln.title }}

| Property     | Value                                        |
|--------------|----------------------------------------------|
| **ID**       | `{{ vuln.id }}`                              |
| **Severity** | {{ vuln.severity }}                          |
| **Package**  | `{{ vuln.package_name }}@{{ vuln.version }}` |
{%- if let Some(cvss_score) = vuln.cvss_score %}
| **CVSS Score** | {{ cvss_score }} |
{%- endif %}
| **Upgradable** | {% if vuln.is_upgradable %}✅ Yes{% else %}❌ No{% endif %} |
| **Patchable** | {% if vuln.is_patchable %}✅ Yes{% else %}❌ No{% endif %} |
{%- if vuln.from_paths.len() > 0 %}
| **Occurrences** | {{ vuln.from_paths.len() }} dependency path(s) |
{%- endif %}
{%- if vuln.cve_ids.len() > 0 %}
**CVE IDs:** 
{% for cve in vuln.cve_ids %}
- `{{ cve }}`
{% endfor %}
{% endif %}

{% if vuln.from_paths.len() > 0 %}
**Dependency Paths:**

{% for path in vuln.from_paths %}
- `{{ path }}`
{% endfor %}
{% endif %}

---

{% endfor %}

---

{% endfor %}

---

*Report generated on {{ timestamp }}*  
*Total projects analyzed: {{ projects.len() }}*