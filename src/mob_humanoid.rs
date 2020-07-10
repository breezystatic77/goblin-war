use crate::body_part::*;
use crate::body_part_builder::*;
use crate::mob::Mob;
use enum_map::enum_map;
use petgraph::graph::NodeIndex;
use petgraph::Graph;

pub struct HumanoidMobBuilder {
	mob: Mob,
}

impl HumanoidMobBuilder {
	pub fn new(name: String) -> Self {
		use crate::body_part::BodyPartLayerType::*;
		use crate::damage::DamageType::*;

		let mut graph = Graph::<Option<BodyPart>, BodyPartConnection>::new();

		let chest = graph.add_node(Some(
			BodyPartBuilder::new("chest")
				.severable(false)
				.layer(BodyPartLayer {
					layer_type: Bone,
					max_hp: 100,
					hp: 100,
					..BodyPartLayer::default()
				})
				.layer(BodyPartLayer {
					layer_type: Muscle,
					max_hp: 100,
					hp: 100,
					..BodyPartLayer::default()
				})
				.layer(BodyPartLayer {
					layer_type: Skin,
					max_hp: 100,
					hp: 100,
					..BodyPartLayer::default()
				})
				.build()
				.unwrap(),
		));

		let mut mob = Mob {
			name: name,
			body_part_graph: graph,
			root: chest,
		};

		macro_rules! part {
			($name:expr, $({ $ltype:expr, $lhp:expr }),*
			) => (
				BodyPartBuilder::new($name)
				$(.layer(BodyPartLayer {
					layer_type: $ltype,
					max_hp: $lhp,
					hp: $lhp,
					..BodyPartLayer::default()
				}))*
			);
			($name:expr, $max_hp:expr, limb) => (
				BodyPartBuilder::new($name)
				.layer(BodyPartLayer {
					layer_type: BodyPartLayerType::Bone,
					max_hp: $max_hp,
					hp: $max_hp,
					..BodyPartLayer::default()
				})
				.layer(BodyPartLayer {
					layer_type: BodyPartLayerType::Muscle,
					max_hp: $max_hp,
					hp: $max_hp,
					..BodyPartLayer::default()
				})
				.layer(BodyPartLayer {
					layer_type: BodyPartLayerType::Skin,
					max_hp: $max_hp,
					hp: $max_hp,
					..BodyPartLayer::default()
				})
			);
			($name:expr, $max_hp:expr, limb_artery) => (
				BodyPartBuilder::new($name)
				.layer(BodyPartLayer {
					layer_type: BodyPartLayerType::Bone,
					max_hp: $max_hp,
					hp: $max_hp,
					..BodyPartLayer::default()
				})
				.layer(BodyPartLayer {
					layer_type: BodyPartLayerType::Artery,
					max_hp: $max_hp,
					hp: $max_hp,
					..BodyPartLayer::default()
				})
				.layer(BodyPartLayer {
					layer_type: BodyPartLayerType::Muscle,
					max_hp: $max_hp,
					hp: $max_hp,
					..BodyPartLayer::default()
				})
				.layer(BodyPartLayer {
					layer_type: BodyPartLayerType::Skin,
					max_hp: $max_hp,
					hp: $max_hp,
					..BodyPartLayer::default()
				})
			);
		}

		macro_rules! limb {
			($p:expr, $c:expr) => {
				mob.add_body_part(
					$c.build().unwrap(),
					$p,
					vec![
						BodyPartConnection::Structural,
						BodyPartConnection::BloodSupply,
					],
					)
			};
			(o $p:expr, $c:expr) => {
				mob.add_body_part(
					$c.build().unwrap(),
					$p,
					vec![
						BodyPartConnection::Container,
						BodyPartConnection::BloodSupply,
					],
					)
			};
			(s $p:expr, $c:expr) => {
				mob.add_body_part_sym(
					$c.build().unwrap(),
					$p,
					vec![
						BodyPartConnection::Structural,
						BodyPartConnection::BloodSupply,
					],
					)
			};
			(so $p:expr, $c:expr) => {
				mob.add_body_part_sym(
					$c.build().unwrap(),
					$p,
					vec![
						BodyPartConnection::Container,
						BodyPartConnection::BloodSupply,
					],
					)
			};
			(sb $p:expr, $c:expr) => {
				mob.add_body_part_sym_both(
					$c.build().unwrap(),
					$p,
					vec![
						BodyPartConnection::Structural,
						BodyPartConnection::BloodSupply,
					],
					)
			};
		}

		let torso = limb!(chest, part!("torso", 100, limb_artery));
		let neck = limb!(torso, part!("neck", 100, limb_artery));
		let head = limb!(neck, part!("head", 50, limb_artery));
		let jaw = limb!(head, part!("jaw", 100, limb_artery));
		let tongue = limb!(
			head,
			BodyPartBuilder::new("tongue")
				.severable(true)
				.layer(BodyPartLayer::new(
					Flesh,
					25,
					enum_map! {
						Piercing => 1.0,
						Slashing => 1.0,
						Blunt => 1.0
					},
				))
				.layer(BodyPartLayer::new(
					Muscle,
					25,
					enum_map! {
						Piercing => 1.0,
						Slashing => 1.0,
						Blunt => 1.0
					},
				))
				.layer(BodyPartLayer::new(
					Artery,
					25,
					enum_map! {
						Piercing => 1.0,
						Slashing => 1.0,
						Blunt => 1.0
					},
				))
		);
		let upper_teeth = limb!(
			head,
			BodyPartBuilder::new("upper_teeth").severable(false).layer(
				BodyPartLayer::new(
					Bone,
					25,
					enum_map! {
						Piercing => 1.0,
						Slashing => 1.0,
						Blunt => 1.0
					},
				)
			)
		);
		let upper_teeth = limb!(
			jaw,
			BodyPartBuilder::new("lower_teeth").severable(false).layer(
				BodyPartLayer::new(
					Bone,
					25,
					enum_map! {
						Piercing => 1.0,
						Slashing => 1.0,
						Blunt => 1.0
					},
				)
			)
		);

		let shoulders = limb!(s chest, part!("shoulder", 100, limb_artery));
		let upper_arms = limb!(sb shoulders, part!("upper_arm", 75, limb_artery));
		let elbows = limb!(sb upper_arms, part!("elbows", 50, limb_artery));
		let lower_arms = limb!(sb elbows, part!("lower_arm", 50, limb_artery));
		let hands =
			limb!(sb lower_arms, part!("hand", 25, limb_artery).can_grab(true));
		let f_thumbs = limb!(sb hands, part!("f_thumb", 10, limb));
		let f_indexes = limb!(sb hands, part!("f_index", 10, limb));
		let f_middles = limb!(sb hands, part!("f_middle", 10, limb));
		let f_rings = limb!(sb hands, part!("f_ring", 10, limb));
		let f_pinkies = limb!(sb hands, part!("f_pinky", 10, limb));

		let hips = limb!(s torso, part!("hip", 100, limb_artery));
		let upper_legs = limb!(sb hips, part!("upper_leg", 100, limb_artery));
		let knees = limb!(sb upper_legs, part!("knee", 75, limb_artery));
		let lower_legs = limb!(sb knees, part!("lower_leg", 75, limb_artery));
		let feet = limb!(sb lower_legs, part!("foot", 50, limb_artery));
		let toes = limb!(sb feet, part!("toes", 50, limb));

		let heart = limb!(o chest, part!("heart", {Artery, 10}));
		let lungs = limb!(so chest, part!("lung", {Flesh, 10}, {Artery, 10}));
		let stomach = limb!(o torso, part!("stomach", {Artery, 10}));
		let liver = limb!(o torso, part!("liver", {Artery, 10}));
		let kidneys = limb!(so torso, part!("kidney", {Artery, 10}));
		let intestines = limb!(o torso, part!("intestines", {Artery, 10}));

		Self { mob: mob }
	}

	pub fn build(&self) -> Mob {
		self.mob.clone()
	}
}
