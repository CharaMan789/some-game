use benimator::FrameRate;
use bevy::{prelude::*, sprite::collide_aabb::collide, render::render_resource::Texture, /*text::Text2dBounds*/};

static mut BOOL: bool = true;
static mut COLBOOL: bool = false;
#[derive(Component, Deref)]
struct Animation(benimator::Animation);

#[derive(Component, Deref)]
struct Box1Animation(benimator::Animation);

#[derive(Component, Deref)]
struct Box2Animation(benimator::Animation);

#[derive(Component, Deref)]
struct Box3Animation(benimator::Animation);


#[derive(Component, Deref, Default, DerefMut)]
struct AnimationState(benimator::State);

#[derive(Component, Deref, Default, DerefMut)]
struct Box1AnimationState(benimator::State);

#[derive(Component, Deref, Default, DerefMut)]
struct Box2AnimationState(benimator::State);

#[derive(Component, Deref, Default, DerefMut)]
struct Box3AnimationState(benimator::State);

static mut MOVEMENT_STATE: MovementCheck = MovementCheck::Stationary;

#[derive(PartialEq)]
pub enum MovementCheck {
    MovingLeft,
    MovingRight,
    MovingUp,
    MovingDown,
    Stationary,
}
#[derive(Component)]
pub struct AnimateTranslation;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Box1;

#[derive(Component)]
pub struct Box2;

#[derive(Component)]
pub struct Box3;

#[derive(Component)]
pub struct Checkbox;


const Y: f32 = 60.0;


fn main() {
    App::new()
        //.add_system(spawn_boxes)
        //.add_system(animate_translation)
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_player.label("Player Spawn"))
        .add_system(player_movement)
        .add_startup_system(setup)
        .add_system(collision_check_2)
        .add_system(collision_check_3)
        .add_system(collision_check_1)
        .add_system(animate_player)
        .add_startup_system(spawn_boxes.after("Player Spawn"))
        .run();
}

fn setup(mut commands: Commands, audio: Res<Audio>, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    unsafe{
        if BOOL{
            audio.play(asset_server.load("sounds/audio2.ogg"));
        }else if COLBOOL{
            println!("No Audio");
        }
    }


    // let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    // let text_style = TextStyle{
    //     font,
    //     font_size: 60.0,
    //     color: Color::WHITE,
    // };
    // let text_allignment = TextAlignment::CENTER;
    // commands.spawn((Text2dBundle{
    //                 text: Text::from_section("translation", text_style.clone()).with_alignment(text_alignment),
    //                 ..default()
    //                 },
    //                 AnimateTranslation,
    // ));
}

// fn animate_translation(
//     time: Res<Time>,
//     mut query: Query<&mut Transform, (With<Text>, With<AnimateTranslation>)>,
// ){
//     for mut transform in &mut query{
//         transform.translation.x = 100.0 * time.seconds_since_startup().sin() as f32 - 400.0;
//         transform.translation.y = 100.0 * time.seconds_since_startup().cos() as f32;
//     }
// }

// fn spawn_boxes(
//     mut commands: Commands,
//     assets: Res<AssetServer>,
//     mut textures: ResMut<Assets<TextureAtlas>>,
//     //assets: Res<AssetServer>,
// ) {

//     let animation = Animation(benimator::Animation::from_indices(
//         0..=4,
//         FrameRate::from_fps(5.0),
//     ));
//     let animation1 = Animation(benimator::Animation::from_indices(
//         0..=4,
//         FrameRate::from_fps(5.0),
//     ));
//     let animation2 = Animation(benimator::Animation::from_indices(
//         0..=4,
//         FrameRate::from_fps(5.0),
//     ));
//     commands
//         .spawn_bundle(SpriteSheetBundle {
//             texture_atlas: textures.add(TextureAtlas::from_grid(
//                 assets.load("case_1.png"),
//                 Vec2::new(32.0, 32.0),
//                 5,
//                 1,
//             )),
//             transform: Transform {
//                 scale: Vec3::splat(5.0),
//                 translation: Vec3::new(-10.0, 4.0, 0.5),
//                 ..default()
//             },
//             ..default()
//         })
//         .insert(Box1)
//         .insert( animation)
//         .insert(Box1AnimationState::default());

//     commands
//         .spawn_bundle(SpriteSheetBundle {
//            texture_atlas: textures.add(TextureAtlas::from_grid(
//             assets.load("case_2.png"),
//             Vec2::new(32.0, 32.0),
//             5,
//             1,
//            )),
//             transform: Transform {
//                 scale: Vec3::splat(5.0),
//                 translation: Vec3::new(150.0 - 10.0, 4.0, 0.5),
//                 ..default()
//             },
//             ..default()
//         })
//         .insert(animation1)
//         .insert(Box2AnimationState::default())
//         .insert(Box2);

//     commands
//         .spawn_bundle(SpriteSheetBundle {
//             texture_atlas: textures.add(TextureAtlas::from_grid(
//                 assets.load("case_2.png"),
//                 Vec2::new(32.0, 32.0),
//                 5,
//                 1,
//             )), 
//             transform: Transform {
//                 scale: Vec3::new(5.0, 5.0,5.0),
//                 translation: Vec3::new(300.0 - 10.0, 4.0, 0.5),
//                 ..default()
//             },
//             ..default()
//         })
//         .insert(animation2)
//         .insert(Box3AnimationState::default())
//         .insert(Box3);
// }

fn spawn_player(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
) {
    let _image = "pngg.png";
    let animation = Animation(benimator::Animation::from_indices(
        0..=2,
        FrameRate::from_fps(6.0),
    ));

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.add(TextureAtlas::from_grid(
                assets.load("player_3.png"),
                Vec2::new(32.0, 32.0),
                3,
                1,
            )),
            transform: Transform {
                translation: Vec3::new(0.0, -100.0, 2.0),
                scale: Vec3::splat(4.0),
                ..default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(animation)
        .insert(AnimationState::default());

    /*commands
    .spawn_bundle(SpriteBundle {
        texture: assets.load(image),
        transform: Transform {
            scale: Vec3::new(0.2, 0.2, 1.0),
            translation: Vec3::new(0.0, -100.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Player);*/
}

fn spawn_boxes(
    mut commands: Commands, 
    assets: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
){
    let animation = Box1Animation(benimator::Animation::from_indices(
        0..=4,
        FrameRate::from_fps(3.0 ),
    ));
    let animation2 = Box2Animation(benimator::Animation::from_indices(
        0..=4,
        FrameRate::from_fps(3.0),
    ));
    let animation3 = Box3Animation(benimator::Animation::from_indices(
        0..=4,
        FrameRate::from_fps(3.0),
    ));
    
    commands.spawn_bundle(SpriteSheetBundle{
        texture_atlas: textures.add(TextureAtlas::from_grid(
            assets.load("case_1.png"),
            Vec2::new(32.0, 32.0),
            5,
            1,
        )),
        transform: Transform{
            scale: Vec3::splat(5.0),
            translation: Vec3::new(-10.0,4.0, 0.0),
            ..default()
        },
        ..default()
    })
        .insert(Box1)
        .insert(animation)
        .insert(Box1AnimationState::default());

    commands.spawn_bundle(SpriteSheetBundle{
        texture_atlas: textures.add(TextureAtlas::from_grid(
            assets.load("case_2.png"),
            Vec2::new(32.0,32.0),
            5,
            1,
        )),
        transform: Transform{
            scale: Vec3::splat(5.0),
            translation: Vec3::new(150.0 - 10.0, 4.0, 0.0),
            ..default()
        },
        ..default()
    })
        .insert(Box2)
        .insert(animation2)
        .insert(Box2AnimationState::default());

    commands.spawn_bundle(SpriteSheetBundle{
        texture_atlas: textures.add(TextureAtlas::from_grid(
            assets.load("case_2.png"),
            Vec2::new(32.0, 32.0),
            5,
            1,
        )),
        transform: Transform{
            scale: Vec3::splat(5.0),
            translation: Vec3::new(300.0 - 10.0, 4.0, 0.0),
            ..default()
        },
        ..default()
    })
        .insert(Box3)
        .insert(animation3)
        .insert(Box3AnimationState::default());

}

fn animate_player(
    time: Res<Time>,
    mut query: Query<(&mut AnimationState, &mut TextureAtlasSprite, &Animation)>,
) {
    for (mut player, mut texture, animation) in query.iter_mut() {
        unsafe{
            if MOVEMENT_STATE == MovementCheck::MovingUp
                || MOVEMENT_STATE == MovementCheck::MovingDown
                || MOVEMENT_STATE == MovementCheck::MovingLeft
                || MOVEMENT_STATE == MovementCheck::MovingRight
            {
                player.update(animation, time.delta());
                texture.index = player.frame_index();
            } else if MOVEMENT_STATE == MovementCheck::Stationary {
                println!("Stationary");
            }
        }
    }
}

fn player_movement(mut player_query: Query<&mut Transform, With<Player>>, kb: Res<Input<KeyCode>>) {
    for mut transform in player_query.iter_mut() {
        if kb.pressed(KeyCode::Up) {
            transform.translation.y += 2.0;
            unsafe {
                MOVEMENT_STATE = MovementCheck::MovingUp;
            }
        } else if kb.pressed(KeyCode::Down) {
            transform.translation.y -= 2.0;
            unsafe {
                MOVEMENT_STATE = MovementCheck::MovingDown;
            }
        } else if kb.pressed(KeyCode::Left) {
            transform.translation.x -= 2.0;
            unsafe {
                MOVEMENT_STATE = MovementCheck::MovingLeft;
            }
        } else if kb.pressed(KeyCode::Right) {
            transform.translation.x += 2.0;
            unsafe {
                MOVEMENT_STATE = MovementCheck::MovingRight;
            }
        } else {
            unsafe {
                MOVEMENT_STATE = MovementCheck::Stationary;
            }
        }
    }
}
fn collision_check_1(
    player_query: Query<&Transform, (With<Player>, (Without<Box1>, Without<Box2>, Without<Box3>))>,
    box1_query: Query<&Transform, (With<Box1>, (Without<Player>, Without<Box3>, Without<Box2>))>,
    //kb: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Box1AnimationState, &mut TextureAtlasSprite, &Box1Animation)>,
    //audio: Res<Audio>,
    //asset_server: Res<AssetServer>,
) -> (){
    let mut num:i32 = 0;
    for ptf in player_query.iter() {
        for b1 in box1_query.iter() {
            let collision = collide(
                ptf.translation,
                Vec2::new(30.0, 15.0),
                b1.translation,
                Vec2::splat(Y),
            );
            if collision.is_some() {
                for (mut box1, mut texture, animation) in query.iter_mut() {
                    num += 1;
                    box1.update(animation, time.delta());
                    texture.index = box1.frame_index();
                    if num < 5 {
                        break;
                    }
                }
                unsafe{
                    COLBOOL = true;
                    BOOL = false;
                }
            }
        }
    }

}
fn collision_check_2(
    player_query: Query<&Transform, (With<Player>, (Without<Box1>, Without<Box2>, Without<Box3>))>,
    box2_query: Query<&Transform, (With<Box2>, (Without<Player>, Without<Box1>, Without<Box3>))>,
    //kb: Res<Input<KeyCode>>,
    time: Res<Time>,
    //mut commands: Commands,
    mut query: Query<(&mut Box2AnimationState, &mut TextureAtlasSprite, &Box2Animation)>,
) {
    for ptf in player_query.iter() {
        for b2 in box2_query.iter() {
            let collision = collide(
                ptf.translation,
                Vec2::new(30.0, 30.0),
                b2.translation,
                Vec2::splat(Y),
            );
            if collision.is_some(){
                println!("Collision for Box2");
                for (mut box2, mut texture, animation) in query.iter_mut(){
                    box2.update(animation, time.delta());
                    texture.index = box2.frame_index();
                }
                unsafe{
                    COLBOOL = true;
                    BOOL = false;
                }
            }
        }
    }
}

fn collision_check_3(
    player_query: Query<&Transform, (With<Player>, (Without<Box1>, Without<Box2>, Without<Box3>))>,
    box3_query: Query<&Transform, (With<Box3>, (Without<Player>, Without<Box2>, Without<Box1>))>,
    //kb: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Box3AnimationState, &mut TextureAtlasSprite, &Box3Animation)>,
    //mut commands: Commands,
) {
    for ptf in player_query.iter() {
        for b3 in box3_query.iter() {
            let collision = collide(
                ptf.translation,
                Vec2::new(30.0, 30.0),
                b3.translation,
                Vec2::splat(Y),
            );
            if collision.is_some(){
                println!("Collision for Box3");
                for (mut box3, mut texture, animation) in query.iter_mut(){
                    box3.update(animation, time.delta());
                    texture.index = box3.frame_index();
                }
                unsafe{
                    COLBOOL = true;
                    BOOL = false;
                }
            }
        }
    }
}

