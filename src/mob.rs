use enum_map::enum_map;
use petgraph::graph::EdgeReference;
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use serde::Serialize;
use std::default::Default;
use std::fmt::Display;

use crate::body_part::{
	BodyPart, BodyPartConnection, BodyPartLayer, BodyPartLayerType,
};
use crate::body_part_builder::BodyPartBuilder;
use crate::damage::DamageType;

#[derive(Debug, Clone)]
pub struct Mob {
	pub name: String,
	pub body_part_graph: Graph<Option<BodyPart>, BodyPartConnection>,
	pub root: NodeIndex,
}

impl Mob {
	pub fn add_body_part(
		&mut self,
		body_part: BodyPart,
		parent: NodeIndex,
		connections: Vec<BodyPartConnection>,
	) -> NodeIndex {
		let new_part = self.body_part_graph.add_node(Some(body_part));

		for connection_type in connections {
			self
				.body_part_graph
				.add_edge(parent, new_part, connection_type);
		}
		new_part
	}

	pub fn add_body_part_sym(
		&mut self,
		body_part: BodyPart,
		parent: NodeIndex,
		connections: Vec<BodyPartConnection>,
	) -> (NodeIndex, NodeIndex) {
		let new_left = self.add_body_part(
			BodyPart {
				name: format!("{}_l", body_part.name.as_str()),
				..body_part.clone()
			},
			parent,
			connections.clone(),
		);

		let new_right = self.add_body_part(
			BodyPart {
				name: format!("{}_r", body_part.name.as_str()),
				..body_part.clone()
			},
			parent,
			connections.clone(),
		);

		(new_left, new_right)
	}

	pub fn add_body_part_sym_both(
		&mut self,
		body_part: BodyPart,
		(parent_left, parent_right): (NodeIndex, NodeIndex),
		connections: Vec<BodyPartConnection>,
	) -> (NodeIndex, NodeIndex) {
		let new_left = self.add_body_part(
			BodyPart {
				name: format!("{}_l", body_part.name.as_str()),
				..body_part.clone()
			},
			parent_left,
			connections.clone(),
		);

		let new_right = self.add_body_part(
			BodyPart {
				name: format!("{}_r", body_part.name.as_str()),
				..body_part.clone()
			},
			parent_right,
			connections.clone(),
		);

		(new_left, new_right)
	}

	fn recursive_print(
		&self,
		node: NodeIndex,
		connection: Option<EdgeReference<BodyPartConnection>>,
		depth: u8,
	) {
		use colored::*;
		use petgraph::visit::EdgeRef;
		use BodyPartConnection::*;

		let node_formatted = format!(
			"{}",
			self
				.body_part_graph
				.node_weight(node)
				.unwrap()
				.as_ref()
				.unwrap()
		);
		let connection_type = match connection {
			Some(c) => c.weight(),
			None => &Structural,
		};

		println!(
			"{}{}",
			(0..depth).map(|_| "  ").collect::<String>(),
			match connection_type {
				Structural => node_formatted.blue(),
				BloodSupply => node_formatted.red(),
				Container => node_formatted.purple(),
			}
		);

		let print_connections = [Structural, Container];

		for e in self
			.body_part_graph
			.edges_directed(node, petgraph::Direction::Outgoing)
		{
			if print_connections.iter().any(|v| v == e.weight()) {
				self.recursive_print(e.target(), Some(e), depth + 1);
			}
		}
	}

	pub fn print(&self) {
		self.recursive_print(self.root, None, 0);
		// println!("{:?}", self.body_part_graph);
	}
}
