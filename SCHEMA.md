# Solidus with Discourse

## Install ruby

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

-   create discourse app

```bash
git clone https://github.com/discourse/discourse.git discourse
cd ~/discourse
```

## Discourse

-   clone source

```bash
git clone https://github.com/discourse/discourse.git
cd discourse
git checkout latest-release
bundle install
```

-   edit config/database.yml

```yaml
username: postgres
```

-   create database tables

```bash
bundle exec rake db:create
bundle exec db:migrate
bundle exec rake admin:create
bundle exec rails s
```

## Solidus

-   create rails app

```bash
rails new solidus
cd solidus
```

## config/database.yml

```yaml
adapter:  postgresql
username: postgres
database: solidus_development
```

-   add Solidus gems to your Gemfile

```ruby
gem 'pg'
gem 'solidus'
gem 'solidus_auth_devise'
gem 'solidus_gateway'
gem 'solidus_related_products'
gem 'solidus_print_invoice' , github: 'solidusio-contrib/solidus_print_invoice'
gem 'solidus_comments'
gem 'solidus_wishlist', github: 'boomerdigital/solidus_wishlist'
gem 'solidus_email_to_friend', github: 'boomerdigital/solidus_email_to_friend'
```

-   generate database tables

```bash
bundle install
bundle exec rake railties:install:migrations
bundle exec rake db:migrate
bundle exec rake db:seed
bundle exec rake spree_sample:load
bundle exec rails s
```

-   start application server

```bash
bundle exec rake db:seed
bundle exec rake spree_sample:load
bundle exec rails s # http://localhost:3000
```

-   export tables

```bash
pg_dump -O -x -s -U postgres discourse_development > discourse.sql
pg_dump -O -x -s -U postgres solidus_development > solidus.sql
```
