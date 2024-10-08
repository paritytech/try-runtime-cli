searchState.loadedDescShard("tokio_util", 0, "Utilities for working with Tokio.\nAdaptors from <code>AsyncRead</code>/<code>AsyncWrite</code> to Stream/Sink\nCompatibility between the <code>tokio::io</code> and <code>futures-io</code> …\nModule defining an Either type.\nHelpers for IO related tasks.\nSynchronization primitives\nA simple <code>Decoder</code> and <code>Encoder</code> implementation that splits up …\nAn error occurred while encoding or decoding a chunk.\nA simple <code>Decoder</code> and <code>Encoder</code> implementation that just …\nDecoding of frames via buffers.\nTrait of helper objects to write out messages as bytes, …\nThe type of unrecoverable frame decoding errors.\nThe type of encoding errors.\nA unified <code>Stream</code> and <code>Sink</code> interface to an underlying I/O …\n<code>FramedParts</code> contains an export of the data of a Framed …\nA <code>Stream</code> of messages decoded from an <code>AsyncRead</code>.\nA <code>Sink</code> of frames encoded to an <code>AsyncWrite</code>.\nAn IO error occurred.\nAn IO error occurred.\nThe type of decoded frames.\nA simple <code>Decoder</code> and <code>Encoder</code> implementation that splits up …\nAn error occurred while encoding or decoding a line.\nThe maximum chunk length was exceeded.\nThe maximum line length was exceeded.\nReturns backpressure boundary\nReturns backpressure boundary\nReturns a reference to the underlying codec wrapped by …\nThe codec\nReturns a mutable reference to the underlying codec …\nReturns a mutable reference to the underlying codec …\nAttempts to decode a frame from the provided buffer of …\nA default method available to be called when there are no …\nA default method available to be called when there are no …\nReturns a reference to the underlying decoder.\nReturns a mutable reference to the underlying decoder.\nReturns a mutable reference to the underlying decoder.\nEncodes a frame into the buffer provided.\nReturns a reference to the underlying encoder.\nReturns a mutable reference to the underlying encoder.\nReturns a mutable reference to the underlying encoder.\nProvides a <code>Stream</code> and <code>Sink</code> interface for reading and …\nProvides a <code>Stream</code> and <code>Sink</code> interface for reading and …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nProvides a <code>Stream</code> and <code>Sink</code> interface for reading and …\nReturns a mutable reference to the underlying I/O stream …\nReturns a mutable reference to the underlying I/O stream …\nReturns a mutable reference to the underlying I/O stream …\nReturns a pinned mutable reference to the underlying I/O …\nReturns a pinned mutable reference to the underlying I/O …\nReturns a pinned mutable reference to the underlying I/O …\nReturns a reference to the underlying I/O stream wrapped by\nReturns a reference to the underlying I/O stream wrapped by\nReturns a reference to the underlying I/O stream wrapped by\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConsumes the <code>FramedWrite</code>, returning its underlying I/O …\nConsumes the <code>FramedRead</code>, returning its underlying I/O …\nConsumes the <code>Framed</code>, returning its underlying I/O stream.\nConsumes the <code>Framed</code>, returning its underlying I/O stream, …\nThe inner transport used to read bytes to and write bytes …\nFrame a stream of bytes based on a length prefix\nMaps the codec <code>U</code> to <code>C</code>, preserving the read and write …\nMaps the decoder <code>D</code> to <code>C</code>, preserving the read buffer …\nMaps the encoder <code>E</code> to <code>C</code>, preserving the write buffer …\nReturns the maximum line length when decoding.\nReturns the maximum chunk length when decoding.\nCreates a new <code>BytesCodec</code> for shipping around raw bytes.\nCreate a new, default, <code>FramedParts</code>\nReturns a <code>LinesCodec</code> for splitting up data into lines.\nReturns a <code>AnyDelimiterCodec</code> for splitting up data into …\nCreates a new <code>FramedWrite</code> with the given <code>encoder</code>.\nCreates a new <code>FramedRead</code> with the given <code>decoder</code>.\nProvides a <code>Stream</code> and <code>Sink</code> interface for reading and …\nReturns a <code>LinesCodec</code> with a maximum line length limit.\nReturns a <code>AnyDelimiterCodec</code> with a maximum chunk length …\nThe buffer with read but unprocessed data.\nReturns a reference to the read buffer.\nReturns a reference to the read buffer.\nReturns a mutable reference to the read buffer.\nReturns a mutable reference to the read buffer.\nUpdates backpressure boundary\nUpdates backpressure boundary\nCreates a new <code>FramedRead</code> with the given <code>decoder</code> and a …\nProvides a <code>Stream</code> and <code>Sink</code> interface for reading and …\nA buffer with unprocessed data which are not written yet.\nReturns a reference to the write buffer.\nReturns a reference to the write buffer.\nReturns a mutable reference to the write buffer.\nReturns a mutable reference to the write buffer.\nConfigure length delimited <code>LengthDelimitedCodec</code>s.\nA codec for frames delimited by a frame head specifying …\nAn error when the number of bytes read is more than max …\nRead the length field as a big endian integer\nCreates a new length delimited codec builder with default …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nDelta between the payload length specified in the header …\nSets the number of bytes used to represent the length field\nSets the number of bytes in the header before the length …\nSets the unsigned integer type used to represent the …\nRead the length field as a little endian integer\nSets the max frame length in bytes\nReturns the current max frame setting\nRead the length field as a native endian integer\nCreates a new length delimited codec builder with default …\nCreates a new <code>LengthDelimitedCodec</code> with the default …\nCreate a configured length delimited <code>LengthDelimitedCodec</code>\nCreate a configured length delimited <code>Framed</code>\nCreate a configured length delimited <code>FramedRead</code>\nCreate a configured length delimited <code>FramedWrite</code>\nSets the number of bytes to skip before reading the payload\nUpdates the max frame setting.\nA compatibility layer that allows conversion between the …\nExtension trait that allows converting a type implementing …\nExtension trait that allows converting a type implementing …\nExtension trait that allows converting a type implementing …\nExtension trait that allows converting a type implementing …\nWraps <code>self</code> with a compatibility layer that implements …\nWraps <code>self</code> with a compatibility layer that implements …\nWraps <code>self</code> with a compatibility layer that implements …\nWraps <code>self</code> with a compatibility layer that implements …\nReturns the argument unchanged.\nGet a mutable reference to the <code>Future</code>, <code>Stream</code>, <code>AsyncRead</code>, …\nGet a reference to the <code>Future</code>, <code>Stream</code>, <code>AsyncRead</code>, or …\nCalls <code>U::from(self)</code>.\nReturns the wrapped item.\nCombines two different futures, streams, or sinks having …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nA helper that wraps a <code>Sink</code><code>&lt;</code><code>Bytes</code><code>&gt;</code> and converts it into a …\nAn adapter that lets you inspect the data that’s being …\nAn adapter that lets you inspect the data that’s being …\nConvert an <code>AsyncRead</code> into a <code>Stream</code> of byte chunks.\nConvert a <code>Sink</code> of byte chunks into an <code>AsyncWrite</code>.\nConvert a <code>Stream</code> of byte chunks into an <code>AsyncRead</code>.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGets a mutable reference to the underlying stream.\nGets a mutable reference to the underlying sink.\nGets a mutable reference to the underlying sink.\nGets a pinned mutable reference to the underlying stream.\nGets a reference to the underlying stream.\nGets a reference to the underlying sink.\nGets a reference to the underlying sink.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConsumes this <code>BufWriter</code>, returning the underlying stream.\nConsumes this <code>SinkWriter</code>, returning the underlying sink.\nConsumes the <code>InspectWriter</code>, returning the wrapped writer\nConsumes the <code>InspectReader</code>, returning the wrapped reader\nConsumes this <code>CopyToBytes</code>, returning the underlying sink.\nConsumes this <code>StreamReader</code>, returning a Tuple consisting …\nConvert a stream of byte chunks into an <code>AsyncRead</code>.\nCreates a new <code>SinkWriter</code>.\nConvert an <code>AsyncRead</code> into a <code>Stream</code> with item type …\nCreate a new <code>InspectWriter</code>, wrapping <code>write</code> and calling <code>f</code> …\nCreate a new <code>InspectReader</code>, wrapping <code>reader</code> and calling <code>f</code> …\nCreates a new <code>CopyToBytes</code>.\nTry to read data from an <code>AsyncRead</code> into an implementer of …\nTry to write data from an implementer of the <code>Buf</code> trait to …\nRead data from an <code>AsyncRead</code> into an implementer of the …\nConvert an <code>AsyncRead</code> into a <code>Stream</code> with item type …\nA token which can be used to signal a cancellation request …\nA wrapper for cancellation token which automatically …\nA wrapper around <code>Semaphore</code> that provides a <code>poll_acquire</code> …\nError returned by the <code>PollSender</code> when the channel is …\nA wrapper around <code>mpsc::Sender</code> that can be polled.\nA reusable <code>Pin&lt;Box&lt;dyn Future&lt;Output = T&gt; + Send + &#39;a&gt;&gt;</code>.\nA Future that is resolved once the corresponding …\nA Future that is resolved once the corresponding …\nAborts the current in-progress send, if any.\nAdds <code>n</code> new permits to the semaphore.\nReturns the current number of available permits.\nCancel the <code>CancellationToken</code> and all child tokens which …\nReturns a <code>Future</code> that gets fulfilled when cancellation is …\nReturns a <code>Future</code> that gets fulfilled when cancellation is …\nCreates a <code>CancellationToken</code> which will get cancelled …\nCreates a clone of the <code>CancellationToken</code> which will get …\nClones this <code>PollSender</code>.\nObtain a clone of the inner semaphore.\nCloses the semaphore.\nCloses this sender.\nReturns stored cancellation token and removes this drop …\nCreates a <code>DropGuard</code> for this token.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet a pinned reference to the underlying future.\nGets a reference to the <code>Sender</code> of the underlying channel.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nGet back the inner semaphore.\nConsumes the stored value, if any.\nReturns <code>true</code> if the <code>CancellationToken</code> is cancelled.\nChecks whether this sender is been closed.\nCreates a new <code>CancellationToken</code> in the non-cancelled state.\nCreate a new <code>PollSemaphore</code>.\nCreate a new <code>ReusableBoxFuture&lt;T&gt;</code> containing the provided …\nCreates a new <code>PollSender</code>.\nPoll the future stored inside this box.\nPoll the future stored inside this box.\nPoll to acquire a permit from the semaphore.\nPoll to acquire many permits from the semaphore.\nAttempts to prepare the sender to receive a value.\nRuns a future to completion and returns its result wrapped …\nSends an item to the channel.\nReplace the future currently stored in this box.\nReplace the future currently stored in this box.")