searchState.loadedDescShard("sp_staking", 0, "A crate which contains primitives that are useful for …\nAccountId type used by the staking system.\nAccountId type used by the staking system.\nAccountId type used by the staking system.\nA type that belongs only in the context of an <code>Agent</code>.\nBalance type used by the staking system.\nBalance type used by the staking system.\nBalance type used by the staking system.\nMeans of converting Currency to VoteWeight.\nTrait to provide delegation functionality for stakers.\nTrait to provide functionality for direct stakers to …\nA type that belongs only in the context of a <code>Delegator</code>.\nCounter for the number of eras that have passed.\nA snapshot of the stake backing a single validator in the …\nA snapshot of the stake backing a single validator in the …\nChilling.\nThe amount of exposure for an era that an individual …\nDeclaring desire to nominate, delegate, or generally …\nA generic staking event listener.\nType for identifying a page.\nMetadata for Paged Exposure of a validator such as total …\nSimple index type with which we can count sessions.\nA struct that reflects stake that an account has in the …\nRepresentation of the status of a staker.\nRepresentation of a staking account, which may be a stash …\nA generic representation of a staking implementation.\nSet of low level apis to manipulate staking ledger.\nDeclaring desire in validate, i.e author blocks.\nThe total amount of the stash’s balance that will be at …\nTotal active portion of a staker’s <code>Stake</code>, <code>Err</code> if not a …\nReturns effective balance of the <code>Agent</code> account. <code>None</code> if …\nReturns the total amount of funds that is unbonded and can …\nBond (lock) <code>value</code> of <code>who</code>’s balance, while forwarding any …\nBond some extra amount in <code>who</code>’s free balance against the …\nNumber of eras that staked funds must remain bonded for.\nChill <code>who</code>.\nThe current era index.\nAdd delegation to the <code>Agent</code>.\nReturns the total amount of funds delegated. <code>None</code> if not a …\nApply a pending slash to an <code>Agent</code> by slashing <code>value</code> from …\nThe ideal number of active validators.\nWhether or not there is an ongoing election.\nForce a current staker to become completely unstaked, …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nReturns whether a staker is FULLY unbonding, <code>Err</code> if not a …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nSplits an <code>Exposure</code> into <code>PagedExposureMetadata</code> and multiple …\nChecks whether an account <code>staker</code> has been exposed in an …\nReturns whether a staker is unbonding, <code>Err</code> if not a staker …\nChecks whether or not this is a validator account.\nChecks whether the staker is a virtual account.\nMigrate <code>value</code> of delegation to <code>delegator</code> from a migrating …\nMigrate an existing <code>Nominator</code> to <code>Agent</code> account.\nMigrate an existing staker to a virtual staker.\nThe minimum amount required to bond in order to set …\nThe minimum amount required to bond in order to set …\nHave <code>who</code> nominate <code>validators</code>.\nGet the nominations of a stash, if they are a nominator, …\nNumber of nominators backing this validator.\nCommon traits and types that are useful for describing …\nFired when someone sets their intention to nominate.\nFired when someone removes their intention to nominate, …\nFired when an existing nominator updates their nominations.\nFired when a staker is slashed.\nFired when the stake amount of someone updates.\nFired when someone is fully unstaked.\nFired when someone sets their intention to validate.\nFired when someone removes their intention to validate, …\nFired when an existing validator updates their preferences.\nFired when a portion of a staker’s balance has been …\nThe portions of nominators stashes that are exposed.\nThe portions of nominators stashes that are exposed.\nThe validator’s own stash that is exposed.\nThe validator’s own stash that is exposed.\nNumber of pages of nominators.\nThe total balance of this chunk/page.\nReturns pending slashes posted to the <code>Agent</code> account. None …\nRegister <code>Agent</code> such that it can accept delegation.\nRemoves <code>Agent</code> registration.\nEnable/disable the given code depending on …\nSet the reward destination for the ledger associated with …\nReturns the fraction of the slash to be rewarded to …\nReturns the <code>Stake</code> of <code>who</code>.\nReturn a stash account that is controlled by a <code>controller</code>.\nReturn the status of the given staker, <code>Err</code> if not staked …\nThe total stake that <code>stash</code> has in the staking system. This …\nThe total balance backing this validator.\nThe total balance backing this validator.\nTotal stake of a staker, <code>Err</code> if not a staker.\nSchedule a portion of the active bonded balance to be …\nAmount of funds exposed.\nBook-keep a new bond for <code>keyless_who</code> without applying any …\nThe stash account of the nominator in question.\nWithdraw or revoke delegation to <code>Agent</code>.\nUnlock any funds schedule to unlock before or at the …\nA trait similar to <code>Convert</code> to convert values from <code>B</code> an …\nA naive implementation of <code>CurrencyConvert</code> that simply …\nAn implementation of <code>CurrencyToVote</code> tailored for chain’s …\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConvert u128 to balance.\nConvert balance to u64.\nThe report has already been submitted.\nIdentifier which is unique for this kind of an offence.\nThe kind of an offence, is a byte string representing some …\nLongevity, in blocks, for the evidence report validity.\nA trait implemented by an offence report.\nNumber of times the offence of this authority was already …\nA details about an offending authority for a particular …\nErrors that may happen on offence reports.\nAn abstract system to publish, check and process offence …\nA trait to take action on an offence.\nOther error has happened.\nA trait for decoupling offence reporters from the actual …\nA type that represents a point in time on an abstract …\nCheck an offence evidence.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet a mutable reference to the inner from the outer.\nGet a mutable reference to the inner from the outer.\nGet a reference to the inner from the outer.\nGet a reference to the inner from the outer.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns true iff all of the given offenders have been …\nThe offending authority id\nThe list of all offenders involved in this incident.\nA handler for an offence of a particular kind.\nProcess an offence evidence.\nPublish an offence evidence.\nReport an <code>offence</code> and reward given <code>reporters</code>.\nA list of reporters of offences of this authority ID. …\nThe session index that is used for querying the validator …\nA slash fraction of the total exposure that should be …\nA point in time when this offence happened.\nReturn a validator set count at the time when the offence …")