# Font Helper

**Font Helper for Figma for Linux OS.**

<!-- [![Codacy Badge](https://api.codacy.com/project/badge/Grade/476feb557cf447e38f9c94b6944366f7)](https://app.codacy.com/app/ChugunovRoman/figma-linux-font-helper?utm_source=github.com&utm_medium=referral&utm_content=ChugunovRoman/figma-linux-font-helper&utm_campaign=Badge_Grade_Dashboard)
[![Travis CI](https://travis-ci.org/ChugunovRoman/figma-linux-font-helper.svg?branch=master)](https://travis-ci.org/ChugunovRoman/figma-linux-font-helper) -->
<!-- <span class="badge-buymeacoffee"><a href="https://www.buymeacoffee.com/U5hnMuASy" title="Donate to this project using Buy Me A Coffee"><img src="https://img.shields.io/badge/buy%20me%20a%20coffee-donate-yellow.svg" alt="Buy Me A Coffee donate button" /></a></span>
<span class="badge-paypal"><a href="https://www.paypal.com/cgi-bin/webscr?cmd=_s-xclick&hosted_button_id=4DNBUKPV6FBCY&source=url" title="Donate to this project using Paypal"><img src="https://img.shields.io/badge/paypal-donate-yellow.svg" alt="PayPal donate button" /></a></span> -->

## Attention!
Figma has now made a change to their web application so that the request for local fonts will not be made if your browser agent defined like the Linux version.

## Install

Run this in your terminal

<!-- curl https://raw.githubusercontent.com/ChugunovRoman/figma-linux-font-helper/master/res/install.sh | sudo bash -->
<!-- removed tinyurl because piping to bash isn't the best idea by itself. The least we can do is to not add a third-party redirect in between -->
```bash
curl -L https://raw.githubusercontent.com/Figma-Linux/figma-linux-font-helper/master/res/install.sh | bash
```
## How it works

Font Helper is a simple http server which responds with a list of locally installed fonts upon request from Figma client. Figma sends an HTTP request to `localhost` upon loading to see which local fonts are available for usage in the Figma client 

## How to add directories

You can modify Font Helper config file with normal user privileges.
The config is a simple JSON file which is located in `~/.config/figma-linux/settings.json`
```bash
nano ~/.config/figma-linux/settings.json
```

After that You need to restart Font Helper service to apply the changes:
```bash
systemctl --user restart figma-fonthelper.service
```

## Where to find logs

The app put all logs into the `/var/log/fonthelper` directory.
When Font Helper starts it create a new log file in the directory each time.

## Check Font Helper service status

Just type this in terminal for check status of Font Helper:
```bash
systemctl --user status figma-fonthelper.service
```

## Updating Font Helper

Font Helper updates automatically via `figma-fonthelper-updater.service` service.
You don't need to update Font Helper manually by default unless you have disabled the said service


## How to disable auto updates

Disable the updater systemd service:
```bash
systemctl --user stop figma-fonthelper-updater.service
systemctl --user disable ffigma-onthelper-updater.service
```

<br>

To enable auto update again:
```bash
systemctl --user start figma-fonthelper-updater.service
systemctl --user enable figma-fonthelper-updater.service
```

## Check your currently installed Font Helper version

In terminal:
```bash
~/.local/share/figma-fonthelper/fonthelper -v
```
