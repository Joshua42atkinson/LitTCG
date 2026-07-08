# LitTCG Combat System Brainstorm

## Why the current combat feels "limp"

After reviewing `src/core/battle.rs` and the GDD combat sections, the current loop is essentially a **single hidden-stat comparison**:

1. A random enemy word appears.
2. The child plays one of their own pets.
3. The game compares two numbers (semantic distance, shared roots, or Logos/Pathos/Ethos).
4. Damage happens. Enemy does not really fight back. Loop repeats until one HP bar hits zero.

Problems that kill the drama hook:

- **No choices.** The "best" move is whichever word has the highest semantic distance. The player does not need to think about *meaning*.
- **No enemy personality.** The Typo is a static HP bar. It never surprises, counter-attacks, or reveals a weakness.
- **No fail-forward feedback.** Ineffective attacks just deal half damage and deal 20 HP to the player. The child does not learn *why* the word was ineffective.
- **No narrative stakes.** Why does this fight matter? Is it a correction, a rescue, a duel?
- **No class fantasy.** Slime / Golem / Robot have different formulas but they do not feel like different playstyles.
- **No tempo.** There is no speed, turn order, buffs, debuffs, or resource management. Every turn is identical.
- **No mastery journey.** The same combat works the same at grade 1 and grade 12. There is no escalation.

## Design goals for a fun combat system

1. **Isomorphism:** The mechanic must *be* the language skill. Spelling = summoning. Synonyms = attacks. Antonyms = counters. Etymology = critical hits. Grammar = shields/combos. Rhetoric = persuasion/mind control.
2. **Meaningful choice:** Every turn the child should ask, *"Which word is the right tool for this enemy's weakness?"*
3. **Visible feedback:** The pet and enemy should visibly change based on the words used.
4. **Drama curve:** Battles should have setup, rising tension, a decisive moment, and a satisfying payoff.
5. **Educational stealth:** The child learns vocabulary, grammar, and rhetoric without realizing it.

---

## Big list of combat improvement ideas

### 1. Give the enemy a face and a behavior

- **The Typo is a *corrupted version* of a real word.** When the battle starts, the enemy *is* the word "fire" but it is misspelled, twisted, or wearing the wrong suffix ("firent", "firy", "fiire"). The child must "correct" it back to the true word.
- **Enemy has a "Corruption Type" that determines its attack pattern:**
  - *Spelling Corruption* — attacks with homophones and misspellings.
  - *Grammar Corruption* — attacks with wrong parts of speech (a verb pretending to be a noun).
  - *Semantic Corruption* — attacks with opposite meanings, nonsense synonyms.
  - *Rhetorical Corruption* — attacks with emotional manipulation, logical fallacies, or exaggeration.
- **Enemy telegraphs its next attack.** A visual cue tells the child what kind of answer will counter it next turn.
- **Enemy has a "shield word."** The Typo protects itself with a prefix or suffix (e.g., "un-" shield, "-ly" shield). The child must play a word that breaks that shield (e.g., play a word starting with "re-" to cancel "un-").
- **Boss enemies are corrupted sentences.** A whole sentence has multiple blanks. The child must play a sequence of pet cards to fill the sentence with correct grammar.

### 2. Make the child's turn into a real decision

Each turn, the child has 4 options tied to the 4 language skills:

| Action | Skill | What it does | Risk/Reward |
|--------|-------|--------------|-------------|
| **Synonym Strike** | Semantics | Play a pet whose word is a synonym of the enemy's true word. | High damage if close; misses if wrong. |
| **Antonym Counter** | Semantics | Play a pet whose word is an antonym of the enemy's next attack. | Blocks damage and deals counter damage. |
| **Etymology Pierce** | Grammar | Identify the enemy's root or suffix. | Breaks shields, 2x damage, reveals true word. |
| **Rhetorical Appeal** | Rhetoric | Build a persuasion meter (Robot) or insight meter (Slime). | Charge up a powerful class move or "calm" the enemy. |

- The child can only play 1 pet per turn, but the pet has a **stamina bar**. Using the same pet every turn makes it tired and weak.
- Roster building matters: bring a synonym specialist, an antonym tank, a root-breaker, and a persuader.

### 3. Add class-specific playstyles (not just formulas)

- **Slime (Logic/Semantics):** "Analyze" action reveals the enemy's stats (concreteness, valence, intensity, dominance). Its special move is **Synonym Chain** — play 2-3 synonyms in a row for combo damage.
- **Golem (Grammar/Structure):** "Fortify" action builds a grammar shield. Its special move is **Sentence Smite** — build a 3-word sentence where each word matches a slot (WHO, WHAT DOING, WHAT) for massive damage.
- **Robot (Rhetoric/Social):** "Persuade" action fills a rhetoric meter. Its special move is **Reframe** — convert the enemy to your side for 1 turn, causing it to attack itself.

### 4. Turn order and tempo

- **Speed stat = Intensity (VAAM).** High-intensity words act first. Slow, calm words act last but heal or shield.
- **Initiative meter:** both pet and enemy fill a meter. The child can "wait" to charge a stronger attack but risks taking a hit first.
- **Turn economy:** each turn the child gets 1 action + 1 "word power" token. Tokens can be spent to boost damage, shield, or swap pets.

### 5. Affinity, friends, and enemies

- Each word has a **synonym web** and **antonym web** (from the database + WordNet-style expansion).
- If the child plays a word that is a *friend* of the last word used, they get a **Resonance** bonus (+10% per matching theme).
- If the child plays a word that is an *enemy* of the last word, they get a **Discord** bonus (higher damage but also self-damage).
- Example: "fire" → "flame" (friend) → "inferno" (friend) = chain bonus. "fire" → "water" (enemy) = big clash damage.

### 6. Etymology as a combat mini-game

- The enemy has a **hidden root** (e.g., "rupt" in "interrupt"). During the battle, the child can "scan" to reveal one letter of the root per turn.
- Once the full root is revealed, the child can play a word with the same root to deal a **Root Strike** (critical hit + shield break).
- Golems are especially good at this. They can scan faster.

### 7. Grammar as shields and combos

- **Part-of-speech slots:** The enemy has a "sentence structure" that the child must match. For example:
  - Enemy casts *"The {NOUN} {VERB} {ADVERB}."*
  - The child plays 3 pets: one noun, one verb, one adverb. If all three match, the combo deals 3x damage and the enemy is stunned.
- **Prefix/Suffix shields:**
  - Enemy shielded by "anti-" → play a word with "pro-" to break it.
  - Enemy shielded by "-less" → play a word with "-ful" to break it.
- **Tense agreement:** play a past-tense verb when the enemy attacks in the past; present-tense when the enemy is in the present. Match = bonus, mismatch = penalty.

### 8. Rhetoric as a persuasion/mind game

- **Appeal meter:** Robot pets build Pathos/Ethos/Logos during the battle.
  - *Logos appeal:* answer a logic challenge (e.g., "What is the opposite of this word?").
  - *Pathos appeal:* answer an emotional challenge (e.g., "Which word feels happier?").
  - *Ethos appeal:* answer an authority challenge (e.g., "Which word is more formal?").
- When the meter is full, the Robot can **Convince** the enemy to surrender, skip a turn, or attack itself.
- **Logical fallacy traps:** The enemy may use a "straw man" or "ad hominem" attack. The child must identify the fallacy to counter.

### 9. Status effects and conditions

Use language concepts as status effects:

| Condition | Cause | Effect |
|-----------|-------|--------|
| **Stunned** | Play a homophone of the enemy's word. | Enemy skips its next turn. |
| **Confused** | Play a word with a double meaning. | Enemy's next attack hits itself. |
| **Inspired** | Play a high-valence word. | Next attack deals +50%. |
| **Shamed** | Play a low-valence word. | Enemy deals -30% damage for 2 turns. |
| **Enlightened** | Reveal an etymology root. | Next Etymology Pierce is automatic critical. |
| **Fortified** | Play a high-concreteness word. | Block 20 damage. |

### 10. Make the battlefield readable

- **Enemy weakness glow:** When the child hovers a pet card over the Typo, the enemy glows if the pet is a good match (synonym, antonym, shared root).
- **Meaning clouds:** During attacks, floating words show the semantic connection ("fire → water → extinguish").
- **Pet reaction:** The pet's FACES expression changes based on the matchup — confident when strong, scared when weak.
- **Critical hit VFX:** When the child hits a root or synonym, the enemy "shatters" into root fragments that float back into the dictionary.

### 11. Progression and difficulty

- **Grade-scaled enemies:** A grade-2 enemy has simple words, one shield, and no special attacks. A grade-8 enemy has multi-root words, mixed shields, and rhetoric traps.
- **Enemy archetypes:**
  - *Malaprop* — uses wrong words that sound similar.
  - *Homonym* — two meanings, child must pick the right one.
  - *Run-On* — fast, attacks many times, but low damage per hit.
  - *Double Negative* — invert the child's answers unless countered.
- **Wave battles:** 2-3 small typos then a boss. Each wave uses a different skill.
- **Timed challenges:** The child has 10 seconds to pick a counter word. Builds pressure without being a quiz.

### 12. Narrative stakes and rewards

- **Rescue vs. Collect:** If you win, the corrupted word is "rescued" and added to your collection as a *cured* version. If you lose, it becomes a "forgotten word" and you must study it later.
- **Pet evolution through combat:** Pets only evolve when used in battle. Each victory adds a "battle scar" or "honor mark" visually.
- **The antagonist:** "The Red Pen" or "The Error" sends stronger typos as the child masters words. Boss battles unlock districts.
- **District corruption:** Each district has a "corruption level." Battles reduce it. When it hits zero, the district rewards a legendary word.

### 13. Multiplayer / Social combat

- **PvP Duel:** Two players each bring a roster. They take turns playing pet cards. The winner is the one who best counters the other's word choices.
- **Co-op Sentence Boss:** Two players combine their pets to fill a giant sentence. Each player handles different slots.
- **Asynchronous:** Leave a "duel challenge" for a friend with a specific enemy word. They can respond later.

### 14. Accessibility options

- **Multiple-choice mode:** Instead of typing a word, the child picks from 3-4 synonym/antonym options.
- **Hint system:** Spend a token to reveal the enemy's part of speech or root.
- **No-timer mode:** Remove the 10-second decision timer for younger players or anxiety.
- **Voice spelling:** Use Android speech recognition to say the counter word out loud.

### 15. Micro-feedback loop improvements

- **After every attack, show the reason:**
  - "Great! 'Water' is the opposite of 'Fire' — antonym counter!"
  - "Almost. 'Rock' and 'Stone' are too similar — no distance bonus."
  - "Perfect root match! 'Interrupt' and 'Rupture' both come from 'rupt' (break)."
- **Battle review screen:** After the fight, show 3 words used and 1 thing learned about each.
- **Mastery rewards tied to combat:** A word used in battle advances faster than one only collected.

---

## What we can do with the current code right now

These are the smallest changes that would make the existing `battle.rs` feel dramatically better:

1. **Add a 1-turn enemy telegraph.** The enemy shows a "next attack type" icon (synonym / antonym / root / rhetoric). The child must play the matching action type to counter.
2. **Give the enemy a simple attack.** If the child does not counter, the enemy deals damage based on its own intensity/dominance stats.
3. **Add a "Resonance" bonus** for playing words that are synonyms of the last word used.
4. **Show the reason** for damage numbers on screen, not just in logs.
5. **Add a combo counter** when the child plays 2+ effective cards in a row.
6. **Make the enemy react visually:** shake when hit hard, smirk when the attack is ineffective, panic when low HP.

---

## Homework for you and your wife

### 1. Answer these questions together

- What is the *emotional fantasy* of combat? Are you a librarian correcting a book, a trainer taming wild words, a duelist defending meaning, or a doctor healing sick language?
- What should a 7-year-old feel after winning a battle? What should they feel after losing?
- Should the enemy fight *back*, or should it be a puzzle the child solves? (Turn-based attack vs. puzzle boss.)
- Which is more important: building a strong roster of pets, or playing well during the battle?
- Should there be a timer, or is the game calmer without it?

### 2. Try this mini-playtest

- Open a children’s vocabulary app or flashcard deck.
- Pick two words: one for the pet, one for the enemy.
- Ask: *"What would make this fight feel smart, not just lucky?"*
- Write down 3 real words and the combat move you would use for each.

### 3. Compare to reference games

- **Prodigy Math:** How do math questions become combat? What can we steal?
- **Pokémon:** What makes choosing a Pokémon feel strategic? (Types, moves, status, switching.)
- **Hades / Slay the Spire:** How do roguelike deck builders create tension with simple choices?
- **Wordle / NYT Connections:** How do word games create "aha!" moments without violence?
- **Bookworm Adventures:** Direct predecessor — what did it do well and what feels dated?

### 4. Decide on the core tension

Pick one of these as the *primary* tension and one as the *secondary*:

- **Tension of knowledge:** "I know the right word but can I remember it under pressure?"
- **Tension of choice:** "I have three good words, but which one is the best tool?"
- **Tension of risk:** "Should I use my best pet now or save it for the boss?"
- **Tension of mastery:** "I lost last time because I didn't know this root. Now I do."
- **Tension of story:** "If I lose, this district stays corrupted."

### 5. Look at your child's playtest behavior (if you have one handy)

- What does a kid do when they don't know a word? Do they guess, ask, freeze, or look at the pet's face?
- What makes them laugh or excited?
- Do they care about winning, or about collecting cool pets?

---

## Things to explore before we implement

1. **Synonym/antonym data source.** Do we have a real synonym list in the database, or are we only using semantic distance? Accurate synonym/antonym data is needed for the "strike/counter" design.
2. **Part-of-speech tagging.** Does the database know if each word is a noun, verb, adjective, adverb? Needed for grammar-shield and sentence-combo mechanics.
3. **Etymology coverage.** How many words have root/suffix data? The GDD says 25 roots + 27 suffixes. Is that enough for combat variety?
4. **WordNet / Open English WordNet.** Could we integrate a free lexical database to get synonym/antonym/hypernym/hyponym data without building it ourselves?
5. **Visual clarity budget.** How much UI can a 7-year-old parse in a battle? 2 HP bars + 1 word + 1 icon is probably the max.
6. **Audio feedback.** Every correct answer should have a satisfying sound, every root a chime, every critical a roar. TTS of the words during battle would reinforce learning.
7. **Word difficulty and combat difficulty.** Should a legendary word always win, or should the child still need to use it skillfully? (No pay-to-win feeling.)

---

## Recommended next steps

1. **Do not change code yet.** First, answer the homework questions above and pick one primary tension.
2. Once you have a clear combat fantasy, we will design a **paper prototype** (a 5-turn battle using 3 pet cards and 1 enemy card) and test it verbally.
3. Then we will implement **one** small combat change (e.g., enemy telegraph + counter attack) before adding more complexity.
4. After that feels good, we will add the class-specific special moves.
5. Finally, we will tie combat rewards to the mastery/evolution system so the loop has long-term meaning.


---

# Appendix: Literary Devices & Authoring Tips for LitTCG

## Why this matters for combat (and the whole game)

LitTCG is about **words as living creatures**. The more we borrow from real writing craft, the more the game feels like a playground for language rather than a spelling test. Kids who love this game should naturally start playing with words in their own speaking, writing, and storytelling.

The goal is **meta-communication at low levels**: giving kids the words to talk about how words work. Not "grammar rules," but *"this word is pretending,"* *"this word sounds fast,"* *"this word makes a picture in my head."*

---

## Literary devices as game mechanics

### Sound devices (ear-level craft)

| Device | What it is | Game mechanic idea |
|--------|-----------|-------------------|
| **Alliteration** | Repeated starting sounds: *"Peter Piper picked..."* | Play a word whose first letter matches the enemy's word for a **Sound Strike** bonus. |
| **Assonance** | Repeated vowel sounds: *"frost" / "road"* | Match the vowel sound of the enemy's root for a **Resonance** bonus. |
| **Consonance** | Repeated consonant sounds inside words: *"litter" / "letter"* | Inner-sound matching creates a **Rhythm Shield**. |
| **Onomatopoeia** | Words that sound like their meaning: *"buzz," "clang," "whisper"* | Onomatopoeia pets have built-in sound effects and do extra "noise damage." |
| **Rhyme** | Ending sounds match: *"cat" / "hat"* | Play a rhyming word for a **Rhyme Combo** that chains into another attack. |
| **Meter / Rhythm** | The beat of syllables: *"da-DUM da-DUM da-DUM"* | Long, rhythmic words can "charge" a powerful move. Short, choppy words act fast. |

### Meaning devices (mind-level craft)

| Device | What it is | Game mechanic idea |
|--------|-----------|-------------------|
| **Metaphor** | One thing *is* another: *"time is a thief"* | Play a metaphorically related word to bypass an enemy's shield. |
| **Simile** | One thing is *like* another: *"fast as lightning"* | Simile words create a **Comparison** buff that boosts the next attack. |
| **Personification** | Giving human traits to non-human things | Core to the game: every word becomes a pet with a face. |
| **Hyperbole** | Exaggeration: *"I'm so hungry I could eat a horse"* | Hyperbole words deal exaggerated damage but have a chance to "miss" because they're over-the-top. |
| **Understatement** | Making something small: *"It's just a scratch"* (when it's not) | Lowers enemy threat or calms it. Useful for Robot rhetoric. |
| **Oxymoron** | Two opposite ideas together: *"deafening silence"* | Oxymoron words confuse the enemy, making it attack itself. |
| **Irony** | When expectation and reality clash | Irony words reverse the last attack type: an ineffective hit becomes a critical hit. |
| **Symbolism** | A word stands for a bigger idea | Symbolic words give team-wide aura buffs (e.g., "dove" = peace, lowers enemy aggression). |
| **Imagery** | Words that create a sensory picture | High-concreteness words already do this; lean into the 5-senses mapping. |
| **Allusion** | A reference to another story or myth | Allusion words (e.g., "Titan," "Narcissus") summon a mini-story effect. |

### Structure devices (sentence-level craft)

| Device | What it is | Game mechanic idea |
|--------|-----------|-------------------|
| **Repetition / Anaphora** | Repeating a word at the start: *"I want... I want... I want..."* | Repeating the same word family across 3 turns builds a **Chant** that auto-crits. |
| **Epistrophe** | Repeating at the end: *"...of the people, by the people, for the people"* | Ending-sound chains build a **Cadence** bonus. |
| **Juxtaposition** | Putting two unlike things together: *"fire and ice"* | Playing two antonym pets back-to-back creates a **Clash** explosion. |
| **Pun** | A word with two meanings: *"I used to be a banker, but I lost interest"* | Puns stun the enemy; the child must spot the double meaning. |
| **Double Entendre** | A word or phrase with two interpretations | Double-meaning words can be played as either a noun or a verb in sentence combos. |
| **Malapropism** | Wrong word that sounds right: *"He is the very pineapple of politeness"* | A malaprop enemy type is confused by sound-alike words. |
| **Portmanteau** | Blended word: *"brunch," "motel"* | Portmanteau pets combine two words' stats; advanced crafting. |
| **Palindrome** | Same forwards and backwards: *"radar," "level"* | Palindrome words can be played twice in one turn. |

### Rhetorical devices (persuasion-level craft)

| Device | What it is | Game mechanic idea |
|--------|-----------|-------------------|
| **Ethos** | Appeal to credibility/character | Robot pets build **Trust** meter to make enemies surrender. |
| **Pathos** | Appeal to emotion | High-valence words create empathy or fear. |
| **Logos** | Appeal to logic | High-concreteness words or root analysis appeal to logos. |
| **Rhetorical question** | A question for effect, not an answer | Enemy casts a question; the child must answer with the correct word type. |
| **Analogy** | Comparing two relationships | Analogies as puzzle battles: "Fire is to hot as ice is to ___?" |
| **Rule of Three** | Lists of three feel complete | A 3-word combo (noun, verb, adjective) deals triple damage. |

---

## Authoring tips and tricks for kids

These are tricks real writers use. We can turn them into prompts, pet behaviors, quest text, or post-battle reflections.

### 1. Show, don't tell
- *Don't say:* "The dragon was scary."
- *Say:* "The dragon's breath turned the grass to ash."
- **Game use:** Pet descriptions never say "happy" or "angry." They describe movement, color, sound, and heat. This teaches the same habit.

### 2. Use the five senses
- Sight, sound, smell, taste, touch, and even temperature.
- **Game use:** Word stats can be mapped to senses:
  - Concreteness → "How easy is it to see/touch?"
  - Intensity → "How loud is it?"
  - Valence → "How does it smell/feel emotionally?"

### 3. Start in the middle
- Good stories often begin with action already happening.
- **Game use:** Battles start with the enemy already attacking. No long intro.

### 4. Ask a question early
- If the reader is curious, they keep reading.
- **Game use:** Battle intro: *"Why did 'fire' become 'firy'? Can you fix it?"*

### 5. Make characters want something
- Every character needs a goal, even a small one.
- **Game use:** The pet wants to protect its meaning. The Typo wants to spread confusion. The NPC wants a specific sentence built.

### 6. Use concrete details, not abstract labels
- *Abstract:* "It was a bad day."
- *Concrete:* "My backpack zipper broke and my sandwich fell in the mud."
- **Game use:** High-concreteness words are stronger in combat. This rewards specific language.

### 7. Build patterns and then break them
- Repetition creates expectation; breaking it creates surprise.
- **Game use:** The enemy attacks 3 times with synonyms; the child expects a 4th, but a well-timed antonym reverses the pattern.

### 8. Let the reader (player) figure it out
- Don't explain everything. Let the audience discover.
- **Game use:** Don't tell the child the answer. Show them the enemy's face, its shield color, and a hint. Let them guess.

### 9. End with a change
- A scene works best when something is different at the end.
- **Game use:** After battle, the word is changed: corrupted → corrected. The district is slightly less corrupted. The pet evolved. The child learned something.

### 10. Play with voice
- A sentence can sound like a robot, a pirate, a professor, or a kindergartner.
- **Game use:** Robot pets sound persuasive and formal. Golems sound structured. Slimes sound analytical and fluid.

---

## Meta-communication vocabulary for kids

These are low-level phrases kids can use to talk about how words work. We should teach them in-game, not in a tutorial.

### Talking about meaning
- "This word is the opposite of..."
- "This word means the same as..."
- "This word makes me think of..."
- "This word is pretending to be..."
- "This word is hiding inside..." (root)

### Talking about sound
- "This word sounds like..."
- "This word starts the same as..."
- "This word has a beat like..."
- "This word feels fast / slow / heavy / light."

### Talking about grammar
- "This word is a person/place/thing." (noun)
- "This word is an action." (verb)
- "This word is describing." (adjective/adverb)
- "This word has a little word stuck on it." (prefix/suffix)

### Talking about effect
- "This word makes me feel..."
- "This word makes a picture in my head."
- "This word sounds strong / soft / scary / funny."
- "This word is trying to convince me."

### Talking about craft (advanced)
- "This is a comparison."
- "This is an exaggeration."
- "This word is doing two jobs at once."
- "This word is making a pattern."
- "This word is surprising me on purpose."

---

## How to weave these into gameplay loops

### Pet collection → writing practice
- When a child summons a new pet, the game asks them to write one sentence using the word. They get a bonus if the sentence uses a specific device (e.g., "Use the word 'thunder' in a sentence with a sound word.")
- Sentences can be saved to the pet's "lore card" and displayed in the collection screen.

### Combat → literary device practice
- The enemy's shield is a device. The child must identify and counter it.
- Example: "Enemy is wrapped in **Hyperbole**. Use an **Understatement** pet to calm it down."
- Post-battle: "You used a **metaphor** (fire = anger) to win. That means comparing two things."

### Quests → sentence building
- The NPC quest is a sentence with missing slots. The child must not only fill the grammar slot but also make the sentence more interesting using a device.
- Example: "Make this sentence more exciting using a sound word."
- Original: "The dog ran." → Improved: "The dog **clattered** down the hall."

### Pet evolution → authoring milestones
- A pet evolves when the child demonstrates the word in 3 different contexts: combat, quest, and a written sentence.
- Each evolution adds a visual badge: "Metaphor Master," "Alliteration Ace," "Root Hunter."

### Daily prompts → creative writing
- A "Daily Verse" prompt gives a starting phrase and asks the child to finish it with one literary device.
- Example: "The silence was so loud that..." (hyperbole/onomatopoeia prompt)
- Save favorites; best ones become loading-screen quotes.

---

## Starter list of "writer moves" we can gamify

1. **Make a comparison** (simile/metaphor)
2. **Exaggerate** (hyperbole/understatement)
3. **Give a thing a face** (personification)
4. **Make a sound** (onomatopoeia/alliteration)
5. **Repeat on purpose** (anaphora/epistrophe)
6. **Use opposites** (oxymoron/juxtaposition/antonym)
7. **Break a pattern** (irony/subversion)
8. **Build a picture** (imagery/concreteness)
9. **Persuade** (ethos/pathos/logos)
10. **Build a sentence** (grammar slots/part of speech)
11. **Find the hidden root** (etymology)
12. **Match the rhythm** (meter/rhyme/consonance)
13. **Make a joke** (pun/double meaning)
14. **Tell a tiny story** (anecdote/allusion)
15. **Use the rule of three** (pattern of three)

---

## Homework additions

### For you and your wife

1. **Which literary device is the most "LitTCG"?** If a kid could only learn one writing trick from this game, which should it be? (Personification is a strong candidate because the whole game is words becoming creatures.)

2. **What is the youngest age that should understand a device?** A 6-year-old can understand personification and onomatopoeia. A 10-year-old can handle metaphor and alliteration. A 12-year-old can handle irony, rhetoric, and etymology. This maps directly to grade levels.

3. **Should the game teach the *name* of the device, or just let kids feel it?** Teaching the name gives them meta-language. Letting them feel it first keeps it playful. We can do both: feel it first, name it after victory.

4. **Can you think of a real sentence your kid wrote recently that could become a pet or a battle?** Bring 2-3 examples. We can design enemies and moves from real child language.

5. **What writing do you want your kid to do more of?** Creative stories? Descriptions? Arguments? Poetry? Jokes? The answer changes which devices we prioritize.

### Things to explore

- **Children's poetry anthologies** (Shel Silverstein, Jack Prelutsky, Dr. Seuss) — these are treasure troves of gamified devices.
- **Figurative language worksheets** for grades 2-8 — shows what teachers already expect kids to know.
- **Storytelling games** like *Rory's Story Cubes*, *Once Upon a Time*, or *Dixit* — how do they turn prompts into narratives?
- **Mad Libs** — already matches our quest system; study what makes the results funny.
- **The Electric Company / Schoolhouse Rock** — classic examples of turning grammar into catchy mechanics.

---

## Recommended next step for this section

Once we have picked the combat fantasy and core tension, the next decision is: **which 3-4 literary devices become the verbs of the game?** (The actions the child performs every turn.) The rest become flavor, enemy shields, and evolution badges.

A likely starting set for ages 6-12:
1. **Synonym / Antonym** (meaning opposites — already in the game)
2. **Etymology / Roots** (grammar archaeology — already in the game)
3. **Personification** (words as creatures — already the core theme)
4. **Alliteration / Rhyme** (sound play — easy to add and fun)

These four cover the semantic, grammatical, creative, and sound dimensions of language without overwhelming the player.


---

# Appendix: Economy & Resource Management for LitTCG

## Why resource management is a hidden fun engine

Combat is the **drama**, but economy is the **meaning behind every choice**. Without scarcity, trade-offs, and long-term investment, collecting words feels like checking boxes. With a good economy, every word matters: *Should I use this rare word now? Should I save it? Should I trade it? Should I master it or keep exploring?*

Resource management also creates **autonomy**. The child owns their collection, makes decisions, and lives with the consequences. That is a core driver of engagement.

---

## Resources that already exist in the game

| Resource | Where it lives | Current use | Economic potential |
|----------|---------------|-------------|-------------------|
| **Letter crystals** | `LetterStash` | Collected, spent to spell words | The raw material of the economy. Scarcity and spawn bias create value. |
| **Words / pets** | `PetCollection` | Collected, used in battle/quest | The primary capital. Rarity and mastery determine value. |
| **Mastery** | `SpellBook` / `PetCollection` | Tracks how much a word has been used | The "experience currency" that gates evolution. |
| **XP / Evolution points** | Quest rewards | Levels up grade manager | Long-term progression currency. |
| **Grade manager** | `quest.rs` | Tracks player difficulty | The "skill tier" that gates what content is available. |

Missing resources that would add economic depth:

| Resource | What it could be | Fun effect |
|----------|-----------------|------------|
| **Ink / Verse Power** | Energy spent to play a pet in battle | Limits how many pets can be used per fight; creates save-or-spend tension. |
| **Stamina** | Per-pet fatigue | Prevents using the same best pet every turn. Forces roster rotation. |
| **Attention / Focus** | Player-level resource for "scanning" enemies | You can only inspect one enemy weakness per turn; choose wisely. |
| **Bond points** | Emotional currency from petting/feeding | Used to unlock special moves or evolution branches. |
| **Etymology shards** | Drops from defeating typo enemies | Craft new prefixes/suffixes or combine into portmanteau words. |
| **District reputation** | Favor with NPCs in each district | Unlocks quests, discounts, or legendary words. |
| **Lore pages** | Collectible fragments from battles | Unlock enemy backstories, district lore, or pet dream poetry. |

---

## Core economic principles to apply

### 1. Scarcity creates value

- Rare letters (Z, X, Q) should feel exciting to find, not annoying.
- Rare words should be harder to obtain but visibly more powerful.
- If every word is equally easy to get, none feel special.
- **Rule:** Common words = easy to collect, shallow mastery. Rare words = hard to collect, deep mastery, unique abilities.

### 2. Every choice has a trade-off

- **Use a word in battle** → it gains mastery but loses stamina.
- **Save a word for a quest** → it might be the perfect fit later, but you don't get battle XP now.
- **Feed a word to another pet** → sacrifice one pet to boost another.
- **Trade a word to an NPC** → lose a pet, gain a letter, item, or lore.
- **Spend ink on a big attack** → less ink for healing or switching later.

### 3. Investment pays off over time

- A word used in 3 different contexts (battle, quest, written sentence) evolves.
- Mastery is not instant. The child must decide which words are worth investing in.
- This creates **attachment** to specific pets.
- **Design tip:** Make the first evolution feel achievable so kids get hooked, then make later evolutions require real investment.

### 4. Resources should be sinkable, not just collectible

- If a resource only accumulates, it becomes meaningless.
- Sinks:
  - Evolution consumes evolution points.
  - Crafting consumes etymology shards.
  - Feeding consumes lesser words or letters.
  - Attuning consumes bond points.
  - Buying a hint consumes a small amount of ink or lore pages.

### 5. Different currencies for different domains

- **Letters** → spelling / summoning
- **Ink** → combat actions
- **Bond points** → pet relationship / evolution branches
- **Etymology shards** → crafting / suffixes / prefixes
- **District reputation** → unlocking content
- **Mastery** → word-specific power

Multiple currencies prevent one optimal strategy from dominating the game.

### 6. Visible feedback on resource flows

- Show +1 XP floating up after a successful attack.
- Show a shard breaking off a defeated enemy.
- Show the pet's bond meter fill when fed.
- Show the district corruption meter drop after a quest.
- Make the economy **felt**, not just tracked in a spreadsheet.

---

## Economic systems we can add

### 1. Letter economy

- **Vowel scarcity:** Vowels are more common but also more valuable because you can't spell words without them.
- **Consonant clusters:** Getting "TH" or "CH" as a combined letter crystal is a small jackpot.
- **Curriculum bias:** Letters spawn biased toward grade-appropriate words so the child is rarely stuck with unusable letters.
- **Letter trading:** NPCs buy surplus letters for ink or shards.
- **Letter decay:** Letters that sit unused for too long "fizzle" and must be refreshed by spelling something. (Prevents hoarding.)

### 2. Pet economy (roster as portfolio)

- **Roster size limit:** You can only bring 3-6 pets. This forces strategic choice.
- **Pet fatigue:** Pets used in battle get tired. Overused pets deal less damage and miss more often.
- **Pet maintenance:** Pets need to be "fed" related words or they become "hungry" and lose a small amount of mastery over time.
- **Pet trading:** Some NPCs want specific words. Trade a duplicate pet for a rare letter or a lore page.
- **Pet retirement:** A mastered pet can be "retired" to your collection, giving a permanent small bonus to future pets of the same element or class.

### 3. Ink / Verse Power economy

- Every turn in combat costs 1 ink to play a pet.
- Special moves cost 2-3 ink.
- Scanning an enemy costs 1 ink.
- Switching pets costs 1 ink.
- Ink regenerates slowly each turn, or the child can spend a letter crystal to refill it.
- This turns combat into a **manageable budget puzzle** rather than a button-mashing sequence.

### 4. Mastery and evolution economy

- Mastery is a per-word resource. It only increases when the word is used in meaningful contexts.
- **Mastery sources:**
  - Battle victory: +1
  - Quest slot filled: +1
  - Written sentence: +1
  - Fed to another pet: +0.5 (receiver gains, giver does not lose)
- **Mastery sinks:**
  - Evolution threshold: spend mastery to evolve.
  - Attunement: spend mastery to align to a channel.
  - Dream layer unlock: spend mastery at "Mastered" tier.
- This makes mastery feel like a real investment, not just a progress bar.

### 5. Crafting economy

- **Etymology shards** drop from defeated typos.
- Combine shards to create prefixes, suffixes, or portmanteau words.
- Example: 5 "fire" shards + 3 "storm" shards = "firestorm" (a new pet with merged stats).
- Crafting teaches morphology in a hands-on way.
- **Risk:** Crafting might fail and produce an Unstable Mutant instead.

### 6. District economy

- Each district has a **corruption level** and a **reputation meter**.
- Battles reduce corruption. Quests increase reputation.
- High reputation unlocks:
  - Better letter spawn rates
  - Rare word quests
  - NPC discounts
  - Legendary boss battles
- Low reputation means districts are harder (more typos, fewer resources).
- This gives the world a sense of **shared economic health**.

### 7. Shop / barter economy

- NPCs act as merchants:
  - **The Scribe** trades letters for ink.
  - **The Librarian** trades lore pages for rare words.
  - **The Alchemist** trades shards for prefix/suffix items.
  - **The Bard** trades mastered words for cosmetic pet flourishes or dream poetry.
- Prices vary by district, time of day, and reputation.
- No coins. The economy is **barter-based**, so every trade is a meaningful sacrifice.

### 8. Daily economy loops

- **Daily login bonus:** A single rare letter or a "spark" (small ink refill).
- **Daily quest:** A specific NPC asks for a sentence using a particular device. Reward: bond points or a lore page.
- **Daily challenge:** Defeat a specific typo with only words of a certain class. Reward: etymology shards.
- These are gentle hooks without being exploitative or manipulative (important for a children's app).

---

## Economic tension points that create fun

1. **The rare word dilemma.** You finally got "ephemeral." Do you use it in a tough battle now, or save it for a quest that specifically needs an abstract word?
2. **The stamina trap.** Your best pet is tired. Do you push through, or swap to a weaker pet and risk losing the battle?
3. **The shard gamble.** You have 10 shards. Do you craft a new word, or spend them to upgrade a pet you already love?
4. **The roster puzzle.** The enemy is a Golem. You know Slimes beat Golems, but your only Slime is low-level. Do you bring it anyway?
5. **The trade temptation.** An NPC offers a legendary letter for a pet you are attached to. Is it worth it?
6. **The mastery cliff.** A word is one use away from evolution. Do you use it in a safe battle to guarantee the evolution, or in a risky boss fight for a bigger payoff?

---

## How economy connects to "fun"

| Fun driver | How economy supports it |
|-----------|------------------------|
| **Autonomy** | The child chooses which words to invest in, which to trade, which to save. |
| **Mastery** | Managing scarce resources well is a skill that improves over time. |
| **Purpose** | Every battle and quest feeds into a long-term collection and progression goal. |
| **Drama** | Scarcity creates stakes: losing a pet or running out of ink matters. |
| **Discovery** | New resources (shards, lore, rare letters) feel like treasure. |
| **Creativity** | Crafting and trading let kids experiment with combinations. |
| **Narrative** | The district corruption/reputation system makes the world feel responsive. |

---

## Homework additions for economy

### For you and your wife

1. **What should be the *scarcest* resource in the game?** Rare letters, ink, high-level words, or your time/attention? The scarcest resource defines the core tension.

2. **Should kids ever lose a pet permanently?** Losing things creates real stakes, but it can also feel bad. Is there a middle ground (pets get "forgotten" and can be re-summoned later)?

3. **Do you want a marketplace or only barter?** A marketplace with prices feels like an economy. Barter feels more like a storybook. Which fits the tone?

4. **What is the " coolest thing" a kid should save up for?** A legendary word? A pet evolution? A district transformation? A new skin? This tells us what the economy is building toward.

5. **Should the game punish hoarding or encourage it?** Hoarding is realistic for collectors, but it can stall progression. A small decay or fatigue system encourages use without being punishing.

### Things to explore

- **Animal Crossing / Stardew Valley:** How do they make daily resource loops feel cozy rather than stressful?
- **Slay the Spire / Hades:** How do they make limited resources create high-stakes decisions?
- **Pokémon:** The PC box is a collection; the active party is a portfolio. How do they make both feel meaningful?
- **Diablo / Path of Exile:** Loot explosions and crafting systems. What can we borrow for etymology shards?
- **Kids' board games like Catan or Monopoly Jr.:** How do they teach trading and scarcity without being too complex?
- **Educational economy games:** *DragonBox*, *Prodigy*, *Minecraft Education* — how do they make resource gathering feel like learning?

---

## Recommended next step for economy

Before adding new systems, decide **one primary resource** and one **sink** that will drive the first 30 minutes of play. A good starting pair:

- **Primary resource:** Letters (already in the game)
- **Secondary resource:** Ink / Verse Power (new)
- **Sink:** Pet evolution and roster fatigue

Once that loop feels good, add etymology shards and crafting as the mid-game layer, then district reputation and shops as the late-game layer.

