lp-shell << EOF
execfile('/code/rust-tools/packaging/ppa/stats.py')
getDownloadCounts('rust')
EOF
