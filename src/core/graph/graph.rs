use rustc_hash::FxHashMap;

use super::{Media, Node, NodeId};
use crate::core::packet::Packet;
use crate::message::Result;

pub struct Graph {
	nodes: Vec<Box<dyn Node>>,
	edges: FxHashMap<NodeId, Vec<NodeId>>,
	entries: FxHashMap<usize, NodeId>,
}

impl Graph {
	pub fn new() -> Self {
		Self { nodes: Vec::new(), edges: FxHashMap::default(), entries: FxHashMap::default() }
	}

	pub fn add<N: Node + 'static>(&mut self, node: N) -> NodeId {
		let id = NodeId::new(self.nodes.len());
		self.nodes.push(Box::new(node));
		id
	}

	pub fn link(&mut self, from: NodeId, to: NodeId) {
		self.edges.entry(from).or_default().push(to);
	}

	pub fn set_entry(&mut self, track_id: usize, node_id: NodeId) {
		self.entries.insert(track_id, node_id);
	}

	pub fn process(&mut self, packet: Packet) -> Result<Vec<Packet>> {
		let entry = match self.entries.get(&packet.track_id) {
			Some(id) => *id,
			None => return Ok(Vec::new()),
		};

		let order = self.execution_order(entry);
		let mut current: Vec<Media> = vec![Media::Packet(packet)];

		for node_id in order {
			let node = &mut self.nodes[node_id.value()];
			let mut next = Vec::new();
			for input in current {
				let outputs = node.process(input)?;
				next.extend(outputs);
			}
			current = next;
		}

		let mut packets = Vec::new();
		for media in current {
			if let Media::Packet(p) = media {
				packets.push(p);
			}
		}
		Ok(packets)
	}

	pub fn flush(&mut self) -> Result<Vec<Packet>> {
		let entry_ids: Vec<(usize, NodeId)> = self.entries.iter().map(|(&k, &v)| (k, v)).collect();
		let mut all_packets = Vec::new();

		for (_track_id, entry) in entry_ids {
			let order = self.execution_order(entry);
			let mut pending: Vec<Media> = Vec::new();

			for node_id in &order {
				let node = &mut self.nodes[node_id.value()];

				let mut next = Vec::new();
				for input in pending {
					next.extend(node.process(input)?);
				}

				next.extend(node.flush()?);

				pending = next;
			}

			for media in pending {
				if let Media::Packet(p) = media {
					all_packets.push(p);
				}
			}
		}

		Ok(all_packets)
	}

	fn execution_order(&self, entry: NodeId) -> Vec<NodeId> {
		let mut order = Vec::new();
		let mut current = entry;
		loop {
			order.push(current);
			match self.edges.get(&current) {
				Some(successors) if !successors.is_empty() => {
					current = successors[0];
				}
				_ => break,
			}
		}
		order
	}
}
