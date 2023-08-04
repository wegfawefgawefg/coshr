# coshr
composite multiple files togethor. shrinks lines above 80 chars. useful for giving somebody code context (like, gpt4)

## Usage
```bash
coshr sketch.js style.css index.html  | xclip -selection clipboard
```
or using the wrapper script 'csc' included, which is installed by install.sh
```bash
csc sketch.js style.css index.html
```




## Install
Get Dependencies
- rustc / cargo
- xclip

Run the install script
```bash
./install.sh
```


