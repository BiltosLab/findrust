# Findthat

this simple app does exactly what ```find``` does on unix systems , surpisingly it runs much faster than my C implementation "Skill issue ofc" 

this was meant to be educational nothing more nothing less.


## Usage:

```shell
Usage findthat <PATH> <KEYWORD> OPTIONAL <-searchhidden>
```

Example:
``` findthat . main.rs -searchhidden ```

## Build and Install:
this was not tested on anything other than arch linux but it should work on Mac OS,Windows as well.

Build with ```cargo build --release```

Or just build & install with:
``` sudo chmod +x install.sh && ./install.sh```

