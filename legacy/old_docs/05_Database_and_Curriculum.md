# 5. Database and Curriculum

Communication Class is designed for scalability and institutional integration (B2B). The Rust binary does not contain hardcoded educational content; it is entirely data-driven via `database.rs`.

## Embedded JSON Engine
The `assets/` directory contains massive datasets that are deserialized at runtime:
- `word_database.json` (1.4MB): Contains psycholinguistic stats (Concreteness, Valence, Arousal, Dominance) and Grade Levels for thousands of words.
- `synonym_database.json` (2.1MB): Maps words to their synonyms, antonyms, and distractors for the combat engine.
- `etymology_db.json`: Maps Greek and Latin roots/suffixes to elemental typings (e.g., "pyr" ➔ Fire) and combat roles (e.g., "-tion" ➔ Caster).
- `quest_data.json`: Contains the "Meme Templates" (Mad-Libs) for NPCs.
- `lore_db.json`: NPC dialogue and schedules (Dawn, Day, Dusk, Night).

## Institutional Analytics
The database schema explicitly includes:
- **`GradeLevel`**: Automatically scales the difficulty of quests and typos based on the player's XP.
- **`CommonCoreStandard`**: Maps words directly to educational standards (e.g., "L.4.5" for 4th-grade language arts). This allows the engine to output telemetry data that integrates directly with school district grading systems.
