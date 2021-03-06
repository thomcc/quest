use crate::{Object, Result};
use super::Value;
use std::hash::Hash;
use std::borrow::Borrow;
use std::iter::FromIterator;
use std::sync::RwLock;

#[derive(Debug)]
pub struct Parents(RwLock<Inner>);

#[derive(Debug, Clone)]
enum Inner {
	None,
	Builtin(Vec<Object>),
	Object(Object)
}

impl Clone for Parents {
	#[inline]
	fn clone(&self) -> Self {
		Self::from_inner(self.0.read().unwrap().clone())
	}
}

impl Parents {
	#[inline]
	fn from_inner(inner: Inner) -> Self {
		Parents(RwLock::new(inner))
	}
}

impl Default for Parents {
	#[inline]
	fn default() -> Self {
		Self::from_inner(Inner::None)
	}
}

impl From<Value> for Parents {
	#[inline]
	fn from(value: Value) -> Self {
		Self::from_inner(Inner::Object(value.into()))
	}
}

impl From<Parents> for Value {
	#[inline]
	fn from(parents: Parents) -> Self {
		Value::from(Object::from(parents))
	}
}

impl From<Parents> for Object {
	fn from(parents: Parents) -> Self {
		match parents.0.into_inner().unwrap() {
			Inner::None => Object::default(),
			Inner::Builtin(vec) => vec.into(),
			Inner::Object(obj) => obj
		}
	}
}

impl From<Vec<Object>> for Parents {
	#[inline]
	fn from(vec: Vec<Object>) -> Self {
		Self::from_inner(Inner::Builtin(vec))
	}
}

impl From<()> for Parents {
	#[inline]
	fn from(_: ()) -> Self {
		Self::from_inner(Inner::None)
	}
}

impl From<Object> for Parents {
	#[inline]
	fn from(obj: Object) -> Self {
		Self::from_inner(Inner::Object(obj))
	}
}

impl FromIterator<Object> for Parents {
	fn from_iter<I: IntoIterator<Item=Object>>(iter: I) -> Self {
		Self::from(iter.into_iter().collect::<Vec<Object>>())
	}
}

impl Parents {
	pub fn add_parent(&mut self, parent: Object) -> Result<()> {
		let mut inner = self.0.write().unwrap();
		match *inner {
			Inner::None => *inner = Inner::Builtin(vec![parent]),
			Inner::Builtin(ref mut vec) => vec.push(parent),
			Inner::Object(ref obj) => { obj.call_attr_lit("push", &[&parent])?; },
		}

		Ok(())
	}
	pub fn to_object(&self) -> Object {
		let mut inner = self.0.write().unwrap();
		match *inner {
			Inner::None => {
				let obj = Object::default();
				*inner = Inner::Object(obj.clone());
				obj
			},
			Inner::Builtin(ref mut vec) => {
				let obj = Object::from(std::mem::replace(vec, vec![]));
				*inner = Inner::Object(obj.clone());
				obj
			},
			Inner::Object(ref obj) => obj.clone()
		}
	}

	fn with_iter<F: FnOnce(std::slice::Iter<'_, Object>) -> Result<R>, R>(&self, f: F) -> Result<R> {
		match *self.0.read().unwrap() {
			Inner::None => f([].iter()),
			Inner::Builtin(ref parents) => f(parents.iter()),
			Inner::Object(ref object) => 
				object.downcast_call::<crate::types::List>().and_then(|list| f(list.iter()))
		}
	}

	pub fn keys(&self) -> Result<Vec<Object>> {
		self.with_iter(|iter| Ok(iter.map(|x| x.clone()).collect()))
	}

	pub fn has_lit<K: Hash + Eq + ?Sized>(&self, key: &K) -> Result<bool>
	where
		for <'a> &'a str: Borrow<K>
	{
		self.with_iter(|iter| {
			for parent in iter {
				if parent.has_attr_lit(key)? {
					return Ok(true)
				}
			}
			Ok(false)
		})
	}

	pub fn get_lit<K: Hash + Eq + ?Sized>(&self, key: &K) -> Result<Option<Value>>
	where
		for <'a> &'a str: Borrow<K>
	{
		self.with_iter(|iter| {
			for parent in iter {
				if let Some(value) = parent.get_value_lit(key)? {
					return Ok(Some(value))
				}
			}
			Ok(None)
		})
	}

	pub fn has_obj(&self, key: &Object) -> Result<bool> {
		self.with_iter(|iter| {
			for parent in iter {
				if parent.has_attr(key)? {
					return Ok(true)
				}
			}
			Ok(false)
		})
	}

	pub fn get_obj(&self, key: &Object) -> Result<Option<Value>> {
		self.with_iter(|iter| {
			for parent in iter {
				if let Some(value) = parent.get_value(key)? {
					return Ok(Some(value))
				}
			}
			Ok(None)
		})
	}
}

// impl IntoIterator for Parents {
// 	type Item = Object;
// 	type IntoIter = Vec<Object> as Iterator;
// 	fn into_iter(self) -> Self::IntoIter {
// 		Vec::from(self).into()
// 	}
// }


















