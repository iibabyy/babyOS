#!/usr/bin/env bash
set -euo pipefail

# === CONFIGURATION ===
DEPS_DIR="tools/deps"
LIBBURNIA_BASE_URL="https://files.libburnia-project.org/releases"
PKGCONFIG_URL="https://pkg-config.freedesktop.org/releases/pkg-config-0.29.2.tar.gz"

# === COLORS ===
RED="\033[0;31m"
GREEN="\033[0;32m"
YELLOW="\033[1;33m"
RESET="\033[0m"

# === HELPERS ===

download_and_extract() {
    local name="$1"
    local url="$2"

    if [ -d "$name" ]; then
        return
    fi

    curl -sSLO "$url"
    tar -xzf "${name}.tar.gz"
    rm -f "${name}.tar.gz"
}

uninstall_from_source() {
    local dir="$1"
    local configure_opts="$2"
    local post_patch="$3"

    if [ ! -d "$dir" ]; then
        echo -e "${RED}Directory $dir not found — cannot uninstall${RESET}"
        return
    fi

    (
        cd "$dir"
        eval "$post_patch"
		echo "Configuring $dir..."
        ./configure --prefix=/usr --disable-static $configure_opts >/dev/null
        sudo make uninstall
    )
    rm -rf "$dir"
    echo -e "${GREEN}Removed $dir${RESET}"
}

# === UNINSTALL TARGETS ===

uninstall_pkg_config() {
    local name="pkg-config-0.29.2"
    download_and_extract "$name" "$PKGCONFIG_URL"
    uninstall_from_source "$name" \
        "--with-internal-glib --disable-host-tool --docdir=/usr/share/doc/$name" \
        ""
}

uninstall_libisofs() {
    local name="libisofs-1.5.6"
    download_and_extract "$name" "$LIBBURNIA_BASE_URL/$name.tar.gz"
    uninstall_from_source "$name" "" ""
}

uninstall_libburn() {
    local name="libburn-1.5.6"
    download_and_extract "$name" "$LIBBURNIA_BASE_URL/$name.tar.gz"
    uninstall_from_source "$name" "" "sed -i 's/catch_int ()/catch_int (int signum)/' test/poll.c"
}

uninstall_libisoburn() {
    local name="libisoburn-1.5.6"
    download_and_extract "$name" "$LIBBURNIA_BASE_URL/$name.tar.gz"
    uninstall_from_source "$name" "--enable-pkg-check-modules" ""
}

# === MAIN ===

sudo -v # refresh sudo credentials
mkdir -p "$DEPS_DIR"
cd "$DEPS_DIR"

if ! command -v pkg-config >/dev/null 2>&1; then
	echo -e "${RED}pkg-config needed to check other dependencies${RESET}"
	exit 1
fi

# Uninstall in reverse dependency order
if pkg-config --exists libisoburn-1; then
    uninstall_libisoburn
else
    echo -e "${GREEN}libisoburn already uninstalled${RESET}"
fi

if pkg-config --exists libburn-1; then
    uninstall_libburn
else
    echo -e "${GREEN}libburn already uninstalled${RESET}"
fi

if pkg-config --exists libisofs-1; then
    uninstall_libisofs
else
    echo -e "${GREEN}libisofs already uninstalled${RESET}"
fi

if command -v pkg-config >/dev/null 2>&1; then
    uninstall_pkg_config
else
    echo -e "${GREEN}pkg-config already uninstalled${RESET}"
fi

cd ..
rm -rf "$DEPS_DIR"

echo -e "All libraries uninstalled"