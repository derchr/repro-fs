First run `cargo build`
then run the scripts in this order:

```
./create-partition-table.sh
./flash-partition-table.sh
./flash-storage.sh
./flash-app.sh
```

In the output, the readme.txt will be truncated (first 3 letters/bytes missing)

