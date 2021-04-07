# -*- coding: utf-8 -*-
import sys
import time
import pytest
import htmlmin
import ultraminifyhtml

brit_co_frontpage_html = open('brit_co_fp.html', 'r').read().decode('utf-8')

@pytest.mark.benchmark(
    group="minify",
    min_time=0.1,
    max_time=0.5,
    min_rounds=10,
    timer=time.time,
    disable_gc=True,
    warmup=False
)
def test_ultraminifyhtml_minify(benchmark):
    @benchmark
    def parse():
        print(ultraminifyhtml.minify(brit_co_frontpage_html, minify_js=True, minify_css=True))

@pytest.mark.benchmark(
    group="minify",
    min_time=0.1,
    max_time=0.5,
    min_rounds=10,
    timer=time.time,
    disable_gc=True,
    warmup=False
)
def test_htmlmin_minify(benchmark):
    @benchmark
    def parse():
        print(htmlmin.minify(brit_co_frontpage_html))
