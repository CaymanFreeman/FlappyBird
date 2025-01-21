use super::*;
use bevy::audio::Volume;
use std::time::Duration;

#[derive(Resource)]
pub(crate) struct Score(pub(crate) u32);

#[derive(Component)]
pub(crate) struct ScoreDisplay;

#[derive(Component)]
pub(crate) struct ScoreTimer(Timer);

pub(crate) fn spawn_score_display(
    mut commands: Commands,
    mut score: ResMut<Score>,
    fonts: Res<FontAssets>,
) {
    score.0 = 0;
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(SCORE_DISPLAY_TOP_MARGIN_PX),
                justify_self: JustifySelf::Center,
                ..Default::default()
            },
            ScoreDisplay,
        ))
        .with_children(|parent| {
            spawn_outlined_text(
                parent,
                &score.0.to_string(),
                fonts.title_font.clone(),
                SCORE_DISPLAY_FONT_SIZE_PX,
                SCORE_DISPLAY_COLOR,
                SCORE_DISPLAY_OUTLINE_COLOR,
                SCORE_DISPLAY_OUTLINE_WIDTH_PX,
            );
        });
}

pub(crate) fn despawn_score_display(
    mut commands: Commands,
    score_display_query: Query<Entity, With<ScoreDisplay>>,
) {
    if let Ok(score_display) = score_display_query.get_single() {
        commands.entity(score_display).despawn_recursive();
    }
}

impl ScoreTimer {
    pub(crate) fn new() -> ScoreTimer {
        ScoreTimer(Timer::new(
            Duration::from_secs_f32(SCORE_TIMER_INTERVAL),
            TimerMode::Repeating,
        ))
    }
}

pub(crate) fn handle_score_timer(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut timer_query: Query<&mut ScoreTimer>,
    mut score_display_query: Query<&Children, With<ScoreDisplay>>,
    mut text_query: Query<&mut Text>,
    time: Res<Time>,
    audio: Res<AudioAssets>,
) {
    if let Ok(mut delay_timer) = timer_query.get_single_mut() {
        if delay_timer.0.tick(time.delta()).just_finished() {
            score.0 += 1;
            commands.spawn((
                AudioPlayer::new(audio.score_sound.clone()),
                PlaybackSettings {
                    volume: Volume::new(SCORE_SOUND_VOLUME),
                    ..Default::default()
                },
            ));
            if let Ok(children) = score_display_query.get_single_mut() {
                for &child in children.iter() {
                    if let Ok(mut text) = text_query.get_mut(child) {
                        text.0 = score.0.to_string();
                    }
                }
            }
        }
    }
}
