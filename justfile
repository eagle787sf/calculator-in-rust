name := 'cosmic-calculator'
appid := 'dev.eagle787sf.CosmicCalculator'

rootdir := ''
prefix := '/usr'

base-dir := absolute_path(clean(rootdir / prefix))
bin-dir := base-dir / 'bin'
share-dir := base-dir / 'share'

default: build-release

build-debug *args:
    cargo build {{args}}

build-release *args:
    cargo build --release {{args}}

check *args:
    cargo clippy --all-features {{args}} -- -W clippy::pedantic

run *args:
    cargo run --release {{args}}

install:
    install -Dm0755 target/release/{{name}} {{bin-dir}}/{{name}}
    install -Dm0644 res/{{appid}}.desktop {{share-dir}}/applications/{{appid}}.desktop
    install -Dm0644 res/icons/hicolor/scalable/apps/{{appid}}.svg {{share-dir}}/icons/hicolor/scalable/apps/{{appid}}.svg

uninstall:
    rm -f {{bin-dir}}/{{name}}
    rm -f {{share-dir}}/applications/{{appid}}.desktop
    rm -f {{share-dir}}/icons/hicolor/scalable/apps/{{appid}}.svg

clean:
    cargo clean

test:
    cargo test
