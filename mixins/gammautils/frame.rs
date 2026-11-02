// Once a frame: the gamma the renderer draws with.
fn gammautils_frame(game: &mut crate::game::Game<'_>, elapsed: std::time::Duration) {
    crate::game::gammautils::frame(game, elapsed);
}
