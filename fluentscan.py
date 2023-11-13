#!/usr/bin/python

# Author: @xstraok
# Modified by Observer KRypt0n_ <https://github.com/krypt0nn>

import os
import sys
import glob
import re
import pydoc

valid_commands = ["diff", "unused", "missing"]
if len(sys.argv) >= 2 and sys.argv[1] != "diff":
    path = "assets/locales/en/"

elif len(sys.argv) < 3:
    print(f"Command format: ./fluentscan.py [command] [locale]\nAvailable commands: {valid_commands}")
    sys.exit()

else: path = "assets/locales/" + sys.argv[2] + "/"

command = sys.argv[1]
if command not in valid_commands:
    print(f"Invalid command \"{command}\". Available commands: {valid_commands}")
    sys.exit()

try:
    open(path + "/main.ftl", "r").close()

except:
    print(f"{path} does not exist")

    sys.exit()

def dict_compare(d1, d2):
    d1_keys = set(d1.keys())
    d2_keys = set(d2.keys())

    shared_keys = d1_keys.intersection(d2_keys)

    added = d1_keys - d2_keys
    removed = d2_keys - d1_keys

    same = set(o for o in shared_keys if d1[o] == d2[o])

    return added, removed, same

def to_dict(text):
    result={}

    for i in text:
        if " =" in i:
            try:
                result[i.split()[0]] = ' '.join(i.split()[2:])

            except:
                pass

        elif i:
            result[list(result.keys())[-1]] += i

    return result

def get_line_num(file,pattern):
    line = 1

    file.seek(0)
    text = file.read()

    for i in text.split("\n"):
        if sys.argv[1] == "diff":
            if i == pattern:
                return line

        if pattern in i:
            return line

        line += 1

# format = {variable : file name}
used_entries = {}
unused_entries = {}
missing_entries = {}
# format = {variable : [file name, line number]}
script_entries = {}
all_entries = {}

# format = {variable : definition}
every_variable = {}

files = glob.glob("src/**/*.rs", recursive=True)

output=""

for filename in os.listdir("assets/locales/en"):
    with open(os.path.join("assets/locales/en", filename), 'r') as locale_file:
        created_locale = open(path + filename)

        en_variables = to_dict(locale_file)
        every_variable.update(en_variables)

        for i in en_variables:
            all_entries[i] = [filename, get_line_num(locale_file, i)]
        # resets the file read
        locale_file.seek(0)

        locale_variables = to_dict(created_locale)

        added, removed, same = dict_compare(en_variables, locale_variables)

        # perhaps theres a better way to do this
        # FIXME : output isn't sorted
        if command == "diff" and (added or removed or same):
            output += f"[{created_locale.name[15:]}]\n"
            if added:
                output += "  [Added]\n"
                for i in added:
                    output += f"    {get_line_num(locale_file,i)} | {i} = {en_variables[i].strip()}\n"

            if removed:
                output += "  [Removed]\n"
                for i in removed:
                    output += f"    {get_line_num(created_locale,i)} | {i} = {locale_variables[i].strip()}\n"

            if same:
                output += "  [Untranslated]\n"
                for i in same:
                    output += f"    {get_line_num(locale_file,i)} | {i} = {en_variables[i].strip()}\n"
            output += "\n"
            continue
        # on some cycles the code below runs, even if there are differences
        #else:
            #print(f"No differences spotted between \"en\" and \"{sys.argv[2]}\" locale")
            #sys.exit()

        # for "missing" and "unused" command
        for i in files:
            with open(i, "r") as script:
                # for unused entries
                for j in list(en_variables.keys()):
                    script.seek(0)

                    if f"\"{j}\"" in script.read():
                        used_entries[j] = script.name

                    script.seek(0)
                # forgive me for my programming war crimes, this is 6 indentations!!!!!!
                # FIXME: it only indexes the first match
                for line in script.read().split("\n"):
                    if ("tr(\"" in line) and ("#" not in line):
                        index = line.find('tr!("')
                        indexLast = line.find('")',index)
                        var_name = re.sub('[^\\w-]+', '', line[index:indexLast].replace('tr!("', '').replace("Some", ""))

                        script_entries[var_name] = [script.name, get_line_num(script, var_name)]

## results stage ##

if command == "unused":
    printed = []
    for i in all_entries:
        if i not in used_entries:
            unused_entries[i] = all_entries[i]

    for i in unused_entries:
        if all_entries[i][0] not in printed:
            printed.append(all_entries[i][0])
            output += f"[en/{all_entries[i][0]}]\n  [Unused]\n"
        output += f"    {all_entries[i][1]} | {i} = {every_variable[i].strip()}\n"

    output = re.sub("\\[en", "\n[en", output)

    if not output:
        print("Nothing is unused")
        sys.exit()

# The way "unused" and "missing" process the data is very similar, perhaps it will be possible to do this in a function instead?
elif command == "missing":
    printed = []
    for i in script_entries:
        if i not in all_entries:
            missing_entries[i] = script_entries[i][0]

    for i in missing_entries:
        if script_entries[i][0] not in printed:
            printed.append(script_entries[i][0])
            output += f"[{missing_entries[i]}]\n  [Missing]\n"
        output += f"    {script_entries[i][1]} | {i}\n"

    output = re.sub("\\[src/", "\n[src/", output)
    if not output:
        print("Nothing is missing")
        sys.exit()

pydoc.pager(output)
