# TAPM

The Amnesic Password Manager

## How it works

TAPM, uses the SHA256 algorithm to generate a password, however, it takes into account the little time cost of this function. To counter this, it generates a profile of iterations for that machine's hardware, with a ±10% error margin, to ensure different iterations amongst the same machines.

## To Do

- [ ] generate the iteration profile for the machine on the first launch
- [ ] flag for using a custom iteration profile
