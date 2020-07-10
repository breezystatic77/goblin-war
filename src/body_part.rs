use crate::damage::{DamageInstance, DamageResult, DamageType};
use enum_map::{enum_map, EnumMap};
use serde::Serialize;
use std::fmt::Display;

#[derive(Clone, Copy, Serialize, Debug)]
pub enum BodyPartLayerType {
	Skin,
	Muscle,
	Bone,
	Flesh,
	Artery,
}

impl Default for BodyPartLayerType {
	fn default() -> Self {
		BodyPartLayerType::Skin
	}
}

#[derive(Clone, Copy, Debug)]
pub struct BodyPartLayer {
	pub layer_type: BodyPartLayerType,
	pub max_hp: u32,
	pub hp: u32,
	pub damage_multipliers: EnumMap<DamageType, f32>,
}

impl Default for BodyPartLayer {
	fn default() -> Self {
		Self {
			layer_type: BodyPartLayerType::Skin,
			max_hp: 100,
			hp: 100,
			damage_multipliers: enum_map! {
				DamageType::Piercing => 1.0,
				DamageType::Slashing => 1.0,
				DamageType::Blunt => 1.0
			},
		}
	}
}
impl BodyPartLayer {
	pub fn new(
		layer_type: BodyPartLayerType,
		max_hp: u32,
		damage_multipliers: EnumMap<DamageType, f32>,
	) -> Self {
		BodyPartLayer {
			layer_type: layer_type,
			max_hp: max_hp,
			hp: max_hp,
			damage_multipliers: damage_multipliers,
		}
	}
}

#[derive(Clone, Copy, Serialize, Debug)]
pub enum BodyPartType {
	Limb,
	Organ,
}

#[derive(Clone, Copy, Serialize, Debug, PartialEq)]
pub enum BodyPartConnection {
	Structural,
	BloodSupply,
	Container,
}

impl Default for BodyPartConnection {
	fn default() -> Self {
		BodyPartConnection::Structural
	}
}

#[derive(Debug, Clone)]
pub struct BodyPart {
	pub name: String,
	pub part_type: BodyPartType,
	pub layers: Vec<BodyPartLayer>,
	pub severable: bool,
	pub can_grab: bool,
}

impl Default for BodyPart {
	fn default() -> Self {
		Self {
			name: "unnamed_bodypart".to_string(),
			part_type: BodyPartType::Limb,
			layers: vec![],
			severable: true,
			can_grab: false,
		}
	}
}

impl Display for BodyPart {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}: {}/{} -- {} layers",
			self.name,
			self.hp(),
			self.max_hp(),
			self.layers.len()
		)
	}
}

impl BodyPart {
	pub fn hp(&self) -> u32 {
		let mut total = 0;
		for layer in &self.layers {
			total += layer.hp;
		}
		total
	}
	pub fn max_hp(&self) -> u32 {
		let mut total = 0;
		for layer in &self.layers {
			total += layer.max_hp;
		}
		total
	}
	pub fn hp_avg(&self) -> u32 {
		self.hp() / self.layers.len() as u32
	}
	pub fn max_hp_avg(&self) -> u32 {
		self.max_hp() / self.layers.len() as u32
	}

	pub fn take_damage(&mut self, damage: DamageInstance) -> DamageResult {
		use crate::damage::DamageResult::*;
		use crate::damage::DamageType::*;
		let total_hp = self.hp();
		let mut total_damage: i32 = 0;
		match damage.damage_type {
			Piercing => {
				for layer in &mut self.layers {
					let multiplier = layer.damage_multipliers[Piercing];
					let diff = (damage.amount as f32 * multiplier) as u32;
					total_damage += diff as i32;
					layer.hp += diff;
				}
			}
			Slashing => {
				let hp_values: Vec<u32> = self.layers.iter().map(|&l| l.hp).collect();
				for (i, layer) in &mut self.layers.iter_mut().enumerate() {
					let layers_above = &hp_values[i..hp_values.len()];
					if !layers_above.iter().any(|&hp| hp > 0) {
						let multiplier = layer.damage_multipliers[Slashing];
						let diff = (damage.amount as f32 * multiplier) as u32;
						total_damage += diff as i32;
						layer.hp += diff;
					};
				}
			}
			Blunt => {
				let len = self.layers.len() as f32;
				for layer in self.layers.iter_mut() {
					let multiplier = layer.damage_multipliers[Blunt];
					let diff = (damage.amount as f32 * multiplier / len) as u32;
					total_damage += diff as i32;
					layer.hp += diff;
				}
			}
		};

		let result = if total_damage as u32 > total_hp {
			if self.severable && damage.damage_type != Piercing {
				Severed
			} else {
				Destroyed
			}
		} else {
			TookDamage(damage)
		};

		return result;
	}
}
