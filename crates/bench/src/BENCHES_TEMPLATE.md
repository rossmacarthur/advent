<!-- Generated Markdown. DO NOT EDIT. -->

# Benches

{% for year in years -%}
## {{ year.name }}

| Day | Parse | Part 1 | Part 2 |
| --- | ----: | -----: | -----: |
{% for day in year.days -%}
| **{{ day.name }}** | {{ day.parse }} | {{ day.part1 }} | {{ day.part2 }} |
{% endfor -%}
{% endfor -%}
