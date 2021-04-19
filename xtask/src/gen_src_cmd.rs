use flood_tide_gen::{FixupType, MetaType, Pasc};

pub fn do_gen_src() -> anyhow::Result<()> {
    flood_tide_gen::do_gen_src(
        Pasc::Void,
        "xtask/src/aki-gsub-cmd.txt",
        Some("src/conf/cmd.help.rs.txt"),
        Some("src/conf/cmd.match.rs.txt"),
        |opt_str| {
            let tup = match opt_str.lon_or_sho() {
                "color" => (false, false, MetaType::Other("opt_color_when".into())),
                "exp" => (false, true, opt_str.meta_type.clone()),
                "format" => (false, true, opt_str.meta_type.clone()),
                _ => return None,
            };
            Some(FixupType::from_tuple(tup))
        },
    )
}
