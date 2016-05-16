#![allow(dead_code)]

use std::collections::HashMap;
use util::populate_from_file;

slr_def!
{
	#[derive(Clone, Debug, Default)]
    pub struct SpellConfig
    {
        poem: String = "".to_string(),
        summon: String = "".to_string()
    }
}

slr_def!
{
	#[derive(Clone, Debug)]
    pub struct SpellsConfig
    {
        spells: Vec<SpellConfig> = vec![]
    }
}

pub fn load_spells() -> HashMap<String, String>
{
	let mut config = SpellsConfig::new();
	populate_from_file("data/spells.cfg", &mut config).unwrap();
	
	let mut ret = HashMap::new();
	for spell in config.spells
	{
		let mut cleaned = String::new();
		for line in spell.poem.lines()
		{
			for word in line.split_whitespace()
			{
				cleaned.push_str(word);
				cleaned.push_str(" ");
			}
			cleaned.pop();
			cleaned.push_str("\n");
		}
		cleaned.pop();
		ret.insert(cleaned, spell.summon.clone());
	}
	ret
}
