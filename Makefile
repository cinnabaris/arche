dist=dist

build: api www
	cd $(dist) && tar cfJ ../$(dist).tar.xz *

api:
	cargo build --release
	strip -s target/release/arche
	mkdir -p $(dist)
	-cp -r target/release/arche db templates log4rs.yml LICENSE README.md $(dist)/

www:
	cd desktop && npm run build
	-cp -r desktop/dist $(dist)/public

schema:
	DATABASE_URL="postgres://postgres:@localhost:5432/arche" diesel print-schema > src/schema/postgresql.rs
	DATABASE_URL="mysql://root:test@localhost:3306/arche" diesel print-schema > src/schema/mysql.rs

clean:
	cargo clean
	-rm -r $(dist) $(dist).tar.xz desktop/dist
