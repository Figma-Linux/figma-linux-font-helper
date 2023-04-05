# paths
DATA_DIR=$(HOME)/.local/share
CONFIG_DIR=$(HOME)/.config
APP_DATA_DIR = $(DATA_DIR)/figma-fonthelper
APP_CONFIG_DIR = $(CONFIG_DIR)/figma-linux

run:
	cargo run

build:
	cargo build --release

archive:
	cargo build --release
	cp -rf ./target/release/font_helper ./res/fonthelper
	cd ./res
	tar cJf ../fonthelper.tar.xz ./{*.service,fonthelper*,updater.sh}

release:
	git tag -a v$(VER) -m "Release v$(VER)"
	git push --tags origin master

install:
	./res/install.sh

uninstall:
	systemctl --user stop figma-fonthelper.service || echo ""
	systemctl --user stop figma-fonthelper-updater.service || echo ""
	systemctl --user disable figma-fonthelper.service || echo ""
	systemctl --user disable figma-fonthelper-updater.service || echo ""

	rm -rf $(APP_DATA_DIR)

	rm -rf $(CONFIG_DIR)/systemd/user/figma-fonthelper.service
	rm -rf $(CONFIG_DIR)/systemd/user/figma-fonthelper-updater.service
