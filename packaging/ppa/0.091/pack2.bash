#!/bin/bash
exit 1

# basic init
dh_make -e me@kevincantu.org

# delete various files of .ex junk and so on
# copy the license.txt onto the tail end of copyright
# add dependencies (control)

# package build
# requires keys already be setup
debuild -S -sa

# test
sudo pbuilder build ../rust_0.09-1.dsc
# add dependencies (python, curl(maybe to remove))

# upload
dput ppa:kevincantu/rust rust_0.09-1_source.changes



