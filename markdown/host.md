# [zoe.soutter.com](/)
---

# Host

[Github repo](https://github.com/MagicPotatoBean/host)

Host is a simple, small CLI app I wrote using rust-script (explained further down) which allows you to host files over LAN (or WAN if you enable port-forwarding) so you dont need to find some paid service to do the same.

## How to use it

Using host.rs can be a little bit confusing at first, but for the most part it is fairly simple:

``` sh
    $ host.rs -a 0.0.0.0:80 -f ./file_to_share.txt /name_on_server.txt
```
"-a 0.0.0.0:80" means:
"host these files on any address, on port 80" (the standard HTTP port)

"-f ./file_to_share.txt /name_on_server.txt" means:
"host the file stored at ./file_to_share.txt, and make it accessible from http://YOUR-IP-ADDRESS/name_on_server.txt

The need for the second parameter representing name-on-server was needed as if you had two files with the same name, you simply couldnt host both at the same time without renaming them, but now, you can simply change the name-on-server.

## Rust-script

I wrote this app using [rust-script](https://discourse.nixos.org/t/nix-users-you-can-fearlessly-start-using-rust-scripts-already/35521/4) where the cargo manifest is stored inside the source file, allowing you to execute the file like a shell script (if you have the [Nix package manager/Nixos](https://nixos.org/download/) installed)
