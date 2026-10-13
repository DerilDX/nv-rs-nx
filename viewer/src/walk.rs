//! Walking through the cell as the player, and going through load doors.
//!
//! The rules (speeds, when running and jumping are allowed, the jump) are
//! `world::locomotion`'s; the collision is the models' Havok shapes
//! (`cellview::ViewerScene::collision`) and the capsule is the `physics`
//! crate's character.

use bevy::prelude::*;
use cellview::{space, DoorData, EYE_HEIGHT};
use physics::{Character, CharacterShape, Collider};
use world::locomotion::{self, SpeedSettings};

use crate::exterior::{door_start, PendingExterior};
use crate::{FlyCamera, GameFiles, PendingScene};

/// The current cell's solid surfaces.
#[derive(Resource)]
pub struct CellCollision(pub Collider);

/// The current cell's load doors.
#[derive(Resource)]
pub struct Doors(pub Vec<DoorData>);

/// The line under the crosshair (a door's destination).
#[derive(Component)]
pub struct Prompt;

#[derive(Resource)]
pub struct Player {
    pub walking: bool,
    pub character: Character,
    /// Where the cell started the player, for R.
    start: [f32; 3],
    /// False while the ground under the player is still loading (outdoors).
    pub ready: bool,
    /// Dropped from the free camera (`): the landing does no damage.
    from_camera: bool,
    /// The mover's flags from the keys this frame (`0093e860`: forward,
    /// back, left, right, running) and the speed they move the player at,
    /// for the third-person body's animations (`player_body`).
    pub moving: world::animation::MoveFlags,
    pub speed: f32,
    /// Sneaking: the mover's flag 0x400, which the Sneak control (Left
    /// Ctrl) toggles (`0093e860`, see [`may_toggle_sneak`]).
    pub sneaking: bool,
    /// How far the eye has gone from the standing height toward the
    /// sneaking one (0 to 1; [`eye_offset`]).
    sneak_blend: f32,
    /// Always Run (the player's +0x651; `None` until set from
    /// `bAlwaysRunByDefault`), which the Always Run control toggles; and
    /// Auto Move (+0x652), which the Auto Move control toggles and any
    /// movement key ends (`0093e860`).
    always_run: Option<bool>,
    auto_move: bool,
    /// The game keeps the player's running speed once when it loads
    /// (`0055d760`, `physics::controller::set_reference_speed`); set at the
    /// first walking update here.
    reference_speed_set: bool,
    /// Counts the character proxy updates (for `clutter` to apply each
    /// update's touches once).
    pub surface_serial: u32,
}

impl Player {
    pub fn new(walking: bool) -> Self {
        Self {
            walking,
            character: Character::new([0.0; 3]),
            start: [0.0; 3],
            ready: true,
            from_camera: false,
            moving: world::animation::MoveFlags::default(),
            speed: 0.0,
            sneaking: false,
            sneak_blend: 0.0,
            always_run: None,
            auto_move: false,
            reference_speed_set: false,
            surface_serial: 0,
        }
    }

    /// Puts the player at a cell's arrival point (feet, game units).
    pub fn arrive(&mut self, feet: [f32; 3]) {
        self.start = feet;
        self.character = Character::new(feet);
        self.ready = true;
    }

    /// The physical player's feet for a view at `eye` (game units).
    ///
    /// While walking, scripts and other systems may move the camera without
    /// moving the character. Reports and saves must keep using the character
    /// position in that case. Free-camera mode retains its legacy convention
    /// of treating the camera as the player and subtracting eye height.
    pub fn position_for_view(&self, eye: [f32; 3]) -> [f32; 3] {
        if self.walking {
            self.character.feet
        } else {
            [eye[0], eye[1], eye[2] - EYE_HEIGHT]
        }
    }
}

/// The console key (`) switches between walking and flying (the free
/// camera is the viewer's; the game's is the console's `tfc`). F is the
/// game's view key (`player_camera`).
pub fn toggle_walking(
    keys: Res<ButtonInput<KeyCode>>,
    mut player: ResMut<Player>,
    cameras: Query<&Transform, With<FlyCamera>>,
) {
    if !keys.just_pressed(KeyCode::Backquote) {
        return;
    }
    player.walking = !player.walking;
    if player.walking {
        // Land wherever the camera is.
        if let Ok(transform) = cameras.single() {
            let [x, y, z] = game_point(transform.translation);
            player.character = Character::new([x, y, z - EYE_HEIGHT]);
            player.from_camera = true;
        }
    }
}

/// A point from Bevy's space (meters, y up) back to the game's (units, z
/// up).
pub fn game_point(p: Vec3) -> [f32; 3] {
    let s = 1.0 / space::METERS_PER_UNIT;
    [p.x * s, -p.z * s, p.y * s]
}

/// The people the player runs into: everyone alive here, upright
/// cylinders where they stand. People's controllers all have the game's
/// one size (`physics::CharacterShape::PLAYER`, 128 tall); a creature's
/// radius comes from its skeleton (`fighting::Kit`), its height taken as
/// 128 × its scale (a guess: the game sizes it from the skeleton's
/// `BSBound`). The dead don't block (their bodies are on the `DEADBIP`
/// layer, which the player's controller passes), nor do people sitting
/// down or seated in furniture (`ai::passed_through`).
fn people(
    walkers: &Query<&crate::ai::Walker>,
    state: &world::scripting::GameState,
) -> Vec<physics::Person> {
    walkers
        .iter()
        .filter(|w| {
            !state.dead.contains(&w.reference) && !crate::ai::passed_through(state, w.reference)
        })
        .map(|w| {
            let creature = w.kit.as_ref().is_some_and(|k| k.creature.is_some());
            physics::Person {
                feet: w.position,
                radius: w
                    .kit
                    .as_ref()
                    .map_or(CharacterShape::PLAYER.radius, |k| k.radius),
                height: CharacterShape::PLAYER.height * if creature { w.scale } else { 1.0 },
            }
        })
        .collect()
}

/// What can stop the Sneak control from toggling sneaking (`0093e860`,
/// at `00940d5b`): the player dead, in furniture (`GetSitSleepState`,
/// actor vfunc +0x214), or in one of the animation actions its table
/// skips (`00944234`/`0094423c`, by `GetAnimAction` + 1: equipping 0,
/// unequipping 1, reloading 8 and actions 10–16; the names are the GECK's
/// for `GetAnimAction`). Swimming and the other actor tests on the way
/// (vfuncs +0x230, +0x234, `00437bf0`, `00437bd0`, movement flag 0x40)
/// aren't modelled here.
// Translated from 0093e860 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn may_toggle_sneak(dead: bool, in_furniture: bool, anim_action: Option<u8>) -> bool {
    // By anim action + 1 (-1 none … 17): 1 skips the toggle.
    const SKIP: [u8; 19] = [0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0];
    let index = anim_action.map_or(0, |a| usize::from(a) + 1);
    !dead && !in_furniture && SKIP.get(index).is_some_and(|&s| s == 0)
}

/// The first-person camera node's height (`Camera1st`, the game's eye in
/// first person: `00952ff0`, the node `0094e1d0` keeps at `011e07d0`)
/// on the first-person skeleton in its movement idles: standing
/// (`mtidle.kf`) and sneaking (`sneakmtidle.kf`, which lowers `Bip01
/// Looking`). `None` when the files can't be read.
pub fn camera_node_heights(game: &cellview::Game) -> Option<(f32, f32)> {
    let bytes = game
        .assets
        .read(&assets::mesh_path(world::actor::FIRST_PERSON_SKELETON))
        .ok()??;
    let skeleton = nif::Nif::parse(bytes).ok()?.skeleton().ok()?;
    let camera = skeleton
        .iter()
        .position(|b| b.name.eq_ignore_ascii_case("Camera1st"))?;
    let height = |file: &str| {
        let s = crate::viewmodel::sequence(game, &format!("Characters\\_1stPerson\\{file}"))?;
        Some(nif::posed_layers(&skeleton, &[(&s, s.start)])[camera].translation[2])
    };
    Some((height("mtidle.kf")?, height("sneakmtidle.kf")?))
}

/// How far below the standing eye the eye is: the camera node's drop
/// between the two idles × how far the switch has gone. The switch is
/// taken as a straight cross-fade over `fAnimationDefaultBlend` (0.2 s),
/// the game's default blend for a new group (`004949a0`); that the eye
/// follows it linearly is this viewer's reading of the blend.
pub fn eye_offset(heights: Option<(f32, f32)>, blend: f32) -> f32 {
    heights.map_or(0.0, |(stand, sneak)| {
        (sneak - stand) * blend.clamp(0.0, 1.0)
    })
}

/// What walking needs for sneaking: the weapon's state (its animation
/// action, the sights), sounds, the blend time, the camera node's heights;
/// and the attached squares' land, which keeps the player on it.
type SneakParts<'w, 's> = (
    ResMut<'w, crate::combat::PlayerAttack>,
    ResMut<'w, crate::sounds::SoundRequests>,
    Option<Res<'w, crate::actors::AnimSettings>>,
    Local<'s, Option<Option<(f32, f32)>>>,
    Option<Res<'w, crate::ai::CellNav>>,
);

/// Walking: the keys set the wanted speed, the character moves through the
/// cell's collision and around the people in it, and the camera sits at eye
/// height (lower while sneaking, [`eye_offset`]). Not while scripts have
/// turned movement off (`DisablePlayerControls`). Landing from a fall
/// hurts as the game's falls do (`world::combat::land`).
#[allow(clippy::too_many_arguments)]
pub fn walk(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut collision: ResMut<CellCollision>,
    game: Res<GameFiles>,
    mut state: ResMut<crate::dialogue::DialogueState>,
    mut player: ResMut<Player>,
    mut cameras: Query<(&mut Transform, &FlyCamera)>,
    walkers: Query<&crate::ai::Walker>,
    mut settings: Local<Option<SpeedSettings>>,
    (mut attack, mut sounds, anim, mut heights, nav): SneakParts,
    (mouse, controls): (
        Res<ButtonInput<MouseButton>>,
        Option<Res<crate::controls::Controls>>,
    ),
) {
    if !player.walking || !player.ready {
        return;
    }
    collision.0.set_people(people(&walkers, &state.0));
    let locked = state.0.controls_off[world::scripting::controls::MOVEMENT]
        || state.0.dead.contains(&world::dialogue::PLAYER_REF);
    let Ok((mut transform, camera)) = cameras.single_mut() else {
        return;
    };
    // Home: back to where the place started the player (R reloads).
    if keys.just_pressed(KeyCode::Home) {
        let start = player.start;
        player.character = Character::new(start);
    }
    let order = &game.0.order;
    let player_ref = world::dialogue::PLAYER_REF;
    let now = time.elapsed_secs();
    let controls = controls.map(|c| *c).unwrap_or_default();
    // Sneak (control 8, Left Ctrl by default): each press toggles the
    // mover's sneak flag, with the crouch sound (`NPCHumanCrouchDown` /
    // `NPCHumanCrouchUp`) and out of the sights (`008bb650(0, 0, 0)`).
    let in_furniture = state.0.furniture.contains_key(&player_ref);
    if controls.sneak.just_pressed(&keys, &mouse)
        && !state.0.controls_off[world::scripting::controls::MOVEMENT]
        && may_toggle_sneak(
            state.0.dead.contains(&player_ref),
            in_furniture,
            attack.anim_action(now),
        )
    {
        player.sneaking = !player.sneaking;
        let sound = if player.sneaking {
            "NPCHumanCrouchDown"
        } else {
            "NPCHumanCrouchUp"
        };
        if let Some(s) = order.form_by_editor_id(sound) {
            sounds.0.push(s);
        }
        attack.iron_sights = false;
    }
    // Forward and right on the ground, in game space: yaw 0 looks north.
    let forward = [-camera.yaw.sin(), camera.yaw.cos()];
    let right = [camera.yaw.cos(), camera.yaw.sin()];
    // Always Run (control 10) and Auto Move (control 11) toggle on their
    // press; a movement key held ends Auto Move (`0093e860`, `00940c84` …
    // `00940d48`).
    let always_run = *player.always_run.get_or_insert(controls.always_run_default);
    if controls.always_run.just_pressed(&keys, &mouse) {
        player.always_run = Some(!always_run);
    }
    if controls.auto_move.just_pressed(&keys, &mouse) {
        player.auto_move = !player.auto_move;
    }
    let move_keys = [KeyCode::KeyW, KeyCode::KeyS, KeyCode::KeyD, KeyCode::KeyA];
    if move_keys.iter().any(|&k| keys.pressed(k)) || locked {
        player.auto_move = false;
    }
    let auto = player.auto_move;
    let mut wish = [0.0f32; 2];
    for (key, dir, sign) in [
        (KeyCode::KeyW, forward, 1.0),
        (KeyCode::KeyS, forward, -1.0),
        (KeyCode::KeyD, right, 1.0),
        (KeyCode::KeyA, right, -1.0),
    ] {
        let held = keys.pressed(key) || (auto && key == KeyCode::KeyW);
        if held && !locked {
            wish[0] += dir[0] * sign;
            wish[1] += dir[1] * sign;
        }
    }
    let len = (wish[0] * wish[0] + wish[1] * wish[1]).sqrt();
    let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
    let sneak = player.sneaking;
    let settings = settings.get_or_insert_with(|| SpeedSettings::read(order));
    let over_encumbered = state.0.over_encumbered(order, player_ref);
    // With Always Run on, Shift (Run) walks; off, it runs. Not
    // over-encumbered nor looking down the sights
    // (`world::locomotion::may_run`; grabbing isn't here).
    let wanted = player.always_run.unwrap_or(true) != shift;
    let running = locomotion::may_run(settings, wanted, over_encumbered, attack.iron_sights, 0.0);
    // The game's walking or running speed for the player
    // (`world::locomotion::speed`: SpeedMult, legs, weapon away or drawn,
    // armour, sneaking, and running's perks).
    let speed = locomotion::speed(order, &state.0, settings, player_ref, running, sneak);
    let velocity = if len > 0.0 {
        [wish[0] / len * speed, wish[1] / len * speed]
    } else {
        [0.0, 0.0]
    };
    // How the player moves, for who notices them (`world::detection`):
    // running sets its own flag (0x200) sneaking or not.
    state.0.player_moving = len > 0.0;
    state.0.player_running = len > 0.0 && running;
    state.0.player_sneaking = sneak;
    // The same as the game's movement flags, for `IsMoving`, `IsRunning`
    // and `IsSneaking` (`world::more_functions::movement`).
    {
        use world::more_functions::movement as m;
        let mut flags = 0;
        for (key, flag) in [
            (KeyCode::KeyW, m::FORWARD),
            (KeyCode::KeyS, m::BACK),
            (KeyCode::KeyA, m::LEFT),
            (KeyCode::KeyD, m::RIGHT),
        ] {
            let held = keys.pressed(key) || (auto && key == KeyCode::KeyW);
            if held && !locked {
                flags |= flag;
            }
        }
        if state.0.player_running {
            flags |= m::RUNNING;
        }
        if sneak {
            flags |= m::SNEAKING;
        }
        player.moving = world::animation::MoveFlags {
            forward: flags & m::FORWARD != 0,
            backward: flags & m::BACK != 0,
            left: flags & m::LEFT != 0,
            right: flags & m::RIGHT != 0,
            running: state.0.player_running,
            ..Default::default()
        };
        player.speed = if len > 0.0 { speed } else { 0.0 };
        world::more_functions::report(
            &mut state.0,
            world::dialogue::PLAYER_REF,
            world::more_functions::Seen {
                movement: flags,
                ..Default::default()
            },
        );
    }
    let shape = CharacterShape::PLAYER;
    // Jump: not over-encumbered; `fJumpHeightMin` × the player's scale (1).
    let jump =
        (keys.just_pressed(KeyCode::Space) && !locked && locomotion::may_jump(over_encumbered))
            .then(|| locomotion::jump_height(settings, 1.0, false));
    let dt = time.delta_secs();
    // The running speed `fSpeedPct` is measured against, kept once
    // (`0055d760`).
    if !player.reference_speed_set {
        physics::controller::set_reference_speed(world::animation::run_speed(
            order, &state.0, player_ref,
        ));
        player.reference_speed_set = true;
    }
    // One controller move a frame (`bhkCharacterController::Move`): on the
    // ground at the wanted velocity, in the air steered 0.3 of the way to
    // it; a jump asked for here leaves the ground the next frame.
    player.character.update_controlled(
        &collision.0,
        &shape,
        velocity,
        jump,
        locomotion::air_gain(locomotion::AIR_CONTROL),
        dt,
    );
    player.surface_serial = player.surface_serial.wrapping_add(1);
    // Outdoors, feet more than 30 under the land are put on it, the player's
    // too (`MobileObject::Move`, `0092f260` at `0093012a`; `world::ground`).
    let feet = player.character.feet;
    let land = nav
        .as_deref()
        .and_then(|n| n.land_height(state.0.player_world, feet));
    let lifted = world::ground::kept_above_land(feet[2], land);
    if lifted != feet[2] {
        player.character.feet[2] = lifted;
        player.character.ground = lifted;
    }
    if let Some(fell) = player.character.fell.take() {
        if !std::mem::take(&mut player.from_camera) {
            let hurt = world::combat::land(
                &game.0.order,
                &mut state.0,
                world::dialogue::PLAYER_REF,
                fell,
            );
            if hurt > 0.0 {
                println!("Fell {fell:.0} units: {hurt:.0} damage.");
            }
        }
    }
    // The eye: lower while sneaking, moving there over the blend.
    let blend_time = anim.map_or(0.2, |a| a.0.default_blend).max(1e-3);
    let target = if sneak { 1.0 } else { 0.0 };
    let step = dt / blend_time;
    player.sneak_blend = if player.sneak_blend < target {
        (player.sneak_blend + step).min(target)
    } else {
        (player.sneak_blend - step).max(target)
    };
    let heights = *heights.get_or_insert_with(|| camera_node_heights(&game.0));
    let [x, y, z] = player.character.feet;
    let eye = z + EYE_HEIGHT + eye_offset(heights, player.sneak_blend);
    transform.translation = Vec3::from(space::point([x, y, eye]));
}
/// The load door the crosshair is on, within reach (`crosshair`, the
/// game's view caster).
pub(crate) fn door_in_view<'a>(
    doors: &'a [DoorData],
    crosshair: &crate::crosshair::Crosshair,
) -> Option<&'a DoorData> {
    let r = crosshair.target()?;
    doors.iter().find(|d| d.reference == r.0)
}

/// The reference the crosshair is on within reach, for a door that opens
/// where it stands (not a load door; [`is_door`] tells): its leaf's
/// collision belongs to it (`preview::cell::CellScene::collider`) and
/// swings with it (`doors::update_doors`), so the pick meets the leaf
/// wherever it is.
pub(crate) fn opening_door_in_view(crosshair: &crate::crosshair::Crosshair) -> Option<esm::FormId> {
    crosshair.target()
}
/// Whether a reference is a door (its base a `DOOR`): other owners of the
/// collider's triangles (clutter bodies, `clutter`) aren't doors to open.
pub(crate) fn is_door(order: &esm::LoadOrder, reference: esm::FormId) -> bool {
    world::scripting::base_of(order, reference)
        .and_then(|b| order.get(b))
        .is_some_and(|b| b.entry.header.kind.as_bytes() == b"DOOR")
}

/// Load doors: the crosshair line names where the door in view leads, and
/// E goes through it, into the next interior or out to a worldspace.
#[allow(clippy::too_many_arguments)]
pub fn doors(
    // The Activate control is the player's (`PlayerCharacter::Update`,
    // `00943250`): the controls' buffer as the player's step read it
    // (`crate::poll`), though this runs after the poll for order.
    player_input: Res<crate::poll::PlayerInput>,
    game: Res<GameFiles>,
    doors: Res<Doors>,
    crosshair: Res<crate::crosshair::Crosshair>,
    mut pending: ResMut<PendingScene>,
    mut pending_exterior: ResMut<PendingExterior>,
    mut prompt: Query<&mut Text, With<Prompt>>,
    talk_target: Res<crate::dialogue::TalkTarget>,
    conversation: Res<crate::dialogue::Conversation>,
    activatable: Res<crate::scripts::Activatable>,
    mut activate: ResMut<crate::scripts::ActivateRequest>,
    mut state: ResMut<crate::dialogue::DialogueState>,
    scripts: Res<crate::scripts::Scripts>,
    mut sounds: ResMut<crate::sounds::SoundRequests>,
    (mut lockpicking, menus): (
        ResMut<crate::lockpick::Lockpicking>,
        Res<crate::menus::Menus>,
    ),
) {
    let keys = &player_input.keys;
    // A lock just picked: the player uses the door or container, as the
    // game has them do after the lockpicking menu (`00573170`): E again.
    let again = lockpicking.again.take();
    // Someone to talk to (or talking) takes the prompt and E.
    if talk_target.0.is_some() || conversation.0.is_some() {
        return;
    }
    let door = door_in_view(&doors.0, &crosshair);
    // A door that swings open where it stands, when no load door is in view.
    let swing = door
        .is_none()
        .then(|| opening_door_in_view(&crosshair))
        .flatten()
        .filter(|&r| is_door(&game.0.order, r));
    let line = match (door, swing, &activatable.0) {
        // Nothing while the lockpicking menu or one of the game's menus is up
        // (the roll-over is the HUD's, which their masks hide: ui::hud::parts_for_menu).
        _ if lockpicking.is_open() || menus.game_open => String::new(),
        (Some(d), _, _) => format!("E) Open door to {}", d.cell_label),
        (None, Some(d), _) => crate::doors::prompt(&game.0.order, &state.0, d).to_string(),
        // A scripted object (a machine, a switch) when no door is in view.
        (None, None, Some((_, name))) => format!("E) {name}"),
        (None, None, None) => String::new(),
    };
    for mut text in &mut prompt {
        if text.0 != line {
            text.0 = line.clone();
        }
    }
    if let Some(r) = again {
        // A container (or a door no longer in view) is used as E on it.
        let in_view = door.is_some_and(|d| d.reference == r.0) || swing == Some(r);
        if !in_view {
            activate.0 = Some(r);
            return;
        }
    }
    // The game's menus have E while they're open (game_menus).
    let pressed = !menus.game_open && (keys.just_pressed(KeyCode::KeyE) || again.is_some());
    let Some(door) = door else {
        if !pressed {
            return;
        }
        if let Some(reference) = swing {
            // Opening or closing runs the door's script, as the load
            // doors' do; then the game's door rules (`world::doors`): the
            // swing, its sound, and nothing while it's still swinging.
            if crate::scripts::door_opens(
                &game.0.order,
                &scripts.0,
                &mut state.0,
                reference,
                &mut lockpicking.request,
            ) {
                let player = world::dialogue::PLAYER_REF;
                match crate::doors::activate(
                    &game.0.order,
                    &mut state.0,
                    &mut sounds,
                    reference,
                    Some(player),
                ) {
                    world::doors::Activated::Opening => println!("The door opens."),
                    world::doors::Activated::Closing => println!("The door closes."),
                    world::doors::Activated::Busy => {}
                }
            }
        } else if let Some((reference, _)) = &activatable.0 {
            activate.0 = Some(*reference);
        }
        return;
    };
    if !pressed {
        return;
    }
    let reference = esm::FormId(door.reference);
    if !crate::scripts::door_opens(
        &game.0.order,
        &scripts.0,
        &mut state.0,
        reference,
        &mut lockpicking.request,
    ) {
        return;
    }
    // The door's opening sound (`SNAM` on its base).
    if let Some(base) = world::scripting::base_of(&game.0.order, reference) {
        if let (Some(open), _) = world::sound::door_sounds(&game.0.order, base) {
            sounds.0.push(open);
        }
    }
    let Some(cell) = door.cell else {
        return;
    };
    // Going through a door moves the player: fast travel a script turned
    // off comes back (unless it asked to keep it off).
    world::script_functions::player_moved(&mut state.0);
    if !door.interior && door.world.is_some() {
        match door_start(&game.0, door) {
            Ok(start) => pending_exterior.0 = Some(start),
            Err(e) => println!("Couldn't go out to {}: {e}", door.cell_label),
        }
        return;
    }
    println!("Loading {} ...", door.cell_label);
    let started = std::time::Instant::now();
    match game.0.load_cell_now(esm::FormId(cell), &state.0.disabled) {
        Ok(mut scene) => {
            let [x, y, z] = door.arrive;
            scene.start = cellview::Start {
                eye: [x, y, z + EYE_HEIGHT],
                heading: door.arrive_heading,
                via: "the door",
            };
            for note in &scene.notes {
                println!("  {note}");
            }
            println!(
                "Loaded {} in {:.1} s.",
                scene.cell,
                started.elapsed().as_secs_f32()
            );
            pending.0 = Some(scene);
        }
        Err(e) => println!("Couldn't load {}: {e}", door.cell_label),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `0093e860`'s skip table by `GetAnimAction` + 1: drawing, putting
    /// away and reloading keep the sneak toggle off; attacking doesn't.
    #[test]
    fn the_sneak_toggle_waits_for_drawing_and_reloading() {
        assert!(may_toggle_sneak(false, false, None));
        assert!(!may_toggle_sneak(false, false, Some(0)));
        assert!(!may_toggle_sneak(false, false, Some(1)));
        assert!(may_toggle_sneak(false, false, Some(2)));
        assert!(!may_toggle_sneak(false, false, Some(8)));
        assert!(may_toggle_sneak(false, false, Some(9)));
        assert!(!may_toggle_sneak(false, false, Some(10)));
        assert!(may_toggle_sneak(false, false, Some(17)));
        // Dead, or in a chair: no.
        assert!(!may_toggle_sneak(true, false, None));
        assert!(!may_toggle_sneak(false, true, None));
    }

    /// The first-person camera node drops from 118 (`mtidle.kf`) to 78
    /// (`sneakmtidle.kf`) in the game's files: the eye goes 40 lower, part
    /// way while the blend runs.
    #[test]
    fn sneaking_lowers_the_eye_by_the_camera_nodes_drop() {
        let heights = Some((118.0, 78.0));
        assert_eq!(eye_offset(heights, 0.0), 0.0);
        assert_eq!(eye_offset(heights, 1.0), -40.0);
        assert_eq!(eye_offset(heights, 0.5), -20.0);
        assert_eq!(eye_offset(None, 1.0), 0.0);
    }

    #[test]
    fn game_and_bevy_space_round_trip() {
        let p = [100.0, -200.0, 300.0];
        let back = game_point(Vec3::from(space::point(p)));
        assert!(back.iter().zip(p).all(|(a, b)| (a - b).abs() < 1e-3));
    }
}
