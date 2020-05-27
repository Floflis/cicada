# RC File

When cicada shell is invoked as an interactive
[login shell](https://github.com/mitnk/cicada/blob/master/docs/install.md#user-content-set-cicada-as-your-login-shell),
or with the `--login`/`-l` option, it will reads and executes commands from
first existing file of below list:

- `$XDG_CONFIG_HOME/cicada/cicadarc`
- `$HOME/.config/cicada/cicadarc` (if `$XDG_CONFIG_HOME` not defined)
- `$HOME/.cicadarc`

> Hint: In non-login shell mode, you can apply RC file with
> [source](https://github.com/mitnk/cicada/blob/master/docs/builtins.md#user-content-source):
> `$ source <path-to-rc-file>`.

Here is a sample RC file:

```
# handle envs
export RUST_BACKTRACE='full'
export COPYFILE_DISABLE=1
export PATH="/usr/local/bin:$PATH"

# define aliases
alias ls="ls -G"
alias ll="ls -lh"
alias foo='echo foo bar | wc'

# run regular commands
echo "cicada started at `date`" >> /tmp/some-random.log
touch /tmp/cicada-started

# include another rc file
source ~/.cicadarc_local
```
