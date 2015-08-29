# simplemap

<p align="center">
  <a href="https://travis-ci.org/dpc/simplemap-rs">
      <img src="https://img.shields.io/travis/dpc/simplemap-rs/master.svg?style=flat-square" alt="Build Status">
  </a>
  <a href="https://crates.io/crates/dpc-simplemap">
      <img src="http://meritbadge.herokuapp.com/dpc-simplemap?style=flat-square" alt="crates.io">
  </a>
  <a href="https://gitter.im/dpc/dpc">
      <img src="https://img.shields.io/badge/GITTER-join%20chat-green.svg?style=flat-square" alt="Gitter Chat">
  </a>
  <br>
  <strong><a href="//dpc.github.io/simplemap-rs/">Documentation</a></strong>
</p>


## Introduction

Simple Map with default for missing values and compacting (removal of default values from underlying map).

## Usage

Simplemap version 0.0.2 compiles only with Rust nightly. If you want stable-compatibility, stick with version 0.0.1.

In `Carto.toml`

	[dependencies]
	dpc-simplemap = "*"

In `src/main.rs`:

	extern crate simplemap;
