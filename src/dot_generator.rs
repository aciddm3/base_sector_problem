use crate::sector::*;

pub fn generate (src : &[Sector]) -> String {
	let mut result = String::from("strict graph {\n");

	for index in src.iter() {
		result.push_str(&generate_sector(index));
	}
	result.push('}');
	result
}

fn generate_sector (sec : &Sector) -> String {
	let based = sec.is_based;
	let seci = sec.id;
	let mut relations_str = String::new();
	for index in sec.relations.iter() {
		relations_str.push_str(
			format!("\ti{seci} -- i{index} ;\n").as_str()
		);
	}
	let result = format!("\ti{seci} [label=<
    \t\t<B>{seci}</B>
    \t\t<BR/>
    \t\t<FONT POINT-SIZE=\"10\">
    \t\t\tCost:{}<BR/>
	\t\t</FONT>
    \t\t>, color={}];\n",
	sec.cost, 
	if based {"green"} else {"black"}
	);
	result + relations_str.as_str()
}