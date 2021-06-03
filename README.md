# Unique Victory

## Prerequisites
- [ARCropolis 1.1.3 or higher](https://github.com/Raytwo/ARCropolis/releases/latest)
- [Skyline (Should come with ARCropolis)](https://github.com/skyline-dev/skyline/releases/tag/beta)
- [ACMD Hook (master)](https://github.com/ultimate-research/acmd_hook/releases/tag/master)
- [NRO Hook (master)](https://github.com/ultimate-research/nro-hook-plugin/releases/tag/master)

## Setup
Download the latest version of this plugin and put it in:
`sd:/atmosphere/contents/01006A800016E000/romfs/skyline/plugins`

## Usage
Create a folder called `VictoryStage` in the following path:
`sd:/atmosphere/contents/01006A800016E000/romfs/`

Inside that folder, create a config file called `config.toml` and put the following information in it:
```toml
[<Character Code Name>]
default="<Folder in VictoryStage>"
c0X="<Another Folder in VictoryStage>" # X can be a number from 0 to 7, and there can be multiple (for ex, c01, c03, etc...)
```
Now you can create folders in `VictoryStage` then put your files in them. So for example, it'd look something like this:
`sd:/atmosphere/contents/01006A800016E000/romfs/VictoryStage/<Some Folder>/<arc path>`

An example of a config file:
```toml
[mario]
default="bob"
c01="whomp"
c06="castle"

[younglink]
default="younglink"
c01="bob"

[koopa]
default="bowser"
c01="younglink"

[peach]
default="castle"
c01="bob"

[sonic]
default="Brawl"
```