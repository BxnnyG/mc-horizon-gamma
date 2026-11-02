// The mod's row in Options > Mods, with Gamma Utils' keys.
fn gammautils_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::gammautils as g;
    mods.push(crate::game::native_hooks::NativeMod {
        id: g::ID.into(),
        title: format!("{} {}", g::NAME, g::VERSION),
        description: g::DESCRIPTION.into(),
        keys: g::KEYS.iter().map(|k| [k.0.into(), k.1.into(), k.2.into()]).collect(),
    });
}
