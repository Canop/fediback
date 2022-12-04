use {
    termimad::MadSkin,
};

pub fn make_skin() -> MadSkin {
    match terminal_light::luma() {
        Ok(luma) if luma > 0.6 => MadSkin::default_light(),
        Ok(_) => MadSkin::default_dark(),
        Err(_) => MadSkin::default(), // this skin works in both light and dark
    }
}

