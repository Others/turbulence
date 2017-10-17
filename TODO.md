TODO
====

Implement more PRNGs:
---------------------
- SFMT
    - See [this paper](http://www.math.sci.hiroshima-u.ac.jp/~m-mat/MT/ARTICLES/sfmt.pdf)
    - Blocked on stable SIMD
    - And u128 support would also be great for ease of implementation
    - (Unstable only implementation under consideration)
- PCG 
- MWC
- xorshiro
- MT19937-64
- Generalize the XorShift implementation


Implement CSPRNGs (if deemed safe):
----------------
- ChaCha20
- ISAAC64

Implement statistical tests:
----------------------------
- Birthday spacings
- etc.


Misc:
-----
- Write README
- Add threadsafe RNG
- Generalize the SeedableRng trait 
- Add a way of composing RNGs
- Consider a more general way of writing RNGs 
    - Get rid of boilerplate based on generator word size
- Better document where implementation information comes from