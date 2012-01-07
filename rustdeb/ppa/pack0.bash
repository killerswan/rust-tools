#!/bin/bash

if [ ! 2 -eq $# ]
then
   echo "Usage: $(basename "$0") HASH REL_VERSION"
   exit 1
else
   HASH="$1"
   VERSION="$2"
fi

# rust source
git clone https://github.com/graydon/rust rust_${VERSION}

# llvm and libuv source
cd rust_${VERSION}
git submodule init
git submodule update

# just to be sure
#git reset --hard d2218d9c9cb4cbd28e3de44c152f1b270f185e58
git reset --hard ${HASH}


