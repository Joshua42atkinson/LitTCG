# **Semantic Slime: 40-Hour Research Spec & GDD**

# **Table of Contents**

1. 1\. Introduction & Executive Summary  
2. 2\. Academic & Commercial Gamification Research  
3. 3\. Chapter I: Core Game Systems & Combat Mechanics Deep Dive  
   1. 3.1 Linguistic Triad Combat Mathematics  
   2. 3.2 The Card Deck & Hand Engine (deck.rs)  
   3. 3.3 ASL Hand Tracking & Spatial Gestural Inputs  
4. 4\. UI/UX Spatial Controls  
5. 5\. Technical Art and Mesh Specifications  
6. 6\. Audio and Pitch Telemetry  
7. 7\. The 12 Districts Lore Encyclopedia  
8. 8\. XREAL Aura Hardware Specs & Memory Budget  
9. 9\. The Generative Pet Generation Pipeline  
10. 10\. The Grade 1-5 Evolution System  
11. 11\. The Teacher-as-Mentor Framework

# **1\. Introduction & Executive Summary**

Semantic Slime is a unified 40-hour pedagogical combat system designed for K-12 English Language Arts (ELA) integration. Synthesizing 'The Semantic Slime Technical Bible', 'The Semantic Siege', and the 'SEP Engine Spec', this system transforms abstract linguistic structures into tangible tactical encounters.

The core objective is the gamification of the "Linguistic Weapon Triad," where players move through iterative cycles of Grammar, Semantics, and Rhetoric. By utilizing a Bevy-based Entity Component System (ECS), the game ensures that every sentence constructed by the student serves as a functional blueprint for a combat unit. This 40-hour curriculum provides a rigorous transition from basic syntactic construction to complex rhetorical persuasion, all set within a high-fidelity tactical environment.

# **2\. Pedagogical & Cognitive Research Foundations**

The design of Semantic Slime is grounded in a rigorous, academic analysis of foundational pillars of educational and ludic theory, specifically Sweller's Cognitive Load Theory (CLT) as applied to the 'Linguistic Physics Engine'.

## **Sweller's Cognitive Load Theory (CLT)**

To ensure K-12 accessibility, the curriculum adheres to [Sweller’s Cognitive Load Theory](https://www.instructionaldesign.org/theories/cognitive-load-theory/). In the Semantic Slime system, cognitive load is mathematically mapped to physical variables within the railway environment:

* Intrinsic Load (Train Mass): Mathematically determined by phonetic plosive counts via cmudict. Words with higher plosive density increase the inertia of the Syntax Golems.  
* Extraneous Load (Track Friction): Maps to UI and syntax latency within the bevy\_picking implementation. Friction increases as student attention is diverted from core linguistic construction.  
* Germane Load (Combustion/Speed): Calculated using syntactical Dependency Distance. High semantic integration results in greater forward velocity, rewarding deep internal modeling of language rules.

## **Procedural Rhetoric**

Following [Ian Bogost’s Procedural Rhetoric](https://bogost.com/books/persuasive_games/), Semantic Slime argues that games make arguments through processes. The game's Bevy-based ECS rules teach grammatical boundaries not through text, but through functional blueprints. Drawing parallels to [Seymour Papert’s Constructionism](https://web.media.mit.edu/~jconfino/papert.html), spelling is treated as an act of physical, geometric block assembly where a student builds "Semantic Slimes" to solve tactical problems. If grammatical validity is false, the `Transform` component is disabled, making the consequences of error procedural.

# **3\. Chapter I: Core Game Systems & Combat Mechanics Deep Dive**

## **3.1 Linguistic Triad Combat Mathematics**

The combat efficacy is determined by the interaction between Grammar, Semantics, and Rhetoric scalars. The final damage output for a Syntax Golem is calculated as follows:  
$$Damage\_{final} \= (Base\_{Syntax} \\times \\sum\_{i=1}^{n} Modifier\_{Complexity}) \\times \\Phi\_{Rhetoric}$$  
Where:

* **Base Syntax:** Functional status of the body. If grammatical validity is false, value is 0\.  
* **Modifier Complexity:** Scalars derived from semantic precision and adjective density.  
* **Rhetoric Multiplier (\\Phi):** Derived from Ethos, Pathos, and Logos appeals.

Defensive mitigation is handled by the Concreteness offset, reducing incoming Persuasion Damage based on the semantic density of the defender's linguistic structure:  
$$Mitigation \= \\frac{1}{1 \+ e^{-(\\text{Concreteness} \- \\text{Complexity})}}$$

## **3.2 The Card Deck & Hand Engine (deck.rs)**

Within the Bevy ECS, the \`deck.rs\` module manages the lifecycle of parsed word entities represented as tactical cards. The engine utilizes the following state flow:

| State | Logic & ECS Interaction |
| :---- | :---- |
| Draw | Pulls Semantic Slime entities from the Lexical Scrap pool into the active Hand component. |
| Shuffle | Randomizes entity ID ordering within the deck storage vector to ensure varied syntactic opportunities. |
| Play/Commit | Triggers the SEP Engine transformation, moving entities from Hand to Combat-field as active Golems or Slimes. |
| Discard | Sends used or broken syntax components back to the unformed thought void for recycling. |

## **3.3 ASL Hand Tracking & Spatial Gestural Inputs**

Semantic Slime utilizes Android XR hand-tracking APIs to enable ASL-fingerspelling for word construction. The input system tracks 21 distinct joint nodes per hand to validate linguistic input.

# **3\. Chapter II: Aesthetics, Audio & WebXR Art Bible**

## **The SEP (Semantic Emotive Primitive) Engine Visuals**

The visual representation of the SEP Engine follows the 4-Wheel Jungian Matrix (Container, Aura, Focus, Action). Bevy standard materials and custom vertex shaders shape the procedural slime meshes. The system utilizes Bezier control points to dynamically deform the 3D GLTF meshes generated via Micro-Trellis based on semantic vectors.

## **Audio & Pitch Telemetry**

The system utilizes the Web Audio API pitch detection system to track microtonal cents deviation during ASL and vocal inputs. The Kokoro TTS sidecar voice parameters are dynamically adjusted based on valence and arousal vectors parsed from student input, ensuring the auditory feedback matches the pet's semantic state.

| Interaction Type | Technical Specification |
| :---- | :---- |
| ASL Fingerspelling | Parsing of semantic vectors via Janus-Pro-1B based on real-time joint angle configurations. |
| Pinch-to-Assemble | Collision boundaries on Syntax Golem meshes allow physical dragging of noun-phrases into verb-slots. |
| Rhetoric Swipe | Spatial gestural inputs to scale the Impact Radius of Rhetoric Robots using velocity-based scalars. |

# **3\. The Linguistic Weapon Triad**

The core loop of the SEP (Semantic-Engine-Processing) involves a recursive transformation: **Grammar \-\> Semantics \-\> Rhetoric \-\> Grammar**.

## **Tactical Implementations (Bevy ECS)**

The game utilizes the Bevy ECS architecture to map linguistic components to game objects:

* **Syntax Golems (The Body):** Defined by strict grammatical rules. If the syntax is broken (e.g., a dangling modifier), the Golem’s `Transform` component is disabled, rendering it immobile.  
* **Semantic Slimes (The Spirit):** These entities represent the "meaning" or "context" of the input. They apply `StatusEffects` based on the definitions of the words used. A "frozen" adjective literally applies a `FrozenComponent` to the target.  
* **Rhetoric Robots (The Will):** These represent the persuasive intent (Ethos, Pathos, Logos). They act as multipliers for the Syntax Golems and Semantic Slimes, determining the "Impact Radius" and "Persuasion Damage" on enemy AI.

# **4\. 10-Week / 40-Hour Game Progression & Curriculum Map**

| Week | ELA Common Core Target | Gameplay Loop | Narrative Arc |
| :---- | :---- | :---- | :---- |
| 1-2 | Parts of Speech & Noun Phrases | Harvesting Lexical Scrap to build basic Syntax Golems. | Arrival at the Lexis Terminus; the first train car is hijacked. |
| 3-4 | Verb Tense & Agreement | Sequencing combat actions using proper temporal markers. | Entering the District of the Caretaker; repairing the engine. |
| 5-6 | Adjectives & Semantic Precision | Infusing Slimes with specific elemental properties via modifiers. | The Fog of Ambiguity descends upon the Great Railway. |
| 7-8 | Clause Structure & Coordination | Linking multiple Golems into "Compound Units" for heavy sieges. | The Siege of the Shadow Library begins. |
| 9-10 | Rhetorical Appeals (Ethos, Pathos, Logos) | Final Boss: Deploying Rhetoric Robots to deconstruct the antagonist's logic. | Reaching the 12th District; restoring the Great Railway of Lexis. |

# **5\. Semantic Railway World Lore & World-Building**

## **The Great Railway of Lexis**

The world is a singular, infinite railway line cutting through a void of unformed thought. Every "Train Car" is a repository of a specific linguistic era. The "Great Railway" is the only thing connecting the disparate thoughts of the world together.

## **The 12 Districts Lore Encyclopedia**

The Great Railway circles a central void, passing through 12 Districts mapped to the chromatic scale from C to B. Each district is governed by an archetype and specific CCSS targets:

1. 

| Key/Archetype | District | Focus | CCSS Target | Quest & Dialogue Mechanics |
| :---- | :---- | :---- | :---- | :---- |
| C \- The Innocent | Garden District | Somatic: Arrival | CCSS.ELA-LITERACY.L.4.5 | NPC Barnaby requires simple nouns. Mad-Lib: "I see a \[Noun\] in the garden." |
| C\# \- The Outlaw | Outlaw Outpost | Somatic: Tension | CCSS.ELA-LITERACY.L.5.1 | NPC Nyx requires irregular verbs to break the Silence. Quest: Hijack the train car. |
| D \- The Sage | Shadow Library | Somatic: Thought | Semantic Density | Requires high-precision adjectives to identify ancient Slime entities. |
| D\# \- The Explorer | Great Railway | Somatic: Search | Fragmented Syntax | Quest: Repair syntax components using experimental rhetoric. |
| E \- The Caretaker | Maintenance Bay | Somatic: Repair | Verb Tense | Dialogue requires temporal markers to fix damaged Syntax Golems. |
| F \- The Jester | Irony Junction | Somatic: Play | Rhetorical Appeals | NPC uses puns and reversals. Quest: Deconstruct boss logic. |
| F\# \- The Lover | Adjective Valley | Somatic: Feeling | Descriptive Language | Requires sensory adjectives to flourish local flora and fauna. |
| G \- Everyman | Central Station | Somatic: Belonging | Sentence Structure | Forging basic noun-verb-object structures for new players. |
| G\# \- The Hero | The Front Line | Somatic: Action | Complex Clauses | Quest: Link multiple Golems into "Compound Units" for siege. |
| A \- The Magician | SEP Hub | Somatic: Change | Linguistic Reality | Physical dragging of noun-phrases into verb-slots to spawn entities. |
| A\# \- The Ruler | Hierarchy Hall | Somatic: Order | Subordination | Dialogue requires mandatory complex, hierarchical structures. |
| B \- The Creator | Final Destination | Somatic: Legacy | Persuasive Rhetoric | Quest: Forge personal linguistic legacies via Sovereign Avatars. |

# **6\. XREAL Aura Hardware Specs & Memory Budget**

The XREAL Aura platform operates on a high-fidelity 16GB RAM model, requiring precise memory orchestration to maintain the concurrent execution of the OS, Bevy client, and the local edge-AI suite. The OS and Compositor overhead accounts for a significant portion of the memory footprint, necessitating the use of INT4 quantization for the generative models to fit within the remaining 4.2GB edge-AI allocation.

| Component | Optimization Level | Memory Allocation |
| :---- | :---- | :---- |
| Janus-Pro-1B (Semantic Vision/LLM) | INT4 Quantized | \~1.2 GB |
| Micro-Trellis 3D (Mesh Gen) | INT4 Quantized | \~2.0 GB |
| Kokoro TTS (Sidecar) | N/A | \~1.0 GB |
| **Total AI Local Budget** | — | **4.2 GB** |

# **7\. The Generative Pet Generation Pipeline**

The transformation of linguistic input into a physical game entity follows a multi-stage generative pipeline. This workflow begins with the user inputting an ASL-fingerspelled word, which is parsed into semantic vectors. These vectors are processed by the Janus-Pro-1B model to define the entity's core attributes. The resulting data is piped into Micro-Trellis for the generation of a 3D GLTF mesh. Finally, the entity is rendered within the Bevy engine using the specialized FACES shader, ensuring a high-fidelity visual representation of the student's semantic construction.

# **8\. The Grade 1-5 Evolution System**

The Semantic Slime entities undergo a five-stage evolutionary progression, mirroring the student's academic growth from Grade 1 to 5\. To trigger an evolution, the student must "feed" the pet appropriate vocabulary that aligns with ELA standards. This process ensures that the pet's physical complexity is a direct reflection of the student's linguistic proficiency.

| Evolution Stage | Entity Form | Linguistic Requirement |
| :---- | :---- | :---- |
| 1 | Phoneme Seed | Minimalist SEP primitive; basic phonemic input. |
| 2 | Syntax Bud | Noun/Verb pairings and simple subject-predicate structures. |
| 3 | Semantic Larva | Inclusion of descriptive adjectives and semantic precision. |
| 4 | Rhetorical Cocoon | Complex clause structures and coordinating conjunctions. |
| 5 | Sovereign Avatar | Detailed generative mesh with CAGED scale capabilities; persuasive rhetoric. |

# **9\. The Teacher-as-Mentor (Human-in-the-Loop) Framework**

The system is designed to function as a collaborative environment between AI logistics and human mentorship. While the AI manages the technical execution of the SEP Engine, the teacher or parent serves as the constructivist mentor. Utilizing the Parent Dashboard, mentors can steer curriculum styles and review asynchronous PracticeRecorder files, ensuring that the student's individual learning trajectory is aligned with academic goals while maintaining the tactical engagement of the platform.

# **Section IV: The 40-Hour Master Curriculum Matrix**

The following matrix outlines the rigorous 10-week curriculum, mapping ELA Common Core standards to the procedural mechanics of the Semantic Slime ECS architecture. Each week represents 4 hours of classroom engagement.

| Week | Core Focus | CCSS Standards | Game Loop & Mechanics | Thematic Quest & Lore |
| :---- | :---- | :---- | :---- | :---- |
| 1 | Morphology | CCSS.ELA-LITERACY.L.4.3 | Harvesting Lexical Scrap; Phoneme Seed generation via Micro-Trellis. | Arrival at Lexis Terminus; planting the first Phoneme Seeds in the Garden District. |
| 2 | Noun Phrases | CCSS.ELA-LITERACY.L.4.5 | Syntax Bud assembly; mapping nouns to physical Golem components. | The Garden District Expansion; naming the flora to stabilize the biome. |
| 3 | Verb Tense | CCSS.ELA-LITERACY.L.5.1 | Temporal markers in deck.rs; sequencing Golem actions in real-time. | Repairing the engine in the Maintenance Bay using proper verb agreement. |
| 4 | Subject-Verb Agreement | CCSS.ELA-LITERACY.L.5.2 | Validation of the Transform component; Golem mobility checks. | The Outlaw Outpost; hijacking the train car by resolving syntactic tension. |
| 5 | Semantics | CCSS.ELA-LITERACY.L.6.4 | Semantic Slime infusion; applying StatusEffects via adjective density. | The Fog of Ambiguity; using precise adjectives to identify hidden enemies. |
| 6 | Descriptive Language | CCSS.ELA-LITERACY.L.6.5 | Mesh deformation via Micro-Trellis; visual feedback for sensory words. | Restoring Adjective Valley; nourishing Semantic Larvae with sensory details. |
| 7 | Clause Structure | CCSS.ELA-LITERACY.L.7.1 | Compound Unit formation; linking multiple Golems into complex structures. | The Siege of the Shadow Library; bridging disparate thoughts into clauses. |
| 8 | Coordination | CCSS.ELA-LITERACY.L.7.3 | Rhetorical Cocoon stage; utilizing coordinating conjunctions for damage multipliers. | Defending Hierarchy Hall; organizing the railway's chaotic metadata. |
| 9 | Rhetoric | CCSS.ELA-LITERACY.L.8.1 | Rhetoric Robot TTS negotiation; social Persuasion Damage loops. | The Irony Junction; deconstructing boss logic using Ethos and Pathos. |
| 10 | Persuasion | CCSS.ELA-LITERACY.L.8.3 | Sovereign Avatar deployment; final generative mesh with CAGED scaling. | Reaching the 12th District; restoring the Great Railway of Lexis legacy. |

# **Section VI: Parent/Teacher Ingestion Specifications**

## **Ingestion Pipeline Mechanics**

The system allows for direct integration of teacher-defined lesson plans through the Janus-Pro-1B model. When a teacher inputs a unit block, the model parses the text into semantic vectors, identifying core ELA targets and vocabulary weights. This ensures that the local edge-AI generated content—including Golem attributes and quest parameters—remains aligned with the specific pedagogical goals of the instructor.

## **Hot-Swapping Systems Architecture**

Parsed data is serialized into Bevy-compatible JSON formats. These assets are handled by the \`database.rs\` resource manager, which facilitates a hot-swapping mechanism. This architecture allows the game state to update quest objectives, NPC dialogue, and entity definitions at runtime without requiring an application reboot, providing a seamless transition between ELA lessons and tactical gameplay.

# **Section VII: Project Roadmap, Telemetry, and Future Directions**

## **3-Phase Roadmap**

**Phase 1: Local Core.** Finalization of the Bevy ECS and the deck.rs engine. Implementation of local INT4 quantized Janus-Pro-1B and Micro-Trellis models on XREAL Aura hardware.  
**Phase 2: Async Pipeline.** Integration of the Parent/Teacher Dashboard and the PracticeRecorder file system. Optimization of the Kokoro TTS sidecar for microtonal pitch telemetry.  
**Phase 3: Spatial XR.** Deployment of multi-user spatial classroom environments and Sovereign Avatar generative mesh scaling via the FACES shader.

## **Telemetry Specifications**

The system logs local JSON telemetry via \`save.rs\`, adhering to COPPA compliance. Key metrics include 'Schema Acquisition Rate', which tracks the speed of student linguistic integration, and 'Cognitive Fuel Consumption', which measures the germane load relative to syntactical dependency distance.

## **Future Speculations**

Future iterations of Semantic Slime will integrate with the TRINITY ID AI OS ecosystem, allowing for persistent Sovereign Avatars across multiple educational applications. Development focuses on multi-user spatial classrooms where students can collaboratively build "Compound Units" for large-scale rhetorical sieges in shared XR environments.

# **Chapter VIII: 2D 8-Bit Android Prototype (Rapid Deployment Spec)**

This chapter outlines the rapid deployment specification for a 2D 8-bit Android prototype designed for high-efficiency execution on consumer mobile hardware. By pivoting from 3D spatial rendering to a constrained 2D environment, the system optimizes for broader K-12 accessibility while maintaining the core pedagogical mechanics of the Semantic Slime ECS.

## **1\. Technical Architecture & Cross-Compilation**

The prototype utilizes Bevy 0.18.1 as the primary engine. To maximize performance on Android devices, the 3D and PBR pipelines are explicitly disabled in the feature set, enabling only the 2D sprite and UI crates. The compilation workflow utilizes the Android NDK via \`cargo ndk\` to generate shared libraries, which are then packaged into the Tauri 2.0 Android Shell as defined in the workspace \`tauri.conf.json\`.

## **2\. Mobile UI/UX & Touch-Gesture Controls**

Spatial WebXR controls are translated into a 2D touch interface utilizing Bevy's \`TouchInput\` event reader. Users trace spelling patterns directly on the screen to construct linguistic units. The "pinch/drag" spatial actions are replaced with swipe-based gestures that allow students to assemble noun-phrases and verb-slots on a localized 16x16 grid. This grid provides a physical boundary for the construction of Syntax Golems within the limited screen real estate of a mobile device.

## **3\. Low-Overhead Local Asset Pipeline**

To ensure compatibility with lower-spec consumer Android devices, the Micro-Trellis 3D generation system is disabled. In its place, the prototype employs a low-overhead procedural 2D sprite generator. This system layers, tints, and scales retro 8-bit body components—specifically the base, focus (eyes), and action (mouth)—based on the psycholinguistic vectors parsed from student input. This ensures that the visual representation of the Semantic Slime remains tied to linguistic data without the memory cost of 3D mesh generation.

## **4\. Local Database Fallback**

For environments where local LLM inference is disabled, the system utilizes a local database fallback. This mechanism relies entirely on pre-loaded SQLite and Dexie JSON databases located in the \`assets/\` folder and managed by \`database.rs\`. This fallback reduces the total RAM footprint of the application to \<1.5GB, ensuring stable execution on devices with limited memory.

| Deployment Parameter | Specification Details |
| :---- | :---- |
| Engine Version | Bevy 0.18.1 (2D Features Only) |
| Build Chain | Cargo NDK \+ Tauri 2.0 Android Shell |
| Input Logic | TouchInput Event Reader (16x16 Grid) |
| Memory Profile | \<1.5GB RAM (LLM-Disabled State) |

