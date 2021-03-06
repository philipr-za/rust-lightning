//! Various user-configurable channel limits and settings which ChannelManager
//! applies for you.

/// Top-level config which holds ChannelHandshakeLimits and ChannelConfig.
#[derive(Clone, Debug)]
pub struct UserConfig {
	/// Limits applied during channel creation.
	pub channel_limits: ChannelHandshakeLimits,
	/// Channel config which affects behavior during channel lifetime.
	pub channel_options: ChannelConfig,
}

impl UserConfig {
	/// Provides sane defaults for most configurations (but with 0 relay fees!)
	pub fn new() -> Self{
		UserConfig {
			channel_limits: ChannelHandshakeLimits::new(),
			channel_options: ChannelConfig::new(),
		}
	}
}

/// Optional channel limits which are applied during channel creation.
///
/// These limits are only applied to our counterparty's limits, not our own.
///
/// Use 0/<type>::max_value() as appropriate to skip checking.
#[derive(Copy, Clone, Debug)]
pub struct ChannelHandshakeLimits {
	/// Minimum allowed satoshis when a channel is funded, this is supplied by the sender and so
	/// only applies to inbound channels.
	pub min_funding_satoshis: u64,
	/// The remote node sets a limit on the minimum size of HTLCs we can send to them. This allows
	/// you to limit the maximum minimum-size they can require.
	pub max_htlc_minimum_msat: u64,
	/// The remote node sets a limit on the maximum value of pending HTLCs to them at any given
	/// time to limit their funds exposure to HTLCs. This allows you to set a minimum such value.
	pub min_max_htlc_value_in_flight_msat: u64,
	/// The remote node will require we keep a certain amount in direct payment to ourselves at all
	/// time, ensuring that we are able to be punished if we broadcast an old state. This allows to
	/// you limit the amount which we will have to keep to ourselves (and cannot use for HTLCs).
	pub max_channel_reserve_satoshis: u64,
	/// The remote node sets a limit on the maximum number of pending HTLCs to them at any given
	/// time. This allows you to set a minimum such value.
	pub min_max_accepted_htlcs: u16,
	/// Outputs below a certain value will not be added to on-chain transactions. The dust value is
	/// required to always be higher than this value so this only applies to HTLC outputs (and
	/// potentially to-self outputs before any payments have been made).
	/// Thus, HTLCs below this amount plus HTLC transaction fees are not enforceable on-chain.
	/// This setting allows you to set a minimum dust limit for their commitment transactions,
	/// reflecting the reality that tiny outputs are not considered standard transactions and will
	/// not propagate through the Bitcoin network.
	/// Defaults to 546, or the current dust limit on the Bitcoin network.
	pub min_dust_limit_satoshis: u64,
	/// Maximum allowed threshold above which outputs will not be generated in their commitment
	/// transactions.
	/// HTLCs below this amount plus HTLC transaction fees are not enforceable on-chain.
	pub max_dust_limit_satoshis: u64,
	/// Before a channel is usable the funding transaction will need to be confirmed by at least a
	/// certain number of blocks, specified by the node which is not the funder (as the funder can
	/// assume they aren't going to double-spend themselves).
	/// This config allows you to set a limit on the maximum amount of time to wait. Defaults to
	/// 144 blocks or roughly one day and only applies to outbound channels.
	pub max_minimum_depth: u32,
	/// Set to force the incoming channel to match our announced channel preference in
	/// ChannelConfig.
	/// Defaults to true to make the default that no announced channels are possible (which is
	/// appropriate for any nodes which are not online very reliably).
	pub force_announced_channel_preference: bool,
}

impl ChannelHandshakeLimits {
	/// Provides sane defaults for most configurations.
	///
	/// Most additional limits are disabled except those with which specify a default in individual
	/// field documentation. Note that this may result in barely-usable channels, but since they
	/// are applied mostly only to incoming channels that's not much of a problem.
	pub fn new() -> Self {
		ChannelHandshakeLimits {
			min_funding_satoshis: 0,
			max_htlc_minimum_msat: <u64>::max_value(),
			min_max_htlc_value_in_flight_msat: 0,
			max_channel_reserve_satoshis: <u64>::max_value(),
			min_max_accepted_htlcs: 0,
			min_dust_limit_satoshis: 546,
			max_dust_limit_satoshis: <u64>::max_value(),
			max_minimum_depth: 144,
			force_announced_channel_preference: true,
		}
	}
}

/// Options which apply on a per-channel basis and may change at runtime or based on negotiation
/// with our counterparty.
#[derive(Copy, Clone, Debug)]
pub struct ChannelConfig {
	/// Amount (in millionths of a satoshi) the channel will charge per transferred satoshi.
	/// This may be allowed to change at runtime in a later update, however doing so must result in
	/// update messages sent to notify all nodes of our updated relay fee.
	pub fee_proportional_millionths: u32,
	/// Set to announce the channel publicly and notify all nodes that they can route via this
	/// channel.
	///
	/// This should only be set to true for nodes which expect to be online reliably.
	///
	/// As the node which funds a channel picks this value this will only apply for new outbound
	/// channels unless ChannelHandshakeLimits::force_announced_channel_preferences is set.
	///
	/// This cannot be changed after the initial channel handshake.
	pub announced_channel: bool,
}

impl ChannelConfig {
	/// Provides sane defaults for most configurations (but with zero relay fees!).
	pub fn new() -> Self {
		ChannelConfig {
			fee_proportional_millionths: 0,
			announced_channel: false,
		}
	}
}

//Add write and readable traits to channelconfig
impl_writeable!(ChannelConfig, 8+1, {
	fee_proportional_millionths,
	announced_channel
});
