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
        lints.append({
            'name': lint_name,
            'file_name':  file_name.replace('.rs', ''),
            'visits_expression_early': 'impl EarlyExpressionPass' in lint_file,
            'visits_statement_early': 'impl EarlyStatementPass' in lint_file,
            'visits_expression_late': 'impl LateExpressionPass' in lint_file,
            'visits_statement_late': 'impl LateStatementPass' in lint_file,
        })

# Sort them alphabetically
lints = sorted(lints, key=lambda i: i['name'])

# Declare everything in the mod's file
new_mods = ''
for lint in lints:
    new_mods += 'mod {file_name};\n'.format(file_name=lint['file_name'])
    new_mods += 'pub use {file_name}::{lint};\n'.format(
        file_name=lint['file_name'], lint=lint['name'])
with open('../src/lints.rs', 'w') as f:
    f.write(new_mods)

# Gather the old calls
duck = open('../src/duck.rs', "r").read()

opreations = [
    {
        'name': 'early expression',
        'tag': 'visits_expression_early',
        'function_name': 'run_early_lint_on_expression',
        'args': 'expression, span, reports'
    },
    {
        'name': 'early statement',
        'tag': 'visits_statement_early',
        'function_name': 'run_early_lint_on_statement',
        'args': 'statement, span, reports'
    },
    {
        'name': 'late expression',
        'tag': 'visits_expression_late',
        'function_name': 'run_late_lint_on_expression',
        'args': 'expression, collection, span, reports'
    },
    {
        'name': 'late statement',
        'tag': 'visits_statement_late',
        'function_name': 'run_late_lint_on_statement',
        'args': 'statement, collection, span, reports'
    }
]

for operation in opreations:
    name = operation['name']
    tag = operation['tag']
    function_name = operation['function_name']
    args = operation['args']

    search = re.search(
        r'( +)// @{name} calls.+\n((\n|.)+?) +// @end {name} calls.+'.format(name=name), duck)
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
            new_call += '{tabs}self.{function_name}::<{lint}>({args});\n'.format(
                tabs=tabs,
                function_name=function_name,
                lint=lint['name'],
                args=args,
            )

    # Replace the calls in the file
    duck = duck.replace(old_call, new_call)

# Flush to the file
open('../src/duck.rs', 'w').write(duck)
print("Finished updating lint calls!")
