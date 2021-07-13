
uint::construct_uint! {
	pub struct U256(4);
}
#[derive(Debug)]
pub struct Block {
	pub(crate) pre_hash: String,
	pub(crate) nonce: usize,
	pub(crate) data: String,
	pub(crate) hash: String,
	pub(crate) difficulty: usize
}