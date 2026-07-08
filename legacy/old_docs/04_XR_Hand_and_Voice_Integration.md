# 4. XR Hand and Voice Integration

To fully realize the holistic pedagogical vision, the engine heavily utilizes physical tracking and auditory synthesis.

## The Hands: Kinetic Gesture System (`hand_tracking.rs`)
The player interacts with the game in XR using their physical hands.
- **ASL Recognition**: Simple heuristics calculate the distance between wrist and index tip to recognize base signs (e.g., 'A' vs 'L').
- **Pinch-and-Drag**: Players "grab" floating concepts in the air. 
- **Grammar Assembly**: To summon a Grammar Golem, the player must pinch three concepts in sequential order (Subject ➔ Verb ➔ Object).
- **Kinetic Arousal**: The delta of the wrist's movement speed across frames is calculated as `gesture_intensity`. Fast, aggressive physical movements spawn pets with inherently higher `Arousal` (Speed) in combat.

## The Mouth: TTS & Rhetoric (`chat.rs`)
- **Social Combat**: The `Rhetoric Robot` class relies on "Social Combat". The engine connects to a local Kokoro TTS sidecar API (`af_bella` voice).
- **Dynamic Arguments**: When a Rhetoric Robot attacks, it generates a logical string based on semantic relations (e.g., "I present a paradox! 'Cold' is the complete antithesis of 'Hot'!") and speaks it aloud in the 3D space, debuffing the enemy.
- **Taming**: The player can also interact with their own pets using physical inputs (Pinch/Pet/Feed), triggering dynamic auditory responses from the pets.
