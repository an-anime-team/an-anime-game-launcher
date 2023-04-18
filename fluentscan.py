#!/usr/bin/python

# Author: @xstraok
# Modified by Observer KRypt0n_ <https://github.com/krypt0nn>

import os
import sys
import glob
import re

valid_args=["diff","unused","missing"]

if len(sys.argv)<3 or len(sys.argv) == 0:
    print("missing arguments")
    sys.exit()

if sys.argv[2] not in valid_args:
    print(f"invalid argument:{sys.argv[2]}\n Valid arguments: {valid_args}")
    sys.exit()

path = "assets/locales/" + sys.argv[1] + "/"
try:
    x=open(path+"/main.ftl","r")
    x.close()
except:
    print(f"{path} does not exist")
    sys.exit()

all_entries={}

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

def get_line_num(text,pattern):
    line=1
    for i in text.split("\n"):
        if pattern in i:
            return line
        line += 1

for filename in os.listdir("assets/locales/en"):
    with open(os.path.join("assets/locales/en", filename), 'r') as locale_file:
        created_locale = open(path + filename)

        expected=to_dict(locale_file)
        all_entries.update(expected)
        expected2=to_dict(created_locale)

        added, removed, same = dict_compare(expected, expected2)
        if sys.argv[2] == "unused" or sys.argv[2] == "missing":
            files = glob.glob("src/" + '/**/*.rs', recursive=True) 
            used=[]
            vars={}
            for i in files:
                with open(i,"r") as script:
                    text=script.read()
                    if sys.argv[2] == "unused":
                        for j in expected:
                            if '"'+j+'"' in text:
                                used.append(j)
                    elif sys.argv[2] == "missing":
                        for j in text.split():
                            # TODO: ignore comments
                            if 'tr("' in j:
                                index=j.find('tr("')
                                var_name=re.sub('[^\\w-]+', '',
                                            j[index:].replace('tr("','')
                                            .replace("Some",""))
                                # TODO: index multiple matches
                                vars[var_name] = [script.name, get_line_num(text,var_name)]

            if sys.argv[2] == "unused":
                for i in expected:
                    if i not in used:
                        print(f"[{locale_file.name}]\n"
                              "  [Unused]\n"
                              f"    {i}")


            continue

        if (added or removed or same) and sys.argv[2] == "diff":
            print(f"[{created_locale.name[15:]}]")

            if added:
                print("  [Added]")

                for i in added:
                    print(f"    {i} = {expected[i]}")

            if removed:
                print("  [Removed]")

                for i in removed:
                    print(f"    {i} = {expected2[i]}")

            #workaround
            if same and same != "set()":
                print("  [Untranslated]")

                for i in same:
                    print(f"    {i} = {expected[i]}")

            print("")

if sys.argv[2] == "missing":
    added, removed, same = dict_compare(vars, all_entries)
    if not added:
        print("nothing is missing")
    for i in added:
        print(f"[{vars[i][0]}, line {vars[i][1]}]\n"
              "  [Missing]\n"
              f"    {i}")
