git fetch mozilla && git checkout incoming && git merge mozilla/incoming && time CFG_DISABLE_VALGRIND=1 make check && echo "INCOMING IS CLEAN"
