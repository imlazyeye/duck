# duck

![GitHub branch checks state](https://img.shields.io/github/checks-status/imlazyeye/duck/main)
![GitHub](https://img.shields.io/github/license/imlazyeye/duck)

A collection of customizable lints to identify common mistakes in GML ([GameMaker Language](https://manual.yoyogames.com/#t=Content.html)).

Currently supports [36 lints](LINTS.md), with more on the way!

`duck` is is a highly flexible linter that enables far stricter rules for GML than GameMaker itself enforces. It is able to detect code that will directly lead to errors as well as enforce styling rules -- all of which are _completely customizable_.

`duck` is also extremely fast. It currently can fully process a 250,000 line project in [less than half a second](#footnotes).

## Table of Contents

- [Features](#features)
  - [Customization](#customization)
  - [Linting](#examples)
  - [Validating GML](#validating-gml)
- [Usage Guide](#usage-guide)
  - [Installation](#instalation)
  - [Creating a configuration file](#creating-a-configuration-file)
  - [Setting lint levels](#setting-lint-levels)
  - [Running the linter](#running-the-linter)
- [Contributing](#contributing)
- [Support and Requests](#support-and-requests)

## Features

### Customization

While `duck` expresses strong opinons on the GML it reads, those opinons are entirely in your control.

#### Lint Levels

`duck` can use a configuration file per-project to change how it behaves. The most basic adjustment you can make is overriding the default "level" of any lint.

```toml
[lint-levels]
and_preference = "allow"
try_catch = "warn"
missing_case_member = "deny"
```

This demonstrates the three different levels: "allow" will tell `duck` to fully ignore the lint, "warn" will mark them as warnings, and "deny" will treat them like full errors.

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

## Linting

Let's use one of `duck`'s more powerful lints as an example: `missing_case_member`.

```js
enum MyEnum {
    Foo,
    Bar,
    Buzz,
}

switch my_enum {
    case MyEnum.Foo: break;
    case MyEnum.Bar: break;
}
```

While this code is acceptable GML, it contains a danger: we do not have a `case` set up if `my_enum` is equal to `MyEnum.Buzz`. Perhaps we did not consider, `MyEnum.Buzz` when writing this code, or maybe it was implemented after this code was written.

Normally, this kind of an issue is difficult to detect. With `duck`, it's trivial:

![example of the missing_case_member lint in action](https://i.imgur.com/VPPfm9e.jpg)

As the suggestions there mention, there's a few ways we could resolve this. We could, of course, add in a case for `MyEnum.Buzz`. We could also add a `default` case to our switch -- `duck` would then recognize that all the bases are covered. We could customize that behavior further by telling `duck` to ignore this lint if we have a default case, _unless_ that default case requests the game to crash -- then `duck` will recognize that the default case is not an intended outcome.

### Validating GML

`duck` can also be used to check for standard errors in GML, often providing more information than GameMaker normally would.

![example of duck detecting a standard gml error](https://i.imgur.com/y42cngr.jpg)

Generally speaking, `duck` tries to support parsing for anything that is valid GML. `duck` also seeks to throw an error for anything GameMaker would. Ideally, if `duck` passes with no errors, you should be confident that it will run in GM as well.

If you find an inconsistency with `duck` and the GameMaker compiler, please [submit an issue!](https://github.com/imlazyeye/duck/issues)

## Usage

Using `duck` is simple. There are a few methods you can use to aqquire it:

### Instalation

To install manually, do the following:

1. Download the latest release here
2. Add `duck` to your PATH environment variable (optional)
   - You can pass `duck` a path directly when using it, but adding it to your `PATH` will be much more convenient

If you're a Rust developer, you can just run `cargo install --git https://github.com/imlazyeye/duck` .

### Running the linter

To run the lint, simply run the lint command!

```
duck lint
```

If you would like to run the linter on a project outside the current directory you are in, you can pass a path like so:

```
duck lint --path path/to/project
```

You can learn how to customize `duck`'s behavior [here](CONFIGURATION.md).

## Contributing

`duck` is designed to be easily extensible, and contributions are extremely welcome! Please see [Contributing](CONTRIBUTING.md) for more information.

## Support and Requests

Please [open an issue](https://github.com/imlazyeye/duck/issues) if you encounter any problems with `duck`, or if you have any feature requests you would like to make!

### Footnotes

- _Benchmark was run on an MacBook Pro 2021 running an M1 Max with 32 GB of memory._
