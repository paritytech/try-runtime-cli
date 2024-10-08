(function() {var type_impls = {
"sp_trie":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-TrieDBBuilder%3C'db,+'cache,+L%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/trie_db/triedb.rs.html#42\">source</a><a href=\"#impl-TrieDBBuilder%3C'db,+'cache,+L%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'db, 'cache, L&gt; <a class=\"struct\" href=\"trie_db/triedb/struct.TrieDBBuilder.html\" title=\"struct trie_db::triedb::TrieDBBuilder\">TrieDBBuilder</a>&lt;'db, 'cache, L&gt;<div class=\"where\">where\n    L: <a class=\"trait\" href=\"sp_trie/trait.TrieLayout.html\" title=\"trait sp_trie::TrieLayout\">TrieLayout</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/trie_db/triedb.rs.html#48\">source</a><h4 class=\"code-header\">pub fn <a href=\"trie_db/triedb/struct.TrieDBBuilder.html#tymethod.new\" class=\"fn\">new</a>(\n    db: &amp;'db dyn <a class=\"trait\" href=\"hash_db/trait.HashDBRef.html\" title=\"trait hash_db::HashDBRef\">HashDBRef</a>&lt;&lt;L as <a class=\"trait\" href=\"sp_trie/trait.TrieLayout.html\" title=\"trait sp_trie::TrieLayout\">TrieLayout</a>&gt;::<a class=\"associatedtype\" href=\"sp_trie/trait.TrieLayout.html#associatedtype.Hash\" title=\"type sp_trie::TrieLayout::Hash\">Hash</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u8.html\">u8</a>&gt;&gt;,\n    root: &amp;'db &lt;&lt;L as <a class=\"trait\" href=\"sp_trie/trait.TrieLayout.html\" title=\"trait sp_trie::TrieLayout\">TrieLayout</a>&gt;::<a class=\"associatedtype\" href=\"sp_trie/trait.TrieLayout.html#associatedtype.Hash\" title=\"type sp_trie::TrieLayout::Hash\">Hash</a> as <a class=\"trait\" href=\"hash_db/trait.Hasher.html\" title=\"trait hash_db::Hasher\">Hasher</a>&gt;::<a class=\"associatedtype\" href=\"hash_db/trait.Hasher.html#associatedtype.Out\" title=\"type hash_db::Hasher::Out\">Out</a>,\n) -&gt; <a class=\"struct\" href=\"trie_db/triedb/struct.TrieDBBuilder.html\" title=\"struct trie_db::triedb::TrieDBBuilder\">TrieDBBuilder</a>&lt;'db, 'cache, L&gt;</h4></section></summary><div class=\"docblock\"><p>Create a new trie-db builder with the backing database <code>db</code> and <code>root</code>.</p>\n<p>This doesn’t check if <code>root</code> exists in the given <code>db</code>. If <code>root</code> doesn’t exist it will fail\nwhen trying to lookup any key.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.with_cache\" class=\"method\"><a class=\"src rightside\" href=\"src/trie_db/triedb.rs.html#54\">source</a><h4 class=\"code-header\">pub fn <a href=\"trie_db/triedb/struct.TrieDBBuilder.html#tymethod.with_cache\" class=\"fn\">with_cache</a>(\n    self,\n    cache: &amp;'cache mut dyn <a class=\"trait\" href=\"sp_trie/trait.TrieCache.html\" title=\"trait sp_trie::TrieCache\">TrieCache</a>&lt;&lt;L as <a class=\"trait\" href=\"sp_trie/trait.TrieLayout.html\" title=\"trait sp_trie::TrieLayout\">TrieLayout</a>&gt;::<a class=\"associatedtype\" href=\"sp_trie/trait.TrieLayout.html#associatedtype.Codec\" title=\"type sp_trie::TrieLayout::Codec\">Codec</a>&gt;,\n) -&gt; <a class=\"struct\" href=\"trie_db/triedb/struct.TrieDBBuilder.html\" title=\"struct trie_db::triedb::TrieDBBuilder\">TrieDBBuilder</a>&lt;'db, 'cache, L&gt;</h4></section></summary><div class=\"docblock\"><p>Use the given <code>cache</code> for the db.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.with_optional_cache\" class=\"method\"><a class=\"src rightside\" href=\"src/trie_db/triedb.rs.html#61-64\">source</a><h4 class=\"code-header\">pub fn <a href=\"trie_db/triedb/struct.TrieDBBuilder.html#tymethod.with_optional_cache\" class=\"fn\">with_optional_cache</a>&lt;'ocache&gt;(\n    self,\n    cache: <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;&amp;'ocache mut dyn <a class=\"trait\" href=\"sp_trie/trait.TrieCache.html\" title=\"trait sp_trie::TrieCache\">TrieCache</a>&lt;&lt;L as <a class=\"trait\" href=\"sp_trie/trait.TrieLayout.html\" title=\"trait sp_trie::TrieLayout\">TrieLayout</a>&gt;::<a class=\"associatedtype\" href=\"sp_trie/trait.TrieLayout.html#associatedtype.Codec\" title=\"type sp_trie::TrieLayout::Codec\">Codec</a>&gt;&gt;,\n) -&gt; <a class=\"struct\" href=\"trie_db/triedb/struct.TrieDBBuilder.html\" title=\"struct trie_db::triedb::TrieDBBuilder\">TrieDBBuilder</a>&lt;'db, 'cache, L&gt;<div class=\"where\">where\n    'ocache: 'cache,</div></h4></section></summary><div class=\"docblock\"><p>Use the given optional <code>cache</code> for the db.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.with_recorder\" class=\"method\"><a class=\"src rightside\" href=\"src/trie_db/triedb.rs.html#72\">source</a><h4 class=\"code-header\">pub fn <a href=\"trie_db/triedb/struct.TrieDBBuilder.html#tymethod.with_recorder\" class=\"fn\">with_recorder</a>(\n    self,\n    recorder: &amp;'cache mut dyn <a class=\"trait\" href=\"sp_trie/trait.TrieRecorder.html\" title=\"trait sp_trie::TrieRecorder\">TrieRecorder</a>&lt;&lt;&lt;L as <a class=\"trait\" href=\"sp_trie/trait.TrieLayout.html\" title=\"trait sp_trie::TrieLayout\">TrieLayout</a>&gt;::<a class=\"associatedtype\" href=\"sp_trie/trait.TrieLayout.html#associatedtype.Hash\" title=\"type sp_trie::TrieLayout::Hash\">Hash</a> as <a class=\"trait\" href=\"hash_db/trait.Hasher.html\" title=\"trait hash_db::Hasher\">Hasher</a>&gt;::<a class=\"associatedtype\" href=\"hash_db/trait.Hasher.html#associatedtype.Out\" title=\"type hash_db::Hasher::Out\">Out</a>&gt;,\n) -&gt; <a class=\"struct\" href=\"trie_db/triedb/struct.TrieDBBuilder.html\" title=\"struct trie_db::triedb::TrieDBBuilder\">TrieDBBuilder</a>&lt;'db, 'cache, L&gt;</h4></section></summary><div class=\"docblock\"><p>Use the given <code>recorder</code> to record trie accesses.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.with_optional_recorder\" class=\"method\"><a class=\"src rightside\" href=\"src/trie_db/triedb.rs.html#79-82\">source</a><h4 class=\"code-header\">pub fn <a href=\"trie_db/triedb/struct.TrieDBBuilder.html#tymethod.with_optional_recorder\" class=\"fn\">with_optional_recorder</a>&lt;'recorder&gt;(\n    self,\n    recorder: <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;&amp;'recorder mut dyn <a class=\"trait\" href=\"sp_trie/trait.TrieRecorder.html\" title=\"trait sp_trie::TrieRecorder\">TrieRecorder</a>&lt;&lt;&lt;L as <a class=\"trait\" href=\"sp_trie/trait.TrieLayout.html\" title=\"trait sp_trie::TrieLayout\">TrieLayout</a>&gt;::<a class=\"associatedtype\" href=\"sp_trie/trait.TrieLayout.html#associatedtype.Hash\" title=\"type sp_trie::TrieLayout::Hash\">Hash</a> as <a class=\"trait\" href=\"hash_db/trait.Hasher.html\" title=\"trait hash_db::Hasher\">Hasher</a>&gt;::<a class=\"associatedtype\" href=\"hash_db/trait.Hasher.html#associatedtype.Out\" title=\"type hash_db::Hasher::Out\">Out</a>&gt;&gt;,\n) -&gt; <a class=\"struct\" href=\"trie_db/triedb/struct.TrieDBBuilder.html\" title=\"struct trie_db::triedb::TrieDBBuilder\">TrieDBBuilder</a>&lt;'db, 'cache, L&gt;<div class=\"where\">where\n    'recorder: 'cache,</div></h4></section></summary><div class=\"docblock\"><p>Use the given optional <code>recorder</code> to record trie accesses.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.build\" class=\"method\"><a class=\"src rightside\" href=\"src/trie_db/triedb.rs.html#90\">source</a><h4 class=\"code-header\">pub fn <a href=\"trie_db/triedb/struct.TrieDBBuilder.html#tymethod.build\" class=\"fn\">build</a>(self) -&gt; <a class=\"struct\" href=\"trie_db/triedb/struct.TrieDB.html\" title=\"struct trie_db::triedb::TrieDB\">TrieDB</a>&lt;'db, 'cache, L&gt;</h4></section></summary><div class=\"docblock\"><p>Build the <a href=\"trie_db/triedb/struct.TrieDB.html\" title=\"struct trie_db::triedb::TrieDB\"><code>TrieDB</code></a>.</p>\n</div></details></div></details>",0,"sp_trie::TrieDBBuilder"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()