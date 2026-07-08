# **Semantic Slime: 2D Prototype Developer Guide**

This document outlines the technical architecture for the Semantic Slime 2D prototype using Bevy 0.18.1. It focuses on the implementation of the coordinate system, touch-based interaction models, psycholinguistic rendering systems, and the turn-based state machine.

# **1\. App Scaffold & 2D Grid Setup**

The application utilizes a fixed 16x16 coordinate grid to manage entity positioning and spatial queries. This grid is stored as a global resource to decouple the logical game space from the physical pixel-based rendering space.use library::prelude::\*;

\#\[derive(Resource)\]

pub struct GameGrid {

    pub width: u32,

    pub height: u32,

    pub cell\_size: f32,

}

impl Default for GameGrid {

    fn default() \-\> Self {

        Self {

            width: 16,

            height: 16,

            cell\_size: 40.0,

        }

    }

}

pub fn setup\_scaffold(mut commands: Commands) {

    commands.spawn(Camera2dBundle::default());

    commands.insert\_resource(GameGrid::default());

}

fn main() {

    App::new()

        .add\_plugins(DefaultPlugins.set(WindowPlugin {

            primary\_window: Some(Window {

                title: "Semantic Slime 2D".into(),

                ..default()

            }),

            ..default()

        }))

        .insert\_resource(GameGrid::default())

        .add\_systems(Startup, setup\_scaffold)

        .run();

}

# **2\. Touch-Gesture & Spelling Parser (input.rs)**

The input system tracks multi-touch interactions to distinguish between single-finger swipe traces (for spelling) and multi-touch dragging (for moving word cards).\#\[derive(Resource, Default)\]

pub struct ActiveGestures {

    pub traces: HashMap\<u64, Vec\<Vec2\>\>,

}

\#\[derive(Component)\]

pub struct DraggableCard {

    pub touch\_id: Option\<u64\>,

}

pub fn handle\_touch\_input(

    mut touch\_evr: EventReader\<TouchInput\>,

    mut gestures: ResMut\<ActiveGestures\>,

    mut cards: Query\<(\&mut Transform, \&mut DraggableCard)\>,

) {

    for ev in touch\_evr.read() {

        match ev.phase {

            TouchPhase::Started \=\> {

                gestures.traces.insert(ev.id, vec\!\[ev.position\]);

            }

            TouchPhase::Moved \=\> {

                if let Some(trace) \= gestures.traces.get\_mut(\&ev.id) {

                    trace.push(ev.position);

                }

                

                for (mut transform, mut card) in cards.iter\_mut() {

                    if card.touch\_id \== Some(ev.id) {

                        transform.translation.x \= ev.position.x;

                        transform.translation.y \= ev.position.y;

                    }

                }

            }

            TouchPhase::Ended | TouchPhase::Canceled \=\> {

                gestures.traces.remove(\&ev.id);

            }

        }

    }

}

# **3\. Procedural 2D Slime Tinting and Scaling (render.rs)**

Slime avatars are procedurally modified based on the semantic attributes of their associated words. The system maps Concreteness to Scale, Valence to Color (Tint), and Arousal to the animation pulse frequency.t  
\#\[derive(Component)\]  
pub struct PetAvatar2D {  
pub valence: f32,      // Range \-1.0 to 1.0 (Mapped to Color)  
pub arousal: f32,      // Range 0.0 to 1.0 (Mapped to Animation Speed)  
pub concreteness: f32, // Range 0.0 to 1.0 (Mapped to Scale)  
}

pub fn update\_slime\_visuals(  
time: Res,  
mut query: Query\<(\&PetAvatar2D, \&mut Sprite, \&mut Transform)\>,  
) {  
for (avatar, mut sprite, mut transform) in query.iter\_mut() {  
// Concreteness \-\> Scale  
let scale\_factor \= 0.5 \+ (avatar.concreteness \* 1.5);  
transform.scale \= Vec3::splat(scale\_factor);    // Valence \-\> Color Tint (Subdued Blue for negative, Warm Gold for positive)

    let tint \= if avatar.valence \>= 0.0 {

        Color::rgb(1.0, 0.9, 0.5 \* avatar.valence)

    } else {

        Color::rgb(0.5, 0.6, 1.0 \+ avatar.valence)

    };

    sprite.color \= tint;

    // Arousal \-\> Animation Speed (Pulse)

    let pulse \= (time.elapsed\_seconds() \* (1.0 \+ avatar.arousal \* 5.0)).sin() \* 0.1;

    transform.scale \+= Vec3::splat(pulse);

}

}

\#\# 4\. Turn-Based Card Deck State Flow (deck.rs & battle.rs)

The game logic is governed by a \`GameState\` machine. Card transitions (Draw, Shuffle, Play) are handled via ECS events and state-specific systems.

| State | Responsibility | Transition Criteria |

| :--- | :--- | :--- |

| \*\*Draw\*\* | Instantiate card entities from deck resource | On completion of draw animation |

| \*\*Play\*\* | Enable touch systems for interaction | On card collision with slime target |

| \*\*Shuffle\*\* | Return unused cards to deck and randomize | On turn end signal |

\`\`\`rust

\#\[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)\]

pub enum BattleState {

    \#\[default\]

    Draw,

    Play,

    Shuffle,

}

\#\[derive(Component)\]

pub struct Card {

    pub word: String,

    pub cost: u32,

}

\#\[derive(Resource)\]

pub struct Deck {

    pub cards: Vec\<String\>,

}

pub fn draw\_cards\_system(

    mut commands: Commands,

    mut deck: ResMut\<Deck\>,

    mut state: ResMut\<NextState\<BattleState\>\>,

) {

    for \_ in 0..5 {

        if let Some(word) \= deck.cards.pop() {

            commands.spawn((

                Card { word, cost: 1 },

                DraggableCard { touch\_id: None },

                SpriteBundle::default(),

            ));

        }

    }

    state.set(BattleState::Play);

}  
