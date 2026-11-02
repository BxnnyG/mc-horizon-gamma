// The mod's settings page.
fn gammautils_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::gammautils as g;
    if mod_id == g::ID {
        buttons.extend(g::labels(game));
    }
}
