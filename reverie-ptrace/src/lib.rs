/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 * All rights reserved.
 *
 * This source code is licensed under the BSD-style license found in the
 * LICENSE file in the root directory of this source tree.
 */

//! Reverie ptrace backend.
//!
//! ptraced task implements `Guest` trait.
//!
//! `TracedTask` implements handlers for ptrace events including
//! seccomp. Notable ptrace events include:
//!
//! `PTRACE_EVENT_EXEC`: `execvpe` is about to return, tracee stopped
//!  at entry point.
//!
//! `PTRACE_EVENT_FORK/VFORK/CLONE`: when `fork`/`vfork`/`clone` is about
//! to return
//!
//! `PTRACE_EVENT_SECCOMP`: seccomp stop caused by `RET_TRACE`
//! NB: we patch syscall in seccomp ptrace stop.
//!
//! `PTRACE_EVENT_EXIT`: process is about to exit
//!
//! signals: tracee's pending signal stop.
//!
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![feature(internal_output_capture)]
#![feature(never_type)]
#![feature(int_roundings)]

mod children;
mod cp;
#[allow(unused)]
#[cfg(target_arch = "x86_64")]
mod debug;
#[cfg(target_arch = "x86_64")]
pub mod decoder;
mod error;
mod gdbstub;
mod perf;
pub mod regs;
mod stack;
mod task;
pub mod testing;
mod timer;
mod tracer;
mod validation;
mod vdso;

pub use perf::is_perf_supported;
pub use tracer::spawn_fn;
pub use tracer::spawn_fn_with_config;
pub use tracer::GdbConnection;
pub use tracer::Tracer;
pub use tracer::TracerBuilder;
