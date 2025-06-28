TARGET := "mightty"

ASSETS_DIR := "extra"
RELEASE_DIR := "target/release"
MANPAGE := ASSETS_DIR + "/man/mightty.1.scd"
MANPAGE_MSG := ASSETS_DIR + "/man/mightty-msg.1.scd"
MANPAGE_CONFIG := ASSETS_DIR + "/man/mightty.5.scd"
MANPAGE_CONFIG_BINDINGS := ASSETS_DIR + "/man/mightty-bindings.5.scd"
TERMINFO := ASSETS_DIR + "/mightty.info"
COMPLETIONS_DIR := ASSETS_DIR + "/completions"

APP_NAME := "Mightty.app"
APP_TEMPLATE := ASSETS_DIR + "/osx/" + APP_NAME
APP_DIR := RELEASE_DIR + "/osx"
APP_BINARY := RELEASE_DIR + "/" + TARGET
APP_BINARY_DIR := APP_DIR + "/" + APP_NAME + "/Contents/MacOS"
APP_EXTRAS_DIR := APP_DIR + "/" + APP_NAME + "/Contents/Resources"
APP_COMPLETIONS_DIR := APP_EXTRAS_DIR + "/completions"

DMG_NAME := "Mightty.dmg"
DMG_DIR := RELEASE_DIR + "/osx"

# Default recipe
default: help

# Print this help message
help:
    @just --list

# Build a release binary
binary: binary-native

# Build a universal release binary
binary-universal: mightty-universal

# Build native binary
binary-native: mightty-native

mightty-native:
    MACOSX_DEPLOYMENT_TARGET="10.11" cargo build --release

mightty-universal:
    MACOSX_DEPLOYMENT_TARGET="10.11" cargo build --release --target=x86_64-apple-darwin
    MACOSX_DEPLOYMENT_TARGET="10.11" cargo build --release --target=aarch64-apple-darwin
    @lipo target/{x86_64,aarch64}-apple-darwin/release/{{TARGET}} -create -output {{APP_BINARY}}

# Create an Alacritty.app
app: app-native

# Create a universal Alacritty.app
app-universal: (app-build "universal")

# Create native Alacritty.app
app-native: (app-build "native")

app-build variant: (mightty-variant variant)
    @mkdir -p {{APP_BINARY_DIR}}
    @mkdir -p {{APP_EXTRAS_DIR}}
    @mkdir -p {{APP_COMPLETIONS_DIR}}
    @scdoc < {{MANPAGE}} | gzip -c > {{APP_EXTRAS_DIR}}/mightty.1.gz
    @scdoc < {{MANPAGE_MSG}} | gzip -c > {{APP_EXTRAS_DIR}}/mightty-msg.1.gz
    @scdoc < {{MANPAGE_CONFIG}} | gzip -c > {{APP_EXTRAS_DIR}}/mightty.5.gz
    @scdoc < {{MANPAGE_CONFIG_BINDINGS}} | gzip -c > {{APP_EXTRAS_DIR}}/mightty-bindings.5.gz
    @tic -xe mightty,mightty-direct -o {{APP_EXTRAS_DIR}} {{TERMINFO}}
    @cp -fRp {{APP_TEMPLATE}} {{APP_DIR}}
    @cp -fp {{APP_BINARY}} {{APP_BINARY_DIR}}
    @cp -fp {{COMPLETIONS_DIR}}/_mightty {{APP_COMPLETIONS_DIR}}
    @cp -fp {{COMPLETIONS_DIR}}/mightty.bash {{APP_COMPLETIONS_DIR}}
    @cp -fp {{COMPLETIONS_DIR}}/mightty.fish {{APP_COMPLETIONS_DIR}}
    @touch -r "{{APP_BINARY}}" "{{APP_DIR}}/{{APP_NAME}}"
    @codesign --remove-signature "{{APP_DIR}}/{{APP_NAME}}"
    @codesign --force --deep --sign - "{{APP_DIR}}/{{APP_NAME}}"
    @echo "Created '{{APP_NAME}}' in '{{APP_DIR}}'"

mightty-variant variant:
    #!/usr/bin/env sh
    if [ "{{variant}}" = "native" ]; then
        just mightty-native
    elif [ "{{variant}}" = "universal" ]; then
        just mightty-universal
    fi

# Create an Alacritty.dmg
dmg: dmg-native

# Create a universal Alacritty.dmg
dmg-universal: (dmg-build "universal")

# Create native Alacritty.dmg
dmg-native: (dmg-build "native")

dmg-build variant: (app-build variant)
    @echo "Packing disk image..."
    @ln -sf /Applications {{DMG_DIR}}/Applications
    @hdiutil create {{DMG_DIR}}/{{DMG_NAME}} \
        -volname "Mightty" \
        -fs HFS+ \
        -srcfolder {{APP_DIR}} \
        -ov -format UDZO
    @echo "Packed '{{APP_NAME}}' in '{{APP_DIR}}'"

# Mount disk image
install: install-native

# Mount universal disk image
install-universal: (install-variant "universal")

# Mount native disk image
install-native: (install-variant "native")

install-variant variant: (dmg-build variant)
    @open {{DMG_DIR}}/{{DMG_NAME}}

# Remove all build artifacts
clean:
    @cargo clean
