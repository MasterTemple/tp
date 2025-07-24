# tp 🧻

_Create Typst Projects from Local Templates_

This is an interactive CLI tool, but you may specify arguments to skip prompts.

You can either create new projects with their own directory with `tp new`, or you can initialize a project in the current directory with `tp init`.

I have created this because I find that I am repeatedly wanting to use the same Typst config across different papers I write.

## Installation

```bash
cargo install --git https://github.com/MasterTemple/tp
```

## Examples

Given your templates folder looks like this:
```bash
templates/
├── Template 1
│   ├── conf.typ
│   └── main.typ
└── None
    └── main.typ
```

### Create New (Interactive)

```bash
❯ tp new
> Enter a project name: Some Cool Project
> Select a template: Template 1
```

results in

```bash
Some Cool Project/
├── conf.typ
└── main.typ
```

### Initialize Current Directory (Interactive)

Given an empty folder `An Already Created Folder`
```bash
❯ tp init
> Select a template: Default
```

results in

```bash
.
├── conf.typ
└── main.typ
```

### Get Template Directory

```bash
❯ tp dir
Template Directory: "/home/dgmastertemple/.local/share/tp/templates"
```

## Template Locations

All templates are taken from `{data_dir}/tp/templates/` where [`data_dir`](https://docs.rs/dirs-next/2.0.0/dirs_next/fn.data_dir.html) is determined by your OS:

| Platform | Value                                    | Example                                  |
| -------- | ---------------------------------------- | ---------------------------------------- |
| Linux    | `$XDG_DATA_HOME` or `$HOME`/.local/share | /home/alice/.local/share                 |
| macOS    | `$HOME`/Library/Application Support      | /Users/Alice/Library/Application Support |
| Windows  | `{FOLDERID_RoamingAppData}`              | C:\Users\Alice\AppData\Roaming           |

The `None` template is automatically generated and just creates an empty `.typ` file.


## CLI Usage

All arguments beyond the initial command (`new` or `init`) are optional, because there will be interactive prompts to enter the necessary information.

```bash
❯ tp -h
Usage: tp <COMMAND>

Commands:
  new   Create a new project from a template
  init  Initialize a new project in the current directory
  dir   Print the template directory
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## `tp new`

```bash
❯ tp new -h
Create a new project from a template

Usage: tp new [OPTIONS]

Options:
  -p, --project <PROJECT>    The project name
  -t, --template <TEMPLATE>  The template to use
  -n, --no-main              Whether or not to rename `main.typ` to `<Project>.typ`
  -h, --help                 Print help
```

## `tp init`

```bash
❯ tp init -h
Initialize a new project in the current directory

Usage: tp init [OPTIONS]

Options:
  -t, --template <TEMPLATE>  The template to use
  -n, --no-main              Whether or not to rename `main.typ` to `<Project>.typ`
  -h, --help                 Print help
```

## `tp dir`

```bash
❯ tp dir -h
Print the template directory

Usage: tp dir

Options:
  -h, --help  Print help
```
