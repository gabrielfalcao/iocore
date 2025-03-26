#!/bin/sh

tput clear
cat <<EOF
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

INFO
----

The panic arises within the function 'bidirectional_merge' within
library/core/src/slice/sort/shared/smallsort.rs line 837 (source code link: http://bit.ly/3Rjqj4f)

~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

HOW TO DEBUG (https://asciinema.org/a/fACGO5bApWAE8d4vrtOE4KuwF)
------------

1. once the lldb prompt opens below, run the command "process launch"
then repeat the command "p left != left_end || right != right_end"
until the repl prints "true"

2. once the repl prints "true" tell the prompt to print the variables
in the context with the command "frame variable". Take note of the
memory addresses of the variables "left", "left_end", "right" and
"right_end" for the last step.

3. type "c" and hit enter, the prompt crashes printing all the paths
underwent sorting along with their memory addresses.

4. Exit lldb then search the error output for the memory addresses
noted in step 2 and notice that the same iocore::Path address appears
to be overwritten via unsafe code in the rust core library.

~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
EOF

lldb "target/debug/deps/test_walk-1d09b5417310fdbd" --one-line "br set -f library/core/src/slice/sort/shared/smallsort.rs -l 836"
