# nedots

[![builds.sr.ht status](https://builds.sr.ht/~nedia.svg)](https://builds.sr.ht/~nedia?)

<!-- [![#lines](https://img.shields.io/tokei/lines/sr.ht/~nedia/nedots.rs?label=%23lines)](https://img.shields.io/tokei/lines/sr.ht/~nedia/nedots.rs?label=%23lines) -->

[sourcehut](https://git.sr.ht/~nedia/nedots.rs) | [github](https://github.com/aidenlangley/nedots.rs) | [crates.io](https://crates.io/crates/nedots)

A smart, safe and intuitive dotfiles management tool.

Run `nedots help` / `nedots --help` / `nedots` for documentation.

If you'd like to see it in action, you can check out a [build](https://builds.sr.ht/~nedia/job/916855).
The builds run with the maximum level of verbosity, so everything it does is
logged to `stdout`. The builds showcase all of the functionality - it's not the
most tangible "demo", but it does give you a peek at what it does.

## TODO

- I'd avoid running this as `root`. I need to ensure that file operations are
  carried out by the correct user.

## `nedots.yml`

You can find the configuration file @ `$XDG_CONFIG_HOME/nedots/nedots.yml`. If
`$XDG_CONFIG_HOME` is not set, then `$HOME/.config/nedots/nedots.yml`.

When you use `nedots`, you maintain a small file that contains the address of your
remote git repository, a list of directories or files, and a list of git submodules.

| Field     | Type            | Description                                                                                                         |
| :-------- | :-------------- | :------------------------------------------------------------------------------------------------------------------ |
| remote    | `String`        | The remote `git` repository address. `https://` or `ssh` (`git@`) work, but `ssh` (read/write) should be preferred. |
| sources   | `List<String>`  | A list of directories or files that `nedots` will manage.                                                           |
| git_repos | `List<GitRepo>` | A list of `GitRepo`, see `GitRepo` model below.                                                                     |

### GitRepo

| Field  | Type     | Description                                   |
| :----- | :------- | :-------------------------------------------- |
| remote | `String` | Remote git repository.                        |
| path   | `String` | Local path of `GitRepo`, relative to `$HOME`. |

Here is a small example:

```yml
remote: git@git.sr.ht:~nedia/nedots
sources:
  - .config/bspwm
  - .profile
  - /etc/hostname
  - Wallpapers
git_repos:
  - repo:
    remote: git@git.sr.ht:~nedia/config.nvim
    path: .config/nvim
```

### sources

Operations such as `install` will copy these files from `$XDG_DATA_HOME/nedots/{source}`
to `$HOME/{source}`. If `$XDG_DATA_HOME` is not set, then `$HOME/.local/share/nedots`
is used.

Conversely, `gather` will copy files from `$HOME/{source}` to `$XDG_DATA_HOME/nedots/{source}`.

Directories will be copied recursively. Paths can be owned by you, or another, but the correct
permissions will be required at runtime to perform operations on paths that are not owned by
you - you will need to run `sudo nedots` to `install` or `gather` files in `/etc/` for example.

## Usage

So you're new to this style of storing your dotfiles but the chances are you've
got a git repository for storing your dotfiles. That's some of the work done already.

```sh
# To set `nedots` up in its default location `$XDG_DATA_HOME/nedots` or `$HOME/.local/share/nedots`.
nedots init <remote>
```

Next, you'll need to edit `.nedots.yml` - `sources` specifically, so that `nedots sync`
can collect your files. It'll then copy them to `$XDG_DATA_HOME/nedots/dots`.

```sh
# Recommend adding `--nopush` so `nedots` won't push these changes to remote.
# You can review them first. Gather is responsible for collecting your files.
nedots sync --gather --nopush
```

File structure will then look something like this:

```sh
tree .local/share/nedots/dots -a
.local/share/nedots/dots
├── etc
│   └── hostname
└── home
    └── aiden
        ├── .config
        │   └── bspwm
        │       ├── bspwmrc
        │       ├── external_rules
        │       └── floating_desktop
        ├── .profile
        └── Wallpapers
            ├── wallhaven-9dxo58.jpg
            ├── wallhaven-dp118j.png
            ├── wallhaven-l35myl.jpg
            ├── wallhaven-q2mgqd.jpg
            ├── wallhaven-v9dez8.jpg
            ├── wallhaven-vqxgql.png
            ├── wallhaven-wqpgw6.png
            ├── wallhaven-y89del.png
            └── wallhaven-zm9kpy.jpg

7 directories, 14 files

```

That's it! Now anytime you make changes to the files or folders defined in
`sources`, running `nedots sync -g/--gather` will collect the files and push
them to remote.

## Install

| OS/Method       | Command                                                                    |
| :-------------- | :------------------------------------------------------------------------- |
| Cargo           | `cargo install nedots`                                                     |
| GitHub/Releases | Download binary [here](https://github.com/aidenlangley/nedots.rs/releases) |

<!-- | Arch      | `yay -S nedots`        | -->

## Build from Source

See [rustup](https://rustup.rs/).

```sh
# rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# stable channel
rustup toolchain install stable
rustup default stable

# clone & build
git clone https://git.sr.ht/~nedia/nedots.rs
cd nedots.rs
cargo build --release

# symlink to .local/bin
ln -s target/release/nedots ~/.local/bin/nedots
```
