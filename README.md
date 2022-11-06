# Lithe

A slim template engine by using [Pest](https://github.com/pest-parser/pest) and
written in Rust.


## Usage

The status is still `alpha` ;)

```zsh
# input file
% cat /path/to/file.slim
doctype html
# FIXME: output
% lithe-cli /path/to/file.slim
Document { type: Some(DocumentType { name: "html", public_id: "", system_id: "" }), children: [] }
```


## Development

```zsh
% cargo test --lib -- --nocapture
```


## License

`Apache-2.0`

```text
Lithe
Copyright 2021 Yasuhiro Яша Asaka

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
