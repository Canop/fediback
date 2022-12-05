# build a new release of fediback
# This isn't used for normal compilation but for the building of the official releases
version=$(sed 's/version = "\([0-9.]\{1,\}\)"/\1/;t;d' Cargo.toml | head -1)

echo "Building release $version"

# make the build directory and compile for all targets
./compile-all-targets.sh

# add the readme in the build directory
echo "This is fediback. More info and installation instructions on https://github.com/Canop/fediback" > build/README.md

# publish version number
echo "$version" > build/version

# prepare the release archive
rm fediback_*.zip
zip -r "fediback_$version.zip" build/*

# copy it to releases folder
mkdir releases
cp "fediback_$version.zip" releases
