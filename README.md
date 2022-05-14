# duck

![tests](https://img.shields.io/testspace/tests/imlazyeye/imlazyeye:duck/main)
![GitHub](https://img.shields.io/badge/license-MIT-green)
![experimental](https://img.shields.io/badge/stability-experimental-important)


## ⚠️ duck is not yet released and is unstable! An announcement will be made when 0.1.0 is released. ⚠️

A fast and flexible analyzer for GML ([GameMaker Language](https://manual.yoyogames.com/#t=Content.html)).

![example of the missing_case_member lint in action](https://i.imgur.com/i3b6sH1.jpg)


duck is is a highly flexible analyzer that enables far stricter rules for GML than GameMaker itself enforces. It is able to detect code that will directly lead to errors as well as enforce styling rules -- all of which are _completely customizable_.

duck is also extremely fast. It currently can fully process a 250,000 line project in [less than half a second](#footnotes).

## Table of Contents

- [Features](#features)
  - [Type Checking](#type-checking)
  - [Lints](#lints)
  - [Customization](#customization)
- [Usage Guide](#usage-guide)
  - [Installation](#instalation)
  - [Creating a config](#creataing-a-config)
  - [Running the linter](#running-the-linter)
- [Contributing](#contributing)
- [Support and Requests](#support-and-requests)

## Features

### Type Checking

duck's most powerful feature is its ability to type-check GML without compromising on the language's flexibility. This feature is currently in development and is not enabled on releases. Developers can enable type analysis by enabling the `solve` feature. More information coming soon!

### Lints

duck comes with a variety of lints. Many are purely stylistic, such as `single_equals_comparision`, which can discourage use of `=` in comparisions instead of `==`, and `collapsable_if`, which can detect when you could combine two if statements into one.

Other lints attempt to offer more powerful analysis over your code than you are offered with GameMaker. For example, `missing_case_member` can detect if a switch statement that matches over an enum is missing a member from that type. `suspicous_constant_usage` can detect a wide variety of errors that will still compile in GameMaker.

duck currently supports [36 lints](LINTS.md). You can use `duck explain <LINT_NAME>` to learn more about each lint as you encounter them.

### Customization

While duck expresses strong opinons on the GML it reads, those opinons are entirely in your control.

#### Lint Levels

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

#### Tags

Sometimes you need to break the rules. For example, while I may want `globalvar` to be banned from my codebase, I might have one or two excpetions You can tag the specific occurance of the usage to acknowledge (and ignore) the lint.

```js
// #[allow(deprecated)]
globalvar my_globalvar;
```

Tags are a great way to enable lints on things you don't want to _fully_ ban, but want to keep a close eye on.

duck also takes note of any tag that follows the following syntax:

```js
// #[tag]
// #[tag_with_parameter(parameter)]
```

Developers can use duck as a library to fetch all expressions / statements that are tagged in the source code, opening the doors to many new tools that don't need to worry about parsing GML themselves.

## Usage

Using duck is simple. There are a few methods you can use to aqquire it:

### Instalation

To install manually, do the following:

1. Download the latest release here
2. Add duck to your PATH environment variable (optional)
   - You can pass duck a path directly when using it, but adding it to your `PATH` will be much more convenient

If you're a Rust developer, you can just run `cargo install --git https://github.com/imlazyeye/duck` .

### Creating a config

You can learn how to customize duck's behavior [here](CONFIGURATION.md).

### Running duck

To run duck, simply use the `run` command!

```
duck run
```

If you would like to run the linter on a project outside the current directory you are in, you can pass a path like so:

```
duck run --path path/to/project
```

## Contributing

duck is designed to be easily extensible and contributions are extremely welcome! Please see [Contributing](CONTRIBUTING.md) for more information.

## Support and Requests

Please [open an issue](https://github.com/imlazyeye/duck/issues) if you encounter any problems with duck, or if you have any feature requests you would like to make!

### Footnotes

- _Benchmark was run on an MacBook Pro 2021 running an M1 Max with 32 GB of memory._
