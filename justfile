default:
    @just --list

init:
    curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh                            
    rustup target add thumbv7em-none-eabihf
    

build-nuttx:
    rm -rf nuttx-export
    cd nuttxspace/nuttx && make export
    mv nuttxspace/nuttx/nuttx-export-12.6.0.tar.gz nuttx-export.tar.gz
    tar zxf nuttx-export.tar.gz
    rm -rf nuttx-export.tar.gz
    mv nuttx-export-12.6.0 nuttx-export

# build:
#     cargo +nightly build -Z build-std=core,alloc --target thumbv7em-nuttx-eabihf