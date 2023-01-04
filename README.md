# nedots

[![builds.sr.ht status](https://builds.sr.ht/~nedia.svg)](https://builds.sr.ht/~nedia?)

[sourcehut](https://git.sr.ht/~nedia/nedots.rs) | [github](https://github.com/aidenlangley/nedots.rs)

A smart, safe and intuitive dotfiles management tool.

Run `nedots help` / `nedots --help` / `nedots` for documentation.

If you'd like to see it in action, you can check out a [build](https://builds.sr.ht/~nedia/job/916706).
The builds run with the maximum level of verbosity, so everything it does is
logged to `stdout`. The builds showcase all of the functionality - it's not the
most tangible "demo", but it does give you a peek at what it does.

## `.nedots.yml`

When you use `nedots`, you maintain a small file that contains the address of your
remote git repository, a list of directories or files, and a list of git submodules.

| Field      | Type            | Description                                                                                                         |
| :--------- | :-------------- | :------------------------------------------------------------------------------------------------------------------ |
| root       | `String`        | The root directory. Dotfiles are stored here, as well as backups.                                                   |
| dots_dir   | `String`        | This directory is appended to root if relative. Dotfiles are stored here.                                           |
| backup_dir | `String`        | This directory is also appended to root if relative. Backups are stored here.                                       |
| remote     | `String`        | The remote `git` repository address. `https://` or `ssh` (`git@`) work, but `ssh` (read/write) should be preferred. |
| sources    | `List<String>`  | A list of directories or files that `nedots` will manage.                                                           |
| git_repos  | `List<GitRepo>` | A list of `GitRepo`, see `GitRepo` model below.                                                                     |

### GitRepo

| Field  | Type     | Description                                                                                                 |
| :----- | :------- | :---------------------------------------------------------------------------------------------------------- |
| id     | `String` | An identifier for this `GitRepo`. `nedots install {id}` may be used to only install this particular source. |
| remote | `String` | Remote git repository.                                                                                      |
| path   | `String` | Local path of `GitRepo`, relative to `$HOME`.                                                               |

Here is a small example:

```yml
root: .nedots
dots_dir: dots
backup_dir: backups
remote: git@git.sr.ht:~nedia/nedots
sources:
  - .config/bspwm
  - .profile
  - /etc/hostname
  - Wallpapers
git_repos:
  - repo:
    id: nvim
    remote: git@git.sr.ht:~nedia/config.nvim
    path: .config/nvim
```

### sources

Operations such as `install` will copy these files from `{root}/{dots_dir}/{source}`
to `$HOME/{source}`.

Conversely, `gather` will copy files from `$HOME/{source}` to `{root}/{dots_dir}/{source}`.

Directories will be copied recursively. Paths can be owned by you, or another, but the correct
permissions will be required at runtime to perform operations on paths that are not owned by
you - you will need to run `sudo nedots` to `install` or `gather` files in `/etc/` for example.

## Usage

So you're new to this style of storing your dotfiles but the chances are you've
got a git repository for storing your dotfiles. That's some of the work done already.

```sh
# To set `nedots` up in its default location `$HOME/.nedots`
nedots init <remote>

# Or to choose your own location
nedots --root $HOME/.dotfiles init <remote>

# NOTE: Subsequent calls to `nedots` will need to be passed `--root $HOME/.dotfiles`
# or `--cfgpath $HOME/.dotfiles/.nedots.yml`.
```

Next, you'll need to edit `.nedots.yml` - `sources` specifically, so that `nedots sync`
can collect your files. It'll then copy them to `$HOME/.nedots/dots`.

```sh
# Recommend adding `--nopush` so `nedots` won't push these changes to remote.
# You can review them first.
# Gather is responsible for collecting your files.
nedots sync --gather --nopush

# Or to sync to a different directory
nedots --root $HOME/.dotfiles --dots <dir> sync --gather --nopush

```

File structure will then look something like this:

```sh
tree .nedots/dots -a
.nedots.test/dots
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

| OS/Method | Command                |
| :-------- | :--------------------- |
| Cargo     | `cargo install nedots` |

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
