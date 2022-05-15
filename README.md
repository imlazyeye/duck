# duck

![tests](https://img.shields.io/testspace/tests/imlazyeye/imlazyeye:duck/main)
![GitHub](https://img.shields.io/badge/license-MIT-green)
![experimental](https://img.shields.io/badge/stability-experimental-important)
![gm_version](https://img.shields.io/badge/GM%20Runtime-2022.3.0.497-blue)

## ⚠️ duck is not yet released and is unstable! An announcement will be made when 0.1.0 is released.

A fast and flexible analyzer for GML ([GameMaker Language](https://manual.yoyogames.com/#t=Content.html)).

![example of the missing_case_member lint in action](https://i.imgur.com/i3b6sH1.jpg)

duck is is a highly flexible analyzer that enables far stricter rules for GML than GameMaker itself enforces. It is able to detect code that will directly lead to errors as well as enforce styling rules -- all of which are _completely customizable_.

duck is also extremely fast. It currently can fully process a 250,000 line project in [less than half a second](#footnotes).

## Table of Contents

- [Type Checking](#type-checking)
- [Lints](#lints)
- [Customization](#customization)
- [Instalation](#instalation)
- [Usage](#usage)
- [Support and Requests](#support-and-requests)

## Type Checking

duck's most powerful feature is its ability to type-check GML without compromising on the language's flexibility. This feature is currently in development and is not enabled on releases. Developers can enable type analysis by enabling the `solve` feature. More information coming soon!

## Lints

duck comes with a variety of lints that offer nuanced feedback about your code, ranging from offering stylistic feedback to encouraging better code patterns. 

duck currently supports [36 lints](LINTS.md). You can use `duck explain <LINT_NAME>` to learn more about each lint as you encounter them.

## Customization

duck can use a configuration file per-project to change how it behaves. The most basic adjustment you can make is overriding the default "level" of any lint.

```toml
[lint-levels]
and_preference = "allow"
try_catch = "warn"
missing_case_member = "deny"
```

This demonstrates the three different levels: "allow" will tell duck to fully ignore the lint, "warn" will mark them as warnings, and "deny" will treat them like full errors.

Some lints come with customizable behavior. `english_flavor_violation`, for example, let's you decide between the British or American spelling of GML functions. `var_prefix_violation` let's you decide if you prefer local variables to be prefixed with an underscore (`_foo`) or with nothing at all (`foo`).

You can read more about these customization features and how to set them up [here](CONFIGURATION.md).

### Tags

duck supports parsing for arbitrary tags in the codebase written with the following syntax:

```js
// #[tag]
// #[tag_with_parameter(parameter)]
```

Developers can use duck as a library to fetch all expressions / statements that are tagged in the source code, opening the doors to many new tools that don't need to worry about parsing GML themselves.

These allow for developers to create their own tools for gml while using duck to handle their parsing.

Additionally, duck supports `allow`, `warn` and `deny` tags to customize linting rules on a case by case basis. For example, while I may want `globalvar` to be banned from my codebase, I might have one or two excpetions You can tag the specific occurance of the usage to acknowledge (and ignore) the lint.

```js
// #[allow(deprecated)]
globalvar my_globalvar;
```

Tags are a great way to enable lints on things you don't want to _fully_ ban, but want to keep a close eye on.

## Instalation

The latest release can be found [here](https://github.com/imlazyeye/duck/releases). Rust users can also install with cargo: `cargo install duck`.

## Usage

To run duck, simply use the `run` command!

```
duck run
```

There are a few different options you can use, as well as other commands. Enter `duck help` for more information.

## Support and Requests

Please [open an issue](https://github.com/imlazyeye/duck/issues) if you encounter any problems with duck, or if you have any feature requests you would like to make!

#### Footnotes

- _Benchmark was run on an MacBook Pro 2021 running an M1 Max with 32 GB of memory._
