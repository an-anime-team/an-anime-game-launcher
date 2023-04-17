#!/usr/bin/python

# Author: @xstraok
# Modified by Observer KRypt0n_ <https://github.com/krypt0nn>

import os
import sys

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

        if added or removed or same:
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
