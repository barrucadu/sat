README
======

```
# This is: a && b && c && !d

$ cargo run sat <<EOF
p cnf 4 4
1 0
2 0
3 0
-4 0
EOF

1
2
3
-4

# This is the same problem as before but the variables have an
# interpretation under EUF.  The problem now is:
#
# f(x, y) == x && f(y) == g(x) && f(f(x, y), y) == z && x != z

$ cargo run euf <<EOF
== 1(1 2) 1
== 1(2) 2(1)
== 1(1(1 2) 2) 3
== 1 3
--
p cnf 4 4
1 0
2 0
3 0
-4 0
EOF

Unsatisfiable!
```
