<p align="center">UltraMinifyHTML</p>
<p align="center">An HTML minifier meticulously optimised for both speed and effectiveness written in Rust then ported to Python.</p>

### Intro
Tested with Python 2.7, 3.6, 3.7 and 3.8

It uses `minify-html` Rust library: https://crates.io/crates/minify-html to do minimisation.

### Work in Progress
This library is still pretty much work in progress. Not intended for production usage yet.

### Build dev build
```bash
pip install setuptools-rust==0.10.3
python setup.py develop
```

### Build release
```bash
pip install setuptools-rust==0.10.3
python setup.py install
```

### Benchmarks
See bench directory for more info.
