// SPDX-License-Identifier: BSD-3-Clause

pub struct TreeView<'a, Model>
where
	Model: TreeItem
{
	model: &'a Model
}

pub trait TreeItem
{
	fn displayName(&self) -> String;
	fn children(&self) -> &[&Self];
}
