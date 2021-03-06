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

Test your regexes [here](https://regex101.com/).

## Question 10

Welcome to the final question!

We are going to build an app that allows you to plot the columns of a file!

The final result looks like this:

![image-title-here](/assets/img/python-quiz-explore-timeseries.png)

We start with a few imports.

Make sure to install the same exact versions as me.

We also initialize a new [dash](https://dash.gallery/Portal/) app.

{% highlight python %}

from pathlib import Path
from functools import lru_cache
import csv

import pandas as pd
from dash import Dash # pip install dash==1.16.2
import dash_core_components as dcc # pip install dash-core-components==1.12.1
import dash_html_components as html # pip install dash-html-components==1.1.1
import dash_bootstrap_components as dbc # pip install dash-bootstrap-components==0.11.3
from dash.dependencies import Input, Output
from dash.exceptions import PreventUpdate
import plotly.graph_objects as go # pip install plotly==4.11.0

app = Dash(__name__, external_stylesheets=[dbc.themes.BOOTSTRAP, '/assets/bootstrap.css'])
{% endhighlight %}

Next we define the user interface which has 4 rows.

The first row holds one column with the file path.

The second row holds one column with a multi-select dropdown that allows us to choose which columns to plot on the y-axis

The third row holds three columns with a dropdown for choosing the x-axis column and two input fields that allows for plotting a subset of the rows of the file.

The fourth row holds one column with the plot.

{% highlight python %}

app.layout = dbc.Container([
    dbc.Row([
        dbc.Col([
            dbc.Label('File path', html_for='file_path'),
            dbc.Input(id='file_path', persistence=True),
        ], width=12),
    ]),
    dbc.Row([
        dbc.Col([
            dbc.Label('Columns to plot', html_for='columns'),
            dcc.Dropdown(id='columns', multi=True),
        ], width=12),
    ]),
    dbc.Row([
        dbc.Col([
            dbc.Label('X column (blank will default to df.index)', html_for='x_col'),
            dcc.Dropdown(id='x_col', persistence=True),
        ], width=4),
        dbc.Col([
            dbc.Label('Row start', html_for='row_start'),
            dbc.Input(id='row_start', persistence=True),
        ], width=1),
        dbc.Col([
            dbc.Label('Row end', html_for='row_end'),
            dbc.Input(id='row_end', persistence=True),
        ], width=1),
    ]),
    dbc.Row([
        dbc.Col([
            dcc.Graph(id='output', style={'margin-top': '3px'})
        ], width=12),
    ]),
], fluid=True)
{% endhighlight %}

We define a function to read the `file_path`

Comment below on what you think the first two lines do.

The `@lru_cache(maxsize=None)` attribute caches the function so that a second call to `read_csv` with the same `file_path` doesn't need to read the file again.

{% highlight python %}
@lru_cache(maxsize=None)
def read_csv(file_path):
    with open(file_path) as file:
        sep = csv.Sniffer().sniff(file.readline()).delimiter

    return pd.read_csv(file_path, sep=sep)
{% endhighlight %}

When a new `file_path` is provided we populate the x-axis columns dropdown with the columns found in the file

{% highlight python %}
@app.callback(Output('x_col', 'options'),
              Input('file_path', 'value'))
def x_col_options(file_path):
    try:
        if file_path is None:
            raise PreventUpdate

        file_path = Path(file_path)

        if not file_path.exists():
            raise PreventUpdate

        df = read_csv(file_path)
        options = [{'label': c, 'value': c} for c in df]
        return options
    except:
        return []
{% endhighlight %}

When a new `file_path` is provided we also populate the y-axis columns dropdown with the columns found in the file

{% highlight python %}
@app.callback(Output('columns', 'options'),
              Input('file_path', 'value'))
def columns_options(file_path):
    try:
        if file_path is None:
            raise PreventUpdate

        file_path = Path(file_path)

        if not file_path.exists():
            raise PreventUpdate

        df = read_csv(file_path)
        options = [{'label': c, 'value': c} for c in df]
        return options
    except:
        return []
{% endhighlight %}

Then we plot the data according to what's selected in the other fields:

{% highlight python %}
@app.callback(Output('output', 'figure'),
              Input('file_path', 'value'),
              Input('columns', 'value'),
              Input('x_col', 'value'),
              Input('row_start', 'value'),
              Input('row_end', 'value'))
def output_figure(file_path, columns, x_col, row_start, row_end):
    if file_path is None:
        raise PreventUpdate

    file_path = Path(file_path)

    if not file_path.exists():
        raise PreventUpdate

    if row_start is None:
        row_start = 0

    df = pd.read_csv(file_path)

    if row_end is None:
        row_end = len(df)

    x = df.index
    if x_col is not None:
        x = df[x_col]

    if columns is None or columns == []:
        columns = df.columns

    figure = go.Figure()
    sub_df = df.iloc[int(row_start):int(row_end)]
    for col in columns:
        y = sub_df[col]
        trace = go.Scatter(x=x, y=y, name=col)
        figure.add_trace(trace)

    return figure
{% endhighlight %}

Finally at the very end of the file we run the app

{% highlight python %}
if __name__ == '__main__':
    app.run_server(debug=True)
{% endhighlight %}

Now I want you to copy all this code into a file and run it in a conda terminal with `python "C:\path\to\your\file.py"`

Your second task is to improve this code. There's some intentionally duplicated code across this file.

Think of a way to remove that duplication and rewrite this code.

You can paste code in the comments with &lt;pre&gt;&lt;code class="python"&gt;**your code**&lt;/code&gt;&lt;/pre&gt;

Thanks for joining me on this journey - I hope you've improved your programming skills and enjoyed doing it!

## Question 9

Let's plot!

We'll read some data using pandas and plot it with plotly.

If you don't have plotly installed, open anaconda and type `pip install plotly`

You'll need to run code from this example in your local computer - it doesn't work online.

To save the file (write_image) you'll also need kaleido: `pip install kaleido`

{% highlight python %}
import pandas as pd
import plotly.graph_objects as go

path = r'https://raw.githubusercontent.com/diogofriggo/diogofriggo.github.io/main/data/2022-06-02-python-quizz/masts.csv'
df = pd.read_csv(path)
figure = go.Figure()
trace = go.Scatter(x=df.index, y=df['M1'].iloc[:100])
figure.add_trace(trace)
figure.show()
figure.write_image(r'C:\python_quiz\my_plot.png')
{% endhighlight %}

This is the result:

![image-title-here](/assets/img/python-quiz-plotly.png)

Now your turn. I want you to plot the last 100 rows of all columns of `df`, without code duplication, on the same plot (one line for each column)

#### Answer

{% highlight python %}
import pandas as pd
import plotly.graph_objects as go

path = r'https://raw.githubusercontent.com/diogofriggo/diogofriggo.github.io/main/data/2022-06-02-python-quizz/masts.csv'
df = pd.read_csv(path, index_col='stamp')

figure = go.Figure()

sub_df = df.iloc[:100]
x = sub_df.index
for c in df:
    y = sub_df[c]
    trace = go.Scatter(x=x, y=y, name=c)
    figure.add_trace(trace)

figure.show()
{% endhighlight %}

## Question 8

Today we're going to talk about a very powerful library for manipulating Microsoft Excel files.

There are many such libraries for python, some just write, some read and write but don't support .xlsb files, and so on.

Xlwings is best-of-breed here, it has the most support. To my knowledge it's the only one that can handle .xlsb files.

Plus, their API is gorgeous and simple to use. You can even run VBA macros directly from python.

Below I'll show you how to write a pandas dataframe into an Excel sheet

{% highlight python %}
from pathlib import Path

from numpy.random import randint
import pandas as pd
import xlwings as xw

people = [
    'Leonardo Barcelos',
    'Ciro Bezerra',
    'Antonio Bonow',
    'Juliana Ko',
    'Bernardo Letsch',
    'Camila Mahon',
    'Bernardo Piccoli',
]

df = pd.DataFrame({
    'name': people,
    'random_age': [randint(5, 90) for _ in people],
})

book = xw.Book()
sheet = book.sheets['Sheet1']
sheet.range('A1').options(pd.DataFrame, index=False).value = df

folder = Path(r'C:\python_quiz')
path = folder / 'my_excel_file_written_with_xlwings.xlsx'
book.save(path)
{% endhighlight %}

I want you to familiarize yourself with this library. To this end I want you to read back the dataframe I've written.

That is, I want you to open the file `my_excel_file_written_with_xlwings.xlsx`, read the data from Sheet1 into a pandas DataFrame and print it.

If you're stuck do some searching, xlwings has great documentation online, and stackoverflow is always your friend.

Good luck!

**IMPORTANT**:
* You have to run this code on your local computer, it won't work online. Ask me for help if you can't run python code on your computer.
* If you don't have xlwings installed, type `pip install xlwings` in a conda terminal

Curious: what random age did you get?

#### Answer

{% highlight python %}
from pathlib import Path

import pandas as pd
import xlwings as xw

folder = Path(r'C:\python_quiz')
path = folder / 'my_excel_file_written_with_xlwings.xlsx'
book = xw.Book(path)
sheet = book.sheets['Sheet1']
df = sheet.range('A1').options(pd.DataFrame, expand='table', index=False).value
print(df)
{% endhighlight %}

## Question 7

Date/Time data come in a myriad of formats like
* dd-mm-YYYY HH:MM:SS
* dd/mm/YY HH:MM:SS
* YYYY/mm/dd HH:MM
* ... the list is long

When reading tabular data with pandas this is addressed as shown on [Question 6](#question-6).

Let's explore a different route here to learn a bit about **regular expressions**

Regular expressions are a meta-language that matches generic aspects of text.

For instance 2022-Jun-10 10:07:00 can be matched by the regex pattern: `\d{4}-[A-Za-z]-\d{2} \d{2}:\d{2}:\d{2}`

Where `\d` represents a "digit", `[A-Za-z]` represents any upper or lower case letter between a-z

Why is this stuff useful?

Well, there are many tasks in programming that requires text validation, text replacement, text analysis.

Regex shines in these tasks. For instance a bare bones e-mail validation regex could be represented as `[A-Za-z0-9\.]@[A-Za-z0-9\.]`

Where `\.` means a "." character, we have to escape it with a backslash because it's a special regex character.

#### Answer

{% highlight python %}
df['stamp'] = df['stamp'].replace(r'(\d+)-(\d+)-(\d+) (\d+):(\d+):(\d+)', r'\3/\2/\1 \5:\6', regex=True)
{% endhighlight %}

#### Replacement

Let's look at how Python enables replacing text with regular expressions

{% highlight python %}
import re
names = ['Leonardo Barcelos', 'Ciro Bezerra', 'Antonio Bonow', 'Juliana Ko', 'Bernardo Letsch', 'Camila Mahon', 'Bernardo Piccoli']

for name in names:
    print(re.sub(r'(\w+) (\w+)', r'\2, \1', name))
{% endhighlight %}

Notice that I wrote `(\w+)` between parentheses, this means I can refer to this "group" later in the replacement string `\2, \1`, everything within the first parenthesis can be referred to by `\1` and the second by `\2`

At this point I want you to visit this [excellent website](https://regex101.com/) where you can test your regexes.
Try this example there:

![image-title-here](/assets/img/python-quiz-regex-101.png)

#### Challenge

We want to fix the Date/Time information because we are not happy with 2018-02-01 00:00:00, we'd rather have 01/02/2018 00:00

So this data...

|stamp              |M0       |M1        |M2       |M3       |M4       |
|-------------------|---------|----------|---------|---------|---------|
|2018-02-01 00:00:00|6300.906 |6299.2573 |6300.5864|6301.967 |6299.5825|
|2018-02-01 00:10:00|         |          |         |         |         |
|2018-02-01 00:20:00|839.6042 |826.12946 |819.9327 |822.97186|838.7465 |
|...                |...      |...       |...      |...      |...      |
|2019-01-14 04:50:00|5840.9204|5822.8438 |5876.861 |5816.889 |         |
|2019-01-14 05:00:00|         |          |         |         |         |
|2019-01-14 05:10:00|3806.922 |3632.9902 |3773.146|3932.7952 |3593.6494|

Becomes:

|stamp           |M0       |M1        |M2       |M3       |M4       |
|----------------|---------|----------|---------|---------|---------|
|01/02/2018 00:00|6300.906 |6299.2573 |6300.5864|6301.967 |6299.5825|
|01/02/2018 00:10|         |          |         |         |         |
|01/02/2018 00:20|839.6042 |826.12946 |819.9327 |822.97186|838.7465 |
|...             |...      |...       |...      |...      |...      |
|14/01/2019 04:50|5840.9204|5822.8438 |5876.861 |5816.889 |         |
|14/01/2019 05:00|         |          |         |         |         |
|14/01/2019 05:10|3806.922 |3632.9902 |3773.146|3932.7952 |3593.6494|

How would you do it?

The file can be downloaded from [here](https://raw.githubusercontent.com/diogofriggo/diogofriggo.github.io/main/data/2022-06-02-python-quizz/masts.csv)

Now try without python. Use TextPad!

![image-title-here](/assets/img/python-quiz-text-pad.png)

#### Answer

{% highlight python %}
df['stamp'] = df['stamp'].replace(r'(\d+)-(\d+)-(\d+) (\d+):(\d+):(\d+)', r'\3/\2/\1 \5:\6', regex=True)
{% endhighlight %}

In textpad the same thing can be accomplished as so:

![image-title-here](/assets/img/python-quiz-text-pad-answer.png)

## Question 6

We are frequently interested in computing monthly averages.
So given the following data

|stamp              |M0       |M1        |M2       |M3       |M4       |
|-------------------|---------|----------|---------|---------|---------|
|2018-02-01 00:00:00|6300.906 |6299.2573 |6300.5864|6301.967 |6299.5825|
|2018-02-01 00:10:00|         |          |         |         |         |
|2018-02-01 00:20:00|839.6042 |826.12946 |819.9327 |822.97186|838.7465 |
|...                |...      |...       |...      |...      |...      |
|2019-01-14 04:50:00|5840.9204|5822.8438 |5876.861 |5816.889 |         |
|2019-01-14 05:00:00|         |          |         |         |         |
|2019-01-14 05:10:00|3806.922 |3632.9902 |3773.146|3932.7952 |3593.6494|

Which can be read with

{% highlight python %}
from pathlib import Path

import pandas as pd

path = r'https://raw.githubusercontent.com/diogofriggo/diogofriggo.github.io/main/data/2022-06-02-python-quizz/masts.csv'
df = pd.read_csv(path, index_col='stamp')
df.index = pd.to_datetime(df.index, format='%Y-%m-%d %H:%M:%S')
...
{% endhighlight %}

Notice that we explicitly converted the date column using a date format.
This is good practice because relying on automatic conversion is often the cause of subtle bugs.

How would you aggregate this data and compute the average monthly value for each column?

The following result is expected

|stamp     |M0         |M1         |M2         |M3         |M4         |
|----------|-----------|-----------|-----------|-----------|-----------|
|2018-02-28|3182.790672|3155.896489|3166.897374|2969.798941|3108.065888|
|2018-03-31|3202.755958|3154.444042|3145.965632|2972.840670|3118.956994|
|2018-04-30|3132.793158|3087.355815|3106.374364|2924.153425|3059.461858|
|2018-05-31|3145.004857|3088.831630|3129.638827|2931.660587|3052.573197|
|2018-06-30|3220.298398|3156.571907|3166.012008|2982.580528|3109.262138|
|2018-07-31|3167.404939|3137.609406|3151.203130|2936.418509|3099.439790|
|2018-08-31|3244.117772|3226.308685|3226.393634|3036.134585|3180.053468|
|2018-09-30|3224.100170|3176.540375|3186.701141|2978.785305|3130.259802|
|2018-10-31|3213.069693|3179.801546|3177.254638|2926.380334|3101.536622|
|2018-11-30|3110.279586|3087.364842|3068.246916|2855.224892|2998.917573|
|2018-12-31|3205.605510|3154.213606|3165.648948|2967.999845|3124.375335|
|2019-01-31|3085.073091|3055.162951|3083.339377|2894.613425|2995.476110|

#### Answer

{% highlight python %}
df.groupby(pd.Grouper(freq='M')).mean()
{% endhighlight %}

Notice that `df.resample` generally does the same thing but is slightly different in that it tries to fill gaps in the series.

And it's is much slower than the `pd.Grouper` approach.

## Question 5

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

#### Answer

Well done! And nice catch on the duplicate indices.

{% highlight python %}
# only works locally (doesn't work on the websites)
folder = 'https://raw.githubusercontent.com/diogofriggo/diogofriggo.github.io/main/data/2022-06-02-python-quizz'
dfs = [pd.read_csv(path) for path in folder.rglob(*.csv)]
df = pd.concat(dfs, ignore_index=True)
{% endhighlight %}

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

#### Answer

{% highlight python %}
from pathlib import Path

folder = Path(r'C:\project')

for path in folder.rglob('*.csv'):
    print(path)

{% endhighlight %}

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