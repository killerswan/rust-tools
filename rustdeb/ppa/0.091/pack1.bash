#!/bin/bash

if [ ! 2 -eq $# ]
then
   echo "Usage: $(basename "$0") HASH REL_VERSION"
   exit 1
else
   VERSION="$1"
   HASH="$2"
fi

# stage0 binaries
cd rust
CFG_SRC_DIR="$(pwd)"
export CFG_SRC_DIR
SNAPSHOTDIR="$(pwd)/src/etc"
export SNAPSHOTDIR
mkdir dl
python ../latest-snapshots.py

# source archive
PKGDIR=pkg
ORIG=rust_${VERSION}.orig.tar.gz
cd ..
mkdir ${PKGDIR}
tar -cvz --exclude-vcs -f ${PKGDIR}/${ORIG} rust

cd ${PKGDIR}
tar -xzf ${ORIG}
mv rust rust-${VERSION}


