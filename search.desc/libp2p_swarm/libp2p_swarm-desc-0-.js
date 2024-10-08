searchState.loadedDescShard("libp2p_swarm", 0, "High-level network manager.\nPending connection attempt has been aborted.\nPending connection attempt has been aborted.\nEvent generated by the <code>NetworkBehaviour</code>.\nA connection with the given peer has been closed, possibly …\nNetwork connection information.\nA connection was denied.\nErrors that can occur in the context of an established …\nA connection to the given peer has been opened.\nConnection identifier.\nPossible errors when trying to establish or upgrade an …\nThe provided <code>dial_opts::PeerCondition</code> evaluated to false …\nA new dialing attempt has been initiated by the …\nImplemented on objects that can run a <code>Future</code> in the …\nOne of our listeners has reported the expiration of a …\nThe connection handler produced an error.\nAn I/O error occurred on the connection.\nA new connection arrived on a listener and is in the …\nAn error happened on an inbound connection during its …\nThe connection keep-alive timeout expired.\nPossible errors when upgrading an inbound connection.\nOne of the listeners gracefully closed.\nOne of the listeners reported a non-fatal error.\nThe peer identity obtained on the connection matches the …\nThe connection was dropped because it resolved to our own …\nSubstream for which a protocol has been chosen.\nGenerates a delegating <code>NetworkBehaviour</code> implementation for …\nInformation about the connections obtained by …\nOne of our listeners has reported a new local listening …\nNo addresses have been provided by …\nAn error happened on an outbound connection.\nIdentifies a protocol for a stream.\nContains the state of the network, plus the way it should …\nA <code>SwarmBuilder</code> provides an API for configuring and …\nEvent generated by the <code>Swarm</code>.\nParameters passed to <code>poll()</code>, that the <code>NetworkBehaviour</code> has …\n<code>ConnectionHandler</code> of the <code>NetworkBehaviour</code> for all the …\nCustom error that can be produced by the <code>ConnectionHandler</code> …\nCustom event that can be received by the <code>ConnectionHandler</code> …\nCustom event that can be produced by the <code>ConnectionHandler</code> …\nAn error occurred while negotiating the transport …\nAn error occurred while negotiating the transport …\nThe peer identity obtained on the connection did not match …\nThe peer identity obtained on the connection did not match …\nAdd a <strong>confirmed</strong> external address for the local node.\nGet the <code>Multiaddr</code> that is being listened on\nReturns a reference to the provided <code>NetworkBehaviour</code>.\nReturns a mutable reference to the provided …\nBuilds a <code>Swarm</code> with the current configuration.\nAttempt to gracefully close a connection.\nReturns the currently connected peers.\nGets counters for ongoing network connections.\nDial a known or unknown peer.\nNumber of addresses concurrently dialed for a single …\nDisconnects a peer by its peer ID, closing all connections …\nAttempt to downcast to a particular reason for why the …\nAttempt to downcast to a particular reason for why the …\nRun the given future in the background until it ends.\nList all <strong>confirmed</strong> external address for the local node.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nOnce a connection to a remote peer is established, a …\nHow long to keep a connection alive once it is idling.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nChecks whether there is an established connection to a …\nStarts listening on the given address. Returns an error if …\nGet the <code>ListenerId</code> of this listen attempt\nReturns an iterator that produces the list of addresses we…\nReturns the peer ID of the swarm passed as parameter.\nThe maximum number of inbound streams concurrently …\nReturns information about the connections underlying the …\nCreates a new <code>Swarm</code> from the given <code>Transport</code>, …\nConstruct a new protocol from a static string slice.\nCreates an <em>unchecked</em> <code>ConnectionId</code>.\nConfigures the number of events from the <code>NetworkBehaviour</code> …\nThe total number of connections, both pending and …\nThe total number of established connections.\nThe number of established incoming connections.\nThe number of established outgoing connections.\nThe number of connected peers, i.e. peers with whom at …\nThe total number of pending connections, both incoming and …\nThe number of incoming connections being established.\nThe number of outgoing connections being established.\nConfigures the size of the buffer for events sent by a …\nRemove an external address for the local node.\nRemove some listener.\nConfigures an override for the substream upgrade protocol …\nAttempt to construct a protocol from an owned string.\nExtract the <code>TBehaviourOutEvent</code> from this <code>SwarmEvent</code> in …\nNumber of addresses concurrently dialed for a single …\nCreates a new <code>Config</code> from the given executor. The <code>Swarm</code> is …\nCreates a new <code>SwarmBuilder</code> from the given transport, …\nHow long to keep a connection alive once it is idling.\nThe maximum number of inbound streams concurrently …\nConfigures the number of events from the <code>NetworkBehaviour</code> …\nConfigures the size of the buffer for events sent by a …\nConfigures an override for the substream upgrade protocol …\nBuilds a new <code>Config</code> from the given <code>tokio</code> executor.\nBuilds a new <code>SwarmBuilder</code> from the given transport, …\nCreates a new <code>SwarmBuilder</code> from the given transport, …\nThe new address that is being listened on.\nThe expired address.\nThe addresses that the listener was listening on. These …\nReason for the disconnection, if it was not a successful …\n<code>Some</code> when the new connection is an outgoing connection. …\nIdentifier of the connection.\nIdentifier of the connection.\nIdentifier of the connection.\nIdentifier of the connection.\nIdentifier of the connection.\nIdentifier of the connection.\nEndpoint of the connection that has been opened.\nEndpoint of the connection that has been closed.\nThe error that happened.\nError that has been encountered.\nThe listener error.\nHow long it took to establish this connection\nThe listener that is listening on the new address.\nThe listener that is no longer listening on the address.\nThe listener that closed.\nThe listener that errored.\nLocal connection address. This address has been earlier …\nLocal connection address. This address has been earlier …\nNumber of established connections to this peer, including …\nNumber of other remaining connections to this same peer.\nIdentity of the peer that we have connected to.\nIdentity of the peer that we have connected to.\nIf known, <code>PeerId</code> of the peer we tried to reach.\nIdentity of the peer that we are connecting to.\nReason for the closure. Contains <code>Ok(())</code> if the stream …\nAddress used to send back data to the remote.\nAddress used to send back data to the remote.\n<code>FromSwarm</code> variant that informs the behaviour that the …\nInforms the behaviour that the <code>ConnectedPoint</code> of an …\nDisconnect all connections.\nNotify an arbitrary connection handler.\nThe options which connections to close.\nInstructs the <code>Swarm</code> to initiate a graceful close of one or …\n<code>FromSwarm</code> variant that informs the behaviour about a …\nInforms the behaviour about a closed connection to a peer.\n<code>FromSwarm</code> variant that informs the behaviour about a newly …\nInforms the behaviour about a newly established connection …\nHandler for all the protocols the network behaviour …\nInstructs the swarm to start a dial.\n<code>FromSwarm</code> variant that informs the behaviour that the dial …\nInforms the behaviour that the dial to a known or unknown …\n<code>FromSwarm</code> variant that informs the behaviour that a …\nInforms the behaviour that a multiaddr we were listening …\n<code>FromSwarm</code> variant that informs the behaviour that an …\nIndicates to the <code>Swarm</code> that the provided address is …\nInforms the behaviour that an external address of the …\n<code>FromSwarm</code> variant that informs the behaviour that an …\nIndicates to the <code>Swarm</code> that we are no longer externally …\nInforms the behaviour that an external address of the …\nUtility struct for tracking the external addresses of a …\nEnumeration with the list of the possible events to pass …\nInstructs the <code>Swarm</code> to return an event when it is being …\nUtility struct for tracking the addresses a <code>Swarm</code> is …\n<code>FromSwarm</code> variant that informs the behaviour that an error …\nInforms the behaviour that an error happened on an …\nInstructs the <code>Swarm</code> to listen on the provided address.\n<code>FromSwarm</code> variant that informs the behaviour that a …\nInforms the behaviour that a listener closed.\n<code>FromSwarm</code> variant that informs the behaviour that a …\nInforms the behaviour that a listener experienced an error.\nA <code>NetworkBehaviour</code> defines the behaviour of the local node …\n<code>FromSwarm</code> variant that informs the behaviour about a new …\nReports a new candidate for an external address to the …\nInforms the behaviour that we have discovered a new …\n<code>FromSwarm</code> variant that informs the behaviour that we have …\nInforms the behaviour that we have started listening on a …\n<code>FromSwarm</code> variant that informs the behaviour that a new …\nInforms the behaviour that a new listener was created.\nThe options w.r.t. which connection handler to notify of …\nInstructs the <code>Swarm</code> to send an event to the handler …\nNotify a particular connection handler.\nDisconnect a particular connection.\nParameters passed to <code>poll()</code>, that the <code>NetworkBehaviour</code> has …\nInstructs the <code>Swarm</code> to remove the listener.\nIterator returned by <code>supported_protocols</code>.\nA command issued from a <code>NetworkBehaviour</code> for the <code>Swarm</code>.\nEvent generated by the <code>NetworkBehaviour</code> and that the swarm …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCallback that is invoked for every established inbound …\nCallback that is invoked for every established outbound …\nCallback that is invoked for every new inbound connection.\nCallback that is invoked for every outbound connection …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns an <code>Iterator</code> over all external addresses.\nReturns an <code>Iterator</code> over all listen addresses.\nMap the handler event.\nMap the event the swarm will return.\nInforms the behaviour about an event generated by the …\nInforms the behaviour about an event from the <code>Swarm</code>.\nFeed a <code>FromSwarm</code> event to this struct.\nFeed a <code>FromSwarm</code> event to this struct.\nPolls for things that swarm should do.\nReturns the list of protocol the behaviour supports when a …\nWhether to close a specific or all connections to the …\nThe event to send.\nThe options w.r.t. which connection handler to notify of …\nThe peer for whom a <code>ConnectionHandler</code> should be notified.\nThe peer to disconnect.\nImplementation of <code>NetworkBehaviour</code> that can be either in …\nImplementation of <code>ConnectionHandler</code> that can be in the …\nReturns a mutable reference to the inner <code>NetworkBehaviour</code>.\nReturns a reference to the inner <code>NetworkBehaviour</code>.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns <code>true</code> if <code>Toggle</code> is enabled and <code>false</code> if it’s …\nA new dialing attempt is always initiated, only subject to …\nOptions to configure a dial to a known or unknown peer.\nA new dialing attempt is initiated <em>only if</em> the peer is …\nA new dialing attempt is initiated <em>only if</em> there is …\nThe available conditions under which a new dialing attempt …\nSpecify a single address to dial the unknown peer.\nSpecify a set of addresses to be used to dial the known …\nBuild the final <code>DialOpts</code>.\nBuild the final <code>DialOpts</code>.\nBuild the final <code>DialOpts</code>.\nSpecify a <code>PeerCondition</code> for the dial.\nSpecify a <code>PeerCondition</code> for the dial.\nGet the <code>ConnectionId</code> of this dial attempt.\nIn addition to the provided addresses, extend the set via …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nRetrieves the <code>PeerId</code> from the <code>DialOpts</code> if specified or …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nOverride Number of addresses concurrently dialed for a …\nOverride Number of addresses concurrently dialed for a …\nOverride role of local node on connection. I.e. execute …\nOverride role of local node on connection. I.e. execute …\nOverride role of local node on connection. I.e. execute …\nDial a known peer.\nDial an unknown peer.\nImplementation of <code>NetworkBehaviour</code> that doesn’t do …\nAn implementation of <code>ConnectionHandler</code> that neither …\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThe remote now supports these additional protocols.\n<code>ConnectionEvent</code> variant that informs the handler about a …\nInforms the handler about a change in the address of the …\nThe upgrade produced an error.\nThe upgrade produced an error.\nClose the connection for the given reason.\nEnumeration with the list of the possible stream events to …\nA handler for a set of protocols used on a connection with …\nEvent produced by a handler.\nImplementation of <code>ConnectionHandler</code> that combines two …\n<code>ConnectionEvent</code> variant that informs the handler that …\nInforms the handler that upgrading an outbound substream …\nEquivalent to <code>OutboundUpgrade::Error</code>.\nEquivalent to <code>InboundUpgrade::Error</code>.\nThe type of errors returned by <code>ConnectionHandler::poll</code>.\nA type representing the message(s) a <code>NetworkBehaviour</code> can …\n<code>ConnectionEvent</code> variant that informs the handler about the …\nInforms the handler about the output of a successful …\n<code>ConnectionEvent</code> variant that informs the handler about …\nInforms the handler about the output of a successful …\nEquivalent to <code>OutboundUpgrade::Future</code>.\nEquivalent to <code>InboundUpgrade::Future</code>.\nThe type of additional information returned from …\nThe inbound upgrade for the protocol(s) used by the …\nImplemented automatically on all types that implement …\nEquivalent to <code>UpgradeInfo::Info</code>.\nEquivalent to <code>UpgradeInfo::InfoIter</code>.\nAn IO or otherwise unrecoverable error happened.\nAn IO or otherwise unrecoverable error happened.\nHow long the connection should be kept alive.\n<code>ConnectionEvent</code> variant that informs the handler that …\nInforms the handler that upgrading an inbound substream to …\nThe local <code>ConnectionHandler</code> added or removed support for …\nWrapper around a protocol handler that turns the input …\nWrapper around a protocol handler that turns the output …\nNo protocol could be agreed upon.\nNo protocol could be agreed upon.\nClose the connection as soon as possible.\nEvent that is sent to a <code>NetworkBehaviour</code>.\nA <code>ConnectionHandler</code> that opens a new substream for each …\nConfiguration parameters for the <code>OneShotHandler</code>\nThe type of additional information passed to an …\nThe outbound upgrade for the protocol(s) used by the …\nRequest a new outbound substream to be opened with the …\nImplemented automatically on all types that implement …\nEquivalent to <code>OutboundUpgrade::Output</code>.\nEquivalent to <code>InboundUpgrade::Output</code>.\nImplementation of <code>ConnectionHandler</code> that returns a pending …\nAn <code>Iterator</code> over all protocols that have been added.\n<code>ConnectionEvent</code> variant that informs the handler about a …\nAn <code>Iterator</code> over all protocols that have been removed.\nThe remote <code>ConnectionHandler</code> now supports a different set …\nThe remote no longer supports these protocols.\nWe learned something about the protocols supported by the …\nWraps around a type that implements <code>OutboundUpgradeSend</code>, …\nError that can happen on an outbound substream opening …\nConfiguration of inbound or outbound substream protocol(s) …\nThe opening attempt timed out before the negotiation was …\nThe opening attempt timed out before the negotiation was …\nA type representing message(s) a <code>ConnectionHandler</code> can …\nIf nothing new happens, the connection should be closed at …\nImplemented automatically on all types that implement …\nKeep the connection alive.\nReturns until when the connection should be kept alive.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nBorrows the contained protocol info.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConverts the substream protocol configuration into the …\nWhether the event concerns an inbound stream.\nWhether the event concerns an outbound stream.\nReturns true for <code>Yes</code>, false otherwise.\nKeep-alive timeout for idle connections.\nThe <code>InboundUpgrade</code> to apply on inbound substreams to …\nReturns a mutable reference to the listen protocol …\nReturns a reference to the listen protocol configuration.\nIf this is a <code>Close</code> event, maps the content to something …\nIf this is a <code>Custom</code> event, maps the content to something …\nAdds a closure that turns the input event into something …\nMaps a function over the protocol info.\nAdds a closure that turns the output event into something …\nIf this is an <code>OutboundSubstreamRequest</code>, maps the <code>info</code> …\nIf this is an <code>OutboundSubstreamRequest</code>, maps the protocol (…\nMaps a function over the protocol upgrade.\nMap the inner <code>StreamUpgradeError</code> type.\nMaximum number of concurrent outbound substreams being …\nA <code>ConnectionHandler</code> implementation that combines multiple …\nCreates a <code>OneShotHandler</code>.\nCreate a new <code>SubstreamProtocol</code> from the given upgrade.\nInforms the handler about an event from the …\nTimeout for outbound substream upgrades.\nReturns the number of pending requests.\nShould behave like <code>Stream::poll()</code>.\nEquivalent to <code>UpgradeInfo::protocol_info</code>.\nCreates a new <code>ConnectionHandler</code> that selects either this …\nOpens an outbound substream with <code>upgrade</code>.\nBorrows the timeout for the protocol upgrade.\nBorrows the contained protocol upgrade.\nEquivalent to <code>InboundUpgrade::upgrade_inbound</code>.\nEquivalent to <code>OutboundUpgrade::upgrade_outbound</code>.\nSets a new timeout for the protocol upgrade.\nThe protocol(s) to apply on the substream.\nIt is an error if two handlers share the same protocol …\nIndex and protocol name pair used as <code>UpgradeInfo::Info</code>.\nThe aggregated <code>InboundOpenInfo</code>s of supported inbound …\nA <code>ConnectionHandler</code> for multiple <code>ConnectionHandler</code>s of the …\nInbound and outbound upgrade for all <code>ConnectionHandler</code>s.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThe protocol name bytes that occured in more than one …\nCreate and populate a <code>MultiHandler</code> from the given handler …\nImplementation of <code>NetworkBehaviour</code> that doesn’t do …\nImplementation of <code>ConnectionHandler</code> that doesn’t handle …\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.")