use bevy::{prelude::*, utils::HashMap};
use crate::player::Player;
use crate::sprites::SpriteComponent;

#[derive(Component)]
pub struct Collider {
    pub radius: f32,
    pub colliding_entities: Vec<Entity>,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            colliding_entities: vec![],
        }
    }
}

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collision_detection);
    }
}

fn collision_detection(
    mut commands: Commands,
    mut query: Query<(Entity, &SpriteComponent, &mut Collider)>,
    mut player_query: Query<(Entity, &mut Player, &mut Collider), Without<SpriteComponent>>
) {
    let mut colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

    // First phase: Detect collisions
    for (entity_a, transform_a, collider_a) in query.iter() {

        // Detect player collision (WORKAROUND since player is not a SpriteComponent and time is running out)
        for (player_entity, mut player, collider_player) in player_query.iter_mut() {
            let distance = transform_a.position.distance(Vec3::new(player.x, player.y, player.z));

            if distance < collider_a.radius + collider_player.radius {

                println!("Hit player");

                player.update_health(-10);
                commands.entity(entity_a).despawn_recursive();
            }
        }

        // Detect Sprite on sprite collision
        for (entity_b, transform_b, collider_b) in query.iter() {
            if entity_a != entity_b {
                let distance = transform_a.position
                    .distance(transform_b.position);
                if distance < collider_a.radius + collider_b.radius {
                    println!("Collision");
                    colliding_entities
                        .entry(entity_a)
                        .or_insert_with(Vec::new)
                        .push(entity_b);
                }
            }
        }
    }

    // Second phase: Update colliders.
    // TODO: Perform some specific action depending on collision type (sound, projectile etc...)
    for (entity, _, mut collider) in query.iter_mut() {
        collider.colliding_entities.clear();

        if let Some(collisions) = colliding_entities.get(&entity) {
            collider
                .colliding_entities
                .extend(collisions.iter().copied());
        }
    }
}
