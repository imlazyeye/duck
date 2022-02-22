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
            'visits_expression': 'fn visit_expression' in lint_file,
            'visits_statement': 'fn visit_statement' in lint_file,
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

search = re.search(
    r'( +)// @expression calls.+\n((\n|.)+?) +// @end expression calls.+', duck)
expression_tabs = search.group(1)
old_expression_call = search.group(2)

search = re.search(
    r'( +)// @statement calls.+\n((\n|.)+?) +// @end statement calls.+', duck)
statement_tabs = search.group(1)
old_statement_call = search.group(2)

# Print what we'll be adding to expressions...
old_lints = []
for line in old_expression_call.splitlines():
    lint = re.search(
        r'self.try_run_lint_on_expression::<(\w+)>', line).group(1)
    old_lints.append(lint)
    if not any(d['name'] == lint for d in lints if d['visits_expression']):
        print("Removing '{lint}'...".format(lint=lint))
for lint in lints:
    if lint['visits_expression'] and not lint['name'] in old_lints:
        print("Adding '{lint}'...".format(lint=lint['name']))


# Make the new expression calls
new_expression_call = ''
for lint in lints:
    if lint['visits_expression']:
        new_expression_call += '{tabs}self.try_run_lint_on_expression::<{lint}>(expression, span, reports);\n'.format(
            tabs=expression_tabs,
            lint=lint['name']
        )


# Print what we'll be adding to statements...
old_lints = []
for line in old_statement_call.splitlines():
    lint = re.search(r'self.try_run_lint_on_statement::<(\w+)>', line).group(1)
    old_lints.append(lint)
    if not any(d['name'] == lint for d in lints if d['visits_statement']):
        print("Removing '{lint}'...".format(lint=lint))
for lint in lints:
    if lint['visits_statement'] and not lint['name'] in old_lints:
        print("Adding '{lint}'...".format(lint=lint['name']))

# Make the new statement calls
new_statement_call = ''
for lint in lints:
    if lint['visits_statement']:
        new_statement_call += '{tabs}self.try_run_lint_on_statement::<{lint}>(statement, span, reports);\n'.format(
            tabs=statement_tabs, lint=lint['name']
        )

# Replace the calls in the file
duck = duck.replace(old_expression_call, new_expression_call)
duck = duck.replace(old_statement_call, new_statement_call)
open('../src/duck.rs', 'w').write(duck)
print("Finished updating lint calls!")
