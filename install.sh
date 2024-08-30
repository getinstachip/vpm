#!/usr/bin/env sh
set -eu

# Downloads the latest release from the GitHub API and installs it
# into /usr/local/bin. If you'd prefer to do this manually, instructions are at
# https://github.com/getinstachip/vpm.

main() {
    platform="$(uname -s)"
    arch="$(uname -m)"
    temp="$(mktemp -d "/tmp/vpm-XXXXXX")"

    if [ "$platform" = "Darwin" ]; then
        platform="apple-darwin"
    elif [ "$platform" = "Linux" ]; then
        platform="linux"
    else
        echo "Unsupported platform $platform"
        exit 1
    fi

    case "$arch" in
        arm64 | aarch64)
            arch="aarch64"
            ;;
        x86_64)
            arch="x86_64"
            ;;
        i686)
            arch="i686"
            ;;
        armv7l)
            arch="armv7"
            ;;
        *)
            echo "Unsupported architecture $arch"
            exit 1
            ;;
    esac

    if which curl >/dev/null 2>&1; then
        curl () {
            command curl -fL "$@"
        }
    elif which wget >/dev/null 2>&1; then
        curl () {
            wget -O- "$@"
        }
    else
        echo "Could not find 'curl' or 'wget' in your path"
        exit 1
    fi

    "$platform" "$@"
}

linux() {
    echo "Fetching the latest VPM release information"
    release_info=$(curl -s "https://api.github.com/repos/getinstachip/vpm/releases/latest")

    # Determine the appropriate download URL
    download_url=$(echo "$release_info" | grep "browser_download_url.*vpm-.*-$arch-unknown-linux-musl.tar.gz" | cut -d '"' -f 4)
    if [ -z "$download_url" ]; then
        download_url=$(echo "$release_info" | grep "browser_download_url.*vpm_.*_$arch.deb" | cut -d '"' -f 4)
    fi

    if [ -z "$download_url" ]; then
        echo "Could not find a suitable download URL for vpm-$arch-$platform"
        exit 1
    fi

    echo "Downloading VPM"
    curl "$download_url" -o "$temp/vpm-package"

    # Install based on file type
    if [[ "$download_url" == *.tar.gz ]]; then
        tar -xzf "$temp/vpm-package" -C "$temp"
        sudo cp "$temp/vpm" /usr/local/bin/vpm
    elif [[ "$download_url" == *.deb ]]; then
        sudo dpkg -i "$temp/vpm-package"
    fi
}

macos() {
    echo "Fetching the latest VPM release information"
    release_info=$(curl -s "https://api.github.com/repos/getinstachip/vpm/releases/latest")
    download_url=$(echo "$release_info" | grep "browser_download_url.*vpm-.*-$arch-$platform.tar.gz" | cut -d '"' -f 4)

    if [ -z "$download_url" ]; then
        echo "Could not find a suitable download URL for vpm-$arch-$platform.tar.gz"
        exit 1
    fi

    echo "Downloading VPM"
    curl "$download_url" -o "$temp/vpm-package.tar.gz"
    tar -xzf "$temp/vpm-package.tar.gz" -C "$temp"

    sudo cp "$temp/vpm" /usr/local/bin/vpm
}

main "$@"
