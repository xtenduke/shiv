# shivr
Lightweight command runner for monorepos. Can run commands on only packages changed against defined root branch

WIP

### Installing
```
$ cargo install shivr
```

### Usage
```
Usage: shivr [--detect-changes] [--main-branch <main-branch>] [--root-dir <root-dir>] [--package-dir <package-dir>] --command <command> [--concurrency <concurrency>]

Arguments

Options:
  --detect-changes  if shiv should run the command on all packages, or just
                    those changed against main
  --main-branch     main branch name, default "main"
  --root-dir        root dir to run in
  --package-dir     package directory, default "packages"
  --command         command to run on packages
  --concurrency     max number of threads to run, default 0
  --help            display usage information
```

### Example
```
$ shivr --detect-changes --command build
```

### Config files required
Config files mapping out commands to package scripts should be configured in all package dirs like so.
Named `shiv.json`
```
{
    "scripts": [
        {
            "name": "build",
            "run": "yarn build"
        }
    ]
}
```

