# Exercise: Conditionals 3
#
# `elif` lets you chain multiple conditions.  Python tests them in order
# and runs the first branch whose condition is True.
#
# Complete `grade(score)` so it returns:
#   "A" for score >= 90
#   "B" for score >= 80
#   "C" for score >= 70
#   "F" for anything below 70

def grade(score):
    if score >= 90:
        return "A"
    elif score >= 80:
        return "B"
    elif score >= 70:
        return "C"
    # add the remaining elif / else branches
    else:
        return "F"
