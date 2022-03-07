# duck

![GitHub branch checks state](https://img.shields.io/github/checks-status/imlazyeye/duck/main)
![GitHub](https://img.shields.io/github/license/imlazyeye/duck)

A fast and flexible linter for GML ([GameMaker Language](https://manual.yoyogames.com/#t=Content.html)).

![example of the missing_case_member lint in action](https://i.imgur.com/i3b6sH1.jpg)

duck is is a highly flexible linter that enables far stricter rules for GML than GameMaker itself enforces. It is able to detect code that will directly lead to errors as well as enforce styling rules -- all of which are _completely customizable_.

duck is also extremely fast. It currently can fully process a 250,000 line project in [less than half a second](#footnotes).

## Table of Contents

- [Features](#features)
  - [Linting](#examples)
  - [Validating GML](#validating-gml)
  - [Customization](#customization)
- [Usage Guide](#usage-guide)
  - [Installation](#instalation)
  - [Creating a config](#creataing-a-config)
  - [Running the linter](#running-the-linter)
- [Contributing](#contributing)
- [Support and Requests](#support-and-requests)

## Features

### Linting

duck's primary purpose is to lint gml. Many lints are purely stylistic, such as `single_equals_comparision`, which can discourage use of `=` in comparisions instead of `==`, and `collapsable_if`, which can detect when you could combine two if statements into one.

Other lints attempt to offer more powerful analysis over your code than you are offered with GameMaker. For example, `missing_case_member` can detect if a switch statement that matches over an enum is missing a member from that type. `suspicous_constant_usage` can detect a wide variety of mistakes that will still compile in GameMaker.

duck currently supports [36 lints](LINTS.md). You can use `duck explain <LINT_NAME>` to learn more about each lint as you encounter them.

Even more powerful features like type analysis, scoping rules, and more will be introduced in the future. To track upcoming features, you can view [the roadmap](ROADMAP.md).

### Validating GML

duck can also be used to check for standard errors in GML, often providing more information than GameMaker normally would.

![example of duck detecting a standard gml error](https://i.imgur.com/y42cngr.jpg)

Generally speaking, duck tries to support parsing for anything that is valid GML. duck also seeks to throw an error for anything GameMaker would. Ideally, if duck passes with no errors, you should be confident that it will run in GM as well.

If you find an inconsistency with duck and the GameMaker compiler, please [submit an issue!](https://github.com/imlazyeye/duck/issues)

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

#### Lint options

Some lints come with customizable behavior. `english_flavor_violation`, for example, let's you decide between the British or American spelling of GML functions. `var_prefix_violation` let's you decide if you prefer local variables to be prefixed with an underscore (`_foo`) or with nothing at all (`foo`).

```toml
english_flavor = "american"
var_prefixes = false
```

Instructions to set up a config file can be found [here](CONFIGURATION.md).

#### Tags

Sometimes you need to break the rules. Perhaps there is a place in my codebase that I would really like to use a `globalvar` even though it is depreacted. In general though, I still don't want them to be allowed. You can tag the specific occurance of the issue to acknowledge (and ignore) the lint.

```js
// #[allow(deprecated)]
globalvar my_globalvar;
```

Tags are a great way to enable lints on things you don't want to _fully_ ban, but want to keep a close eye on.

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

### Running the linter

To run the lint, simply run the lint command!

```
duck lint
```

If you would like to run the linter on a project outside the current directory you are in, you can pass a path like so:

```
duck lint --path path/to/project
```

## Contributing

duck is designed to be easily extensible, and contributions are extremely welcome! Please see [Contributing](CONTRIBUTING.md) for more information.

## Support and Requests

Please [open an issue](https://github.com/imlazyeye/duck/issues) if you encounter any problems with duck, or if you have any feature requests you would like to make!

### Footnotes

- _Benchmark was run on an MacBook Pro 2021 running an M1 Max with 32 GB of memory._
