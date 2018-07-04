dist=dist
pkg=github.com/cinnabaris/arche/nut
theme=bootstrap

VERSION=`git rev-parse --short HEAD`
BUILD_TIME=`date -R`
AUTHOR_NAME=`git config --get user.name`
AUTHOR_EMAIL=`git config --get user.email`
COPYRIGHT=`head -n 1 LICENSE`
USAGE=`sed -n '3p' README.md`


build: api www
	cd $(dist) && tar cfJ ../$(dist).tar.xz *

www:
	cd dashboard && npm run build
	-cp -r dashboard/build $(dist)/dashboard

api:
	go build -ldflags "-s -w -X ${pkg}.Version=${VERSION} -X '${pkg}.BuildTime=${BUILD_TIME}' -X '${pkg}.AuthorName=${AUTHOR_NAME}' -X ${pkg}.AuthorEmail=${AUTHOR_EMAIL} -X '${pkg}.Copyright=${COPYRIGHT}' -X '${pkg}.Usage=${USAGE}'" -o ${dist}/arche main.go
	-cp -r db locales templates themes package.json package-lock.json README.md LICENSE $(dist)/

clean:
	-rm -r $(dist) $(dist).tar.xz dashboard/build

deps:
	govendor sync
	npm install
	cd dashboard && npm install
