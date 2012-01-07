#!/bin/bash

if [ ! 2 -eq $# ]
then
   echo "Usage: $(basename "$0") HASH REL_VERSION"
   exit 1
else
   HASH="$1"
   VERSION="$2"
fi

# stage0 binaries
cd rust_${VERSION}
CFG_SRC_DIR="$(pwd)"
export CFG_SRC_DIR
SNAPSHOTDIR="$(pwd)/src/etc"
export SNAPSHOTDIR
mkdir dl
python ../latest-snapshots.py

# source archive
cd ..
tar -cvz --exclude-vcs -f rust_${VERSION}.orig.tar.gz rust_${VERSION}


