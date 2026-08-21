# Test List

The list follows ZOMBIES ordering: zero, one, many, and boundary behaviors.
Work on one unchecked item at a time. Check an item only after its test and
minimal implementation are green and committed.

## Zero

- [x] Given an empty slice, when summarized, then `EmptyInput` is returned.

## One

- [x] Given one value, when summarized, then all statistics equal that value.

## Many

- [x] Given two values, when summarized, then all statistics are correct.

## Floating-point boundaries

- [ ] Given signed zeros, when summaries are compared, then their signs do not affect equality.
- [ ] Given one NaN, when summarized, then its zero-based index is returned in an error.
- [ ] Given multiple NaNs, when summarized, then all zero-based indices are returned in an error.
