# Exercise: Conditionals 9
#
# The ternary (conditional) expression lets you pick a value in one line:
#
#   result = value_if_true if condition else value_if_false
#
# Complete `absolute(n)` so it returns n when n >= 0, and -n otherwise.
# Write it as a single return statement using a ternary expression.

def absolute(n):
    return n if n >= 0 else -n
