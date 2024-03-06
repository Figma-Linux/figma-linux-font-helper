#!/usr/bin/env bash

DATA_DIR=${XDG_DATA_HOME:-$HOME/.local/share}
CONFIG_DIR=${XDG_CONFIG_HOME:-$HOME/.config}
APP_DATA_DIR=$DATA_DIR/figma-linux

get_latest_release() {
  curl -Ls --silent "https://github.com/ChugunovRoman/figma-linux-font-helper/releases/latest" | perl -ne 'print "$1\n" if /v([0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,4})/' | head -1;
}

get_latest_release_link_download() {
  local latest=$(get_latest_release);
  echo "http://github.com/ChugunovRoman/figma-linux-font-helper/releases/download/v${latest}/fonthelper.tar.xz"
}

have_new_version() {
  local current=$($DATA_DIR/figma-fonthelper/fonthelper -v);
  local latest=$(get_latest_release);

  if [ ! $current == $latest ]; then
    echo "Need update your version on $latest";
    download;
    install;
  else
    echo "You have latest version";
  fi

}

download() {
  local link=$(get_latest_release_link_download);

  cd /tmp;
  rm -rf ./fonthelper.tar*
  wget "$link";
}

install() {

  pushd $APP_DATA_DIR
  tar xJf /tmp/fonthelper.tar.xz ./fonthelper
  tar xJf /tmp/fonthelper.tar.xz ./updater.sh
  chmod +x ./fonthelper ./updater.sh
  popd

  pushd $CONFIG_DIR/systemd/user
  tar xJOf /tmp/fonthelper.tar.xz ./figma-fonthelper.service
  tar xJOf /tmp/fonthelper.tar.xz ./figma-fonthelper-updater.service

  chmod 644 figma-fonthelper.service
  chmod 644 figma-fonthelper-updater.service
  popd

  systemctl --user daemon-reload

  systemctl --user restart figma-fonthelper.service
  systemctl --user restart figma-fonthelper-updater.service

  systemctl --user enable figma-fonthelper.service
  systemctl --user enable figma-fonthelper-updater.service

  rm -rf ./fonthelper.tar*
}

main() {
  have_new_version;
}

while true; do
  main;
  sleep 360;
done
