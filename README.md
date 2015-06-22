# simplemap

<p align="center">
  <a href="https://travis-ci.org/dpc/simplemap-rs">
      <img src="https://img.shields.io/travis/dpc/simplemap-rs/master.svg?style=flat-square" alt="Build Status">
  </a>
  <a href="https://crates.io/crates/simplemap">
      <img src="http://meritbadge.herokuapp.com/simplemap?style=flat-square" alt="crates.io">
  </a>
  <img src="https://img.shields.io/badge/GITTER-join%20chat-green.svg?style=flat-square" alt="Gitter Chat">
  <br>
  <strong><a href="//dpc.github.io/simplemap-rs/">Documentation</a></strong>
</p>


## Introduction

Simple Map with default for missing values and compacting (removal of default values from underlying map).

## Usage

In `Carto.toml`

	[dependencies]
	dpc-simplemap = "*"

In `src/main.rs`:

	extern crate simplemap;
