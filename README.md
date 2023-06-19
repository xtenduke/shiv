# shiv
Lightweight command runner for monorepos. Can run commands on only packages changed against defined root branch

WIP

### Example
```
cargo run -- --detect-changes --main-branch main --package-dir packages --root-dir /home/jake/Development/group_firewall_mr --command build
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



###### Bins to come
