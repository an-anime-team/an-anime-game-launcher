#!/bin/bash

cd "$(dirname "$0")"

linuxdeploy="linuxdeploy-x86_64.AppImage"
appimagetool="appimagetool-x86_64.AppImage"

icon="../../assets/images/icon.png"
release_bin="../../target/release/anime-game-launcher"

version=$(awk '/^version = "(.+)"$/{print substr($3, 2, length($3) - 2)}' '../../Cargo.toml')

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

cp $release_bin "dist/anime-game-launcher"

echo "Executing LinuxDeploy..."

./$linuxdeploy --appdir dist -d anime-game-launcher.desktop --custom-apprun run.sh -i $icon -o appimage

echo "Executing AppImageTool..."

VERSION=$version ./$appimagetool dist

rm -rf dist
rm -f An_Anime_Game_Launcher_GTK-x86_64.AppImage

cp An_Anime_Game_Launcher_GTK-$version-x86_64.AppImage ../builds/an-anime-game-launcher-gtk-$version.AppImage
