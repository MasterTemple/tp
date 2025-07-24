# tp

_Create Typst Projects from Local Templates_

I have created this because I find that I am repeatedly wanting to use the same Typst config across different papers I write.


## Usage

All arguments beyond the initial command (`new` or `init`) are optional, because there will be interactive prompts to enter the necessary information.

```bash
❯ tp -h
Usage: tp <COMMAND>

Commands:
  new   Create a new project from a template
  init  Initialize a new project in the current directory
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
