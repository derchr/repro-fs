python3 .embuild/espressif/esp-idf-master/components/fatfs/wl_fatfsgen.py --output_file storage.img --partition_size 1048576 storage
esptool.py --chip esp32 -p /dev/ttyUSB0 -b 115200 --before=default_reset --after=hard_reset write_flash --flash_mode dio --flash_freq 40m --flash_size 4MB 0x10000 storage.img
