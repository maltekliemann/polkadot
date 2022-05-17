// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! This variant of Malus backs/approves all malicious candidates crafted by
//! `suggest-garbage-candidate` variant and behaves honestly with other
//! candidates.

use polkadot_cli::{
	prepared_overseer_builder,
	service::{
		AuthorityDiscoveryApi, AuxStore, BabeApi, Block, Error, HeaderBackend, Overseer,
		OverseerConnector, OverseerGen, OverseerGenArgs, OverseerHandle, ParachainHost,
		ProvideRuntimeApi,
	},
};

use crate::{
	interceptor::*,
	variants::{FakeCandidateValidation, FakeCandidateValidationError, ReplaceValidationResult},
};

use std::sync::Arc;

/// Generates an overseer that replaces the candidate validation subsystem with our malicious
/// variant.
pub(crate) struct BackGarbageCandidate;

impl OverseerGen for BackGarbageCandidate {
	fn generate<'a, Spawner, RuntimeClient>(
		&self,
		connector: OverseerConnector,
		args: OverseerGenArgs<'a, Spawner, RuntimeClient>,
	) -> Result<(Overseer<Spawner, Arc<RuntimeClient>>, OverseerHandle), Error>
	where
		RuntimeClient: 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block> + AuxStore,
		RuntimeClient::Api: ParachainHost<Block> + BabeApi<Block> + AuthorityDiscoveryApi<Block>,
		Spawner: 'static + overseer::gen::Spawner + Clone + Unpin,
	{
		let spawner = args.spawner.clone();
		let validation_filter = ReplaceValidationResult::new(
			FakeCandidateValidation::BackingAndApprovalValid,
			FakeCandidateValidationError::InvalidOutputs,
			spawner.clone(),
		);

		prepared_overseer_builder(args)?
			.replace_candidate_validation(move |cv_subsystem| {
				InterceptedSubsystem::new(cv_subsystem, validation_filter)
			})
			.build_with_connector(connector)
			.map_err(|e| e.into())
	}
}
