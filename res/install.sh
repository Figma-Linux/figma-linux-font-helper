#!/bin/bash

echo -e "\n\n"

get_latest_release() {
  curl -Ls --silent "https://github.com/ChugunovRoman/figma-linux-font-helper/releases/latest" | perl -ne 'print "$1\n" if /v([0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,4})/' | head -1;
}

get_latest_release_link_download() {
  local latest=$(get_latest_release);
  echo "http://github.com/ChugunovRoman/figma-linux-font-helper/releases/download/v${latest}/fonthelper.tar.xz"
}

download() {
  local link=$(get_latest_release_link_download);

  cd /tmp;
  rm -rf ./fonthelper.tar*
  wget "$link";
}


install() {
  DATA_DIR=${XDG_DATA_HOME:-$HOME/.local/share}
  CONFIG_DIR=${XDG_CONFIG_HOME:-$HOME/.config}
  SYSTEMD_DIR=${XDG_CONFIG_HOME:-$HOME/.config/systemd/user}
  APP_DATA_DIR=$DATA_DIR/figma-fonthelper
  APP_CONFIG_DIR=$CONFIG_DIR/figma-linux

  mkdir -p $APP_DATA_DIR
  mkdir -p $APP_CONFIG_DIR

  if [ ! -f $APP_CONFIG_DIR/settings.json ]; then
  cat > $APP_CONFIG_DIR/settings.json << EOF
{
  "host": "127.0.0.1",
  "port": "18412",
  "app": {
    "fontDirs": [
      "/usr/share/fonts",
      "$DATA_DIR/fonts"
    ]
  }
}
EOF
  fi

  pushd $APP_DATA_DIR
  tar xJf /tmp/fonthelper.tar.xz ./fonthelper
  tar xJf /tmp/fonthelper.tar.xz ./updater.sh
  chmod +x ./fonthelper ./updater.sh
  popd

  mkdir -p $SYSTEMD_DIR
  pushd $SYSTEMD_DIR
  bash -c "echo $SYSTEMD_DIR"
  bash -c "echo $DATA_DIR"
  bash -c "cd $SYSTEMD_DIR; tar xJOf /tmp/fonthelper.tar.xz ./figma-fonthelper.service | XDG_CONFIG_HOME=$DATA_DIR envsubst > figma-fonthelper.service"
  bash -c "cd $SYSTEMD_DIR; tar xJOf /tmp/fonthelper.tar.xz ./figma-fonthelper-updater.service | XDG_CONFIG_HOME=$DATA_DIR envsubst > figma-fonthelper-updater.service"
  # tar xJOf /tmp/fonthelper.tar.xz ./figma-fonthelper.service > figma-fonthelper.service.tmp
  # tar xJOf /tmp/fonthelper.tar.xz ./figma-fonthelper-updater.service > figma-fonthelper-updater.service.tmp
  # cat figma-fonthelper.service.tmp | XDG_CONFIG_HOME=$DATA_DIR envsubst > figma-fonthelper.service
  # cat figma-fonthelper-updater.service.tmp | XDG_CONFIG_HOME=$DATA_DIR envsubst > figma-fonthelper-updater.service
  # rm figma-fonthelper.service.tmp,figma-fonthelper-updater.service.tmp

  chmod 644 figma-fonthelper.service
  chmod 644 figma-fonthelper-updater.service
  popd

  systemctl --user daemon-reload

  systemctl --user start figma-fonthelper.service
  systemctl --user start figma-fonthelper-updater.service

  systemctl --user enable figma-fonthelper.service
  systemctl --user enable figma-fonthelper-updater.service

  rm -rf ./fonthelper.tar*
}

main() {
  download
  install
}

main;
