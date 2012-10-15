git fetch mozilla && git checkout master && git merge mozilla/master && time CFG_DISABLE_VALGRIND=1 make check && echo "MASTER IS CLEAN"
