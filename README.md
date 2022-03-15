# ProjInfo

Language stats for your projects.

<div align="center">
  <img src="https://raw.githubusercontent.com/dk949/ProjInfo/trunk/src/res/screenshot_0.png" width=80% alt="projinfo of projinfo" />
</div>

## Build and install

Requires the rust tool chain and make. Requires pandoc to build the manpage.
Set the `NOMAN` environment variable variable to build without the manpage.

``` sh
make options # check if install, manpage and config directories are set right
make         # or NOMAN=1 make
sudo make install
```

## Platforms

* Tested on mac and linux, may work on other platforms.
* On windows `langs.json` is expected to be found in
  `C:\Users\USERNAME\AppData\Roaming\projinfo`.
