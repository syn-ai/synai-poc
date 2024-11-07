use crate as pallet_template;
use frame_support::derive_impl;
use sp_runtime::BuildStorage;
use frame_support::pallet_prelude::ConstU32;

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		TemplateModule: pallet_template,
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
	type Block = Block;
}

impl pallet_template::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type MaxVectors = ConstU32<1000>;          
	type MaxVectorLength = ConstU32<1000>;     
	type MaxTagLength = ConstU32<50>;
	type MaxTagsPerVector = ConstU32<10>;  
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}
