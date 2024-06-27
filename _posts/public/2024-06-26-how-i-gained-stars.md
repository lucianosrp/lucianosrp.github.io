---
title: How I gained my first 30+ stars on GitHub
author: luciano
date: 2024-06-26 21:30:00 +0200
categories: [Coding , Blog]
tags: [Aviation, Github]
toc: true
---

I wanted to reflect on how I got my first 30+ stars on a GitHub repository.

## Awesome lists
What are awesome lists? 

>An awesome list is a list of awesome things curated by the community. There are awesome lists about everything from CLI applications to fantasy books. The main repository serves as a curated list of awesome lists.

I really like browsing GitHub for 'awesome lists', one of my favourites is [awesome-selfhosted](https://github.com/awesome-selfhosted/awesome-selfhosted) which contains a list of selfhosted tools and resources.

I wanted to create my own collection of free and open-source aviation project that I know of. While working in aviation, I always had those sources in mind but I never kept a written collection of it, so I am doing this also for my personal benefit. 

As of the writing of this post, my list contains almost 20 entries (mostly data sources). In the coming days I intend to expand it further with more data and tools. Ideally I would love to have a nice mix between data-only sources and custom scripts/tools. 


## I got spotted
Shortly after publishing the list, I got spotted by [Cyber Detective](https://x.com/cyb_detective), an account which specializes on Open Source Intelligence (OSINT). They shared a link of my repository on X which then got 8k views.

I was quite surprised by the rapidity at which my list (just a siple README file) got discovered. It is probably due to the 'osint' and 'awesome-list' tag that I put on GitHub.

## Growth so far

![](https://api.star-history.com/svg?repos=lucianosrp/open-source-aviation&type=Date&theme=dark){:.dark}
![](https://api.star-history.com/svg?repos=lucianosrp/open-source-aviation&type=Date){:.light}

## What's next

My list was featured in other people's awesome-list which is nice to see. To make it more "attractive" (again, it was just a README file), I decided to add a Python script that will look for any data source present in that file and append a table containing information for each data source like the one below:

| name                                                                                                                      | last_modified       | row_count   |
|:--------------------------------------------------------------------------------------------------------------------------|:--------------------|:------------|
| [airports.csv](https://raw.githubusercontent.com/davidmegginson/ourairports-data/main/airports.csv)                       | 2024-06-12 07:53:40 | 79,604      |
| [runways.csv](https://raw.githubusercontent.com/davidmegginson/ourairports-data/main/runways.csv)                         | 2024-06-12 07:53:40 | 45,908      |
| [regions.csv](https://raw.githubusercontent.com/davidmegginson/ourairports-data/main/regions.csv)                         | 2024-06-09 07:53:30 | 3,935       |
| [airport-frequencies.csv](https://raw.githubusercontent.com/davidmegginson/ourairports-data/main/airport-frequencies.csv) | 2024-06-07 07:53:29 | 29,376      |
| [airport-comments.csv](https://raw.githubusercontent.com/davidmegginson/ourairports-data/main/airport-comments.csv)       | 2024-06-01 07:53:29 | 15,442      |
| [navaids.csv](https://raw.githubusercontent.com/davidmegginson/ourairports-data/main/navaids.csv)                         | 2023-06-04 07:53:28 | 11,020      |
| [countries.csv](https://raw.githubusercontent.com/davidmegginson/ourairports-data/main/countries.csv)                     | 2022-11-03 07:53:39 | 248         |
| [airports.dat](https://raw.githubusercontent.com/jpatokal/openflights/master/data/airports.dat)                           | 2019-05-13 11:54:02 | 7,697       |
| [airports-extended.dat](https://raw.githubusercontent.com/jpatokal/openflights/master/data/airports-extended.dat)         | 2019-05-13 11:54:02 | 12,667      |
| [airlines.dat](https://raw.githubusercontent.com/jpatokal/openflights/master/data/airlines.dat)                           | 2017-02-02 11:32:12 | 6,161       |
| [routes.dat](https://raw.githubusercontent.com/jpatokal/openflights/master/data/routes.dat)                               | 2017-02-02 11:32:12 | 67,662      |



I even added a GitHub action to run the script every day, but I recently turned that off (for now) as I was not happy with the way it commits the repository. 

I haven't received any pull requests or comment about the list itself. So if you would like to contribute to it, you are more than welcome! 

Overall, I think the key to success was the simplicity of the content as well as a good series of tags to make it more 'discoverable' on GitHub. 