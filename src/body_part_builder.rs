use crate::body_part::{BodyPart, BodyPartConnection, BodyPartLayer};
use std::fmt;

#[derive(Debug)]
pub struct BuilderError;
impl fmt::Display for BuilderError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "BodyPart must have at least 1 layer")
	}
}

type Result<T> = std::result::Result<T, BuilderError>;

pub struct BodyPartBuilder {
	body_part: BodyPart,
}

impl BodyPartBuilder {
	pub fn new(name: &str) -> Self {
		Self {
			body_part: BodyPart {
				name: name.to_string(),
				..BodyPart::default()
			},
		}
	}
	pub fn layer(mut self, layer: BodyPartLayer) -> Self {
		self.body_part.layers.push(layer);
		self
	}
	pub fn severable(mut self, severable: bool) -> Self {
		self.body_part.severable = severable;
		self
	}
	pub fn can_grab(mut self, can_grab: bool) -> Self {
		self.body_part.can_grab = can_grab;
		self
	}
	pub fn build(&self) -> Result<BodyPart> {
		if self.body_part.layers.len() == 0 {
			return Err(BuilderError);
		}
		Ok(self.body_part.clone())
	}
}
