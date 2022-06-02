---
layout: post
title: Python Quiz!
date: 2022-06-02 09:30:00 -0300
categories: [python]
tags:
- python
- quiz
---

# Welcome to the daily python quiz

Every day a new question will be posted here, don't miss out!

Feel free to look up on google / stackoverflow or run the code on your computer.

You can also paste your python code on [this website](https://www.programiz.com/python-programming/online-compiler/) and run it there to see the results.

<!---
## Question 3

Imagine you have a large table with X, Y and Z columns, the data looks like this

|X      |Y      |Z               |
|-------|-------|----------------|
|358196 |2012576|109.999992084078|
|358246 |2012576|109.999992084078|
|358296 |2012576|109.999939818455|
|...    |...    |...             |
|360496 |2012576|109.999950761219|
|360546 |2012576|110.000004628085|
|360596 |2012576|109.999952210834|

You can read this data with the following code

{% highlight python %}
import pandas as pd

df = pd.read_csv(r'C:\__habits\blog\diogofriggo.github.io\data\2022-06-02-python-quizz\xyz.csv')
{% endhighlight %}

Let's say this table is too big to open in excel and that's why you chose Python

Your task is to compute the length of the vector represented by each X, Y, Z and store it in the column "length"
The result looks like this:

|X      |Y      |Z               |length            |
|-------|-------|----------------|------------------|
|358196 |2012576|109.999992084078|2044203.1558267388|
|358246 |2012576|109.999992084078|2044211.9176817257|
|358296 |2012576|109.999939818455|2044220.6807221149|
|...    |...    |...             |...               |
|360496 |2012576|109.999950761219|2044607.4278188439|
|360546 |2012576|110.000004628085|2044616.2441866691|
|360596 |2012576|109.999952210834|2044625.0617391907|

Here are some related operations that could help you understand how to achieve that:

#### The length of only the first row may be calculated as

{% highlight python %}
import numpy as np
np.sqrt(358196**2 + 2012576**2 + 109.999992084078**2) # 2044203.1558267388
{% endhighlight %}

#### The absolute difference between X and Y may be calculated as

{% highlight python %}
df['absolute_difference'] = (df['X'] - df['Y']).apply(np.abs)
{% endhighlight %}

Which results in:

|X      |Y      |Z               |absolute_difference|
|-------|-------|----------------|-------------------|
|358196 |2012576|109.999992084078|1654380            |
|358246 |2012576|109.999992084078|1654330            |
|358296 |2012576|109.999939818455|1654280            |
|...    |...    |...             |...                |
|360496 |2012576|109.999950761219|1652080            |
|360546 |2012576|110.000004628085|1652030            |
|360596 |2012576|109.999952210834|1651980            |

Given those hints how would you compute the "length" column?
-->

## Question 2

Imagine you have a very large csv file that cannot be opened in Excel and you only need the last 10 lines.

Use this sample file:

{% highlight python %}
path = r'https://raw.githubusercontent.com/diogofriggo/diogofriggo.github.io/main/data/2022-06-02-python-quizz/xyz.csv'
{% endhighlight %}

It looks like this:

|X      |Y      |Z               |
|-------|-------|----------------|
|358196 |2012576|109.999992084078|
|358246 |2012576|109.999992084078|
|358296 |2012576|109.999939818455|
|...    |...    |...             |
|360496 |2012576|109.999950761219|
|360546 |2012576|110.000004628085|
|360596 |2012576|109.999952210834|

You can read this file into a pandas dataframe (df) using this code

{% highlight python %}
import pandas as pd

df = pd.read_csv(path)
{% endhighlight %}

The first five lines of the dataframe can be displayed with this code

{% highlight python %}
print(df.iloc[:5])
{% endhighlight %}

How do you copy to the clipboard the last 10 lines of this table? (Once on your clipboard you can just paste it into excel)

The result looks like this:

|X     |Y      |Z         |
|------|-------|----------|
|360146|2012576|109.999965|
|360196|2012576|110.000009|
|360246|2012576|109.999972|
|360296|2012576|110.000036|
|360346|2012576|110.000057|
|360396|2012576|109.999886|
|360446|2012576|109.999994|
|360496|2012576|109.999951|
|360546|2012576|110.000005|
|360596|2012576|109.999952|

Comment your answer below!

## Question 1

What is the output of the following code?

{% highlight python %}
masts = ['ZA7401', 'ZA7402', 'ZA7403']
print(masts[::-1])
{% endhighlight %}

Create a disqus account and answer below with the #q1 hashtag.

Your answer will only appear publicly after the next question is posted.