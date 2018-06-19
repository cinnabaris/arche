# arche

A complete open source e-commerce solution built with Rust and React(STILL IN DEVELOPMENT).

Inspired by [Solidus](https://github.com/solidusio/solidus) and [Discourse
](https://github.com/discourse/discourse).

## Install

-   install rustup

```bash
curl https://sh.rustup.rs -sSf | sh
```

-   add to your .zshrc

```bash
source $HOME/.cargo/env
```

-   then re-login at first

-   install rust-nightly

```bash
git clone https://github.com/cinnabaris/arche.git
cd arche
rustup override set nightly
make clean
make
```

## Usage

-   testing

```bash
diesel database reset
cargo run
python -m unittest -v
```

## Editor

### atom plugins

-   autosave: enable
-   platformio-ide-terminal
-   file-icons
-   atom-beautify: enable beautify on save
-   ide-rust: disable toolchain update checking
-   language-babel:

### vscode plugins

-   vscode-icons
-   markdownlint
-   rust(rls)

## Notes

-   Generate a random key

```bash
openssl rand -base64 32
```

-   ~/.npmrc

```bash
npm config set prefix '~/.npm-global'
```

-   'Peer authentication failed for user', open file "/etc/postgresql/9.5/main/pg_hba.conf" change line:

```bash
local   all             all                                     peer
TO:
local   all             all                                     md5
```

-   forgot mysql root password

create file  /tmp/reset.mysqld

```sql
SET PASSWORD FOR root@localhost = PASSWORD('blah');
```

edit file /etc/mysql/my.cnf

```text
[mysqld]
init-file=/tmp/reset.mysqld
```

-   RabbitMQ

```bash
rabbitmq-plugins enable rabbitmq_management
rabbitmqctl change_password guest change-me
rabbitmqctl add_user who-am-i change-me
rabbitmqctl set_user_tags who-am-i administrator
rabbitmqctl list_vhosts
rabbitmqctl add_vhost v-host
rabbitmqctl set_permissions -p v-host who-am-i ".*" ".*" ".*"
```

## Documents

-   [For gmail smtp](http://stackoverflow.com/questions/20337040/gmail-smtp-debug-error-please-log-in-via-your-web-browser)
-   [favicon.ico](http://icoconvert.com/)
-   [smver](http://semver.org/)
-   [banner.txt](http://patorjk.com/software/taag/)
-   [msmtp](https://wiki.archlinux.org/index.php/msmtp)
-   [Are we (I)DE yet?](https://areweideyet.com/)
-   [Awesome Rust](https://github.com/rust-unofficial/awesome-rust)
-   [Rocket](https://rocket.rs/)
-   [Diesel](http://diesel.rs/guides/getting-started/)
-   [Ant Design](https://ant.design/docs/react/introduce)
-   [Ant Design Pro](https://pro.ant.design/docs/getting-started)
-   [Bootstrap](http://getbootstrap.com/)
-   [GraphQL](https://graphql.org/learn/)
-   [Alibaba Java Coding Guidelines](https://github.com/alibaba/p3c)
-   [An emoji guide for your commit messages](https://gitmoji.carloscuesta.me/)
