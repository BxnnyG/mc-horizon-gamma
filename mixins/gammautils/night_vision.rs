// The night vision the renderer uses: Gamma Utils' strength while it is on.
fn gammautils_night_vision(game: &crate::game::Game<'_>) -> core::ops::ControlFlow<f32> {
    let v = crate::game::gammautils::night_vision(game);
    if v > 0.0 {
        core::ops::ControlFlow::Break(v)
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
