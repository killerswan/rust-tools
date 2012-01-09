#!/bin/bash
###########################################################
# Kevin Cantu <me@kevincantu.org>
#
# create and upload source package files
# for Rust on Ubuntu
# to https://launchpad.net/~kevincantu/+archive/rust


###########################################################
# ARGS

if [ ! 2 -eq $# ]
then
   echo "Usage: $(basename "$0") VERSION COMMIT"
   exit 1
else
   VERSION="$1"
   COMMIT="$2"
   HERE="$(cd "$(dirname "$0")"; pwd)"
fi


###########################################################
# GET THIS VERSION OF SOURCE READY

# TODO: this is inefficient
git clone https://github.com/graydon/rust rust

# just to be sure
#git reset --hard d2218d9c9cb4cbd28e3de44c152f1b270f185e58
git reset --hard ${COMMIT}

# llvm and libuv source
cd rust
git submodule init
git submodule update

# stage0 binaries
CFG_SRC_DIR="$(pwd)"
SNAPSHOTDIR="$(pwd)/src/etc"
export CFG_SRC_DIR
export SNAPSHOTDIR
mkdir dl
python ${HERE}/latest-snapshots.py

# original source archive
PKGDIR=pkg
ORIG=rust_${VERSION}.orig.tar.gz
cd ${HERE}
mkdir ${PKGDIR}
tar -cvz --exclude-vcs -f ${PKGDIR}/${ORIG} rust


###########################################################
# MODIFICATIONS FOR PACKAGING

# extract original source
cd ${PKGDIR}
tar -xzf ${ORIG}
mv rust rust-${VERSION}

# init package
cd rust-${VERSION}
mkdir debian
DEBFULLNAME="Kevin Cantu"
dh_make --email me@kevincantu.org \
        --single \
        --overlay ${HERE}/deb-templ

gvim --nofork debian/control   # eyeball dependencies
gvim --nofork debian/changelog # comment on changes
cp LICENSE.txt debian/
cp AUTHORS.txt debian/
cp README      debian/

echo "###########################################################"
echo "Now ready to build, test, and upload the package..."
echo ""
#echo "apply patches to original"
#echo "..."
#echo ""
echo "package build and sign:"
echo "$ debuild -S -sa"
echo ""
echo "test binary package creation locally:"
echo "$ sudo pbuilder build ../rust_VERSION.dsc"
echo ""
echo "upload:"
echo "$ dput ppa:kevincantu/rust rust_VERSION_source.changes"
echo ""
echo "###########################################################"


