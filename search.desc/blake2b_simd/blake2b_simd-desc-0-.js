searchState.loadedDescShard("blake2b_simd", 0, "GitHub crates.io Actions Status\nThe number input bytes passed to each call to the …\nA finalized BLAKE2 hash, with constant-time equality.\nThe max key length.\nThe max hash length.\nThe max personalization length.\nA parameter builder that exposes all the non-default …\nThe max salt length.\nAn incremental hasher for BLAKE2b.\nConvert the hash to a byte array. Note that if you’re …\nConvert the hash to a byte slice. Note that if you’re …\nCompute the BLAKE2b hash of a slice of bytes all at once, …\nBLAKE2bp, a variant of BLAKE2b that uses SIMD more …\nReturn the total number of bytes input so far.\nFrom 0 (meaning unlimited) to 255. The default is 1 …\nFinalize the state and return a <code>Hash</code>. This method is …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nHash an input all at once with these parameters.\nSet the length of the final hash in bytes, from 1 to …\nFrom 0 (the default, meaning sequential) to <code>OUTBYTES</code> (64).\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nUse a secret key, so that BLAKE2 acts as a MAC. The …\nIndicates the rightmost node in a row. This can also be …\nInterfaces for hashing multiple inputs at once, using SIMD …\nFrom 0 (meaning BLAKE2X B2 hashes), through 1 (the …\nFrom 0 (the default, meaning unlimited or sequential) to …\nEquivalent to <code>Params::default()</code>.\nEquivalent to <code>State::default()</code> or …\nFrom 0 (the default, meaning leaf or sequential) to 255.\nFrom 0 (the default, meaning first, leftmost, leaf, or …\nAt most <code>PERSONALBYTES</code> (16). Shorter personalizations are …\nAt most <code>SALTBYTES</code> (16). Shorter salts are padded with null …\nSet a flag indicating that this is the last node of its …\nConvert the hash to a lowercase hexadecimal <code>ArrayString</code>.\nConstruct a <code>State</code> object based on these parameters, for …\nAdd input to the hash. You can call <code>update</code> any number of …\nA parameter builder for BLAKE2bp, just like the <code>Params</code> …\nAn incremental hasher for BLAKE2bp, just like the <code>State</code> …\nCompute the BLAKE2bp hash of a slice of bytes all at once, …\nReturn the total number of bytes input so far.\nFinalize the state and return a <code>Hash</code>. This method is …\nReturns the argument unchanged.\nReturns the argument unchanged.\nHash an input all at once with these parameters.\nSet the length of the final hash, from 1 to <code>OUTBYTES</code> (64). …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nUse a secret key, so that BLAKE2bp acts as a MAC. The …\nEquivalent to <code>Params::default()</code>.\nEquivalent to <code>State::default()</code> or …\nConstruct a BLAKE2bp <code>State</code> object based on these …\nAdd input to the hash. You can call <code>update</code> any number of …\nA job for the <code>hash_many</code> function. After calling <code>hash_many</code> …\nThe largest possible value of <code>degree</code> on the target …\nThe parallelism degree of the implementation, detected at …\nReturns the argument unchanged.\nHash any number of complete inputs all at once.\nCalls <code>U::from(self)</code>.\nConstruct a new <code>HashManyJob</code> from a set of hashing …\nGet the hash from a finished job. If you call this before …\nUpdate any number of <code>State</code> objects at once.")