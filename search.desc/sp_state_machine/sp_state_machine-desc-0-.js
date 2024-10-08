searchState.loadedDescShard("sp_state_machine", 0, "Substrate state machine implementation.\nStorage backend trust level.\nSimple Map-based Externalities impl.\nCache type that implements <code>trie_db::TrieCache</code>.\nIn memory arrays of storage values for multiple child …\nStorage proof in compact form.\nDatabase value\nDefault error type to use with state machine trie backend.\nDefault handler of the execution manager.\nState Machine Error bound.\nExternalities Error.\nWraps a read-only backend, call executor, and current …\nTrie backend with in-memory storage.\nTransaction index operation.\nInsert transaction into index.\nTrait for inspecting state in any backend.\nMultiple key value state. States are ordered by root …\nA key value state at any storage level.\nsubstrate trie layout\nsubstrate trie layout, with external value nodes.\nState machine only allows a single level of child trie.\nReexport from <code>hash_db</code>, with genericity set for <code>Hasher</code> …\nIn memory array of storage values.\nIn-memory storage for offchain workers recoding changes …\nThe set of changes that are overlaid onto the backend.\nSimple read-only externalities for any backend.\nRenew existing transaction storage.\nThe substrate state machine.\nAccumulated usage statistics specific to state machine …\nPatricia trie-based storage trait.\nA storage changes structure that can be generated by the …\nIn memory array of storage values.\nStorage key.\nA proof that some set of key-value pairs are included in …\nStorage value.\nSimple HashMap-based Externalities impl.\nPatricia trie-based backend. Transaction type is an …\nBuilder for creating a <code>TrieBackend</code>.\nKey-value pairs storage that is used by trie backend …\nA provider of trie caches that are compatible with …\nPersistent trie database write-access interface for a …\nPersistent trie database write-access interface for a …\nA key-value datastore implemented as a database-backed …\nPanics from trusted backends are considered justified, and …\nPanics from untrusted backend are caught and interpreted …\nUsage statistics for state backend.\nMeasured count of operations and total bytes.\nAccumulates some registered stats.\nAdd transaction index operation.\nAppend a element to storage, init with existing value if …\nApply the given transaction to this backend and set the …\nReturn a new backend with all pending changes.\nReturn a <code>trie_db::TrieDB</code> compatible cache.\nReturns a cache that can be used with a <code>trie_db::TrieDBMut</code>.\nState machine backends. These manage the code and storage …\nStorage backend.\nGet backend storage reference.\nGet backend storage reference.\nBatch insert key/values into backend\nBuild the configured <code>TrieBackend</code>.\nNumber of bytes.\nSize in byte of read queries that hit a modified value.\nSize in bytes of the writes overlay operation.\nCache read statistics.\nGet an iterator over all top changes as been by the …\nGet an iterator over all top changes as been by the …\nGet an optional iterator over all child changes stored …\nGet an optional iterator over all child changes stored …\nReturns an iterator over the keys (in lexicographic order) …\nReturns a double-Option: None if the key is unknown (i.e. …\nAll changes to the child storages.\nGenerate the child storage root using <code>backend</code> and all …\nGet an iterator over all child changes as seen by the …\nGet an iterator over all child changes as seen by the …\nRemoves all key-value pairs which keys share the given …\nClear child storage of given storage key.\nRemoves all key-value pairs which keys share the given …\nCommit all pending changes to the underlying backend.\nCommit the last transaction started by <code>start_transaction</code>.\nDoes the trie contain a given key?\nDoes the trie contain a given key?\nCreate a backend used for checking the proof, using <code>H</code> as …\nLogs a message at the debug level.\nDrain all elements of changeset.\nDrain all changes into a <code>StorageChanges</code> instance. Leave …\nReturns a new empty proof.\nEmpty statistics.\nReturns the estimated encoded size of the compact proof.\nCall this before transferring control to the runtime.\nThis doesn’t test if they are in the same state, only if …\nCompare with another in-memory backend.\nGet backend essence reference.\nExecute a call using the given state backend, overlayed …\nExecute the given closure while <code>self</code>, with <code>proving_backend</code> …\nExecute the given closure while <code>self</code> is set as …\nExecute the given closure while <code>self</code> is set as …\nExecute the given closure while <code>self</code> is set as …\nExecute the given closure while <code>self</code> is set as …\nExecute the given closure <code>f</code> with the externalities set and …\nCheck execution proof, generated by <code>prove_execution</code> call.\nCheck execution proof on proving backend, generated by …\nCall this when control returns from the runtime.\nGet externalities implementation.\nList of active extensions.\nExtensions.\nExtract the <code>StorageProof</code>.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nSets raw storage key/values and a root.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a trie node.\nGet the value stored at key.\nWhat is the value of the given key in this trie?\nObtain a associated value to the given key in storage with …\nPseudo-unique id used for tracing.\nAdd collected state machine to this state.\nInsert a <code>key</code>/<code>value</code> pair into the trie. An empty value is …\nInsert key/value into backend\nInsert values into backend trie.\nInsert key/value\nInsert key/value into backend.\nInspect state with a closure.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nEncode as a compact proof with default trie layout.\nDeconstruct into the inner values\nConsume the offchain storage and iterate over all key …\nConvert into an iterator over encoded trie nodes in …\nCreates a <code>MemoryDB</code> from <code>Self</code>.\nConvert into plain node vector.\nDrains the underlying raw storage key/values and returns …\nConsumes self and returns underlying storage.\nConsume self and returns inner storages\nIs the trie empty?\nWhether no changes are contained in the top nor in any of …\nReturns whether this is an empty proof.\nIterate over all key value pairs by reference.\nReturns an iterator over the keys (in lexicographic order) …\nReturn an iterator on the compact encoded nodes.\nCreate an iterator over encoded trie nodes in …\nPair of key and values from this state.\nReturns the number of nodes in the proof.\nReturn total number of key values in states.\nLogs a message at the error level.\nAll changes to the main storage.\nMemory used.\nMerge the cached data in <code>other</code> into the provider using the …\nMerges multiple storage proofs covering potentially …\nModified value read statistics.\nCreate a new <code>Ext</code> from overlayed changes and read-only …\nCreate a new instance of <code>TestExternalities</code> with storage.\nCreate a new builder instance.\nConstructs a storage proof from a subset of encoded trie …\nCreates new substrate state machine.\nCreate a new instance of <code>BasicExternalities</code>\nNew empty test externalities.\nNew basic externalities with empty storage.\nCreate a new empty instance of in-memory backend.\nCreate a new builder instance.\nCreate a new instance of <code>TestExternalities</code> with code and …\nCreate a new instance of <code>TestExternalities</code> with code and …\nConstructs a storage proof from a subset of encoded trie …\nCreate a new instance of <code>TestExternalities</code> with storage …\nWrite trie nodes statistics.\nRead only access ot offchain overlay.\nA shared reference type around the offchain worker storage.\nConsume all changes (top + children) and return them.\nOffchain state changes to write to the offchain database.\nNumber of operations.\nReference to inner change set.\nMutable reference to inner change set.\nWrite into cached state machine change overlay.\nReturns the overlayed changes.\nStorage of parents, empty for top root or when exporting …\nMove offchain changes from overlay to the persistent store.\nGenerate child storage read proof.\nGenerate storage read proof on pre-created trie backend.\nProve execution using the given state backend, overlayed …\nProve execution using the given trie backend, overlayed …\nGenerate range storage read proof, with child tries …\nGenerate range storage read proof, with child tries …\nGenerate range storage read proof.\nGenerate range storage read proof on an existing trie …\nGenerate storage read proof.\nGenerate storage read proof on pre-created trie backend.\nCheck child storage read proof, generated by …\nCheck child storage read proof on pre-created proving …\nCheck storage read proof, generated by <code>prove_read</code> call.\nCheck storage read proof on pre-created proving backend.\nCheck child storage range proof, generated by …\nCheck storage range proof on pre-created proving backend.\nCheck storage range proof with child trie included, …\nCheck storage range proof on pre-created proving backend.\nRead statistics (total).\nNumber of read query from runtime that hit a modified …\nRegisters the given extension for this instance.\nRegister an extension.\nRemove a <code>key</code> from the trie. Equivalent to making it equal …\nRemove a key and its associated value from the offchain …\nRemoved trie nodes statistics.\nRollback the last transaction started by <code>start_transaction</code>.\nReturn the root of the trie.\nGet trie root.\nSet the value associated with a key under a prefix to the …\nSet a new value for the specified key and child.\nAsk to collect/not to collect extrinsics indices where …\nWrite a key value pair to the offchain storage overlay.\nSet the given <code>parent_hash</code> as the hash of the parent block.\nSet trie root.\nSet a new value for the specified key.\nTimespan of the statistics.\nStart a new nested transaction.\nMoment at which current statistics has been started being …\nState root of the level, for top trie it is as an empty …\nState version to use during tests.\nReturns a double-Option: None if the key is unknown (i.e. …\nRenew existing piece of data storage.\nGenerate the storage root using <code>backend</code> and all changes as …\nTally one read modified operation, of some length.\nTally one write overlay operation, of some length.\nEncode as a compact proof with default trie layout.\nCreates a <code>MemoryDB</code> from <code>Self</code> reference.\nConvert self into a <code>MemoryDB</code>.\nDecode to a full storage_proof.\nConstructs an event at the trace level.\nA transaction for the backend that contains all changes …\nReturns the current nesting depth of the transaction stack.\nChanges to the transaction index,\nGet an list of all index operations.\nThe storage root after applying the transaction.\nCopy the state, with applied updates\nMerge trie nodes into this backend.\nUpdate last keys accessed from this state.\nLogs a message at the warn level.\nUse the given <code>cache</code> for the to be configured <code>TrieBackend</code>.\nUse the given optional <code>cache</code> for the to be configured …\nUse the given optional <code>recorder</code> for the to be configured …\nUse the given <code>recorder</code> for the to be configured <code>TrieBackend</code>…\nWrap the given <code>TrieBackend</code>.\nWrite statistics (total).\nNumber of time a write operation occurs into the state …\nExtrinsic index in the current block.\nExtrinsic index in the current block.\nData content hash.\nReferenced index hash.\nIndexed data size.\nSomething that can be converted into a <code>TrieBackend</code>.\nA state backend is used to read state data and can have …\nThe state backend over which the iterator is iterating.\nWrapper to create a <code>RuntimeCode</code> from a type that …\nThe transaction type used by <code>Backend</code>.\nThe error type.\nAn error type when fetching data is not possible.\nA struct containing arguments for iterating over the …\nAn iterator over storage keys.\nAn iterator over storage keys and values.\nType of the raw storage iterator.\nA trait for a raw storage iterator.\nType of trie backend storage.\nType of trie backend storage.\nReturn the type as <code>TrieBackend</code>.\nGet the child merkle value or None if there is nothing …\nThe info of the child trie over which to iterate over.\nGet child keyed child storage or None if there is nothing …\nGet child keyed storage value hash or None if there is …\nCalculate the child storage root, with given delta over …\nGet the merkle value or None if there is nothing …\nCommit given transaction to storage.\ntrue if a key exists in child storage.\ntrue if a key exists in storage.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nCalculate the storage root, with given delta over what is …\nExtend storage info for benchmarking db\nGet the whitelist for tracking db reads/writes\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nGet an iterator over keys.\nCreate a new instance.\nReturn the next key in child storage in lexicographic …\nFetches the next key from the storage.\nFetches the next key and value from the storage.\nReturn the next key in storage in lexicographic order or …\nGet an iterator over key/value pairs.\nThe prefix of the keys over which to iterate.\nEstimate proof size\nReturns a lifetimeless raw storage iterator.\nGet the read/write count of the db\nRegister stats from overlay of state machine.\nGet the read/write count of the db\nReturn the <code>RuntimeCode</code> build from the wrapped <code>backend</code>.\nUpdate the whitelist for tracking db reads/writes\nThe prefix from which to start the iteration from.\nIf this is <code>true</code> then the iteration will <em>not</em> include the …\nWhether to stop iteration when a missing trie node is …\nGet keyed storage or None if there is nothing associated.\nGet keyed storage value hash or None if there is nothing …\nCalculate the storage root, with given delta over what is …\nQuery backend usage statistics (i/o, memory)\nReturns whether the end of iteration was reached without …\nWipe the state database.")