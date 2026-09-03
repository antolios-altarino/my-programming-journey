# Exercise: Conditionals 10
#
# Putting it all together: classify a number using several conditions.
#
# Complete `classify(n)` so it returns:
#   "large positive"   when n > 100
#   "small positive"   when n > 0  (and not > 100)
#   "zero"             when n == 0
#   "small negative"   when n >= -100  (and not 0 or positive)
#   "large negative"   when n < -100
#
# Use an if / elif / elif / elif / else chain.

def classify(n):
    if n > 100:
        return "large positive"
    elif n > 0:   
        return "small positive"
    elif n == 0:   
        return "zero"
    elif n >= -100:   
        return "small negative"
    else:   
        return "large negative"