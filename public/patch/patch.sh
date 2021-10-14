#!/usr/bin/env bash

# MacOS and *BSD do not have md5sum: use md5 instead
if [[ $(uname) == "Darwin" || $(uname) == *"BSD" ]]; then
	md5sum() {
		md5 -q $@
	}
fi

DIR=$(dirname "${BASH_SOURCE[0]}")
FILE="UnityPlayer.dll"
CEXE="GenshinImpact_Data/upload_crash.exe"
sum=($(md5sum $FILE))

if [ "${sum}" != "38746fe5dbdce04311c84b2394f03686" ]; then
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
	echo " -> Debian/Ubuntu: apt install xdelta3"
	echo " -> Fedora: dnf install xdelta"
	echo " -> Arch/Arch-based: pacman -S xdelta3"
	echo " -> macOS: \"port install xdelta\" or \"brew install xdelta\""
	exit 1
fi

# ===========================================================

echo
echo "--- Setting up blocked servers"

# START OF SUDO DANGER ZONE
etc_hosts="$(cat /etc/hosts)"

# See dev_tools/network.md (up-to-date as of 2.1.0)
servers=$(cat <<EOF
# Genshin logging servers (do not remove!)
0.0.0.0 log-upload-os.mihoyo.com
0.0.0.0 overseauspider.yuanshen.com

EOF
)
if [[ ! "$etc_hosts" == *"$servers"* ]]; then
	echo "[MANDATORY] Adding following logging servers to /etc/hosts"
	echo "            If you really really want to skip this (Ctrl+C),"
	echo "            PLEASE add the entries manually. Otherwise they will receive"
	echo "            logs about The Wine project, hence UNCOVERING THIS PATCH!"
	echo "$servers" | sudo -k tee -a /etc/hosts 
	if [ $? -ne 0 ]; then
		echo "$servers"
		read -p "Please append these lines to your /etc/hosts file now. Enter to continue."
	fi
else
	echo "--- Logging servers are already blocked. Skip."
fi


servers=$(cat <<EOF
# Optional Unity proxy/cdn servers
0.0.0.0 prd-lender.cdp.internal.unity3d.com
0.0.0.0 thind-prd-knob.data.ie.unity3d.com
0.0.0.0 thind-gke-usc.prd.data.corp.unity3d.com
0.0.0.0 cdp.cloud.unity3d.com
0.0.0.0 remote-config-proxy-prd.uca.cloud.unity3d.com

EOF
)
if [[ ! "$etc_hosts" == *"$servers"* ]]; then
	echo "-- Adding proxy/cdn servers"
	echo "$servers" | sudo tee -a /etc/hosts 
	if [ $? -ne 0 ]; then
		read -p "--- FAILED to add the servers. Enter to continue."
	fi
else
	echo "--- Unity proxy/cdn servers are already blocked. Skip."
fi

etc_hosts=""
# END OF SUDO DANGER ZONE

echo ""

# No crashes shall be reported!
echo "--- Renaming the crash reporter"

if [[ -e "$CEXE" ]]; then
	# Replace existing backups
	mv -f "$CEXE" "$CEXE.bak"
fi

# Registry entry to add on startup
cp -f "$DIR/patch_files/mhyprot2_running.reg" .

# Add launcher & force update to ensure the checks are performed
echo "--- Adding launcher script"
cp -f "$DIR/patch_files/launcher.bat" .

# Do the patch now, replace existing backups (hash confirmed)
echo "--- Patching UnityPlayer"
xdelta_fail() {
	mv -vf "$FILE.bak" "$FILE"
	exit 1
}

mv -f "$FILE" "$FILE.bak"
# Perform patch or restore .bak on failure
xdelta3 -d -s "$FILE.bak" "$DIR/patch_files/unityplayer_patch.vcdiff" "$FILE" || xdelta_fail


# Done!
echo "==> Patch applied! Enjoy the game."
echo
echo "[NOTICE]  Please refrain from sharing this project in public so"
echo "          that there can be Linux patches in the future. Thank you."

exit 0
