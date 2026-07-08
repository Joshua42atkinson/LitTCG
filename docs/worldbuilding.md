# LitTCG Worldbuilding: Syllable Springs

## The Big Idea

In the beginning, there was only **The Static** — a fuzzy nothingness where meaning had not yet been invented. Then the first children spoke. Their words fell to the ground like seeds, and from those seeds grew **Syllable Springs**, a city made of language and living creatures.

Every word ever learned is a pet waiting to be born. Every pet is a story. Every story is a spell.

## The World

Syllable Springs is a city built on a giant glowing machine called **The Slime Synthesizer**. It hums day and night, turning raw syllables into creatures called **Etymons** (the pets). The city is divided into four districts, each tuned to a different kind of word-energy.

### Districts

| District | Element | Vibe | NPC Guardians | What kids learn here |
|----------|---------|------|---------------|----------------------|
| **Brainy Borough** | Earth / Fire | Orderly, libraries, labs, permits | Ignis, Ozymandias, Barnaby | Structure, roots, rules, science |
| **Heartwood Grove** | Water / Earth | Cozy, gardens, feelings, community | Yorick, Vlad | Emotion, care, relationships |
| **Action Alley** | Fire / Air | Bright, kinetic, heroic, dramatic | Kael, Zafir, Gribble | Verbs, action, courage, change |
| **Whisper Winds** | Air / Shadow | Mysterious, creative, poetic | Nyx, Martha, Pygmalion | Imagination, transformation, art |

### The Static and The Typos

The Static is the enemy of meaning. It tries to unmake words by replacing them with noise, blur, and confusion. The Typos are The Static's little monsters: glitchy creatures that look like smudged letters and broken spellings. They are not evil, exactly — they are just *wrong*. A child defeats them by spelling the right word.

## The Pets (Etymons)

A pet is born when a child successfully spells a word. Its shape comes from:

- **The word's meaning** (what it represents)
- **Its element** (the district it belongs to)
- **Its role** (how it fights)
- **Its summon class** (what kind of creature it is)
- **Its lore** (the little story that makes it unique)

### Summon Classes (Archetypes)

| Class | Description | What it looks like | How it moves |
|-------|-------------|-------------------|--------------|
| **SemanticSlime** | A blob of living meaning | Translucent, shifting colors, the word floats inside it | Jiggles, oozes, splits and re-forms |
| **GrammarGolem** | A construct made of language rules | Blocky stone/metal, runes carved into its surface | Stomps, clanks, very literal |
| **RhetoricRobot** | A persuasive mechanical speaker | Sleek panels, speaker vents, glowing eyes | Smooth servo motions, gestures dramatically |

### Roles (Combat Jobs)

- **Striker**: Fast, hits hard.
- **Bruiser**: Tough, hits harder but slower.
- **Tank**: Protects the team.
- **Healer**: Repairs other pets.
- **Support**: Buffs, shields, helps.
- **Caster**: Uses words as magic.
- **Assassin**: Sneaky, precise.

## Lore System

Every pet has a tiny lore entry that answers five questions:

1. **What is it called?** (title)
2. **What does it look like and where did it come from?** (description)
3. **Where does it live?** (habitat)
4. **What does it do?** (behavior)
5. **What is a weird fact about it?** (fun_fact)
6. **Which old word-root still lives inside it?** (etymology_hook)
7. **Which NPC watches over it?** (npc_guardian)

This lore is generated automatically by the AI pipeline, but it is also used to make the portrait prompts richer. The image of the pet is not just "a fire dog" — it is "a fire dog that lives in the Synthesizer's warm stone ledges and gets excited when someone says 'ember' near it."

## Example Pet: Thunder

```
Title: Thunder of the Air
Description: A bruiser SemanticSlime born from the word 'thunder', carrying the air element of Whisper Winds.
Habitat: High shelves where loose vowels drift like dust (Whisper Winds).
Behavior: Oozes around syllables and tries to absorb the meaning of 'thunder'.
Fun fact: It gets excited whenever someone says 'boom' or 'rumble' near it.
Etymology hook: Scholars say its ancient root, 'Thor', still hums inside its syllables.
Guardian: Nyx the Rebel.
```

Thunder would look like a stormy, jiggling slime creature with lightning flickering inside its translucent body. Its eyes are bright white. It floats slightly above the ground because it is an Air pet. When it is happy, it makes a low rumble. When it is angry, it crackles.

## Example Pet: Joy

```
Title: The Little Joy
Description: A support SemanticSlime born from the word 'joy', carrying the light element of Action Alley.
Habitat: Sunlit balconies above the Slime Synthesizer (Action Alley).
Behavior: Oozes around syllables and tries to absorb the meaning of 'joy'.
Fun fact: It gets excited whenever someone says 'delight' or 'glee' near it.
Etymology hook: The letters of 'joy' wiggle when no one is looking.
Guardian: Kael the Hero.
```

Joy would look like a small, golden, sparkling slime. It leaves a trail of tiny light motes. It bounces instead of walking. When it is near the player, the screen gets a little brighter.

## The Educational Hook

The world teaches vocabulary through *relationship*:

- A child meets **Barnaby**, who is scared of shadows and needs the word **brave**.
- The child spells B-R-A-V-E. A brave little pet appears.
- The brave pet helps Barnaby cross a dark bridge.
- Later, the child meets **Martha**, who is sad. The child needs the word **comfort**.
- The comfort pet wraps around Martha's house like a warm blanket.

Every word is not just a flashcard — it is a friend who solves a problem in a living world.

## The Day/Night Cycle

Syllable Springs has four times of day, and the NPCs change their dialogue:

- **Dawn**: Hopeful, creative, preparing for the day.
- **Day**: Busy, quest-giving, problem-solving.
- **Dusk**: Worried, enemies appear, dramatic.
- **Night**: Quiet, mysterious, dangerous.

Some pets only appear at night. Some quests only make sense at dawn. The time of day is part of the vocabulary lesson (dawn, dusk, midnight, noon).

## FACES and Pet Emotion

Every pet has a visible emotional state using the FACES protocol:

- **Aura**: color of the pet's glow
- **Container**: body shape tension
- **Focus**: where the pet is looking
- **Action**: what the pet is doing
- **Emotion**: the overall feeling

A scared pet shrinks and turns blue. An excited pet bounces and turns gold. An angry pet sparks and turns red. Children learn to read emotions by watching their pets, which is a core social-emotional learning goal.

## Future Lore Directions

- **Boss creatures**: Misspelled Mega-Beasts made from corrupted words.
- **Legendary words**: Rare, mythic vocabulary that unlocks city-wide events.
- **Seasonal events**: The city changes with the school year.
- **Player-created words**: Eventually, kids can invent their own words and see what pet appears.

## Pipeline Lore Generation

The `lit-asset-forge` crate generates this lore automatically:

```bash
# Generate lore for one word
cargo run --manifest-path crates/lit-asset-forge/Cargo.toml -- lore thunder --element Air --role Bruiser --summon-class SemanticSlime

# Generate a portrait with that lore baked into the prompt
cargo run --manifest-path crates/lit-asset-forge/Cargo.toml -- portrait thunder --element Air --role Bruiser --summon-class SemanticSlime
```

The lore is saved in `asset_manifest.json` alongside the portrait, so the game can display it in the pet collection, the deck builder, and the summon cinematic.
