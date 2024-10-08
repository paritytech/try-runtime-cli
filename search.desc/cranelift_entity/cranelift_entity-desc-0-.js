searchState.loadedDescShard("cranelift_entity", 0, "Array-based data structures using densely numbered entity …\nA slice mapping <code>K -&gt; V</code> allocating dense entity references.\nA small list of entity references allocated from a pool.\nA type wrapping a small integer index should implement …\nA set of <code>K</code> for densely indexed entity references.\nIterate over all keys in order.\nIterate over all keys in order.\nIterate over all keys in order.\nA memory pool for storing lists of <code>T</code>.\nA primary mapping <code>K -&gt; V</code> allocating dense entity …\nA mapping <code>K -&gt; V</code> for densely indexed entity references.\nA sparse mapping of entity references.\nTrait for extracting keys from values stored in a <code>SparseMap</code>…\nA sparse set of entity references.\nGet the list as a mutable slice.\nGet the list as a slice.\nGet the values as a slice.\nPerforms a binary search on the values with a key …\nGet the capacity of this pool. This will be somewhat higher\nReturns the number of elements the map can hold without …\nReturns the cardinality of the set. More precisely, it …\nRemoves all elements from the list.\nClear the pool, forgetting about all lists that use it.\nRemove all entries from this map.\nRemove all entries from this map.\nRemove all entries from this set.\nRemove all elements from the mapping.\nGet the element at <code>k</code> if it exists.\nReturn <code>true</code> if the map contains a value corresponding to …\nCreate a deep clone of the list, which does not alias the …\nMacro which provides the common implementation of a 32-bit …\nAppends multiple elements to the back of the list.\nGet the first element from the list.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConstructs a list from an iterator.\nCreate a new slice from a raw pointer. A safer way to …\nCreate a new list with the contents initialized from a …\nGet the element at <code>k</code> if it exists.\nGet a single element from the list.\nGet the element at <code>k</code> if it exists.\nGet the element at <code>k</code> if it exists.\nReturns a reference to the value corresponding to the key.\nGet the element at <code>k</code> if it exists, mutable version.\nGet a mutable reference to a single element from the list.\nGet the element at <code>k</code> if it exists, mutable version.\nReturns a mutable reference to the value corresponding to …\nGrow the list by inserting <code>count</code> elements at <code>index</code>.\nGet the index that was used to create this entity …\nInserts an element as position <code>index</code> in the list, shifting …\nInsert the element at <code>k</code>.\nInsert a value into the map.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConsumes this <code>PrimaryMap</code> and produces a <code>BoxedSlice</code>.\nIs this map completely empty?\nReturns <code>true</code> if the list has a length of 0.\nIs this map completely empty?\nIs this map completely empty?\nIs this set completely empty?\nReturns true is the map contains no elements.\nCheck if <code>k</code> is a valid key in the map.\nReturns <code>true</code> if the list is valid\nCheck if <code>k</code> is a valid key in the map.\nIterate over all the keys and values in this map.\nIterate over all the keys and values in this map.\nIterate over all the keys and values in this map.\nIterate over all the keys and values in this map, mutable …\nIterate over all the keys and values in this map, mutable …\nIterate over all the keys and values in this map, mutable …\nGet the key of this sparse map value. This key is not …\nIterate over all the keys in this map.\nIterate over all the keys in this map.\nIterate over all the keys in this map.\nIterate over all the keys in this set.\nReturns the last element that was inserted in the map.\nReturns the last element that was inserted in the map.\nReturns the last element that was inserted in the map.\nGet the total number of entity references created.\nGet the number of elements in the list.\nGet the total number of entity references created.\nReturns the number of elements in the map.\nCreate a new entity reference from a small integer. This …\nCreate an <code>Iter</code> iterator that visits the <code>PrimaryMap</code> keys …\nCreate an <code>IterMut</code> iterator that visits the <code>PrimaryMap</code> keys …\nCreate a new empty list.\nCreate a new list pool.\nCreate a new empty map.\nCreate a new empty map.\nCreate a new empty set.\nCreate a new empty mapping.\nGet the key that will be assigned to the next pushed value.\nCompact representation of <code>Option&lt;T&gt;</code> for types with a …\nRemoves and returns the entity from the set if it exists.\nRemove the last value from the map.\nAppends an element to the back of the list. Returns the …\nAppend <code>v</code> to the mapping, assigning a new key which is …\nRemoves the element at position <code>index</code> from the list. …\nRemove a value from the map and return it.\nReserves capacity for at least <code>additional</code> more elements to …\nReserves the minimum capacity for exactly <code>additional</code> more …\nResize the map to have <code>n</code> entries by adding default entries …\nResize the set to have <code>n</code> entries by adding default entries …\nShrinks the capacity of the <code>PrimaryMap</code> as much as possible.\nRemoves the element at <code>index</code> in constant time by switching …\nTake all elements from this list and return them as a new …\nShortens the list down to <code>len</code> elements.\nIterate over all the values in this map.\nIterate over all the values in this map.\nIterate over all the values in this map.\nGet an iterator over the values in the map.\nIterate over all the values in this map, mutable edition.\nIterate over all the values in this map, mutable edition.\nIterate over all the values in this map, mutable edition.\nCreate a new list pool with the given capacity for data …\nCreate a new, empty map with the specified capacity.\nCreate a new empty map with the given capacity.\nCreates a new empty set with the specified capacity.\nCreate a new empty map with a specified default value.\nCreate a <code>Keys</code> iterator that visits <code>len</code> entities starting …\nPacked representation of <code>Option&lt;T&gt;</code>.\nTypes that have a reserved value which can’t be created …\nCreate a default packed option representing <code>None</code>.\nExpand the packed option into a normal <code>Option</code>.\nUnwrap a packed <code>Some</code> value or panic.\nReturns the argument unchanged.\nConvert <code>t</code> into a packed <code>Some(x)</code>.\nConvert an option into its packed equivalent.\nCalls <code>U::from(self)</code>.\nReturns <code>true</code> if the packed option is a <code>None</code> value.\nChecks whether value is the reserved one.\nReturns <code>true</code> if the packed option is a <code>Some</code> value.\nMaps a <code>PackedOption&lt;T&gt;</code> to <code>Option&lt;U&gt;</code> by applying a function …\nCreate an instance of the reserved value.\nTakes the value out of the packed option, leaving a <code>None</code> …\nUnwrap a packed <code>Some</code> value or panic.")