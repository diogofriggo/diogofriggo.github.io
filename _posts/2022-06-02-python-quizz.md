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

You can also paste your python code on [this website](https://www.programiz.com/python-programming/online-compiler/) or [this **other** website](https://replit.com/languages/python3) and run it there to see the results.

<!-- ## Question 5

In Python the easiest way to read a file that has tabular data is with `pd.read_csv`

{% highlight python %}
import pandas as pd

df1 = pd.read_csv(r'https://raw.githubusercontent.com/diogofriggo/diogofriggo.github.io/main/data/2022-06-02-python-quizz/xyz1.csv')
df2 = pd.read_csv(r'https://raw.githubusercontent.com/diogofriggo/diogofriggo.github.io/main/data/2022-06-02-python-quizz/xyz2.csv')
df3 = pd.read_csv(r'https://raw.githubusercontent.com/diogofriggo/diogofriggo.github.io/main/data/2022-06-02-python-quizz/xyz3.csv')
{% endhighlight %}

If `df1`, `df2` and `df3` have the same columns (not necessarily in the same order), you may concatenate them with

{% highlight python %}
df = pd.concat([df1, df2, df3])
{% endhighlight %}

This code has a flaw. Answer what the flaw is and write better code.

Hint: what if you had a thousand .csv files? How would you write this code?
-->

## Question 4

There are many ways to handle file/folder paths in Python.

Python has evolved over the years. The overall best solution nowadays is to use `pathlib` (a Python standard module - it comes installed by default)

Old solutions include a mix of `glob` and `os.walkdir`. You don't need them.

Take a look at how useful pathlib is:

{% highlight python %}
from pathlib import Path

path = Path(r'C:\project\report.docx')

print(path.stem) # report
print(path.suffix) # .docx
print(path.name) # report.docx
print(path.parent) # project
print(path.exists()) # False
print(path.is_file()) # False because it doesn't exist
print(path.is_dir()) # False
print(path.parent.is_dir()) # True

path = Path('C:') / 'project' / 'report'
print(path) # C:/project/report.docx

{% endhighlight %}

If you want to try the above code out, paste it into [this **other** website](https://replit.com/languages/python3) which supports pathlib

Let's say you wanted to print the names of all csv files in a folder

Since folder access doesn't work on the online website assume we have a folder with the following files:
* C:\project\report.docx
* C:\project\report.pdf
* C:\project\file1.csv
* C:\project\file2.csv
* C:\project\file3.csv

The following code would print only

* C:\project\file1.csv
* C:\project\file2.csv
* C:\project\file3.csv

{% highlight python %}
from pathlib import Path

folder = Path(r'C:\project')

for path in folder.glob('*.csv'):
    print(path)

{% endhighlight %}

Here comes the question. Assume you have the following folders/files

* C:\project\report.docx
* C:\project\report.pdf
* C:\project\file1.csv
* C:\project\file2.csv
* C:\project\file3.csv
* C:\project\subfolder\file4.csv

How would you print all .csv files inside C:\project?

Another way to put it is this: how do you traverse folders to find files anywhere in the folder hierarchy, it may be in the root folder or in a deeply nested folder inside it.

Hint: you only need to add **one letter** to the above code.

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

df = pd.read_csv(r'https://raw.githubusercontent.com/diogofriggo/diogofriggo.github.io/main/data/2022-06-02-python-quizz/xyz.csv')
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

Here are some related operations that could help you figure out how to achieve that:

#### The length of a vector (values from the first row) may be calculated as

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

#### Answer

Despite the different answers all of you answered correctly. Well done!

My answer would be

{% highlight python %}
df['length'] = (df['X']**2 + df['Y']**2 + df['Z']**2).apply(np.sqrt)
{% endhighlight %}

I'd go with this answer because by using `apply` we can perform any operation, not just ones that exist in numpy, for instance:

{% highlight python %}

def my_custom_transform(value):
    value * 2 + 3

df['length'] = (df['X']**2 + df['Y']**2 + df['Z']**2).apply(my_custom_transform)
{% endhighlight %}

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

How do you copy to the clipboard the last 10 lines of this table? Once on your clipboard you can just paste it into excel

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

Comment your answer below! Don't forget you can paste the code into [this website](https://www.programiz.com/python-programming/online-compiler/) and experiment with it!

#### Answer

Nice work! The simplest approach was contributed by @Ciro Bezerra

{% highlight python %}
df.iloc[-10:].to_clipboard()
{% endhighlight %}

Note that 'clipboard' is the place on each individual computer that text goes when you copy something (CTRL+C).

So `to_clipboard` won't work on the [the website](https://www.programiz.com/python-programming/online-compiler/) as some of you have noticed.

## Question 1

What is the output of the following code?

{% highlight python %}
masts = ['ZA7401', 'ZA7402', 'ZA7403']
print(masts[::-1])
{% endhighlight %}

Create a disqus account and answer below with the #q1 hashtag.

Your answer will only appear publicly after the next question is posted.

#### Answer

You aced it! The answer is `['ZA7403', 'ZA7402', 'ZA7401']`

In Python you can index a list with the syntax `[start:stop:step]`.

If you don't provide `start` Python uses `0`,
if you don't provide `stop` it assumes `len(list)` (last element + 1) and the default value for `step` is `1`.

Look at the following example. Remember that **Python always start indexing at `0`.**

Also, stop is **not inclusive** as you can check below

{% highlight python %}
numbers = [1, 2, 3, 4, 5, 6, 7, 8]
# element 0: 1
# element 1: 2
# element 2: 3
# element 3: 4
# element 4: 5
# element 5: 6
# element 6: 7
# element 7: 8
print(numbers[1:6:2]) # get me elements from 1 to 6 (but not including 6) in steps of 2
{% endhighlight %}

In the above we get `element 1`, skip `element 2`, get `element 3`, skip `element 4`, get `element 5` and stop because `element 6` is not included.

The result is `[2, 4, 6]`

A `step` of `-1` is a neat trick to index in reverse order, so we'll get the last element first and then the second-to-last and so on.
When the step is negative the default values of `start` and `stop` mentioned above don't make sense and you need to know a little more Python to understand why.