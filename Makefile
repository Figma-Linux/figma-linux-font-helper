# paths
DATA_DIR=$(HOME)/.local/share
CONFIG_DIR=$(HOME)/.config
APP_DATA_DIR = $(DATA_DIR)/figma-fonthelper
APP_CONFIG_DIR = $(CONFIG_DIR)/figma-linux

run:
	cargo run

build:
	cargo build --release

release:
	git tag -a v$(VER) -m "Release v$(VER)"
	git push --tags origin master

install:
	./res/install.sh

uninstall:
	systemctl --user stop figma-fonthelper.service
	systemctl --user stop figma-fonthelper-updater.service
	systemctl --user disable figma-fonthelper.service
	systemctl --user disable figma-fonthelper-updater.service

	rm -rf $(APP_DATA_DIR)

	rm -rf $(CONFIG_DIR)/systemd/user/figma-fonthelper.service
	rm -rf $(CONFIG_DIR)/systemd/user/figma-fonthelper-updater.service
