use super::*;

use crate::body_part::BodyPartLayerType::*;
use crate::body_part::*;
use crate::body_part_builder::*;
use crate::damage::DamageType::*;
use crate::damage::*;
use enum_map::enum_map;
#[test]
fn take_damage() {
	let mut part = BodyPartBuilder::new("Leg")
		.layer(BodyPartLayer {
			layer_type: Bone,
			max_hp: 100,
			hp: 100,
			damage_multipliers: enum_map! {
				Piercing => 0.5,
				Slashing => 0.6,
				Blunt => 2.0
			},
			..BodyPartLayer::default()
		})
		.layer(BodyPartLayer {
			layer_type: Muscle,
			max_hp: 100,
			hp: 100,
			damage_multipliers: enum_map! {
				Piercing => 1.0,
				Slashing => 1.0,
				Blunt => 1.0
			},
			..BodyPartLayer::default()
		})
		.layer(BodyPartLayer {
			layer_type: Skin,
			max_hp: 100,
			hp: 100,
			damage_multipliers: enum_map! {
				Piercing => 1.0,
				Slashing => 2.0,
				Blunt => 1.0
			},
			..BodyPartLayer::default()
		})
		.build()
		.unwrap();
	let res = part.take_damage(DamageInstance {
		amount: -50,
		damage_type: Slashing,
	});

	println!("{:?}", res);
}
