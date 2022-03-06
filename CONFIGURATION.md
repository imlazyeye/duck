# Configuration

duck can be customized with with a `.duck.toml` file in your project's directory.

### Creating a configuration file

To create a new configuration file, navigate in your terminal to the directory of your project and run the following command:

```
duck new-config
```

This will create a file called `.duck.toml` in your project's directory that will be used on subsequent runs of duck. Opening this file will reveal many pre-set properties, some of which you may be able to able to adjust without any instruction. Either way, a full list of the possible values your config can hold are below.

### Setting lint levels

You can additionally section called `[lint_levels]` to specify global lint levels for specific lints. You can see a working example of this [here](#lint-levels).

### Configuration options

| Property                 | Possible Values       | Explanation                                                                                                                                       |
| ------------------------ | --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| todo_keyword             | Any string            | The name of a function in your code base used to mark something as unfinished. Used by `todo`.                                                    |
| max_arguments            | Any number            | The maximum number of arguments the associated lint will allow. Used by `too_many_arguments`.                                                     |
| statement_parentheticals | true, false           | Whether or not statements should have parenthesis over their condition (ex: `if (foo)` vs `if foo`). Used by `statement_parenthetical_violation`. |
| var_prefixes             | true, false           | Whether or not local variables should be prefixed with an underscore (ex: `var _foo` vs `var foo`). Used by `var_prefix_violation`.               |
| english_flavor           | "american", "british" | The spelling of English words you prefer for GameMaker functions (ex: `color` vs `colour`). Used by `english_flavor_violation`.                   |
| length_enum_member_name  | Any string            | A name to ignore in enums that denote its length (ie: `Len`, `Count`). Used by `missing_case_member`.                                             |
| prefer_and_keyword       | true, false           | Whether or not the `and_preference` lint should require the `and` keyword or the `&&` symbol.                                                     |
| prefer_or_keyword        | true, false           | Whether or not the `or_preference` lint should require the `or` keyword or the `\|\|` symbol                                                      |
| prefer_mod_keyword       | true, false           | Whether or not the `mod_preference` lint should require the `mod` keyword or the `%` symbol.                                                      |
| prefer_not_keyword       | true, false           | Whether or not the `not_preference` lint should require the `not` keyword or the `!` symbol.                                                      |
