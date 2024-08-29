# [zoe.soutter.com](/)
---

# Scriptify

[Github repo](https://github.com/MagicPotatoBean/scriptify)

Scriptify is a simple, small CLI app I wrote using rust-script (explained further down) which takes an entire cargo workspace and converts it into a single rust-script file, I actually wrote scriptify using a standard cargo workspace, and then used it to convert itself into a rust-script file!

## How to use it

For a workspace like the following:

``` none
    my_repo/
      ├ Cargo.toml
      ├ Cargo.lock
      └ src/
          ├ main.rs
          ├ my_lib.rs
          └ my_other_lib/
              └ mod.rs
```
You would run:
``` sh
    scriptify.rs -r my_repo -o my_repo/my_repo_script.rs
```
Then, scriptify would take each of your modules and store them in the new script file, using the mod{} block to keep namespacing identical (although I needed to then remove "mod MODULE" calls, otherwise it tries to call in an extra file which now doesn't exist).

Now, to run it, you can just run
``` sh
    my_repo/my_repo_script.rs
```
However, if this doesn't work, you may need to run
``` sh
    chmod +x my_repo/my_repo_script.rs
```
to make the file executable.

## Rust-script

I wrote this app using [rust-script](https://discourse.nixos.org/t/nix-users-you-can-fearlessly-start-using-rust-scripts-already/35521/4) where the cargo manifest is stored inside the source file, allowing you to execute the file like a shell script (if you have the [Nix package manager/Nixos](https://nixos.org/download/) installed)
