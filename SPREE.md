# Spree

## Install

-   install rbenv

```bash
git clone https://github.com/rbenv/rbenv.git ~/.rbenv
cd ~/.rbenv && src/configure && make -C src
```

-   add to ~/.zshrc and re-login

```bash
export PATH="$HOME/.rbenv/bin:$PATH"
eval "$(rbenv init -)"
```

-   install ruby

```bash
git clone https://github.com/rbenv/ruby-build.git "$(rbenv root)"/plugins/ruby-build
git clone https://github.com/rbenv/rbenv-vars.git $(rbenv root)/plugins/rbenv-vars
rbenv global 2.4.2
```

-   create app

```bash
gem install rails -v 5.1
rails new demo
```

-   add Spree gems to your Gemfile

```ruby
gem 'spree', '~> 3.4.4'
gem 'spree_auth_devise', '~> 3.3'
gem 'spree_gateway', '~> 3.3'

gem 'pg', '>= 0.18', '< 2.0'
gem 'mysql2', '>= 0.3.18', '< 0.6.0'
```

-   install

```bash
bundle update i18n
bundle install
```

-   database.yml

```yaml
adapter:  postgresql
encoding: unicode
database: spree
username: postgres

adapter:    mysql2
encoding:   utf8
reconnect:  false
database:   spree
username:   root
password:   test
```

-   generate tables

```bash
bundle exec rake railties:install:migrations
bundle exec rake db:migrate
bundle exec rake db:seed
```

-   export tables

```bash
pg_dump -O -x -s -U postgres spree > pg.sql
mysqldump -uroot -p -d spree > my.sql
```
