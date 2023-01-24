see makefile

to install the loader
`yay -S riscv64-linux-gnu`
to attach with gdb:
`yay -S gdb-multiarch`

```shell
sudo gdb
target remote 127.0.0.1:6666
```
