# git checkout
co() {
    git checkout "$1"
}

# following code allows for tab auto completion when using co
_co_completion() {
  local branches
  branches=$(git branch -a | awk '{print $NF}' | sed 's|remotes/origin/||' | sort -u)
  COMPREPLY=($(compgen -W "$branches" -- "${COMP_WORDS[1]}"))
}
complete -F _co_completion co

# git pull with a rebase
# also deletes any old branches
gup() {
    git pull --rebase && git remote update origin --prune && git fetch -p -t && for branch in $(git for-each-ref --format "%(refname) %(upstream:track)" refs/heads | awk '$2 == "[gone]" {sub("refs/heads/", "", $1); print $1}'); do git branch -D $branch; done
}

# git commit
gcom() {
    git add .
    git commit -m "$1"
}

# git branch (create a new branch and set upstream)
gnew() {
    git checkout -b "$1"
    git push --set-upstream origin "$1"
}

# opens the repo on github in a web browser
# MAY HAVE TO DOWNLOAD wslview FIRST
gopen() {
  url=$(git config --get remote.origin.url)
  wslview $url
}

# git push
gp() {
    git push origin
}

# git push --force, for pushing after a rebase
gpf() {
  git push origin -f
}

# git stash
gst() {
  git stash
}

# git stash pop
gsp() {
  git stash pop
}

# cargo run (for building rust projects)
# must be in parent directory of repo
cr() {
  current_dir="$(basename "$PWD")"

  if [ -d "$current_dir" ]; then
    cd "$current_dir"
    cargo run
    cd ../
  else
    echo "No project folder was found: $current_dir"
  fi
}

# cargo test for running tests in a cargo project
# must be in parent directory
ct() {
  current_dir="$(basename "$PWD")"

  if [ -d "$current_dir" ]; then
    cd "$current_dir"
    cargo test
    cd ../
  else
    echo "No project folder was found: $current_dir"
  fi
}

# cargo format runs a formatter on all files in the src folder
# must be in parent directory
cf() {
  current_dir="$(basename "$PWD")"

  if [ -d "$current_dir" ]; then
    cd "$current_dir"
    cargo fmt
    cd ../
  else
    echo "No project folder was found: $current_dir"
  fi
}