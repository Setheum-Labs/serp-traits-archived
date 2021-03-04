use crate::Change;
use codec::FullCodec;
use codec::{Decode, Encode};
use sp_runtime::{
	traits::{AtLeast32Bit, Bounded, MaybeSerializeDeserialize},
	DispatchError, DispatchResult, RuntimeDebug,
};
use sp_std::{
	cmp::{Eq, PartialEq},
	fmt::Debug,
	result,
};

/// Auction info.
#[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[derive(Encode, Decode, RuntimeDebug)]
pub struct AuctionInfo<AccountId, CurrencyId, Balance, BlockNumber> {
	/// Current bidder, their currency and bid price.
	pub bid: Option<(AccountId, CurrencyId, Balance)>,
	/// Currency accepted for the auction
	pub accepts: Option<CurrencyId>,
	/// Currency dispensed by the auction
	pub dispenses: Option<CurrencyId>,
	/// Define which block this auction will be started.
	pub start: BlockNumber,
	/// Define which block this auction will be ended.
	pub end: Option<BlockNumber>,
}

/// Abstraction over a simple auction system.
pub trait Auction<AccountId, CurrencyId, BlockNumber> {
	/// The currency identifier.
	type CurrencyId: FullCodec + Eq + PartialEq + Copy + MaybeSerializeDeserialize + Debug;
	/// The id of an AuctionInfo
	type AuctionId: FullCodec + Default + Copy + Eq + PartialEq + MaybeSerializeDeserialize + Bounded + Debug;
	/// The price to bid.
	type Balance: AtLeast32Bit + FullCodec + Copy + MaybeSerializeDeserialize + Debug + Default;

	/// The auction info of `id`
	fn auction_info(id: Self::AuctionId) -> Option<AuctionInfo<AccountId, Self::CurrencyId, Self::Balance, BlockNumber>>;
	/// Update the auction info of `id` with `info`
	fn update_auction(id: Self::AuctionId, info: AuctionInfo<AccountId, Self::CurrencyId, Self::Balance, BlockNumber>) -> DispatchResult;
	/// Create new auction with specific startblock and endblock, 
	/// a specific accepted currency and a specific dispensed currency, 
	/// return the id of the auction.
	fn new_auction(
		start: BlockNumber, 
		end: Option<BlockNumber>, 
		accepts: Option<CurrencyId>, 
		dispenses: Option<CurrencyId>,
	) -> result::Result<Self::AuctionId, DispatchError>;
	/// Remove auction by `id`
	fn remove_auction(id: Self::AuctionId);
}

/// The result of bid handling.
pub struct OnNewBidResult<BlockNumber> {
	/// Indicates if the bid was accepted
	pub accept_bid: bool,
	/// The auction end change.
	pub auction_end_change: Change<Option<BlockNumber>>,
}

/// Hooks for auction to handle bids.
pub trait AuctionHandler<AccountId, CurrencyId, Balance, BlockNumber, AuctionId> {
	/// Called when new bid is received.
	/// The return value determines if the bid should be accepted and update
	/// auction end time. Implementation should reserve money from current
	/// winner and refund previous winner.
	fn on_new_bid(
		now: BlockNumber,
		id: AuctionId,
		currency_id: CurrencyId,
		new_bid: (AccountId, CurrencyId, Balance),
		last_bid: Option<(AccountId, CurrencyId, Balance)>,
	) -> OnNewBidResult<BlockNumber>;
	/// End an auction with `winner`
	fn on_auction_ended(id: AuctionId, winner: Option<(AccountId, CurrencyId, Balance)>);
}
