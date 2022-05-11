---
layout: page
title: Posts by Date
permalink: /postsbydate/
sitemap: false
---

<div id="index">

    {% for post in site.posts %}
        {% unless post.next %}
            <h2>{{ post.date | date: '%Y' }}</h2>
        {% else %}
            {% capture year %}{{ post.date | date: '%Y' }}{% endcapture %}
            {% capture nyear %}{{ post.next.date | date: '%Y' }}{% endcapture %}
            {% if year != nyear %}
            {% if forloop.index != 1 %}</ul>{% endif %}
                <h2>{{ post.date | date: '%Y' }}</h2>
            {% endif %}
        {% endunless %}


    {% endfor %}

</div>