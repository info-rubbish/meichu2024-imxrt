nuttx-version := "12.6.0"

default:
    @just --list

init-tool:
    curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh                            
    rustup target add thumbv7em-none-eabihf

init-config:
    rm nuttxspace/nuttx/.config
    cd nuttxspace/nuttx && ./tools/configure.sh -l imxrt1060-evk:lvgl
    diff restore nuttxspace/nuttx/.config
    cd nuttxspace/nuttx && make olddefconfig

init-nuttx:
    mkdir -p nuttxspace
    cd nuttxspace && curl -L https://www.apache.org/dyn/closer.lua/nuttx/{{nuttx-version}}/apache-nuttx-{{nuttx-version}}.tar.gz?action=download -o nuttx.tar.gz
    cd nuttxspace && curl -L https://www.apache.org/dyn/closer.lua/nuttx/{{nuttx-version}}/apache-nuttx-apps-{{nuttx-version}}.tar.gz?action=download -o apps.tar.gz
    cd nuttxspace && tar zxf nuttx.tar.gz --one-top-level=nuttx --strip-components 1
    cd nuttxspace && tar zxf apps.tar.gz --one-top-level=apps --strip-components 1

init: init-tool init-nuttx init-config

config:
    cd nuttxspace/nuttx && make menuconfig
    cd nuttxspace/nuttx && kconfig-tweak --set-str CONFIG_INIT_ENTRYPOINT "nsh_main"
    cd nuttxspace/nuttx && make olddefconfig
    cd nuttxspace/nuttx && make
    cd nuttxspace/nuttx && kconfig-tweak --set-str CONFIG_INIT_ENTRYPOINT "nxp_main"

build-nuttx:
    rm -rf nuttx-export
    cd nuttxspace/nuttx && make export
    mv nuttxspace/nuttx/nuttx-export*.tar.gz nuttx-export.tar.gz
    tar zxf nuttx-export.tar.gz --one-top-level=nuttx-export --strip-components 1
    rm nuttx-export.tar.gz

# build:
#     cargo +nightly build -Z build-std=core,alloc --target thumbv7em-nuttx-eabihf


clean:
    cargo clean
    cd nuttxspace/nuttx && make clean && make

# Add app in hacky way
hack-app:
    cp hack/* nuttxspace/apps/builtin/registry
    rm nuttxspace/apps/builtin/registry/.updated

flash:
    cargo flash --chip MIMXRT1060 --release

fuck-nxp:
    probe-rs erase --chip MIMXRT1060