#!/usr/bin/env bash
set -euo pipefail

# === CONFIGURATION ===
DEPS_DIR="tools/deps"
LIBBURNIA_BASE_URL="https://files.libburnia-project.org/releases"
PKGCONFIG_URL="https://pkg-config.freedesktop.org/releases/pkg-config-0.29.2.tar.gz"

# === HELPERS ===

# Download and extract a tar.gz source if not already extracted
download_and_extract() {
    local name="$1"
    local url="$2"

    if ! [ -d "$name" ]; then
        curl -LO "$url"
        tar -xvzf "${name}.tar.gz"
        rm -f "${name}.tar.gz"
    fi
}

# Configure, build, and install a library
build_and_install() {
    local dir="$1"
    local configure_opts="$2"
    local post_patch="$3"

    (
        cd "$dir"
        eval "$post_patch"
        ./configure --prefix=/usr --disable-static $configure_opts
        make -j"$(nproc)"
        sudo make install
    )
}

# === DEPENDENCIES ===

install_pkg_config() {
    local name="pkg-config-0.29.2"
    download_and_extract "$name" "$PKGCONFIG_URL"
    build_and_install "$name" \
        "--with-internal-glib --disable-host-tool --docdir=/usr/share/doc/$name" \
        ""
}

install_libisofs() {
    local name="libisofs-1.5.6"
    download_and_extract "$name" "$LIBBURNIA_BASE_URL/$name.tar.gz"
    build_and_install "$name" "" ""
}

install_libburn() {
    local name="libburn-1.5.6"
    download_and_extract "$name" "$LIBBURNIA_BASE_URL/$name.tar.gz"
    build_and_install "$name" "" "sed -i 's/catch_int ()/catch_int (int signum)/' test/poll.c"
}

install_libisoburn() {
    local name="libisoburn-1.5.6"
    download_and_extract "$name" "$LIBBURNIA_BASE_URL/$name.tar.gz"
    build_and_install "$name" "--enable-pkg-check-modules" ""
}

# === MAIN SCRIPT ===

sudo -v # refresh sudo credentials
mkdir -p "$DEPS_DIR"
cd "$DEPS_DIR"


if ! command -v pkg-config >/dev/null 2>&1; then
    install_pkg_config
else
    echo "pkg-config already installed"
fi

if ! pkg-config --exists libisofs-1; then
    install_libisofs
else
    echo "libisofs already installed"
fi

if ! pkg-config --exists libburn-1; then
    install_libburn
else
    echo "libburn already installed"
fi

if ! pkg-config --exists libisoburn-1; then
    install_libisoburn
else
    echo "libisoburn already installed"
fi

echo "All dependencies installed"
