#!/bin/bash

get_latest_release_link_download() {
  curl --silent "https://api.github.com/repos/ChugunovRoman/figma-linux-font-helper/releases/latest" | grep '"browser_download_url":' | sed -E 's/.*"([^"]+)".*/\1/';
}

download() {
  local link=$(get_latest_release_link_download);

  cd /tmp;
  rm -rf ./fonthelper.tar*
  wget "$link";
}

install() {
  mkdir -p /opt/FontHelper
  mkdir -p /etc/fonthelper

  chmod 777 /etc/fonthelper -R

  cd /opt/FontHelper;
  tar xJf /tmp/fonthelper.tar.xz ./fonthelper
  tar xJf /tmp/fonthelper.tar.xz ./updater.sh
  chmod +x ./fonthelper ./updater.sh

  cd /lib/systemd/system
  tar xJf /tmp/fonthelper.tar.xz ./fonthelper.service
  tar xJf /tmp/fonthelper.tar.xz ./fonthelper-updater.service

  chmod 644 /lib/systemd/system/fonthelper.service
  chmod 644 /lib/systemd/system/fonthelper-updater.service

  systemctl daemon-reload

  systemctl start fonthelper.service
  systemctl start fonthelper-updater.service

  systemctl enable fonthelper.service
  systemctl enable fonthelper-updater.service
}

main() {
  if [[ $EUID -ne 0 ]]; then
    echo "Need run under root";
    echo "Abort";
    exit 1;
  fi

  download;
  install;
}

main;
