// Copyright 2017-2020 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

//! A fixed hash type.

pub use primitive_types::{H160, H256, H512};

/// Hash conversion. Used to convert between unbound associated hash types in traits,
/// implemented by the same hash type.
/// Panics if used to convert between different hash types.
pub fn convert_hash<H1: Default + AsMut<[u8]>, H2: AsRef<[u8]>>(src: &H2) -> H1 {
	let mut dest = H1::default();
	assert_eq!(dest.as_mut().len(), src.as_ref().len());
	dest.as_mut().copy_from_slice(src.as_ref());
	dest
}
