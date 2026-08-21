# Test List

The list follows ZOMBIES ordering: zero, one, many, and boundary behaviors.
Work on one unchecked item at a time. Check an item only after its test and
minimal implementation are green and committed.

## Zero

- [x] Given an empty slice, when summarized, then no summary is returned.

## One

- [x] Given one value, when summarized, then its sum is itself.
- [x] Given one value, when summarized, then its minimum is itself.
- [x] Given one value, when summarized, then its maximum is itself.
- [x] Given one value, when summarized, then its mean is itself.

## Many

- [x] Given two values, when summarized, then their sum is the total.
- [x] Given two values, when summarized, then the minimum is the smaller value.
- [x] Given two values, when summarized, then the maximum is the larger value.
- [ ] Given two values, when summarized, then their mean is the average.
- [ ] Given many values, when summarized, then their mean is the average.
