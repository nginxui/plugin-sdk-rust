#!/bin/sh
# Builds the reference plugin in release mode and assembles the plugin
# directory for the platform it runs on: the manifest, the documents and the
# executable the manifest names. The manifest lists that platform only, like
# a per platform package does.
#
# Usage: ./package.sh [output directory]
set -eu

here=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$here/../.." && pwd)
out=${1:-$root/target/plugin-package}

os=$(uname -s | tr '[:upper:]' '[:lower:]')
case $(uname -m) in
x86_64 | amd64) arch=amd64 ;;
arm64 | aarch64) arch=arm64 ;;
*)
	echo "package.sh: unsupported architecture $(uname -m)" >&2
	exit 1
	;;
esac

(cd "$root" && cargo build --release -p reference-plugin)

rm -rf "$out"
mkdir -p "$out/bin"
sed -e "s/@PLATFORM@/$os-$arch/g" -e 's/@EXT@//g' "$here/plugin.json.in" >"$out/plugin.json"
cp "$here/README.md" "$here/CHANGELOG.md" "$out/"
cp "$root/LICENSE" "$out/LICENSE"
cp "$root/target/release/reference-plugin" "$out/bin/reference-plugin-$os-$arch"
echo "plugin directory: $out"
