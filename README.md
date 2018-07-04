# ARCHE

 Blog Tool, Publishing Platform, CMS, Issue Tracking and E-Commerce solution(By Go and React).

## Usage

-   install gvm

```bash
zsh < <(curl -s -S -L https://raw.githubusercontent.com/moovweb/gvm/master/binscripts/gvm-installer)
```

-   add to ~/.zshrc

```bash
[[ -s "$HOME/.gvm/scripts/gvm" ]] && source "$HOME/.gvm/scripts/gvm"
```

-   install go

```bash
gvm install go1.11beta1 -B
gvm use go1.11beta1 --default
```

-   download source

```bash
go get -u github.com/kardianos/govendor
go get -d github.com/cinnabaris/arche
cd $GOPATH/src/github.com/cinnabaris/arche
make deps
make
```

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

```text
local   all             all                                     peer
```

    TO:

```text
local   all             all                                     md5
```

## Documents

-   [For gmail smtp](http://stackoverflow.com/questions/20337040/gmail-smtp-debug-error-please-log-in-via-your-web-browser)
-   [favicon.ico](http://icoconvert.com/)
-   [smver](http://semver.org/)
-   [banner.txt](http://patorjk.com/software/taag/)
-   [msmtp](https://wiki.archlinux.org/index.php/msmtp)
-   [GraphQL](https://graphql.org/learn/)
-   [Alibaba Java Coding Guidelines](https://github.com/alibaba/p3c)
-   [An emoji guide for your commit messages](https://gitmoji.carloscuesta.me/)
-   [Let’s Encrypt](https://letsencrypt.org/)
-   [Certbot](https://certbot.eff.org/)
-   [SSL Server Test](https://www.ssllabs.com/ssltest/index.html)
-   [LINE Developers](https://developers.line.me/en/)
