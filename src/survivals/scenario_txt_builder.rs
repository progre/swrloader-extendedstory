const DATA: [(&str, i32, i32); 15] = [
    ("reimu", 0, 10),   // neutral: 0(bgm)
    ("marisa", 11, 11), // neutral: 1(stage, bgm)
    ("sakuya", 12, 12),
    ("alice", 13, 13), // neutral: 1(stage, bgm)
    ("patchouli", 14, 15),
    ("youmu", 15, 14),
    ("remilia", 16, 17),
    ("yuyuko", 17, 16),
    ("yukari", 10, 18),
    ("suika", 5, 19),
    ("udonge", 18, 20),
    ("aya", 3, 21),     // neutral: 3(bgm)
    ("komachi", 2, 22), // neutral: 2(bgm)
    ("iku", 4, 4),
    ("tenshi", 5, 5),
];

pub fn build_scenario_txt(player: &str, index: i32) -> String {
    let current = &DATA[index as usize];
    let first_script = if index == 0 {
        format!(
            r"
Stage:{},0
{}
",
            current.1,
            if player == "remilia" {
                // レミリアのみ濃霧固定
                "Action:left,103"
            } else {
                ""
            },
        )
    } else {
        "".into()
    };
    let weather_script = if player == "remilia" {
        ""
    } else {
        "Action:right,103"
    };
    let additional_script = if current.0 == "suika" {
        // 萃香専用アクション
        "Action:right,107"
    } else {
        ""
    };
    let next_stage_script = match DATA.get((index + 1) as usize) {
        None => "".into(),
        Some(next) => format!(
            r##"
FadeBgm:2000,0
Stage:{},120
Action:right,201
Action:left,150
Action:left,4
Sleep:120;
"##,
            next.1,
        ),
    };
    format!(
        r##"
Character:left,160,0,0,8888FF
Character:right,160,0,1,FF8888

{}
{}
Action:right,100
Action:left,4
Action:left,152

Label:Start
PlayBgm:data/bgm/st{:>02}.ogg
Action:left,2

Sleep:60;
Action:right,101
Action:right,102
{}
Action:left,3
End:

Label:Continue
Action:{},113
End:

Label:Win

Action:right,200
Action:left,202

{}
End:
"##,
        first_script,
        weather_script,
        current.2,
        additional_script,
        // コンティニュー用天候セット
        if player == "remilia" { "left" } else { "right" },
        next_stage_script,
    )
}

pub fn build_ending_txt() -> &'static str {
    r##"#
Label:Start

PlayBgm:data/bgm/sr.ogg
Congratulations!\

End:
"##
}

#[cfg(test)]
mod tests {
    #[test]
    fn zero_padding() {
        assert_eq!(&format!("{:>03}", 1), "001");
    }
}
