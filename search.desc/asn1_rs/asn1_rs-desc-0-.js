searchState.loadedDescShard("asn1_rs", 0, "License: MIT Apache License 2.0 docs.rs crates.io Download …\n<code>Application</code> class of tags (<code>0b01</code>)\nThe <code>Any</code> object is not strictly an ASN.1 type, but holds a …\n<code>Application</code> class of tags (<code>0b01</code>)\nA helper object to parse <code>[APPLICATION n] EXPLICIT T</code>\nA helper object to parse <code>[APPLICATION n] IMPLICIT T</code>\nHelper trait for creating tagged EXPLICIT values\nHelper trait for creating tagged IMPLICIT values\nBerAlias custom derive\nBER recursive parsing reached maximum depth\nBerSequence custom derive\nBerSet custom derive\nBER object does not have the expected type\nBER object does not have the expected value\nNon-special values\nASN.1 <code>BITSTRING</code> type\nASN.1 <code>BMPSTRING</code> type\nASN.1 <code>BOOLEAN</code> type\n<code>Context-Specific</code> class of tags (<code>0b10</code>)\nVerification of DER constraints\nBER Object class of tag\nDER object was expected to be constructed (and found to be …\nDER object was expected to be primitive (and found to be …\nObject must not be constructed\n<code>Context-Specific</code> class of tags (<code>0b10</code>)\nDefinite form (X.690 8.1.3.3)\nDerAlias custom derive\nTrait to automatically derive <code>FromDer</code>\nError types for DER constraints\nDER Failed constraint: {0:?}\nDerSequence custom derive\nDerSet custom derive\nEnd-of-contents octets\nASN.1 <code>ENUMERATED</code> type\nThe <code>Err</code> enum indicates the parser was not successful\nContains the error value\nContains the error value\nContains the error value\nContains the error value\nThe error type for operations of the <code>FromBer</code>, <code>FromDer</code>, and …\nThe parser had an error (recoverable)\nA type parameter for <code>EXPLICIT</code> tagged values.\n<code>BOOLEAN</code> object for value <code>false</code>\nThe parser had an unrecoverable error: we got to the right …\nSignalizes that the first or second component is too large.\nBase trait for BER object parsers\nBase trait for DER object parsers\nASN.1 restricted character string type (<code>GeneralString</code>)\nASN.1 restricted character string type (<code>GraphicString</code>)\nBER/DER object header (identifier and length)\nHolds the result of parsing functions\nASN.1 restricted character string type (<code>Ia5String</code>)\nA type parameter for <code>IMPLICIT</code> tagged values.\nThere was not enough data\nincomplete data, missing: {0:?}\nIndefinite form (X.690 8.1.3.6)\nIndefinite length not allowed\nIndefinite length not allowed\nInfinity (∞).\nASN.1 <code>INTEGER</code> type\nInteger must not be empty\nLeading 0xff in negative Integer encoding\nLeading zeroes in Integer encoding\nBER integer is negative, while an unsigned integer was …\nInteger too large to fit requested type\nBoolean value must be 0x00 of 0xff\nInvalid Date or Time\nInvalid Length\nInvalid Tag\nInvalid Value when parsing object with tag {tag:?} {msg:}\nBER Object Length\nRequesting borrowed data from a temporary object\nDateTime object is missing seconds\nDateTime object is missing timezone\nContains information on needed data if a parser returned …\nNegative infinity (−∞).\nnom error: {0:?}\nNo value.\nNo value.\nObject must be constructed\nASN.1 <code>NULL</code> type\nASN.1 restricted character string type (<code>NumericString</code>)\nASN.1 restricted character string type (<code>ObjectDescriptor</code>)\nASN.1 <code>OCTETSTRING</code> type\nLocal zone, with offset to coordinated universal time\nObject ID (OID) representation which can be relative or …\nAn error for OID parsing functions.\nContains the success value\nContains the success value\nContains the success value\nContains the success value\nA helper object to parse <code>[ n ] EXPLICIT T OPTIONAL</code>\nA helper object to parse <code>[ n ] IMPLICIT T OPTIONAL</code>\nHelper object to parse TAGGED OPTIONAL types (explicit or …\n<code>Private</code> class of tags (<code>0b11</code>)\nHolds the result of BER/DER serialization functions\nASN.1 restricted character string type (<code>PrintableString</code>)\n<code>Private</code> class of tags (<code>0b11</code>)\nA helper object to parse <code>[PRIVATE n] EXPLICIT T</code>\nA helper object to parse <code>[PRIVATE n] IMPLICIT T</code>\nASN.1 <code>REAL</code> type\nA specialized <code>Result</code> type for all operations from this …\nThe <code>SEQUENCE</code> object is an ordered list of heteregeneous …\nAn Iterator over binary data, parsing elements of type <code>T</code>\nThe <code>SEQUENCE OF</code> object is an ordered list of homogeneous …\nThe error type for serialization operations of the <code>ToDer</code> …\nHolds the result of BER/DER encoding functions\nThe <code>SET</code> object is an unordered list of heteregeneous types.\nAn Iterator over binary data, parsing elements of type <code>T</code>\nThe <code>SET OF</code> object is an unordered list of homogeneous …\nContains the required data size in bytes\nSome value of type <code>T</code>.\nSome value of type <code>T</code>.\nInvalid encoding or forbidden characters in string\n<code>BOOLEAN</code> object for value <code>true</code>\nBER/DER Tag as defined in X.680 section 8.4\nA type parameter for tagged values either <code>Explicit</code> or …\nA helper object to parse <code>[ n ] EXPLICIT T</code>\nA helper object to parse <code>[ n ] IMPLICIT T</code>\nA builder for parsing tagged values (<code>IMPLICIT</code> or <code>EXPLICIT</code>)\nHelper object for creating <code>FromBer</code>/<code>FromDer</code> types for …\nASN.1 restricted character string type (<code>TeletexString</code>)\nBase trait for BER string objects and character set …\nCommon trait for all objects that can be encoded using the …\n<code>Universal</code> class of tags (<code>0b00</code>)\nNo timezone provided\nUnexpected Class (expected: {expected:?}, actual: …\nUnexpected Tag (expected: {expected:?}, actual: {actual:?})\n<code>Universal</code> class of tags (<code>0b00</code>)\nASN.1 <code>UniversalString</code> type\nNeeds more data, but we do not know how much\nUnknown tag: {0:?}\nFeature is not yet implemented\nBitstring unused bits must be set to zero\nASN.1 restricted character string type (<code>Utf8String</code>)\nASN.1 restricted character string type (<code>VideotexString</code>)\nASN.1 restricted character string type (<code>VisibleString</code>)\nCoordinated universal time\nZero\nApply the parsing function to the sequence content, …\nApply the parsing function to the set content, consuming …\nCreates a borrowed <code>Any</code> for this object\nAttempt to create ASN.1 type <code>BITSTRING</code> from this object.\nAttempt to create ASN.1 type <code>BMPString</code> from this object.\nAttempt to create ASN.1 type <code>BOOLEAN</code> from this object.\nAttempt to create ASN.1 type <code>BOOLEAN</code> from this object.\nGet the bytes representation of the <em>content</em>\nGet the encoded oid without the header.\nGet the bytes representation of the <em>content</em>\nAttempt to create ASN.1 type <code>EMBEDDED PDV</code> from this object.\nAttempt to create ASN.1 type …\nAttempt to create ASN.1 type <code>ENUMERATED</code> from this object.\nAttempt to create ASN.1 type <code>GeneralizedTime</code> from this …\nAttempt to create ASN.1 type <code>GeneralString</code> from this …\nAttempt to create ASN.1 type <code>GraphicString</code> from this …\nAttempt to create ASN.1 type <code>INTEGER</code> from this object.\nAttempts to convert an <code>Integer</code> to a <code>i128</code>.\nAttempt to create ASN.1 type <code>INTEGER</code> from this object.\nAttempts to convert an <code>Integer</code> to a <code>i16</code>.\nAttempt to create ASN.1 type <code>INTEGER</code> from this object.\nAttempts to convert an <code>Integer</code> to a <code>i32</code>.\nAttempt to create ASN.1 type <code>INTEGER</code> from this object.\nAttempts to convert an <code>Integer</code> to a <code>i64</code>.\nAttempt to create ASN.1 type <code>INTEGER</code> from this object.\nAttempts to convert an <code>Integer</code> to a <code>i8</code>.\nAttempt to create ASN.1 type <code>IA5String</code> from this object.\nAttempt to create ASN.1 type <code>INTEGER</code> from this object.\nAttempt to create ASN.1 type <code>NULL</code> from this object.\nAttempt to create ASN.1 type <code>NumericString</code> from this …\nAttempt to create ASN.1 type <code>OBJECT IDENTIFIER</code> from this …\nAttempt to create ASN.1 type <code>OCTET STRING</code> from this object.\nAttempt to create ASN.1 type <code>OBJECT IDENTIFIER</code> from this …\nAttempt to create an <code>Option&lt;T&gt;</code> from this object.\nAttempt to create ASN.1 type <code>PrintableString</code> from this …\nAttempt to create ASN.1 type <code>REAL</code> from this object.\nAttempt to create ASN.1 type <code>RELATIVE-OID</code> from this object.\nAttempt to create ASN.1 type <code>SEQUENCE</code> from this object.\nAttempt to create ASN.1 type <code>SET</code> from this object.\nAttempt to create ASN.1 type <code>UTF8String</code> from this object.\nAttempt to create ASN.1 type <code>UTF8String</code> from this object.\nAttempt to create a tagged value (EXPLICIT) from this …\nAttempt to create a tagged value (IMPLICIT) from this …\nAttempt to create ASN.1 type <code>TeletexString</code> from this …\nAttempt to create ASN.1 type <code>INTEGER</code> from this object.\nAttempts to convert an <code>Integer</code> to a <code>u128</code>.\nAttempt to create ASN.1 type <code>INTEGER</code> from this object.\nAttempts to convert an <code>Integer</code> to a <code>u16</code>.\nAttempt to create ASN.1 type <code>INTEGER</code> from this object.\nAttempts to convert an <code>Integer</code> to a <code>u32</code>.\nAttempt to create ASN.1 type <code>INTEGER</code> from this object.\nAttempts to convert an <code>Integer</code> to a <code>u64</code>.\nAttempt to create ASN.1 type <code>INTEGER</code> from this object.\nAttempts to convert an <code>Integer</code> to a <code>u8</code>.\nAttempt to create ASN.1 type <code>UniversalString</code> from this …\nAttempt to create ASN.1 type <code>UTCTime</code> from this object.\nAttempt to create ASN.1 type <code>UTF8String</code> from this object.\nAttempt to create ASN.1 type <code>VideotexString</code> from this …\nAttempt to create ASN.1 type <code>VisibleString</code> from this …\nReturn error if class is not the expected class\nReturn error if object is primitive\nReturn error if object length is definite\nReturn error if length is not definite\nReturn error if object is not primitive\nReturn error if tag is not the expected tag\nReturn an iterator over the sequence content, attempting …\nReturn an iterator over the set content, attempting to …\nCreate the BER parser from the builder parameters\nAttempt to parse the sequence as a <code>SEQUENCE OF</code> items …\nAttempt to parse the set as a <code>SET OF</code> items (BER), and …\nCreate a new binary <code>REAL</code>\nAttempt to convert object to <code>BitString&lt;&#39;a&gt;</code> (ASN.1 type: …\nAttempt to convert object to <code>BmpString&lt;&#39;a&gt;</code> (ASN.1 type: …\nAttempt to convert object to <code>bool</code> (ASN.1 type: <code>BOOLEAN</code>).\nReturn the <code>bool</code> value from this object.\nAttempt to convert object to <code>Boolean</code> (ASN.1 type: <code>BOOLEAN</code>).\nGet the encoded oid without the header.\nIs the provided <code>Tag</code> decodable as a variant of this <code>CHOICE</code>?\nReturn the <code>Class</code> of this object\nReturn the (outer) class of this object\nReturn the class of this header.\nThe expected class for the object to parse\nReturn true if this header has the ‘constructed’ flag.\nSerialized DER representation of the sequence content\nSerialized DER representation of the set content\nAutomatically converts between errors if the underlying …\nThe object contents\nGet length of primitive object\nReturn an iterator over the sequence content, attempting …\nReturn an iterator over the set content, attempting to …\nCreate the DER parser from the builder parameters\nAttempt to parse the sequence as a <code>SEQUENCE OF</code> items …\nAttempt to parse the set as a <code>SET OF</code> items (DER), and …\nAdditional documentation: recipes, specific use cases and …\nAttempt to convert object to <code>EmbeddedPdv&lt;&#39;a&gt;</code> (ASN.1 type: …\nAttempt to convert object to <code>Enumerated</code> (ASN.1 type: …\nCreate a <code>TagParser</code> builder for <code>EXPLICIT</code> tagged values\nConstructs a new <code>EXPLICIT TaggedParser</code> with the provided …\nReturns the ‘f32’ value of this <code>REAL</code>.\nReturns the ‘f64’ value of this <code>REAL</code>.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nBuild an OID from an array of object identifier components.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nBuild a <code>TaggedOptional</code> object with class <code>ContextSpecific</code> …\nBuild a <code>TaggedOptional</code> object with class <code>ContextSpecific</code> …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nAttempt to parse input bytes into a BER object\nParse a BER value and apply the provided parsing function …\nSame as <code>Sequence::from_der_and_then</code>, but using BER …\nSame as <code>Set::from_der_and_then</code>, but using BER encoding (no …\nParse a BER tagged value and apply the provided parsing …\nBuild an <code>Integer</code> from a constant array of bytes …\nAttempt to parse input bytes into a DER object (enforcing …\nParse a DER value and apply the provided parsing function …\nParse a DER sequence and apply the provided parsing …\nParse a DER set and apply the provided parsing function to …\nParse a DER tagged value and apply the provided parsing …\nConverts a <code>i128</code> to an <code>Integer</code>\nConverts a <code>i16</code> to an <code>Integer</code>\nConverts a <code>i32</code> to an <code>Integer</code>\nConverts a <code>i64</code> to an <code>Integer</code>\nConverts a <code>i8</code> to an <code>Integer</code>\nAttempt to create a <code>Sequence</code> from an iterator over …\nAttempt to create a <code>Set</code> from an iterator over serializable …\nFlatten all <code>nom::Err</code> variants error into a single error …\nBuild a relative OID from an array of object identifier …\nCreate a new <code>Any</code> from a tag, and BER/DER content\nConverts a <code>u128</code> to an <code>Integer</code>\nConverts a <code>u16</code> to an <code>Integer</code>\nConverts a <code>u32</code> to an <code>Integer</code>\nConverts a <code>u64</code> to an <code>Integer</code>\nConverts a <code>u8</code> to an <code>Integer</code>\nAttempt to convert object to <code>GeneralizedTime</code> (ASN.1 type: …\nAttempt to convert object to <code>GeneralString&lt;&#39;a&gt;</code> (ASN.1 …\nAttempt to convert object to <code>GraphicString&lt;&#39;a&gt;</code> (ASN.1 …\nThe object header\nAttempt to convert object to <code>i128</code> (ASN.1 type: <code>INTEGER</code>).\nAttempt to convert object to <code>i16</code> (ASN.1 type: <code>INTEGER</code>).\nAttempt to convert object to <code>i32</code> (ASN.1 type: <code>INTEGER</code>).\nAttempt to convert object to <code>i64</code> (ASN.1 type: <code>INTEGER</code>).\nAttempt to convert object to <code>i8</code> (ASN.1 type: <code>INTEGER</code>).\nAttempt to convert object to <code>Ia5String&lt;&#39;a&gt;</code> (ASN.1 type: …\nCreate a <code>TagParser</code> builder for <code>IMPLICIT</code> tagged values\nConstructs a new <code>IMPLICIT TaggedParser</code> with the provided …\nHelper macro to declare integers at compile-time\nHelper macro to declare integers at compile-time\nAttempt to convert object to <code>Integer&lt;&#39;a&gt;</code> (ASN.1 type: …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nAttempt to parse the sequence as a <code>SEQUENCE OF</code> items (BER) …\nAttempt to parse the set as a <code>SET OF</code> items (BER) …\nConsume the sequence and return the content\nConsume the set and return the content\nGet the bytes representation of the <em>content</em>\nGet the bytes representation of the encoded oid\nAttempt to parse the sequence as a <code>SEQUENCE OF</code> items (DER) …\nAttempt to parse the set as a <code>SET OF</code> items (DER) …\nConsumes the <code>TaggedParser</code>, returning the wrapped value.\nConverts <code>self</code> into a vector without clones or allocation.\nConverts <code>self</code> into a vector without clones or allocation.\nBuild an error from the provided invalid value\nTest if object class is Application\nTest if object is constructed\nTest if object class is Context-specific\nReturn true if length is definite\nReturns <code>true</code> if this number is not infinite.\nTests if the result is Incomplete\nReturns <code>true</code> if this value is positive infinity or …\nIndicates if we know how many bytes we need\nReturn true if length is definite and equal to 0\nTest if object is primitive\nTest if object class is Private\nTest if bit <code>bitnum</code> is set\nTest if object class is Universal\nReturn an iterator over the sub-identifiers (arcs). …\nReturn the length of this header.\nMaps a <code>Needed</code> to <code>Needed</code> by applying a function to a …\nApplies the given function to the inner error\nMaps <code>Err&lt;error::Error&lt;T&gt;&gt;</code> to <code>Err&lt;error::Error&lt;U&gt;&gt;</code> with the …\nMaps <code>Err&lt;(T, ErrorKind)&gt;</code> to <code>Err&lt;(U, ErrorKind)&gt;</code> with the …\nCreate a new <code>Any</code> from BER/DER header and content\nCreate a new <code>Boolean</code> from the provided logical value.\nCreates a new <code>Integer</code> containing the given value …\nCreate an OID from the ASN.1 DER encoded form. See the …\nCreate a new <code>REAL</code> from the <code>f64</code> value.\nBuilds a <code>SEQUENCE OF</code> from the provided content\nBuild a sequence, given the provided content\nBuilds a <code>SET OF</code> from the provided content\nBuild a set, given the provided content\nCreates <code>Needed</code> instance, returns <code>Needed::Unknown</code> if the …\nCreate a default <code>TaggedParserBuilder</code> builder\nBuild a new <code>OptTaggedParser</code> object.\nBuild a new BER/DER header from the provided values\nCreate a relative OID from the ASN.1 DER encoded form. See …\nBuild a new BER/DER header from the provided tag, with …\nAttempt to convert object to <code>Null</code> (ASN.1 type: <code>NULL</code>).\nAttempt to convert object to <code>NumericString&lt;&#39;a&gt;</code> (ASN.1 …\nAttempt to convert object to <code>ObjectDescriptor&lt;&#39;a&gt;</code> (ASN.1 …\nAttempt to convert object to <code>OctetString&lt;&#39;a&gt;</code> (ASN.1 type: …\nAttempt to convert object to <code>Oid&lt;&#39;a&gt;</code> (ASN.1 type: …\nHelper macro to declare integers at compile-time\nHelper macro to declare integers at compile-time\nApply the parsing function to the sequence content …\nApply the parsing function to the set content …\nParse input as BER, and apply the provided function to …\nGet the content following a BER header\nGet the content following a BER header\nParse input as DER, and apply the provided function to …\nGet the content following a DER header\nGet the content following a DER header\nApply the parsing function to the sequence content …\nApply the parsing function to the set content (consuming …\nAttempt to convert object to <code>PrintableString&lt;&#39;a&gt;</code> (ASN.1 …\nAppends an element to the back of a collection\nAppends an element to the back of a collection\nReturn the raw tag encoding, if it was stored in this …\nAttempt to convert object to <code>Real</code> (ASN.1 type: <code>REAL</code>).\nAttempt to convert object to <code>Oid</code> (ASN.1 type: <code>RELATIVE-OID</code>…\nAttempt to convert object to <code>Sequence&lt;&#39;a&gt;</code> (ASN.1 type: …\nAttempt to convert object to <code>Set&lt;&#39;a&gt;</code> (ASN.1 type: <code>SET</code>).\nReturns true if <code>needle</code> is a prefix of the OID.\nAttempt to convert object to <code>&amp;&#39;a str</code> (ASN.1 type: …\nAttempt to convert object to <code>String</code> (ASN.1 type: <code>UTF8String</code>…\nReturn the <code>Tag</code> of this object\nReturn the (outer) tag of this object\nReturn the tag of this header.\nThe expected tag for the object to parse\nAttempt to convert object to <code>TeletexString&lt;&#39;a&gt;</code> (ASN.1 …\nCheck character set for this object type.\nReturns the number of non-leap seconds since the midnight …\nGet the length of the object (including the header), when …\nWrite the DER encoded representation to a newly allocated …\nWrite the DER encoded representation to a newly allocated …\nSimilar to using <code>to_vec</code>, but uses provided values without …\nSimilar to using <code>to_vec</code>, but uses provided values without …\nConvert the OID to a string representation.\nCreate a deep copy of the oid.\nObtaining ownership\nObtaining ownership\nObtaining ownership\nObtaining ownership\nAttempt to convert object to <code>u128</code> (ASN.1 type: <code>INTEGER</code>).\nAttempt to convert object to <code>u16</code> (ASN.1 type: <code>INTEGER</code>).\nAttempt to convert object to <code>u32</code> (ASN.1 type: <code>INTEGER</code>).\nAttempt to convert object to <code>u64</code> (ASN.1 type: <code>INTEGER</code>).\nAttempt to convert object to <code>u8</code> (ASN.1 type: <code>INTEGER</code>).\nBuild an error from the provided unexpected class\nBuild an error from the provided unexpected tag\nAttempt to convert object to <code>UniversalString&lt;&#39;a&gt;</code> (ASN.1 …\nReturn an adjusted ISO 8601 combined date and time with …\nReturn a ISO 8601 combined date and time with time zone.\nReturn a ISO 8601 combined date and time with time zone.\nAttempt to convert object to <code>UtcTime</code> (ASN.1 type: <code>UTCTime</code>).\nAttempt to convert object to <code>Utf8String&lt;&#39;a&gt;</code> (ASN.1 type: …\nAttempt to convert object to <code>VideotexString&lt;&#39;a&gt;</code> (ASN.1 …\nAttempt to convert object to <code>VisibleString&lt;&#39;a&gt;</code> (ASN.1 …\nUpdate the class of the current object\nSet the expected <code>Class</code> for the builder\nSet the class of this <code>Header</code>\nSet the constructed flags of this <code>Header</code>\nSet the length of this <code>Header</code>\nUpdate header to add reference to raw tag\nUpdate the tag of the current object\nSet the expected <code>Tag</code> for the builder\nSet the tag of this <code>Header</code>\nAttempt to write the DER encoded representation (header …\nAttempt to write the DER encoded representation (header …\nAttempt to write the DER content (all except header) to …\nAttempt to write the DER header to this writer.\nSimilar to using <code>to_der</code>, but uses provided values without …\nSimilar to using <code>to_der</code>, but uses provided values without …\nSimilar to using <code>to_der</code>, but uses header without computing …\nSimilar to using <code>to_der</code>, but uses header without computing …\nDebugging\nBER/DER Custom Derive Attributes\nDocumentation: BER/DER parsing recipes")