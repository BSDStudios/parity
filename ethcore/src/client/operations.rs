// Autogenerated from JSON contract definition using Rust contract convertor.

use std::string::String;
use std::result::Result;
use std::fmt;
use {util, ethabi};
use util::FixedHash;
use util::Uint;

pub struct Operations {
	contract: ethabi::Contract,
	address: util::Address,
	do_call: Box<Fn(util::Address, Vec<u8>) -> Result<Vec<u8>, String> + Send + 'static>,
}
impl Operations {
	pub fn new<F>(address: util::Address, do_call: F) -> Self where F: Fn(util::Address, Vec<u8>) -> Result<Vec<u8>, String> + Send + 'static {
		Operations {
			contract: ethabi::Contract::new(ethabi::Interface::load(b"[{\"constant\":true,\"inputs\":[{\"name\":\"\",\"type\":\"address\"}],\"name\":\"owners\",\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\"}],\"type\":\"function\"},{\"constant\":false,\"inputs\":[{\"name\":\"_client\",\"type\":\"bytes32\"},{\"name\":\"_newOwner\",\"type\":\"address\"}],\"name\":\"resetClientOwner\",\"outputs\":[],\"type\":\"function\"},{\"constant\":true,\"inputs\":[{\"name\":\"_client\",\"type\":\"bytes32\"},{\"name\":\"_release\",\"type\":\"bytes32\"}],\"name\":\"isLatest\",\"outputs\":[{\"name\":\"\",\"type\":\"bool\"}],\"type\":\"function\"},{\"constant\":false,\"inputs\":[{\"name\":\"_txid\",\"type\":\"bytes32\"}],\"name\":\"rejectTransaction\",\"outputs\":[],\"type\":\"function\"},{\"constant\":false,\"inputs\":[{\"name\":\"_newOwner\",\"type\":\"address\"}],\"name\":\"setOwner\",\"outputs\":[],\"type\":\"function\"},{\"constant\":false,\"inputs\":[{\"name\":\"_client\",\"type\":\"bytes32\"}],\"name\":\"removeClient\",\"outputs\":[],\"type\":\"function\"},{\"constant\":false,\"inputs\":[],\"name\":\"rejectFork\",\"outputs\":[],\"type\":\"function\"},{\"constant\":true,\"inputs\":[{\"name\":\"_client\",\"type\":\"bytes32\"},{\"name\":\"_checksum\",\"type\":\"bytes32\"}],\"name\":\"findChecksum\",\"outputs\":[{\"name\":\"o_release\",\"type\":\"bytes32\"},{\"name\":\"o_platform\",\"type\":\"bytes32\"}],\"type\":\"function\"},{\"constant\":false,\"inputs\":[{\"name\":\"_number\",\"type\":\"uint32\"},{\"name\":\"_name\",\"type\":\"bytes32\"},{\"name\":\"_spec\",\"type\":\"bytes32\"}],\"name\":\"proposeFork\",\"outputs\":[],\"type\":\"function\"},{\"constant\":false,\"inputs\":[{\"name\":\"_newOwner\",\"type\":\"address\"}],\"name\":\"setClientOwner\",\"outputs\":[],\"type\":\"function\"},{\"constant\":false,\"inputs\":[{\"name\":\"_release\",\"type\":\"bytes32\"},{\"name\":\"_platform\",\"type\":\"bytes32\"},{\"name\":\"_checksum\",\"type\":\"bytes32\"}],\"name\":\"addChecksum\",\"outputs\":[],\"type\":\"function\"},{\"constant\":false,\"inputs\":[{\"name\":\"_txid\",\"type\":\"bytes32\"}],\"name\":\"confirmTransaction\",\"outputs\":[{\"name\":\"txSuccess\",\"type\":\"uint256\"}],\"type\":\"function\"},{\"constant\":true,\"inputs\":[{\"name\":\"\",\"type\":\"bytes32\"}],\"name\":\"proxy\",\"outputs\":[{\"name\":\"requiredCount\",\"type\":\"uint256\"},{\"name\":\"to\",\"type\":\"address\"},{\"name\":\"data\",\"type\":\"bytes\"},{\"name\":\"value\",\"type\":\"uint256\"},{\"name\":\"gas\",\"type\":\"uint256\"}],\"type\":\"function\"},{\"constant\":false,\"inputs\":[{\"name\":\"_client\",\"type\":\"bytes32\"},{\"name\":\"_owner\",\"type\":\"address\"}],\"name\":\"addClient\",\"outputs\":[],\"type\":\"function\"},{\"constant\":true,\"inputs\":[{\"name\":\"\",\"type\":\"uint32\"}],\"name\":\"forks\",\"outputs\":[{\"name\":\"name\",\"type\":\"bytes32\"},{\"name\":\"spec\",\"type\":\"bytes32\"},{\"name\":\"ratified\",\"type\":\"bool\"},{\"name\":\"requiredCount\",\"type\":\"uint256\"}],\"type\":\"function\"},{\"constant\":true,\"inputs\":[],\"name\":\"owner\",\"outputs\":[{\"name\":\"\",\"type\":\"address\"}],\"type\":\"function\"},{\"constant\":false,\"inputs\":[{\"name\":\"_txid\",\"type\":\"bytes32\"},{\"name\":\"_to\",\"type\":\"address\"},{\"name\":\"_data\",\"type\":\"bytes\"},{\"name\":\"_value\",\"type\":\"uint256\"},{\"name\":\"_gas\",\"type\":\"uint256\"}],\"name\":\"proposeTransaction\",\"outputs\":[{\"name\":\"txSuccess\",\"type\":\"uint256\"}],\"type\":\"function\"},{\"constant\":false,\"inputs\":[],\"name\":\"acceptFork\",\"outputs\":[],\"type\":\"function\"},{\"constant\":true,\"inputs\":[],\"name\":\"clientsRequired\",\"outputs\":[{\"name\":\"\",\"type\":\"uint32\"}],\"type\":\"function\"},{\"constant\":true,\"inputs\":[{\"name\":\"_client\",\"type\":\"bytes32\"},{\"name\":\"_release\",\"type\":\"bytes32\"}],\"name\":\"track\",\"outputs\":[{\"name\":\"\",\"type\":\"uint8\"}],\"type\":\"function\"},{\"constant\":false,\"inputs\":[{\"name\":\"_client\",\"type\":\"bytes32\"},{\"name\":\"_r\",\"type\":\"bool\"}],\"name\":\"setClientRequired\",\"outputs\":[],\"type\":\"function\"},{\"constant\":false,\"inputs\":[{\"name\":\"_release\",\"type\":\"bytes32\"},{\"name\":\"_forkBlock\",\"type\":\"uint32\"},{\"name\":\"_track\",\"type\":\"uint8\"},{\"name\":\"_semver\",\"type\":\"uint24\"}],\"name\":\"addRelease\",\"outputs\":[],\"type\":\"function\"},{\"constant\":true,\"inputs\":[],\"name\":\"latestFork\",\"outputs\":[{\"name\":\"\",\"type\":\"uint32\"}],\"type\":\"function\"},{\"constant\":true,\"inputs\":[{\"name\":\"_client\",\"type\":\"bytes32\"},{\"name\":\"_track\",\"type\":\"uint8\"}],\"name\":\"latestInTrack\",\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\"}],\"type\":\"function\"},{\"constant\":true,\"inputs\":[{\"name\":\"\",\"type\":\"bytes32\"}],\"name\":\"clients\",\"outputs\":[{\"name\":\"owner\",\"type\":\"address\"},{\"name\":\"required\",\"type\":\"bool\"}],\"type\":\"function\"},{\"constant\":true,\"inputs\":[],\"name\":\"proposedFork\",\"outputs\":[{\"name\":\"\",\"type\":\"uint32\"}],\"type\":\"function\"},{\"inputs\":[],\"type\":\"constructor\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"name\":\"from\",\"type\":\"address\"},{\"indexed\":false,\"name\":\"value\",\"type\":\"uint256\"},{\"indexed\":false,\"name\":\"data\",\"type\":\"bytes\"}],\"name\":\"Received\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"name\":\"client\",\"type\":\"bytes32\"},{\"indexed\":true,\"name\":\"txid\",\"type\":\"bytes32\"},{\"indexed\":true,\"name\":\"to\",\"type\":\"address\"},{\"indexed\":false,\"name\":\"data\",\"type\":\"bytes\"},{\"indexed\":false,\"name\":\"value\",\"type\":\"uint256\"},{\"indexed\":false,\"name\":\"gas\",\"type\":\"uint256\"}],\"name\":\"TransactionProposed\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"name\":\"client\",\"type\":\"bytes32\"},{\"indexed\":true,\"name\":\"txid\",\"type\":\"bytes32\"}],\"name\":\"TransactionConfirmed\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"name\":\"client\",\"type\":\"bytes32\"},{\"indexed\":true,\"name\":\"txid\",\"type\":\"bytes32\"}],\"name\":\"TransactionRejected\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"name\":\"txid\",\"type\":\"bytes32\"},{\"indexed\":false,\"name\":\"success\",\"type\":\"bool\"}],\"name\":\"TransactionRelayed\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"name\":\"client\",\"type\":\"bytes32\"},{\"indexed\":true,\"name\":\"number\",\"type\":\"uint32\"},{\"indexed\":true,\"name\":\"name\",\"type\":\"bytes32\"},{\"indexed\":false,\"name\":\"spec\",\"type\":\"bytes32\"}],\"name\":\"ForkProposed\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"name\":\"client\",\"type\":\"bytes32\"},{\"indexed\":true,\"name\":\"number\",\"type\":\"uint32\"}],\"name\":\"ForkAcceptedBy\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"name\":\"client\",\"type\":\"bytes32\"},{\"indexed\":true,\"name\":\"number\",\"type\":\"uint32\"}],\"name\":\"ForkRejectedBy\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"name\":\"forkNumber\",\"type\":\"uint32\"}],\"name\":\"ForkRejected\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"name\":\"forkNumber\",\"type\":\"uint32\"}],\"name\":\"ForkRatified\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"name\":\"client\",\"type\":\"bytes32\"},{\"indexed\":true,\"name\":\"forkBlock\",\"type\":\"uint32\"},{\"indexed\":true,\"name\":\"release\",\"type\":\"bytes32\"},{\"indexed\":false,\"name\":\"track\",\"type\":\"uint8\"},{\"indexed\":false,\"name\":\"semver\",\"type\":\"uint24\"}],\"name\":\"ReleaseAdded\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"name\":\"client\",\"type\":\"bytes32\"},{\"indexed\":true,\"name\":\"release\",\"type\":\"bytes32\"},{\"indexed\":true,\"name\":\"platform\",\"type\":\"bytes32\"},{\"indexed\":false,\"name\":\"checksum\",\"type\":\"bytes32\"}],\"name\":\"ChecksumAdded\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"name\":\"client\",\"type\":\"bytes32\"},{\"indexed\":false,\"name\":\"owner\",\"type\":\"address\"}],\"name\":\"ClientAdded\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"name\":\"client\",\"type\":\"bytes32\"}],\"name\":\"ClientRemoved\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"name\":\"client\",\"type\":\"bytes32\"},{\"indexed\":true,\"name\":\"old\",\"type\":\"address\"},{\"indexed\":true,\"name\":\"now\",\"type\":\"address\"}],\"name\":\"ClientOwnerChanged\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"name\":\"client\",\"type\":\"bytes32\"},{\"indexed\":false,\"name\":\"now\",\"type\":\"bool\"}],\"name\":\"ClientRequiredChanged\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"name\":\"old\",\"type\":\"address\"},{\"indexed\":false,\"name\":\"now\",\"type\":\"address\"}],\"name\":\"OwnerChanged\",\"type\":\"event\"}]").expect("JSON is autogenerated; qed")),
			address: address,
			do_call: Box::new(do_call),
		}
	}
	fn as_string<T: fmt::Debug>(e: T) -> String { format!("{:?}", e) }
	
	/// Auto-generated from: `{"constant":true,"inputs":[{"name":"","type":"address"}],"name":"owners","outputs":[{"name":"","type":"bytes32"}],"type":"function"}`
	#[allow(dead_code)]
	pub fn owners(&self, _1: &util::Address) -> Result<util::H256, String> { 
		let call = self.contract.function("owners".into()).map_err(Self::as_string)?;
		let data = call.encode_call(
			vec![ethabi::Token::Address(_1.clone().0)]
		).map_err(Self::as_string)?;
		let output = call.decode_output((self.do_call)(self.address.clone(), data)?).map_err(Self::as_string)?;
		let mut result = output.into_iter().rev().collect::<Vec<_>>();
		Ok(({ let r = result.pop().ok_or("Invalid return arity")?; let r = try!(r.to_fixed_bytes().ok_or("Invalid type returned")); util::H256::from_slice(r.as_ref()) })) 
	}

	/// Auto-generated from: `{"constant":false,"inputs":[{"name":"_client","type":"bytes32"},{"name":"_newOwner","type":"address"}],"name":"resetClientOwner","outputs":[],"type":"function"}`
	#[allow(dead_code)]
	pub fn reset_client_owner(&self, _client: &util::H256, _new_owner: &util::Address) -> Result<(), String> { 
		let call = self.contract.function("resetClientOwner".into()).map_err(Self::as_string)?;
		let data = call.encode_call(
			vec![ethabi::Token::FixedBytes(_client.as_ref().to_owned()), ethabi::Token::Address(_new_owner.clone().0)]
		).map_err(Self::as_string)?;
		call.decode_output((self.do_call)(self.address.clone(), data)?).map_err(Self::as_string)?;
		
		Ok(()) 
	}

	/// Auto-generated from: `{"constant":true,"inputs":[{"name":"_client","type":"bytes32"},{"name":"_release","type":"bytes32"}],"name":"isLatest","outputs":[{"name":"","type":"bool"}],"type":"function"}`
	#[allow(dead_code)]
	pub fn is_latest(&self, _client: &str, _release: &util::H256) -> Result<bool, String> { 
		let call = self.contract.function("isLatest".into()).map_err(Self::as_string)?;
		let data = call.encode_call(
			vec![ethabi::Token::FixedBytes(_client.as_bytes().to_owned()), ethabi::Token::FixedBytes(_release.as_ref().to_owned())]
		).map_err(Self::as_string)?;
		let output = call.decode_output((self.do_call)(self.address.clone(), data)?).map_err(Self::as_string)?;
		let mut result = output.into_iter().rev().collect::<Vec<_>>();
		Ok(({ let r = result.pop().ok_or("Invalid return arity")?; let r = try!(r.to_bool().ok_or("Invalid type returned")); r })) 
	}

	/// Auto-generated from: `{"constant":false,"inputs":[{"name":"_txid","type":"bytes32"}],"name":"rejectTransaction","outputs":[],"type":"function"}`
	#[allow(dead_code)]
	pub fn reject_transaction(&self, _txid: &util::H256) -> Result<(), String> { 
		let call = self.contract.function("rejectTransaction".into()).map_err(Self::as_string)?;
		let data = call.encode_call(
			vec![ethabi::Token::FixedBytes(_txid.as_ref().to_owned())]
		).map_err(Self::as_string)?;
		call.decode_output((self.do_call)(self.address.clone(), data)?).map_err(Self::as_string)?;
		
		Ok(()) 
	}

	/// Auto-generated from: `{"constant":false,"inputs":[{"name":"_newOwner","type":"address"}],"name":"setOwner","outputs":[],"type":"function"}`
	#[allow(dead_code)]
	pub fn set_owner(&self, _new_owner: &util::Address) -> Result<(), String> { 
		let call = self.contract.function("setOwner".into()).map_err(Self::as_string)?;
		let data = call.encode_call(
			vec![ethabi::Token::Address(_new_owner.clone().0)]
		).map_err(Self::as_string)?;
		call.decode_output((self.do_call)(self.address.clone(), data)?).map_err(Self::as_string)?;
		
		Ok(()) 
	}

	/// Auto-generated from: `{"constant":false,"inputs":[{"name":"_client","type":"bytes32"}],"name":"removeClient","outputs":[],"type":"function"}`
	#[allow(dead_code)]
	pub fn remove_client(&self, _client: &util::H256) -> Result<(), String> { 
		let call = self.contract.function("removeClient".into()).map_err(Self::as_string)?;
		let data = call.encode_call(
			vec![ethabi::Token::FixedBytes(_client.as_ref().to_owned())]
		).map_err(Self::as_string)?;
		call.decode_output((self.do_call)(self.address.clone(), data)?).map_err(Self::as_string)?;
		
		Ok(()) 
	}

	/// Auto-generated from: `{"constant":false,"inputs":[],"name":"rejectFork","outputs":[],"type":"function"}`
	#[allow(dead_code)]
	pub fn reject_fork(&self) -> Result<(), String> { 
		let call = self.contract.function("rejectFork".into()).map_err(Self::as_string)?;
		let data = call.encode_call(
			vec![]
		).map_err(Self::as_string)?;
		call.decode_output((self.do_call)(self.address.clone(), data)?).map_err(Self::as_string)?;
		
		Ok(()) 
	}

	/// Auto-generated from: `{"constant":true,"inputs":[{"name":"_client","type":"bytes32"},{"name":"_checksum","type":"bytes32"}],"name":"findChecksum","outputs":[{"name":"o_release","type":"bytes32"},{"name":"o_platform","type":"bytes32"}],"type":"function"}`
	#[allow(dead_code)]
	pub fn find_checksum(&self, _client: &util::H256, _checksum: &util::H256) -> Result<(util::H256, util::H256), String> { 
		let call = self.contract.function("findChecksum".into()).map_err(Self::as_string)?;
		let data = call.encode_call(
			vec![ethabi::Token::FixedBytes(_client.as_ref().to_owned()), ethabi::Token::FixedBytes(_checksum.as_ref().to_owned())]
		).map_err(Self::as_string)?;
		let output = call.decode_output((self.do_call)(self.address.clone(), data)?).map_err(Self::as_string)?;
		let mut result = output.into_iter().rev().collect::<Vec<_>>();
		Ok(({ let r = result.pop().ok_or("Invalid return arity")?; let r = try!(r.to_fixed_bytes().ok_or("Invalid type returned")); util::H256::from_slice(r.as_ref()) }, { let r = result.pop().ok_or("Invalid return arity")?; let r = try!(r.to_fixed_bytes().ok_or("Invalid type returned")); util::H256::from_slice(r.as_ref()) })) 
	}

	/// Auto-generated from: `{"constant":false,"inputs":[{"name":"_number","type":"uint32"},{"name":"_name","type":"bytes32"},{"name":"_spec","type":"bytes32"}],"name":"proposeFork","outputs":[],"type":"function"}`
	#[allow(dead_code)]
	pub fn propose_fork(&self, _number: u32, _name: &util::H256, _spec: &util::H256) -> Result<(), String> { 
		let call = self.contract.function("proposeFork".into()).map_err(Self::as_string)?;
		let data = call.encode_call(
			vec![ethabi::Token::Uint({ let mut r = [0u8; 32]; util::U128::from(_number as u64).to_big_endian(&mut r); r }), ethabi::Token::FixedBytes(_name.as_ref().to_owned()), ethabi::Token::FixedBytes(_spec.as_ref().to_owned())]
		).map_err(Self::as_string)?;
		call.decode_output((self.do_call)(self.address.clone(), data)?).map_err(Self::as_string)?;
		
		Ok(()) 
	}

	/// Auto-generated from: `{"constant":false,"inputs":[{"name":"_newOwner","type":"address"}],"name":"setClientOwner","outputs":[],"type":"function"}`
	#[allow(dead_code)]
	pub fn set_client_owner(&self, _new_owner: &util::Address) -> Result<(), String> { 
		let call = self.contract.function("setClientOwner".into()).map_err(Self::as_string)?;
		let data = call.encode_call(
			vec![ethabi::Token::Address(_new_owner.clone().0)]
		).map_err(Self::as_string)?;
		call.decode_output((self.do_call)(self.address.clone(), data)?).map_err(Self::as_string)?;
		
		Ok(()) 
	}

	/// Auto-generated from: `{"constant":false,"inputs":[{"name":"_release","type":"bytes32"},{"name":"_platform","type":"bytes32"},{"name":"_checksum","type":"bytes32"}],"name":"addChecksum","outputs":[],"type":"function"}`
	#[allow(dead_code)]
	pub fn add_checksum(&self, _release: &util::H256, _platform: &util::H256, _checksum: &util::H256) -> Result<(), String> { 
		let call = self.contract.function("addChecksum".into()).map_err(Self::as_string)?;
		let data = call.encode_call(
			vec![ethabi::Token::FixedBytes(_release.as_ref().to_owned()), ethabi::Token::FixedBytes(_platform.as_ref().to_owned()), ethabi::Token::FixedBytes(_checksum.as_ref().to_owned())]
		).map_err(Self::as_string)?;
		call.decode_output((self.do_call)(self.address.clone(), data)?).map_err(Self::as_string)?;
		
		Ok(()) 
	}

	/// Auto-generated from: `{"constant":false,"inputs":[{"name":"_txid","type":"bytes32"}],"name":"confirmTransaction","outputs":[{"name":"txSuccess","type":"uint256"}],"type":"function"}`
	#[allow(dead_code)]
	pub fn confirm_transaction(&self, _txid: &util::H256) -> Result<util::U256, String> { 
		let call = self.contract.function("confirmTransaction".into()).map_err(Self::as_string)?;
		let data = call.encode_call(
			vec![ethabi::Token::FixedBytes(_txid.as_ref().to_owned())]
		).map_err(Self::as_string)?;
		let output = call.decode_output((self.do_call)(self.address.clone(), data)?).map_err(Self::as_string)?;
		let mut result = output.into_iter().rev().collect::<Vec<_>>();
		Ok(({ let r = result.pop().ok_or("Invalid return arity")?; let r = try!(r.to_uint().ok_or("Invalid type returned")); util::U256::from(r.as_ref()) })) 
	}

	/// Auto-generated from: `{"constant":true,"inputs":[{"name":"","type":"bytes32"}],"name":"proxy","outputs":[{"name":"requiredCount","type":"uint256"},{"name":"to","type":"address"},{"name":"data","type":"bytes"},{"name":"value","type":"uint256"},{"name":"gas","type":"uint256"}],"type":"function"}`
	#[allow(dead_code)]
	pub fn proxy(&self, _1: &util::H256) -> Result<(util::U256, util::Address, Vec<u8>, util::U256, util::U256), String> { 
		let call = self.contract.function("proxy".into()).map_err(Self::as_string)?;
		let data = call.encode_call(
			vec![ethabi::Token::FixedBytes(_1.as_ref().to_owned())]
		).map_err(Self::as_string)?;
		let output = call.decode_output((self.do_call)(self.address.clone(), data)?).map_err(Self::as_string)?;
		let mut result = output.into_iter().rev().collect::<Vec<_>>();
		Ok(({ let r = result.pop().ok_or("Invalid return arity")?; let r = try!(r.to_uint().ok_or("Invalid type returned")); util::U256::from(r.as_ref()) }, { let r = result.pop().ok_or("Invalid return arity")?; let r = try!(r.to_address().ok_or("Invalid type returned")); util::Address::from(r) }, { let r = result.pop().ok_or("Invalid return arity")?; let r = try!(r.to_bytes().ok_or("Invalid type returned")); r }, { let r = result.pop().ok_or("Invalid return arity")?; let r = try!(r.to_uint().ok_or("Invalid type returned")); util::U256::from(r.as_ref()) }, { let r = result.pop().ok_or("Invalid return arity")?; let r = try!(r.to_uint().ok_or("Invalid type returned")); util::U256::from(r.as_ref()) })) 
	}

	/// Auto-generated from: `{"constant":false,"inputs":[{"name":"_client","type":"bytes32"},{"name":"_owner","type":"address"}],"name":"addClient","outputs":[],"type":"function"}`
	#[allow(dead_code)]
	pub fn add_client(&self, _client: &util::H256, _owner: &util::Address) -> Result<(), String> { 
		let call = self.contract.function("addClient".into()).map_err(Self::as_string)?;
		let data = call.encode_call(
			vec![ethabi::Token::FixedBytes(_client.as_ref().to_owned()), ethabi::Token::Address(_owner.clone().0)]
		).map_err(Self::as_string)?;
		call.decode_output((self.do_call)(self.address.clone(), data)?).map_err(Self::as_string)?;
		
		Ok(()) 
	}

	/// Auto-generated from: `{"constant":true,"inputs":[{"name":"","type":"uint32"}],"name":"forks","outputs":[{"name":"name","type":"bytes32"},{"name":"spec","type":"bytes32"},{"name":"ratified","type":"bool"},{"name":"requiredCount","type":"uint256"}],"type":"function"}`
	#[allow(dead_code)]
	pub fn forks(&self, _1: u32) -> Result<(util::H256, util::H256, bool, util::U256), String> { 
		let call = self.contract.function("forks".into()).map_err(Self::as_string)?;
		let data = call.encode_call(
			vec![ethabi::Token::Uint({ let mut r = [0u8; 32]; util::U128::from(_1 as u64).to_big_endian(&mut r); r })]
		).map_err(Self::as_string)?;
		let output = call.decode_output((self.do_call)(self.address.clone(), data)?).map_err(Self::as_string)?;
		let mut result = output.into_iter().rev().collect::<Vec<_>>();
		Ok(({ let r = result.pop().ok_or("Invalid return arity")?; let r = try!(r.to_fixed_bytes().ok_or("Invalid type returned")); util::H256::from_slice(r.as_ref()) }, { let r = result.pop().ok_or("Invalid return arity")?; let r = try!(r.to_fixed_bytes().ok_or("Invalid type returned")); util::H256::from_slice(r.as_ref()) }, { let r = result.pop().ok_or("Invalid return arity")?; let r = try!(r.to_bool().ok_or("Invalid type returned")); r }, { let r = result.pop().ok_or("Invalid return arity")?; let r = try!(r.to_uint().ok_or("Invalid type returned")); util::U256::from(r.as_ref()) })) 
	}

	/// Auto-generated from: `{"constant":true,"inputs":[],"name":"owner","outputs":[{"name":"","type":"address"}],"type":"function"}`
	#[allow(dead_code)]
	pub fn owner(&self) -> Result<util::Address, String> { 
		let call = self.contract.function("owner".into()).map_err(Self::as_string)?;
		let data = call.encode_call(
			vec![]
		).map_err(Self::as_string)?;
		let output = call.decode_output((self.do_call)(self.address.clone(), data)?).map_err(Self::as_string)?;
		let mut result = output.into_iter().rev().collect::<Vec<_>>();
		Ok(({ let r = result.pop().ok_or("Invalid return arity")?; let r = try!(r.to_address().ok_or("Invalid type returned")); util::Address::from(r) })) 
	}

	/// Auto-generated from: `{"constant":false,"inputs":[{"name":"_txid","type":"bytes32"},{"name":"_to","type":"address"},{"name":"_data","type":"bytes"},{"name":"_value","type":"uint256"},{"name":"_gas","type":"uint256"}],"name":"proposeTransaction","outputs":[{"name":"txSuccess","type":"uint256"}],"type":"function"}`
	#[allow(dead_code)]
	pub fn propose_transaction(&self, _txid: &util::H256, _to: &util::Address, _data: &[u8], _value: util::U256, _gas: util::U256) -> Result<util::U256, String> { 
		let call = self.contract.function("proposeTransaction".into()).map_err(Self::as_string)?;
		let data = call.encode_call(
			vec![ethabi::Token::FixedBytes(_txid.as_ref().to_owned()), ethabi::Token::Address(_to.clone().0), ethabi::Token::Bytes(_data.to_owned()), ethabi::Token::Uint({ let mut r = [0u8; 32]; _value.to_big_endian(&mut r); r }), ethabi::Token::Uint({ let mut r = [0u8; 32]; _gas.to_big_endian(&mut r); r })]
		).map_err(Self::as_string)?;
		let output = call.decode_output((self.do_call)(self.address.clone(), data)?).map_err(Self::as_string)?;
		let mut result = output.into_iter().rev().collect::<Vec<_>>();
		Ok(({ let r = result.pop().ok_or("Invalid return arity")?; let r = try!(r.to_uint().ok_or("Invalid type returned")); util::U256::from(r.as_ref()) })) 
	}

	/// Auto-generated from: `{"constant":false,"inputs":[],"name":"acceptFork","outputs":[],"type":"function"}`
	#[allow(dead_code)]
	pub fn accept_fork(&self) -> Result<(), String> { 
		let call = self.contract.function("acceptFork".into()).map_err(Self::as_string)?;
		let data = call.encode_call(
			vec![]
		).map_err(Self::as_string)?;
		call.decode_output((self.do_call)(self.address.clone(), data)?).map_err(Self::as_string)?;
		
		Ok(()) 
	}

	/// Auto-generated from: `{"constant":true,"inputs":[],"name":"clientsRequired","outputs":[{"name":"","type":"uint32"}],"type":"function"}`
	#[allow(dead_code)]
	pub fn clients_required(&self) -> Result<u32, String> { 
		let call = self.contract.function("clientsRequired".into()).map_err(Self::as_string)?;
		let data = call.encode_call(
			vec![]
		).map_err(Self::as_string)?;
		let output = call.decode_output((self.do_call)(self.address.clone(), data)?).map_err(Self::as_string)?;
		let mut result = output.into_iter().rev().collect::<Vec<_>>();
		Ok(({ let r = result.pop().ok_or("Invalid return arity")?; let r = try!(r.to_uint().ok_or("Invalid type returned")); util::U128::from(r.as_ref()).as_u64() as u32 })) 
	}

	/// Auto-generated from: `{"constant":true,"inputs":[{"name":"_client","type":"bytes32"},{"name":"_release","type":"bytes32"}],"name":"track","outputs":[{"name":"","type":"uint8"}],"type":"function"}`
	#[allow(dead_code)]
	pub fn track(&self, _client: &util::H256, _release: &util::H256) -> Result<u8, String> { 
		let call = self.contract.function("track".into()).map_err(Self::as_string)?;
		let data = call.encode_call(
			vec![ethabi::Token::FixedBytes(_client.as_ref().to_owned()), ethabi::Token::FixedBytes(_release.as_ref().to_owned())]
		).map_err(Self::as_string)?;
		let output = call.decode_output((self.do_call)(self.address.clone(), data)?).map_err(Self::as_string)?;
		let mut result = output.into_iter().rev().collect::<Vec<_>>();
		Ok(({ let r = result.pop().ok_or("Invalid return arity")?; let r = try!(r.to_uint().ok_or("Invalid type returned")); util::U128::from(r.as_ref()).as_u64() as u8 })) 
	}

	/// Auto-generated from: `{"constant":false,"inputs":[{"name":"_client","type":"bytes32"},{"name":"_r","type":"bool"}],"name":"setClientRequired","outputs":[],"type":"function"}`
	#[allow(dead_code)]
	pub fn set_client_required(&self, _client: &util::H256, _r: bool) -> Result<(), String> { 
		let call = self.contract.function("setClientRequired".into()).map_err(Self::as_string)?;
		let data = call.encode_call(
			vec![ethabi::Token::FixedBytes(_client.as_ref().to_owned()), ethabi::Token::Bool(_r)]
		).map_err(Self::as_string)?;
		call.decode_output((self.do_call)(self.address.clone(), data)?).map_err(Self::as_string)?;
		
		Ok(()) 
	}

	/// Auto-generated from: `{"constant":false,"inputs":[{"name":"_release","type":"bytes32"},{"name":"_forkBlock","type":"uint32"},{"name":"_track","type":"uint8"},{"name":"_semver","type":"uint24"}],"name":"addRelease","outputs":[],"type":"function"}`
	#[allow(dead_code)]
	pub fn add_release(&self, _release: &util::H256, _fork_block: u32, _track: u8, _semver: u32) -> Result<(), String> { 
		let call = self.contract.function("addRelease".into()).map_err(Self::as_string)?;
		let data = call.encode_call(
			vec![ethabi::Token::FixedBytes(_release.as_ref().to_owned()), ethabi::Token::Uint({ let mut r = [0u8; 32]; util::U128::from(_fork_block as u64).to_big_endian(&mut r); r }), ethabi::Token::Uint({ let mut r = [0u8; 32]; util::U128::from(_track as u64).to_big_endian(&mut r); r }), ethabi::Token::Uint({ let mut r = [0u8; 32]; util::U128::from(_semver as u64).to_big_endian(&mut r); r })]
		).map_err(Self::as_string)?;
		call.decode_output((self.do_call)(self.address.clone(), data)?).map_err(Self::as_string)?;
		
		Ok(()) 
	}

	/// Auto-generated from: `{"constant":true,"inputs":[],"name":"latestFork","outputs":[{"name":"","type":"uint32"}],"type":"function"}`
	#[allow(dead_code)]
	pub fn latest_fork(&self) -> Result<u32, String> { 
		let call = self.contract.function("latestFork".into()).map_err(Self::as_string)?;
		let data = call.encode_call(
			vec![]
		).map_err(Self::as_string)?;
		let output = call.decode_output((self.do_call)(self.address.clone(), data)?).map_err(Self::as_string)?;
		let mut result = output.into_iter().rev().collect::<Vec<_>>();
		Ok(({ let r = result.pop().ok_or("Invalid return arity")?; let r = try!(r.to_uint().ok_or("Invalid type returned")); util::U128::from(r.as_ref()).as_u64() as u32 })) 
	}

	/// Auto-generated from: `{"constant":true,"inputs":[{"name":"_client","type":"bytes32"},{"name":"_track","type":"uint8"}],"name":"latestInTrack","outputs":[{"name":"","type":"bytes32"}],"type":"function"}`
	#[allow(dead_code)]
	pub fn latest_in_track(&self, _client: &util::H256, _track: u8) -> Result<util::H256, String> { 
		let call = self.contract.function("latestInTrack".into()).map_err(Self::as_string)?;
		let data = call.encode_call(
			vec![ethabi::Token::FixedBytes(_client.as_ref().to_owned()), ethabi::Token::Uint({ let mut r = [0u8; 32]; util::U128::from(_track as u64).to_big_endian(&mut r); r })]
		).map_err(Self::as_string)?;
		let output = call.decode_output((self.do_call)(self.address.clone(), data)?).map_err(Self::as_string)?;
		let mut result = output.into_iter().rev().collect::<Vec<_>>();
		Ok(({ let r = result.pop().ok_or("Invalid return arity")?; let r = try!(r.to_fixed_bytes().ok_or("Invalid type returned")); util::H256::from_slice(r.as_ref()) })) 
	}

	/// Auto-generated from: `{"constant":true,"inputs":[{"name":"","type":"bytes32"}],"name":"clients","outputs":[{"name":"owner","type":"address"},{"name":"required","type":"bool"}],"type":"function"}`
	#[allow(dead_code)]
	pub fn clients(&self, _1: &util::H256) -> Result<(util::Address, bool), String> { 
		let call = self.contract.function("clients".into()).map_err(Self::as_string)?;
		let data = call.encode_call(
			vec![ethabi::Token::FixedBytes(_1.as_ref().to_owned())]
		).map_err(Self::as_string)?;
		let output = call.decode_output((self.do_call)(self.address.clone(), data)?).map_err(Self::as_string)?;
		let mut result = output.into_iter().rev().collect::<Vec<_>>();
		Ok(({ let r = result.pop().ok_or("Invalid return arity")?; let r = try!(r.to_address().ok_or("Invalid type returned")); util::Address::from(r) }, { let r = result.pop().ok_or("Invalid return arity")?; let r = try!(r.to_bool().ok_or("Invalid type returned")); r })) 
	}

	/// Auto-generated from: `{"constant":true,"inputs":[],"name":"proposedFork","outputs":[{"name":"","type":"uint32"}],"type":"function"}`
	#[allow(dead_code)]
	pub fn proposed_fork(&self) -> Result<u32, String> { 
		let call = self.contract.function("proposedFork".into()).map_err(Self::as_string)?;
		let data = call.encode_call(
			vec![]
		).map_err(Self::as_string)?;
		let output = call.decode_output((self.do_call)(self.address.clone(), data)?).map_err(Self::as_string)?;
		let mut result = output.into_iter().rev().collect::<Vec<_>>();
		Ok(({ let r = result.pop().ok_or("Invalid return arity")?; let r = try!(r.to_uint().ok_or("Invalid type returned")); util::U128::from(r.as_ref()).as_u64() as u32 })) 
	}
}