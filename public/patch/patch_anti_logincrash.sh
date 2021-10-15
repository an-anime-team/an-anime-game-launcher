#!/usr/bin/env bash

# MacOS and *BSD do not have md5sum: use md5 instead
if [[ $(uname) == "Darwin" || $(uname) == *"BSD" ]]; then
	md5sum() {
		md5 -q $@
	}
fi

DIR=$(dirname "${BASH_SOURCE[0]}")
FILE="GenshinImpact_Data/Plugins/xlua.dll"
sum=($(md5sum $FILE))

if [ "${sum}" != "526b36c2b8a070db61428b7fe69906a3" ]; then
	# The patch might corrupt invalid/outdated files if this check is skippd.
	echo "Wrong file version or patch is already applied"
	echo "md5sum: ${sum}" && exit 1
fi


# =========== DO NOT REMOVE START ===========
if [[ -e "$DIR/$FILE" ]]; then
	# There is a good reason for this check. Do not pollute the game directory.
	echo "Please move all patch files outside the game directory prior executing."
	echo " -> See README.md for proper installation instructions" && exit 1
fi
# ===========  DO NOT REMOVE END  ===========


if ! command -v xdelta3 &>/dev/null; then
	echo "xdelta3 application is required"
	exit 1
fi

echo "[INFO]    Patch to fix a login and runtime crash"
echo ""

# ===========================================================

echo
echo "--- Applying xLua patch"
xdelta_fail() {
	mv -vf "$FILE.bak" "$FILE"
	exit 1
}

mv -f "$FILE" "$FILE.bak"
# Perform patch or restore .bak on failure
xdelta3 -d -s "$FILE.bak" "$DIR/patch_files/xlua_patch.vcdiff" "$FILE" || xdelta_fail

# Done!
echo "==> Patch applied! Enjoy the game."

exit 0
