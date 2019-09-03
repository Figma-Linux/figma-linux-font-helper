# Font Helper

**Font Helper for Figma for Linux OS.**

[![Codacy Badge](https://api.codacy.com/project/badge/Grade/476feb557cf447e38f9c94b6944366f7)](https://app.codacy.com/app/ChugunovRoman/figma-linux-font-helper?utm_source=github.com&utm_medium=referral&utm_content=ChugunovRoman/figma-linux-font-helper&utm_campaign=Badge_Grade_Dashboard)
[![Travis CI](https://travis-ci.org/ChugunovRoman/figma-linux-font-helper.svg?branch=master)](https://travis-ci.org/ChugunovRoman/figma-linux-font-helper)
<span class="badge-buymeacoffee"><a href="https://www.buymeacoffee.com/U5hnMuASy" title="Donate to this project using Buy Me A Coffee"><img src="https://img.shields.io/badge/buy%20me%20a%20coffee-donate-yellow.svg" alt="Buy Me A Coffee donate button" /></a></span>
<span class="badge-paypal"><a href="https://www.paypal.com/cgi-bin/webscr?cmd=_s-xclick&hosted_button_id=4DNBUKPV6FBCY&source=url" title="Donate to this project using Paypal"><img src="https://img.shields.io/badge/paypal-donate-yellow.svg" alt="PayPal donate button" /></a></span>

## Install

Copy this in terminal

<!-- curl https://raw.githubusercontent.com/ChugunovRoman/figma-linux-font-helper/master/res/install.sh | sudo bash -->
```bash
curl -L http://tiny.cc/2np2bz | bash
```
## How it works

Font Helper is a simple http server witch get requests from figma on localhost, reads fonts and gives it to the Figma.

## Where is logs

The app put all logs into the `/var/log/fonthelper` directory.
When Font Helper starts it create a new log file in the directory each time.

## Check status of Font Helper

Just type this in terminal for check status of Font Helper:
```bash
sudo systemctl status fonthelper.service
```

## How to update Font Helper

You don't need update it manually.
Font Helper updates automatically via `fonthelper-updater.service` service.
<br>If you didn't disable it :)

## How to disable auto updates

Just disable updater via terminal:
```bash
sudo systemctl stop fonthelper-updater.service
sudo systemctl disable fonthelper-updater.service
```

<br>

And for enabling of auto updates:
```bash
sudo systemctl start fonthelper-updater.service
sudo systemctl enable fonthelper-updater.service
```

## Check of installed version

In terminal:
```bash
/opt/FontHelper/fonthelper -v
```
