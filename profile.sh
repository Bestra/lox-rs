sudo dtrace -c './target/release/lox examples/loop.lox' -o out.stacks -n 'profile-997 /execname == "lox"/ { @[ustack(100)] = count(); }'
~/personal/FlameGraph/stackcollapse.pl out.stacks | ~/personal/FlameGraph/flamegraph.pl > graph.svg
