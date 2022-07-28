#!/bin/bash

linuxdeploy="linuxdeploy-x86_64.AppImage"
appimagetool="appimagetool-x86_64.AppImage"

version="0.3.1"

if [ ! -f $linuxdeploy ];
then
    echo "Downloading LinuxDeploy..."

    curl -s -L -o $linuxdeploy https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage

    chmod +x $linuxdeploy
fi

if [ ! -f $appimagetool ];
then
    echo "Downloading AppImageTool..."

    curl -s -L -o $appimagetool https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage

    chmod +x $appimagetool
fi

if [ -d dist ];
then
    rm -rf dist
fi

mkdir dist

cp "../../target/release/anime-game-launcher" "dist/anime-game-launcher"

echo "Executing LinuxDeploy..."

./$linuxdeploy --appdir dist -d anime-game-launcher.desktop --custom-apprun run.sh -i icon.png -o appimage

echo "Executing AppImageTool..."

VERSION=$version ./$appimagetool dist
