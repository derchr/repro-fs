First run `cargo build`
Flash once with `espflash /dev/ttyUSB0 target/xtensa-esp32-espidf/debug/repro-fs` to have a valid bootloader.
Ignore the output in espmonitor.

Then run the scripts in this order:

```
./create-partition-table.sh
./flash-partition-table.sh
./flash-storage.sh
./flash-app.sh
```

In the output (espmonitor), the readme.txt will be truncated (first 3 letters/bytes missing)

