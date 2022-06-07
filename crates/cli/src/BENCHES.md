# benches

| Year                     | Day          | Part 1      | Part 2       |
| ------------------------ | ------------ | -----------:| ------------:|
{% for s in summaries %} | {{ s.year }} | {{ s.day }} | {{ s.benches | find_part_1_mean }} | {{ s.benches | find_part_2_mean }} | 
{% endfor %}
