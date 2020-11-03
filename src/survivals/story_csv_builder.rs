fn build_spell(number: i32, finish: bool) -> String {
    if number < 0 {
        return "248,248,248,248,0".into();
    }
    let spell_idx = number * 4;
    format!(
        "{},{},{},{},{}",
        spell_idx,     // easy
        spell_idx + 1, // normal
        spell_idx + 2, // hard
        spell_idx + 3, // lunatic
        if finish { 2 } else { 1 },
    )
}

fn build_char(stage: i32, name: &str, num_spells: i32) -> Vec<String> {
    // stage, enemy, numBattles, numSpells
    let names = format!(
        "{},{},{},{}",
        stage,
        name,
        match num_spells {
            1 => "1",
            2 => "3",
            3 => "4",
            4 => "6",
            5 => "7",
            _ => panic!(),
        },
        num_spells - 1,
    );
    let spells_src = if name == "tenshi" {
        vec![
            (-1, false),
            (0, false),
            (1, false),
            (-1, false),
            (2, false),
            (3, true),
            (4, true),
        ]
    } else {
        match num_spells {
            1 => vec![(0, true)],
            2 => vec![(-1, false), (0, false), (1, true)],
            3 => vec![(-1, false), (0, false), (1, false), (2, true)],
            4 => vec![
                (-1, false),
                (0, false),
                (1, false),
                (-1, false),
                (2, false),
                (3, true),
            ],
            5 => vec![
                (-1, false),
                (0, false),
                (1, false),
                (-1, false),
                (2, false),
                (3, false),
                (4, true),
            ],
            _ => panic!(),
        }
    };
    let spells = spells_src.into_iter().map(|x| build_spell(x.0, x.1));
    let mut vec = vec![names];
    vec.extend(spells);
    vec
}

pub fn build_story_csv() -> String {
    let charactors_src = [
        ("reimu", 5),
        ("marisa", 3),
        ("sakuya", 4),
        ("alice", 3),
        ("patchouli", 3),
        ("youmu", 4),
        ("remilia", 4),
        ("yuyuko", 3),
        ("yukari", 5),
        ("suika", 5),
        ("udonge", 3),
        ("aya", 4),
        ("komachi", 3),
        ("iku", 4),
        ("tenshi", 5),
    ];
    let charactors = charactors_src
        .iter()
        .enumerate()
        .map(|(i, x)| build_char(i as i32, x.0, x.1))
        .flatten();
    let mut vec = vec![charactors_src.len().to_string()];
    vec.extend(charactors);
    vec.join("\n") + "\n"
}
