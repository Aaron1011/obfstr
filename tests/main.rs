// Ensures the macro machinery works outside the scope of the crate itself...
#[test]
fn main() {
	let (a, b) = (obfstr::random!(u64), obfstr::random!(u64));
	assert_ne!(a, b);

	assert_eq!(obfstr::obfstr!("Hello world"), "Hello world");

	assert_eq!(obfstr::obfstr!("This literal is very very very long to see if it correctly handles long string"),
	                           "This literal is very very very long to see if it correctly handles long string");

	assert_eq!(obfstr::obfstr!("\u{20}\0"), " \0");
	assert_eq!(obfstr::obfstr!("\"\n\t\\\'\""), "\"\n\t\\\'\"");

	assert_eq!(obfstr::obfstr!(L"ABC"), &[b'A' as u16, b'B' as u16, b'C' as u16]);
}
