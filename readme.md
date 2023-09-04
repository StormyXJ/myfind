# MyFind

## Introduction

An implementation of the bash command "find".

## Use guide
the way to use: 
```bash
    cargo run --release <target dir> <Regex> (-v)
```
    

Inorder not to be misunderstood by the shell, you are recommended to surround each `arg` with `""`.

The `"target dir"` and `\"Regex\"` can both be `one or more`.\
Multiple `targets` will increase the `finding dir`.\
Multiple `regexes` will use logic `||(or)` to match the file.\

example:
```bash
cargo run --release "/home/user_name/rust/test_file" "/home/user_name/rust/myfind" "^rust" ".rs" "^[A-Za-z]\w+" -v
```

-h --help　　　　        show the help and exit\
-v --verbose    　　  show the file, `in green`, traveled during the find

And it's worth pointing that the color of the output file with the extension of `rs` will be `blue`.

## Some details

* Two functions (find and travel_dir) are moved to `mod travel`.
* Adding `-v` or `--verbose` to the last argument can print the files visited in the process.
* You can add more than one path in one find.
* The same to regex.
* The `.rs` file will be shown in blue.

## Some examples

The following example is no "-v" option.
```bash
cargo run --release "/home/stormyx/rust/test_file" "/home/stormyx/rust/myfind" "^rust" ".rs"   
```
![Alt text](image.png)

The following example has the "-v" option. And those green one are files visited in the process.

```bash
cargo run --release "/home/stormyx/rust/test_file" "/home/stormyx/rust/myfind" "^rust" ".rs" -v  
 ```
![Alt text](image-1.png)
