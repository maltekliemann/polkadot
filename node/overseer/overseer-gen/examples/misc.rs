use polkadot_overseer_gen::{SpawnNamed, *};

#[derive(Debug, Clone, Copy)]
pub enum SigSigSig {
	Conclude,
	Foo,
}

#[derive(Debug, Clone)]
pub struct DummySpawner;

impl SpawnNamed for DummySpawner {
	fn spawn_blocking(
		&self,
		task_name: &'static str,
		subsystem_name: Option<&'static str>,
		_future: futures::future::BoxFuture<'static, ()>,
	) {
		unimplemented!("spawn blocking {} {}", task_name, subsystem_name.unwrap_or("default"))
	}

	fn spawn(
		&self,
		task_name: &'static str,
		subsystem_name: Option<&'static str>,
		_future: futures::future::BoxFuture<'static, ()>,
	) {
		unimplemented!("spawn {} {}", task_name, subsystem_name.unwrap_or("default"))
	}
}

/// The external event.
#[derive(Debug, Clone)]
pub struct EvX;

impl EvX {
	pub fn focus<'a, T>(&'a self) -> Result<EvX, ()> {
		unimplemented!("focus")
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Yikes;

impl std::fmt::Display for Yikes {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "yikes!")
	}
}

impl std::error::Error for Yikes {}

impl From<polkadot_overseer_gen::OverseerError> for Yikes {
	fn from(_: polkadot_overseer_gen::OverseerError) -> Yikes {
		Yikes
	}
}

impl From<polkadot_overseer_gen::mpsc::SendError> for Yikes {
	fn from(_: polkadot_overseer_gen::mpsc::SendError) -> Yikes {
		Yikes
	}
}

#[derive(Debug, Clone)]
pub struct MsgStrukt(pub u8);

#[derive(Debug, Clone, Copy)]
pub struct Plinko;
