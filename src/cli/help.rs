pub fn print_root_help() {
	let name = env!("CARGO_PKG_NAME");
	let description = env!("CARGO_PKG_DESCRIPTION");
	let version = env!("CARGO_PKG_VERSION");
	println!(
		r#"{name} - v{version}
{description}

usage:
  {name}        -i <file> -o <file> [options]
  {name} play   -i <file>           [options]
  {name} probe  -i <file>           [options]

help:
  -h, --help     show help
  -h  play       help for play
  -h  probe      help for probe

global:
  -i  <file>          input file
  -o  <file>          output file
  -t, --apply  <expr> filters[transforms]

streams:
  -a, --audio    <opts>     audio options
  -v, --video    <opts>     video options
  -s, --subtitle <opts>  subtitle options
"#,
	);
}
