# Demo Files for Examples

This directory is where you should place your CS2 demo files (`.dem`) to test the examples.

## How to Use

1. **Place your demo files here**: Copy your CS2 demo files to this directory
2. **Update the path in examples**: The examples are configured to look for `sample.dem` in this directory
3. **Run the examples**: Use `cargo run --example <example_name>`

## Example Demo File Locations

### Windows
```
C:\Steam\steamapps\common\Counter-Strike Global Offensive\csgo\replays\
```

### Linux
```
~/.steam/steam/userdata/*/730/local/cfg/replays/
```

### macOS
```
~/Library/Application Support/Steam/userdata/*/730/local/cfg/replays/
```

## File Naming

The examples expect a file named `sample.dem`. You can either:
- Rename your demo file to `sample.dem`
- Update the path in the example files to match your filename

## Note

Demo files are typically large and should not be committed to version control. This directory is included in `.gitignore` to prevent accidental commits of demo files.
