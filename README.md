# e-paper bootscreen
Show a simple screen on e-paper display during boot of a systemd system

# build instructions
Select one or both features to run:
* `e-paper` enables the Waveshare 2.13" e-paper display
* `emulator` uses the embedded simulator to show the screen in a window

```
cargo run --release --features emulator -- emulator
```

or

```
cargo run --release --features e-paper -- e-paper
```

For e-paper you need access to `/dev/spidev0.0`. You may need to run as root: `sudo target/release/epd-bootscreen e-paper`

# Installation

On a Raspberry Pi Zero, I've added the `epd-bootscreen.server` to `/lib/systemd/system` and enabled the service using 

```
sudo systemctl enable epd-bootscreen
```

This scripts requires an executable in `/home/pi/epd-bootscreen/` but you can change this to your liking

Please note that epd-bootscreen needs access to `/dev/spidev0.0`. It seems the earliest that this device is available is at the `sysinit` target. I'd like to find out if replacing `spidev` can change this
