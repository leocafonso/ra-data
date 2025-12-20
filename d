#!/usr/bin/env bash

set -e
export RUST_BACKTRACE=full
cd $(dirname $0)
CMD=$1
REV=7bcbf08f1ec848ee36c461bd2330ff82720e6a5b
shift

case "$CMD" in
    download-all)
        rm -rf ./sources/
        git clone https://github.com/leocafonso/ra-data-sources.git ./sources/ -q
        cd ./sources/
        git checkout $REV
    ;;
    install-chiptool)
        cargo install --git https://github.com/embassy-rs/chiptool
    ;;
    sanitize)
        python3 scripts/sanitize_svd.py sources/
    ;;
    extract-all)
        for peri in "$@"; do
            echo "Extracting peripheral $peri..."
            rm -rf tmp/$peri
            mkdir -p tmp/$peri

            for f in sources/svd/*.svd; do
                [ -e "$f" ] || continue
                name=$(basename "$f" .svd)
                echo -n "  processing $name ... "
                if chiptool extract-peripheral --svd "$f" --peripheral "$peri" > "tmp/$peri/$name.yaml" 2> "tmp/$peri/$name.err"; then
                    echo OK
                else
                    if grep -q 'peripheral not found' "tmp/$peri/$name.err"; then
                        echo "No Peripheral"
                    else
                        echo "OTHER FAILURE"
                    fi
                    rm -f "tmp/$peri/$name.yaml"
                fi
            done

            mkdir -p data/registers
            for f in tmp/$peri/*.yaml; do
                if [ -e "$f" ]; then
                    cp "$f" "data/registers/$peri.yaml"
                    echo "  Copied $f to data/registers/$peri.yaml"
                    break
                fi
            done
        done
    ;;
    gen-pac)
        rm -rf build/ra-metapac
        cargo run --release --bin ra-metapac-gen
    ;;
    gen)
        rm -rf build/data
        cargo run --release --bin ra-data-gen
    ;;
    gen-all)
        ./d gen
        ./d gen-pac
    ;;
    check)
        # Check all chips by generating a single lib.rs that includes all of them
        echo "Generating check-all lib.rs..."
        
        LIB_RS="build/ra-metapac/src/lib.rs"
        LIB_RS_BAK="build/ra-metapac/src/lib.rs.bak"
        
        cp "$LIB_RS" "$LIB_RS_BAK"
        
        cat > "$LIB_RS" <<EOF
#![no_std]
pub mod common;
pub mod _peripherals;
pub mod chips {
EOF
        for chip_dir in build/ra-metapac/src/chips/*/; do
            chip=$(basename "$chip_dir")
            echo "    pub mod $chip { include!(\"chips/$chip/pac.rs\"); }" >> "$LIB_RS"
        done
        echo "}" >> "$LIB_RS"
        
        echo "Checking all chips..."
        # Use a target that supports most things, e.g. thumbv7m-none-eabi
        # We might need to run multiple checks for different architectures if they differ significantly,
        # but for PAC syntax, one should be enough.
        cargo check --manifest-path build/ra-metapac/Cargo.toml --target thumbv7m-none-eabi
        
        mv "$LIB_RS_BAK" "$LIB_RS"
        ;;

    *)
        echo "Usage: $0 {install-chiptool|extract-all|gen|gen-all|check}"
        exit 1
    ;;
esac
