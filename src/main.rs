use std::{env::args, process::exit};

use libc::{fork, waitpid, WEXITSTATUS, WIFEXITED};

fn main() {
	let mut a = args()
		.nth(1)
		.unwrap()
		.parse::<i32>()
		.unwrap();
	assert!(a <= 14 && 0 <= a);
	let b = a;

	loop {
		if a == 0 || a == 1 {
			if b == a {
				println!("{a}");
				exit(0);
			} else {
				exit(a);
			}
		}

		let r1 = unsafe { fork() };

		if r1 != 0 {
			let r2 = unsafe { fork() };

			if r2 != 0 {
				let mut s = 0;

				let s1 = &mut 0 as *mut i32;

				unsafe { waitpid(r1, s1, 0) };

				if WIFEXITED(unsafe { *s1 }) {
					s += WEXITSTATUS(unsafe { *s1 });
				} else {
					panic!()
				}

				let s2 = &mut 0 as *mut i32;

				unsafe { waitpid(r2, s2, 0) };

				if WIFEXITED(unsafe { *s2 }) {
					s += WEXITSTATUS(unsafe { *s2 });
				} else {
					panic!()
				}

				if b == a {
					println!("{s}");
					exit(0);
				} else {
					exit(s);
				}
			} else {
				a -= 2;
			}
		} else {
			a -= 1;
		}
	}
}
