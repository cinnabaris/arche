# README

## Install ruby

-   install rbenv

```bash
git clone https://github.com/rbenv/rbenv.git ~/.rbenv
cd ~/.rbenv && src/configure && make -C src
```

-   add to your ~/.zshrc

```bash
export PATH="$HOME/.rbenv/bin:$PATH"
eval "$(rbenv init -)"
alias rake='noglob rake'
```

-   re-login and install ruby

```bash
mkdir -p "$(rbenv root)"/plugins
git clone https://github.com/rbenv/ruby-build.git "$(rbenv root)"/plugins/ruby-build
git clone https://github.com/rbenv/rbenv-vars.git $(rbenv root)/plugins/rbenv-vars
rbenv install 2.5.1
gem install bundler
gem install rubocop
gem install rails
```

## Notes

-   create application script

```bash
rails new arche -d postgresql --api --skip-test
```
