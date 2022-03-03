import os
import re

# Switch to the tools directory
this_file_path = os.path.dirname(__file__)
os.chdir(this_file_path)

# Gather all lints
lints = []
for root, dirs, files in os.walk('../src/lints/'):
    for file_name in files:
        lint_file = open(os.path.join(root, file_name)).read()
        lint_name = re.search(r'impl Lint for (\w+)', lint_file).group(1)
        lint_tag = re.search(
            r'fn tag\(\) -> &\'static str \{\n\s+(.+)', lint_file).group(1)
        lint_level = re.search(
            'fn default_level\(\) -> LintLevel \{\n\s+(.+)', lint_file).group(1)
        explanation = re.search(
            'fn explanation\(\) -> &\'static str \{\n\s+"(.+)"', lint_file).group(1)
        lints.append({
            'name': lint_name,
            'file_name':  file_name.replace('.rs', ''),
            'tag': lint_tag,
            'level': lint_level,
            'explanation': explanation,
            'visits_expression_early': 'impl EarlyExpressionPass' in lint_file,
            'visits_statement_early': 'impl EarlyStatementPass' in lint_file,
            'visits_expression_late': 'impl LateExpressionPass' in lint_file,
            'visits_statement_late': 'impl LateStatementPass' in lint_file,
        })

# Sort them alphabetically
lints = sorted(lints, key=lambda i: i['name'])

# Update the README...
readme = open('../README.md', 'r').read()
counter = re.search(r'Currently supports \[\d+ lints\]', readme).group(0)
readme = readme.replace(
    counter, 'Currently supports [{lint_count} lints]'.format(lint_count=len(lints)))
open('../README.md', 'w').write(readme)
print("Finished updating the README.md!")


# Update the LINTS.md...
lints_md = open('../LINTS.md', 'r').read()
body = re.search(r'\|---\|---\|---\|(?:\n|.)+', lints_md).group(0)
new_body = '|---|---|---|\n'
for lint in lints:
    new_body += '| {tag} | {level} | {explanation}\n'.format(
        tag=lint['tag'].replace('"', ''), level=lint['level'], explanation=lint['explanation'])
lints_md = lints_md.replace(body, new_body)
open('../LINTS.md', 'w').write(lints_md)
print("Finished updating LINTS.md!")


# Declare everything in the mod's file
new_mods = '#![allow(missing_docs)]\n'
for lint in lints:
    new_mods += 'mod {file_name};\n'.format(file_name=lint['file_name'])
    new_mods += 'pub use {file_name}::{lint};\n'.format(
        file_name=lint['file_name'], lint=lint['name'])
with open('../src/lints.rs', 'w') as f:
    f.write(new_mods)

# Gather the old calls
duck_operation = open('../src/core/duck_operation.rs', "r").read()

opreations = [
    {
        'name': 'early expression',
        'tag': 'visits_expression_early',
        'function_name': 'run_early_lint_on_expression',
        'args': 'config, expression, span, reports'
    },
    {
        'name': 'early statement',
        'tag': 'visits_statement_early',
        'function_name': 'run_early_lint_on_statement',
        'args': 'config, statement, span, reports'
    },
    {
        'name': 'late expression',
        'tag': 'visits_expression_late',
        'function_name': 'run_late_lint_on_expression',
        'args': 'config, expression, environment, span, reports'
    },
    {
        'name': 'late statement',
        'tag': 'visits_statement_late',
        'function_name': 'run_late_lint_on_statement',
        'args': 'config, statement, environment, span, reports'
    }
]

for operation in opreations:
    name = operation['name']
    tag = operation['tag']
    function_name = operation['function_name']
    args = operation['args']

    search = re.search(
        r'( +)// @{name} calls.+\n((\n|.)+?) +// @end {name} calls.+'.format(name=name), duck_operation)
    tabs = search.group(1)
    old_call = search.group(2)

    # Print what we'll be adding...
    old_lints = []
    for line in old_call.splitlines():
        search = re.search(function_name + r'::<(\w+)>', line)
        if search != None:
            lint = search.group(1)
            old_lints.append(lint)
            if not any(d['name'] == lint for d in lints if d[tag]):
                print(
                    "Removing '{lint}' from the {name} call...".format(lint=lint, name=name))
    for lint in lints:
        if lint[tag] and not lint['name'] in old_lints:
            print("Adding '{lint}' to the {name} call...".format(
                lint=lint['name'],
                name=name,
            ))

    # Make the new calls
    new_call = ''
    for lint in lints:
        if lint[tag]:
            new_call += '{tabs}Self::{function_name}::<{lint}>({args});\n'.format(
                tabs=tabs,
                function_name=function_name,
                lint=lint['name'],
                args=args,
            )

    # Replace the calls in the file
    duck_operation = duck_operation.replace(old_call, new_call)

# Flush to the file
open('../src/core/duck_operation.rs', 'w').write(duck_operation)

# Now update the full config template...
template = open('../bin/input.rs', 'r').read()
search = re.search(
    r'( +)// @tags\n((\n|.)+?) +// @end tags'.format(), template)
tabs = search.group(1)
old_call = search.group(2)
new_call = ''
for lint in lints:
    new_call += '{tabs}({tag}.into(), {level}),\n'.format(
        tabs=tabs,
        tag=lint['tag'],
        level=lint['level']
    )
template = template.replace(old_call, new_call)

# Flush to the file
open('../bin/input.rs', 'w').write(template)
print("Finished updating lint calls!")
