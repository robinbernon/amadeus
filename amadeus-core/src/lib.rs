#![doc(html_root_url = "https://docs.rs/amadeus-core/0.2.0")]
#![feature(never_type)]
#![feature(specialization)]
#![feature(read_initializer)]
#![recursion_limit = "25600"]

macro_rules! impl_par_dist {
	($($body:tt)*) => {
		$($body)*
		const _: () = {
			use crate::impl_par_dist::*;
			#[allow(unused_imports)]
			use crate::impl_par_dist::{combiner_par_sink,folder_par_sink};
			$($body)*
		};
	}
}
mod impl_par_dist {
	pub use crate::{
		combiner_dist_sink as combiner_par_sink, dist_pipe::DistributedPipe as ParallelPipe, dist_sink::{DistributedSink as ParallelSink, FromDistributedStream as FromParallelStream}, dist_stream::DistributedStream as ParallelStream, folder_dist_sink as folder_par_sink, pool::ProcessSend as Send
	};
}

macro_rules! impl_par_dist_rename {
	($($body:tt)*) => {
		$($body)*
		rename! { [
			ParallelStream DistributedStream
			ParallelSink DistributedSink
			ParallelPipe DistributedPipe
			FromParallelStream FromDistributedStream
			IntoParallelStream IntoDistributedStream
			ParStream DistStream
			Send ProcessSend
			ImplParallelStream ImplDistributedStream
			IterParStream IterDistStream
			into_par_stream into_dist_stream
			par_stream dist_stream
			assert_parallel_sink assert_distributed_sink
			assert_parallel_pipe assert_distributed_pipe
			assert_parallel_stream assert_distributed_stream
		] $($body)* }
	}
}
macro_rules! rename {
	([$($_from:ident $_to:ident)*] $($_body:tt)*) => (rename!(@inner [$] [$($_from $_to)*] $($_body)*););
	(@inner [$d:tt] [$($_from:ident $_to:ident)*] $($_body:tt)*) => (
		macro_rules! __rename {
			$(
				(@munch [$d ($d done:tt)*] $_from $d ($d body:tt)*) => (__rename!{@munch [$d ($d done)* $_to] $d ($d body)*});
			)*
			// (@munch [$d ($d done:tt)*] $d head:expr $d ($d body:tt)*) => (__rename!{@munch [$d ($d done)* $d head] $d ($d body)*});
			// (@munch [$d ($d done:tt)*] $d head:block $d ($d body:tt)*) => (__rename!{@munch [$d ($d done)* $d head] $d ($d body)*});
			(@munch [$d ($d done:tt)*] { $d ($d head:tt)* } $d ($d body:tt)*) => (__rename!{@munch [$d ($d done)* { __rename!{$d ($d head)*} }] $d ($d body)*});
			// (@munch [$d ($d done:tt)*] ( $d ($d head:tt)* ) $d ($d body:tt)*) => (__rename!{@munch [$d ($d done)* ( __rename!{$d ($d head)*} )] $d ($d body)*});
			// (@munch [$d ($d done:tt)*] [ $d ($d head:tt)* ] $d ($d body:tt)*) => (__rename!{@munch [$d ($d done)* [ __rename!{$d ($d head)*} ]] $d ($d body)*});
			(@munch [$d ($d done:tt)*] $d head:tt $d ($d body:tt)*) => (__rename!{@munch [$d ($d done)* $d head] $d ($d body)*});
			(@munch [$d ($d done:tt)*]) => ($d ($d done)*);
			(@__rename $d i:ident) => ($d i);
			($d ($d body:tt)*) => (__rename!{@munch [] $d ($d body)*});
		}
		__rename!($($_body)*);
	);
}

pub mod dist_pipe;
pub mod dist_sink;
pub mod dist_stream;
pub mod file;
pub mod into_dist_stream;
pub mod misc_serde;
pub mod pool;
pub mod sink;
mod source;
pub mod util;

pub use source::*;
