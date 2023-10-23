# test_repo
## Git Branches
I use this to show me what branch I am on when in a git repository.

<img src="./images/branch-image.png" alt="screenshot" width="500"/>

To replicate this, add this code to your `~/.bashrc` file:
```sh
function parse_git_branch () {
  git branch 2> /dev/null | sed -e '/^[^*]/d' -e 's/* \(.*\)/ (\1)/'
}
YELLOW="\[\033[0;33m\]"
GREEN="\[\033[0;32m\]"
BLUE="\[\033[0;31m\]"
NO_COLOR="\[\033[0m\]"
PS1="$GREEN\u@\h$NO_COLOR:\w$BLUE\$(parse_git_branch)$NO_COLOR\$ "
```
Easily open bashrc using `vim ~/.bashrc`.

There are other ways to get a similar effect but this way is pretty straightforward and works with bash shell.

I also really like git graph visualizers as they help explain how the repo looks. The one I use the most is [here](https://marketplace.visualstudio.com/items?itemName=mhutchie.git-graph).

<img src="./images/graph-image.png" alt="screenshot" width="500"/>

## Aliases
The `/aliases` folder has some useful git and rust aliases that I use that I think everyone might find helpful.

To use these copy the file or specific alias you want. Paste the file somewhere such as your home directory, or if you already have an alias file, paste the function in there. 

Add this to your `~/.bashrc` file:
```sh
source /path/to/your/alias/file
```

You can either restart your console or run `source ~/.bashrc` to have the aliases work.