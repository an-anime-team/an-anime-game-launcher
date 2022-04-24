#! /bin/bash

apt update
apt install -y curl gnupg appstream file

mkdir -p ${CI_PROJECT_DIR}/ci-appimage

mv ${CI_PROJECT_DIR}/node_modules/neutralino-appimage-bundler/appimagetool ${CI_PROJECT_DIR}/ci-appimage/appimagetool.AppImage
mv ${CI_PROJECT_DIR}/node_modules/neutralino-appimage-bundler/linuxdeploy ${CI_PROJECT_DIR}/ci-appimage/linuxdeploy.AppImage

pushd ${CI_PROJECT_DIR}/ci-appimage

for FILE in $(ls *.AppImage); do
    mkdir -p ${FILE}.unpack
    mv ${FILE} ${FILE}.unpack
    pushd ${FILE}.unpack
    ./${FILE} --appimage-extract
    popd
done

find .
popd

ln -sf ${CI_PROJECT_DIR}/ci-appimage/appimagetool.AppImage.unpack/squashfs-root/AppRun ${CI_PROJECT_DIR}/node_modules/neutralino-appimage-bundler/appimagetool
ln -sf ${CI_PROJECT_DIR}/ci-appimage/linuxdeploy.AppImage.unpack/squashfs-root/AppRun  ${CI_PROJECT_DIR}/node_modules/neutralino-appimage-bundler/linuxdeploy

ls -l ${CI_PROJECT_DIR}/node_modules/neutralino-appimage-bundler/
