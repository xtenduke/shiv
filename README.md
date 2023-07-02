# shivr
Lightweight command runner for monorepos. Can run commands on only packages changed against defined root branch


### Demo
![](https://github.com/xtenduke/shiv/blob/main/demo.gif)


### Installing
#### Install script

View the script [here](https://raw.githubusercontent.com/xtenduke/shivr/main/download.sh)

```
 # Non root user, can only download
 $ curl https://raw.githubusercontent.com/xtenduke/shivr/main/download.sh | bash
 
 # Root user, can download and install to /usr/local/bin
 $ curl https://raw.githubusercontent.com/xtenduke/shivr/main/download.sh | sudo bash
```

#### From cargo
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
  --concurrency     max number of threads to run, default 1
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

### Development

##### Testing
`setup_test.sh` needs to be run before running `cargo test`