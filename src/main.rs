use petgraph::graph::NodeIndex;
use petgraph::Graph;
use serde::Serialize;
use std::default::Default;
use std::fmt::Display;

use crate::damage::{DamageInstance, DamageType};
use crate::mob_humanoid::HumanoidMobBuilder;

mod body_part;
mod body_part_builder;
mod damage;
mod mob;
mod mob_humanoid;

fn main() {
	let gob = HumanoidMobBuilder::new("Gobbo".to_string()).build();
	gob.print();

	println!("{}", DamageType::Piercing);
	println!("{}", DamageType::Slashing);
	println!("{}", DamageType::Blunt);

	println!(
		"{}",
		DamageInstance {
			amount: 100,
			damage_type: DamageType::Piercing
		}
	);
	println!(
		"{}",
		DamageInstance {
			amount: -100,
			damage_type: DamageType::Blunt
		}
	);
}

#[cfg(test)]
mod body_part_test;
