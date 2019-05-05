# ckb-fake-env
### build
Now it's depend on [modified ckb](https://github.com/rink1969/ckb/tree/rc0.10).

Because it need ckb export a function.

```
cargo build
```

### run
It need only one arg.

It's the transaction hash. Without 0x prefix.

It will output some shell command and args for run.

```
$ RUST_BACKTRACE=full ./target/debug/ckb-fake-env cf3592e963d0c04e68a35841e0494026c36d1f9c010e99f9f40c0e024931b153
args for lock script:
input 0:
command need to run before:
rm -f data/cell_0
ln -s `pwd`/data/cell_0_1 `pwd`/data/cell_0
rm -f cell_field_0_0
ln -s `pwd`/data/cell_field_0_1_0 `pwd`/data/cell_field_0_0
rm -f cell_field_0_1
ln -s `pwd`/data/cell_field_0_1_1 `pwd`/data/cell_field_0_1
rm -f cell_field_0_2
ln -s `pwd`/data/cell_field_0_1_2 `pwd`/data/cell_field_0_2
rm -f cell_field_0_3
ln -s `pwd`/data/cell_field_0_1_3 `pwd`/data/cell_field_0_3
rm -f cell_field_0_4
ln -s `pwd`/data/cell_field_0_1_4 `pwd`/data/cell_field_0_4
rm -f cell_field_0_5
ln -s `pwd`/data/cell_field_0_1_5 `pwd`/data/cell_field_0_5
rm -f cell_field_0_6
ln -s `pwd`/data/cell_field_0_1_6 `pwd`/data/cell_field_0_6
rm -f data/input_field_0_0
ln -s `pwd`/data/input_field_0_1_0 `pwd`/data/input_field_0_0
rm -f data/input_field_0_1
ln -s `pwd`/data/input_field_0_1_1 `pwd`/data/input_field_0_1
args:
4a88cef22e4e71c48c40da51c1d6bd16daa97aa7 4a88cef22e4e71c48c40da51c1d6bd16daa97aa7 02ebb3bcef8da55dc9884fae227ef9784625dc81eafd46c564d1045c7fa1cd9fa3 3045022100e9d363c8f8d5d1213f350bc106612af4e332c962bc99ae388b40a1c461f7d8f002203884f41233d60886388f29e001b3db1f46b0676455f5af94b35a36fe312dfeb1
input 1:
command need to run before:
rm -f data/cell_0
ln -s `pwd`/data/cell_1_1 `pwd`/data/cell_0
rm -f cell_field_0_0
ln -s `pwd`/data/cell_field_1_1_0 `pwd`/data/cell_field_0_0
rm -f cell_field_0_1
ln -s `pwd`/data/cell_field_1_1_1 `pwd`/data/cell_field_0_1
rm -f cell_field_0_2
ln -s `pwd`/data/cell_field_1_1_2 `pwd`/data/cell_field_0_2
rm -f cell_field_0_3
ln -s `pwd`/data/cell_field_1_1_3 `pwd`/data/cell_field_0_3
rm -f cell_field_0_4
ln -s `pwd`/data/cell_field_1_1_4 `pwd`/data/cell_field_0_4
rm -f cell_field_0_5
ln -s `pwd`/data/cell_field_1_1_5 `pwd`/data/cell_field_0_5
rm -f cell_field_0_6
ln -s `pwd`/data/cell_field_1_1_6 `pwd`/data/cell_field_0_6
rm -f data/input_field_0_0
ln -s `pwd`/data/input_field_1_1_0 `pwd`/data/input_field_0_0
rm -f data/input_field_0_1
ln -s `pwd`/data/input_field_1_1_1 `pwd`/data/input_field_0_1
args:
a47f8029997fcc67aff87384daac404f39e31ceb 4a88cef22e4e71c48c40da51c1d6bd16daa97aa7 02ebb3bcef8da55dc9884fae227ef9784625dc81eafd46c564d1045c7fa1cd9fa3 3045022100e9d363c8f8d5d1213f350bc106612af4e332c962bc99ae388b40a1c461f7d8f002203884f41233d60886388f29e001b3db1f46b0676455f5af94b35a36fe312dfeb1
input 2:
command need to run before:
rm -f data/cell_0
ln -s `pwd`/data/cell_2_1 `pwd`/data/cell_0
rm -f cell_field_0_0
ln -s `pwd`/data/cell_field_2_1_0 `pwd`/data/cell_field_0_0
rm -f cell_field_0_1
ln -s `pwd`/data/cell_field_2_1_1 `pwd`/data/cell_field_0_1
rm -f cell_field_0_2
ln -s `pwd`/data/cell_field_2_1_2 `pwd`/data/cell_field_0_2
rm -f cell_field_0_3
ln -s `pwd`/data/cell_field_2_1_3 `pwd`/data/cell_field_0_3
rm -f cell_field_0_4
ln -s `pwd`/data/cell_field_2_1_4 `pwd`/data/cell_field_0_4
rm -f cell_field_0_5
ln -s `pwd`/data/cell_field_2_1_5 `pwd`/data/cell_field_0_5
rm -f cell_field_0_6
ln -s `pwd`/data/cell_field_2_1_6 `pwd`/data/cell_field_0_6
rm -f data/input_field_0_0
ln -s `pwd`/data/input_field_2_1_0 `pwd`/data/input_field_0_0
rm -f data/input_field_0_1
ln -s `pwd`/data/input_field_2_1_1 `pwd`/data/input_field_0_1
args:
96f4093cf179aaa369379402d74f70090fae11ec 4a88cef22e4e71c48c40da51c1d6bd16daa97aa7 02ebb3bcef8da55dc9884fae227ef9784625dc81eafd46c564d1045c7fa1cd9fa3 3045022100e9d363c8f8d5d1213f350bc106612af4e332c962bc99ae388b40a1c461f7d8f002203884f41233d60886388f29e001b3db1f46b0676455f5af94b35a36fe312dfeb1
args for type script:
output 0:
No type script!
```

### depend
It depend on spike and pk.

You can get docker image include spike [here](https://hub.docker.com/r/yangby0cryptape/riscv-tools).

Replace pk with [this](https://github.com/rink1969/riscv-pk/tree/ckb-pk) which add support for ckb vm syscall.

### example
See example code [here](https://github.com/rink1969/ckb-contract-examples)

Copy `data` folder to current dir.

```
# spike ./pk vote a47f8029997fcc67aff87384daac404f39e31ceb 4a88cef22e4e71c48c40da51c1d6bd16daa97aa7 02ebb3bcef8da55dc9884fae227ef9784625dc81eafd46c564d1045c7fa1cd9fa3 3045022100e9d363c8f8d5d1213f350bc106612af4e332c962bc99ae388b40a1c461f7d8f002203884f41233d60886388f29e001b3db1f46b0676455f5af94b35a36fe312dfeb1
bbl loader
debug: 02ebb3bcef8da55dc9884fae227ef9784625dc81eafd46c564d1045c7fa1cd9fa3
debug: 3045022100e9d363c8f8d5d1213f350bc106612af4e332c962bc99ae388b40a1c461f7d8f002203884f41233d60886388f29e001b3db1f46b0676455f5af94b35a36fe312dfeb1
debug: a47f8029997fcc67aff87384daac404f39e31ceb
debug: 4a88cef22e4e71c48c40da51c1d6bd16daa97aa7
debug: total 3 yes 2
```

The result and output will be same as run script in ckb.