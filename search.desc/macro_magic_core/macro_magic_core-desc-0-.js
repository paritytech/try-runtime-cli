searchState.loadedDescShard("macro_magic_core", 0, "This crate contains most of the internal implementation of …\nUsed to parse args passed to the inner pro macro …\nCorresponds with <code>#[proc_macro_attribute]</code>\nCorresponds with <code>#[proc_macro_derive]</code>\nShould be implemented by structs that will be passed to …\nUsed to parse args that were passed to …\nUsed to parse args that were passed to …\nUsed to parse args that were passed to …\nUsed to parse the args for the <code>import_tokens_internal</code> …\nUsed to parse the args for the <code>import_tokens_inner_internal</code>…\nConstant used to load the configured location for …\nCorresponds with <code>#[proc_macro]</code>\nGenerically parses a proc macro definition with support …\nDelineates the different types of proc macro\nSpecifies the <code>Ident</code> for the <code>attr</code> parameter of this proc …\nwhen <code>#[with_custom_parsing(..)]</code> is used, the variable …\nInternal implementation of <code>export_tokens_alias!</code>. Allows …\nThe internal code behind the <code>#[export_tokens]</code> attribute …\nProduces the full path for the auto-generated …\nResolves to the path of the <code>#[export_tokens]</code> macro for the …\nOptional extra data. This is how …\nOptional extra data. This is how …\n“Flattens” an <code>Ident</code> by converting it to snake case.\nReturns the path of the foreign item whose tokens will be …\nUsed by <code>forward_tokens_internal</code>.\nThe internal implementation for the <code>forward_tokens</code> macro.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConstructs a <code>ProcMacro</code> from anything compatible with …\nReturns the argument unchanged.\nInternal implementation for the <code>#[import_tokens_attr]</code> …\nThe internal implementation for the <code>import_tokens_inner</code> …\nThe internal implementation for the <code>import_tokens</code> macro.\nInternal implementation for the <code>#[import_tokens_proc]</code> …\nContains the <code>Item</code> that is being imported (i.e. the item …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThe item whose tokens are being forwarded\nContains the <code>Item</code> that has been imported.\nSafely access a subpath of <code>macro_magic</code>\nSafely access the <code>macro_magic</code> root based on the …\nSpecified the type of this proc macro, i.e. attribute vs …\nContains the override path that will be used instead of …\nParses a proc macro function from a <code>TokenStream2</code> expecting …\nSafely access a subpath of <code>macro_magic::__private</code>\nThe underlying proc macro function definition\nThe path of the item whose tokens are being forwarded\nRepresents the path of the item that is being imported.\nThe <code>Path</code> where the item we are importing can be found.\nContains the underlying <code>TokenStream2</code> inside the brace.\nThe path of the macro that will receive the forwarded …\nThe path of the macro that will receive the forwarded …\nGets the <code>Attribute</code> representation of this proc macro type\nReturns the specified string in snake_case\nGets the <code>&amp;&#39;static str</code> representation of this proc macro …\nA <code>TokenStream2</code> representing the raw tokens for the <code>Ident</code> …\nSpecifies the <code>Ident</code> for the <code>tokens</code> parameter of this proc …\nThe <code>Ident</code> for the <code>tokens</code> variable. Usually called <code>tokens</code> …\nRepresents the <code>Ident</code> that was used to refer to the <code>tokens</code> …\nThe internal implementation for the …")