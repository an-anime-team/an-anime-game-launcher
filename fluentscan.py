#!/usr/bin/python

# Author: @xstraok
# Modified by Observer KRypt0n_ <https://github.com/krypt0nn>

import os
import sys
import glob

valid_args=["diff","unused"]

if len(sys.argv)<3 or len(sys.argv) == 0:
    print("missing arguments")
    sys.exit()

if sys.argv[2] not in valid_args:
    print(f"invalid argument:{sys.argv[2]}")
    sys.exit()

path = "assets/locales/" + sys.argv[1] + "/"

for filename in os.listdir("assets/locales/en"):
    with open(os.path.join("assets/locales/en", filename), 'r') as locale_file:
        created_locale = open(path + filename)

        def to_dict(text):
            result={}

            for i in text:
                if " = " in i:
                    try:
                        result[i.split()[0]] = ' '.join(i.split()[2:])

                    except:
                        pass

                elif i:
                    result[list(result.keys())[-1]] += i

            return result

        def dict_compare(d1, d2):
            d1_keys = set(d1.keys())
            d2_keys = set(d2.keys())

            shared_keys = d1_keys.intersection(d2_keys)

            added = d1_keys - d2_keys
            removed = d2_keys - d1_keys

            modified = {o : (d1[o], d2[o]) for o in shared_keys if d1[o] != d2[o]}

            same = set(o for o in shared_keys if d1[o] == d2[o])

            return added, removed, modified, same

        expected=to_dict(locale_file)
        expected2=to_dict(created_locale)

        # TODO: why modified is not used?
        added, removed, modified, same = dict_compare(expected, expected2)
        if sys.argv[2] == "unused" or sys.argv[2] == "missing":
            files = glob.glob("src/" + '/**/*.rs', recursive=True) 
            used=[]
            for i in files:
                with open(i,"r") as script:
                    text=script.read()
                    if sys.argv[2] == "unused":
                        for j in expected:
                            if '"'+j+'"' in text:
                                used.append(j)
                    #elif sys.argv[2] == "missing":
                        #for j in text.split():
                            #find all cases of tr(
                    
            for i in expected:
                if i not in used:
                    print(f"{i} is not used ({locale_file.name})")

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
    

