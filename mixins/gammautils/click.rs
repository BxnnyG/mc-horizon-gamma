// A click on the mod's settings page.
fn gammautils_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::gammautils as g;
    if mod_id == g::ID {
        g::click(game, button);
    }
}
