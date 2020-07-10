use colored::Colorize;
use enum_map::Enum;
use serde::Serialize;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Copy, Enum, Serialize, Debug, Eq, PartialEq)]
pub enum DamageType {
	/** Damage all layers, and potentially organs. Can't sever. **/
	Piercing,
	/** Damage the top layer with more than 0 hp */
	Slashing,
	/** Distribute total damage across all layers, and potentially organs */
	Blunt,
}

impl DamageType {
	fn format(&self, string: &str) -> colored::ColoredString {
		match self {
			DamageType::Piercing => string.red().italic(),
			DamageType::Slashing => string.magenta().italic(),
			DamageType::Blunt => string.yellow().italic(),
		}
	}
}

impl Display for DamageType {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(
			f,
			"{}",
			self.format(match self {
				DamageType::Piercing => "Piercing",
				DamageType::Slashing => "Slashing",
				DamageType::Blunt => "Blunt",
			})
		)
	}
}

#[derive(Clone, Copy, Serialize, Debug)]
pub struct DamageInstance {
	pub amount: i32,
	pub damage_type: DamageType,
}

impl Display for DamageInstance {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(
			f,
			"{}{} {}",
			if self.amount > 0 {
				self.damage_type.format("+")
			} else {
				"".clear()
			},
			self.damage_type.format(self.amount.to_string().as_str()),
			self.damage_type
		)
	}
}

#[derive(Clone, Copy, Serialize, Debug)]
pub enum DamageResult {
	TookDamage(DamageInstance),
	Destroyed,
	Severed,
	NoDamage,
}

#[cfg(test)]
mod test {
	use super::*;
	use colored::Colorize;

	#[test]
	fn test_colors() {
		assert_eq!(
			format!(
				"{}",
				DamageInstance {
					amount: -10,
					damage_type: DamageType::Slashing
				}
			),
			format!(
				"{}{} {}",
				"".clear(),
				"-10".magenta().italic(),
				"Slashing".magenta().italic()
			)
		);
	}
}
