fn main() {
	glib_build_tools::compile_resources(
		"assets",
		"assets/resources.xml",
		"resources.gresource",
	);
}
