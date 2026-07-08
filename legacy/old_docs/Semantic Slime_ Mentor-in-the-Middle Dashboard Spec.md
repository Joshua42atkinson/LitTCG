# **Semantic Slime: Mentor-in-the-Middle Dashboard Specification**

# **1\. Core Philosophy of the 'Mentor-in-the-Middle' Model**

The Semantic Slime architecture is predicated on the rejection of automated, black-box educational applications. Such systems frequently fail due to a lack of human-in-the-loop constructivist scaffolding. By grounding the system in Vygotsky’s Zone of Proximal Development (ZPD), the "Mentor-in-the-Middle" model ensures that technology acts as a bridge rather than a replacement for human guidance.

Without the active presence of a mentor to provide context-sensitive assistance, automated learning environments often result in cognitive overload or disengagement. This dashboard facilitates a symbiotic relationship where AI handles data parsing and initial content generation, while the mentor provides the critical human oversight necessary to navigate the student’s ZPD effectively.

# **2\. The Ingestion & Curriculum Sandbox Editor**

The Curriculum Sandbox Editor is a controlled environment where the mentor moderates AI-generated content before it enters the live game engine. This stage is critical for ensuring pedagogical accuracy and safety.

| Stage | Mentor Action | Engine Integration |
| :---- | :---- | :---- |
| Parsing Review | Review AI-parsed vocabulary and semantic clusters | Flag for revision or approval |
| Content Rewriting | Modify AI-generated quests and dialogue for tone/difficulty | Save to temporary curriculum buffer |
| Blessing | Final pedagogical validation of all assets | Hot-swap assets into Bevy runtime |
| Live Update | Deployment of approved content without session restart | Real-time Bevy state synchronization |

# **3\. Socratic Intervention Alerts & Scaffolding Systems**

The system monitors real-time telemetry to identify when a student requires human intervention. By analyzing "Cognitive Fuel" (mental stamina/energy levels) and "track Friction" (impediments to progression), the dashboard generates actionable Socratic prompts.

| Telemetry Signal | Condition | Dashboard Intervention |
| :---- | :---- | :---- |
| Low Cognitive Fuel | Rapid decline in task accuracy and reaction time | Alert: Suggest short break or shift to lower-load activity |
| High Track Friction | Repeated failure at a specific semantic or mechanical node | Alert: Provide specific Socratic conversational prompt |
| Stagnant Engagement | High fuel but zero progression in quest dialogue | Alert: Prompt mentor to introduce a "challenging" pivot |

## **Socratic Scaffolding Logic**

When track Friction exceeds established thresholds, the mentor is presented with a real-time intervention panel. This panel contains prompts designed to guide the student toward self-discovery of the solution rather than providing direct answers, maintaining the integrity of the constructivist model.

# **4\. PracticeRecorder & Somatic Portfolio Viewer**

The Somatic Portfolio Viewer provides a high-fidelity interface for the mentor to review physical and vocal student performance. This interface focuses on somatic markers that automated systems often overlook or misinterpret.

* **Audio Analysis:** The viewer displays a timeline overlay showing pitch cents deviation, allowing mentors to visualize vocal precision and tone modulation during practice.  
* **Video Review:** Mentors can view student video entries and annotate specific timeline markers.  
* **Somatic Feedback:** Specialized annotation tools permit feedback on postural alignment and fingerspelling accuracy, ensuring that physical execution matches linguistic intent.

# **5\. Emergent Archetype Class & CCSS Telemetry**

Student gameplay patterns are algorithmically translated into psychological and academic metrics. This dual-mapping provides a holistic view of both the child's internal disposition and their external academic progress.

## **Jungian Psychological Metrics**

The system classifies emergent gameplay patterns into four primary Jungian archetypes:

1. **Innocent:** Characterized by safe exploration and adherence to established quest paths.  
2. **Rebel:** Identified through experimental interactions and attempts to bypass standard game mechanics.  
3. **Sage:** Evidenced by deep engagement with vocabulary parsing and high-accuracy dialogue completion.  
4. **Jester:** Defined by creative play with game physics and unconventional semantic choices.

## **ELA Common Core (CCSS) Telemetry**

Simultaneously, these interactions are mapped to ELA Common Core standards. The dashboard provides a visual heat map of mastery, specifically focusing on vocabulary acquisition, reading comprehension, and somatic communication standards. This ensures that the qualitative psychological data is always grounded in quantitative academic benchmarks.

Person  
Mentor Signature