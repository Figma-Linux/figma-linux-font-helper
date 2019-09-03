# Font Helper

**Font Helper for Figma for Linux OS.**

[![Travis CI](https://travis-ci.org/ChugunovRoman/figma-linux-font-helper.svg?branch=master)](https://travis-ci.org/ChugunovRoman/figma-linux-font-helper)
<!-- [![codecov](https://codecov.io/gh/ChugunovRoman/figma-linux-font-helper/branch/master/graph/badge.svg)](https://codecov.io/gh/ChugunovRoman/figma-linux-font-helper) -->

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
