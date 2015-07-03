Rust-DTL
========

[![Build Status](https://travis-ci.org/tesjin/rust-dtl.svg?branch=master)](https://travis-ci.org/tesjin/rust-dtl)
[![Coverage Status](https://coveralls.io/repos/tesjin/rust-dtl/badge.svg?branch=master)](https://coveralls.io/r/tesjin/rust-dtl?branch=master)
[![Crates.io Status](http://meritbadge.herokuapp.com/dtl)](https://crates.io/crates/dtl)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/tesjin/rust-dtl/master/LICENSE)

Rust-DTL compiles Django Template Language.

This project is inspired by ideas: <https://github.com/erlydtl/erlydtl/>

Example
-------

An Django template is a text file (e.g.: a HTML or CSS file) containing variables to control
the runtime template content, tags to control the runtime template logic and comments, which
get filtered out.

###### `views/welcome.html`
```
{% extends "layouts/main.html" %}
{% block title %}Welcome Page{% endblock %}
{% block content %}replacing the base content - variable: {{ test_var }} after variable {% endblock %}
```
###### `views/layouts/main.html`
```
<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01//EN" "http://www.w3.org/TR/html4/strict.dtd">
<html>
  <head>
    <meta http-equiv="Content-Type" content="text/html; charset=UTF-8">
    <title>MySite - {% block title %}{% endblock %}</title>								 
  </head>
  <body>
    {# TODO: add more text! #}
    <h1>{% block head %}Where my head?!{% endblock %}</h1>
    <p>Hello, {{username}}!</p>
    <p>{% block content %}Some text...{% endblock %}</p>
  </body>
</html>
```
###### `main.rs`
```
extern crate dtl;

use std::path::Path;
use std::error::Error;
use dtl::{Context, Template};

fn main() {
    let mut ctx = Context::new();
    ctx.set("username", Box::new("Ivan Ivanov".to_string()));
    ctx.set("test_var", Box::new("test-barstring".to_string()));
    let mut tpl = Template::new(Path::new("welcome.html"), Path::new("examples/views/"));
    match tpl.compile() {
        Ok(_) => {},
        Err(e) => panic!("{}", e),
    };
    println!("{}", tpl.render(&mut ctx));
}

```
###### `output`
```
<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01//EN" "http://www.w3.org/TR/html4/strict.dtd">
<html>
  <head>
    <meta http-equiv="Content-Type" content="text/html; charset=UTF-8">
    <title>MySite - Welcome Page</title>					 
  </head>
  <body>
    
    <h1>Where my head?!</h1>
    <p>Hello, Ivan Ivanov!</p>
    <p>replacing the base content - variable: test-barstring after variable some text </p>
  </body>
</html>
```

License
-------

Rust-DTL is released under the MIT license.
