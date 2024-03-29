# Lithe

[![pipeline](
https://gitlab.com/grauwoelfchen/lithe/badges/trunk/pipeline.svg)](
https://gitlab.com/grauwoelfchen/lithe/commits/trunk) [![coverage](
https://gitlab.com/grauwoelfchen/lithe/badges/trunk/coverage.svg)](
https://gitlab.com/grauwoelfchen/lithe/commits/trunk) [![crate::lithe-cli](
https://img.shields.io/crates/v/lithe-cli?label=lithe-cli)](
https://crates.io/crates/lithe-cli) [![crate::lithe](
https://img.shields.io/crates/v/lithe?label=lithe)](
https://crates.io/crates/lithe) [![doc::lithe](
https://img.shields.io/docsrs/lithe/latest?color=blue)](https://docs.rs/crate/lithe)

A slim template engine by using [Pest](https://github.com/pest-parser/pest) and
written in Rust.


## Repos

* [GitLab.com](https://gitlab.com/grauwoelfchen/lithe) (+ build pipelines)
* [GitHub](https://github.com/grauwoelfchen/lithe)
* [sourcehut](https://git.sr.ht/~grauwoelfchen/lithe)
* [Codeberg](https://codeberg.org/grauwoelfchen/lithe)


## Usage

The status is still `alpha` ;)

```zsh
# input file
% cat /path/to/file.slim
doctype html
html
  head
    link rel="stylesheet" href="style.css"
  body
# no renderer yet!
% lithe /path/to/file.slim
Document {
  type: Some(DocumentType {
    dtd: DTD {
      spec: "html",
      name: "html"
    },
    name: "html",
    public_id: "",
    system_id: ""
  }),
  children: [Element {
    name: "html",
    attributes: [],
    children: [
      Element {
        name: "head",
        attributes: [],
        children: [Element {
            name: "link",
            attributes: [
              Attr {
                name: "rel",
                value: "stylesheet"
              },
              Attr {
                name: "href",
                value: "style.css"
              }],
            children: []
       }]
      },
      Element {
        name: "body",
        attributes: [],
        children: []
      }
    ]
  }]
}
```

```zsh
```


## Development

```zsh
% cargo test --lib -- --nocapture
```


## License

`Apache-2.0`

```text
Lithe
Copyright 2021-2022 Yasuhiro Яша Asaka

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```
