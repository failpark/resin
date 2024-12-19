use std::{
	path::Path,
	process::exit,
};

use anyhow::Result;
use clap::ArgMatches;
use git2::{
	Commit,
	Diff,
	Index,
	IndexAddOption,
	Oid,
	Reference,
	Repository,
	Signature,
	Tree,
};

use super::utils::fail;
use crate::{
	conf::Config,
	inputs::{
		ask_for_path,
		prompt_for_path,
		Inputs,
	},
	utils::{
		output_info,
		output_success,
	},
};

pub fn get_repo() -> Repository {
	let repo = Repository::open_from_env();
	if let Err(ref e) = repo {
		fail(e);
	}
	repo.unwrap()
}

pub fn commit_changes(conf: &Config, args: &ArgMatches, inputs: &Inputs) -> Result<()> {
	let repo = get_repo();
	if args.get_one::<bool>("all").is_some() {
		add_all(&mut get_index(&repo));
	} else {
		check_emptiness(&repo)
	};
	let signoff = if conf.get_sign() {
		format_signoff(&get_signatures(&repo)).unwrap_or_default()
	} else {
		String::new()
	};
	commit(&repo, &gen_commit_msg(inputs, signoff));
	Ok(())
}

// Im handling the err ... don't know why rustc complains
#[allow(unused_must_use)]
fn add(index: &mut Index, path: &Path) {
	index
		.add_all(path, IndexAddOption::DEFAULT, None)
		.map_err(fail);
	index.write().map_err(fail);
}

// Im handling the err ... don't know why rustc complains
#[allow(unused_must_use)]
fn add_all(index: &mut Index) {
	index
		.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)
		.map_err(fail);
	index.write().map_err(fail);
}

/// Check if there are is anything in staging
/// if not we should offer to specify a path or exit
pub fn check_emptiness(repo: &Repository) {
	let mut index = get_index(repo);
	let err_msg = "Your staging area is empty";
	if is_empty(repo) {
		output_info(err_msg);
		if !ask_for_path() {
			exit(0)
		}
		let path = prompt_for_path();
		add(&mut index, path.as_path());
		// if the index is still empty... just fail
		if is_empty(repo) {
			fail(err_msg)
		}
	}
}

fn get_head_tree(repo: &Repository) -> Tree {
	let head = get_head(repo).peel_to_tree();
	if let Err(ref e) = head {
		fail(e);
	}
	head.unwrap()
}

fn is_empty(repo: &Repository) -> bool {
	get_diff(repo).deltas().len() == 0
}

fn get_diff(repo: &Repository) -> Diff {
	let head = get_head_tree(repo);
	let diff = repo.diff_tree_to_index(Some(&head), None, None);
	if let Err(ref e) = diff {
		fail(e);
	}
	diff.unwrap()
}

fn get_index(repo: &Repository) -> Index {
	let index = repo.index();
	if let Err(ref e) = index {
		fail(e);
	}
	index.unwrap()
}

fn get_signatures(repo: &Repository) -> Signature {
	let sig = repo.signature();
	if let Err(ref e) = sig {
		fail(e);
	}
	sig.unwrap()
}

fn get_head(repo: &Repository) -> Reference {
	let head = repo.head();
	if let Err(ref e) = head {
		fail(e);
	}
	head.unwrap()
}

fn get_commit<'a>(ref_: &'a Reference<'a>) -> Commit<'a> {
	let commit = ref_.peel_to_commit();
	if let Err(ref e) = commit {
		fail(e);
	}
	commit.unwrap()
}

fn get_tree(repo: &Repository, oid: Oid) -> Tree {
	let tree = repo.find_tree(oid);
	if let Err(ref e) = tree {
		fail(e);
	}
	tree.unwrap()
}

fn write_changes(index: &mut Index) -> Oid {
	let res = index.write_tree();
	if let Err(ref e) = res {
		fail(e);
	}
	res.unwrap()
}

fn commit(repo: &Repository, message: &str) {
	let sig = get_signatures(repo);
	let head = get_head(repo);
	let parent = get_commit(&head);
	let mut index = get_index(repo);
	let oid = write_changes(&mut index);
	let tree = get_tree(repo, oid);
	let res = repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &[&parent]);
	// TODO: sign commit
	match res {
		Err(e) => fail(e),
		Ok(_) => output_success("Committed changes"),
	}
}

pub fn get_branch_name(repo: &Repository) -> Option<String> {
	let branch = get_head(repo);
	branch.name().map(|val| val.into())
}

fn format_scope(scope: &Option<&str>) -> String {
	match scope {
		None => String::new(),
		Some(scope) => format!("({scope})"),
	}
}

fn gen_commit_msg(inputs: &Inputs, signoff: String) -> String {
	let Inputs {
		change_type,
		scope,
		description,
		long_description,
		breaking_changes,
		ticket,
	} = inputs;
	let scope = format_scope(scope);
	let exclamation = if breaking_changes.is_empty() { "" } else { "!" };
	let ticket = if !ticket.is_empty() {
		ticket.to_owned() + "\n"
	} else {
		ticket.to_owned()
	};

	format!(
		"{change_type}{scope}{exclamation}: {description}
{ticket}{long_description}\n
{breaking_changes}\n
{signoff}"
	)
	// breaking change and or signoff could be missing
	.trim_end()
	.into()
}

fn format_signoff(signature: &Signature) -> Option<String> {
	let name = signature.name();
	let email = signature.email();
	if let (Some(name), Some(email)) = (name, email) {
		Some(format!("Signed-off-by: {name} <{email}>"))
	} else {
		None
	}
}
