# LitTCG: Schools Go-to-Market Plan
## For an All-In Sprint to Get LitTCG Into Schools

**Date:** July 7, 2026  
**Product:** LitTCG — Literary Trading Card Game  
**Purpose:** Define a realistic, aggressive plan to turn LitTCG into a school-ready literacy product and sell it to K–12 classrooms, with a clear-eyed view of timelines, revenue potential, and what must be built.

---

## Executive Summary

You want to go all-in on LitTCG and sell it to schools. That is possible, but it requires shifting the product from a **consumer game** to a **classroom literacy intervention**. The same core mechanic — spelling words to summon creatures — works for both, but schools need different packaging, different features, and different proof.

**Reality check:** A school district sale typically takes 3–18 months. Free pilots convert to paid licenses only after teachers see student engagement and learning gains. If you work 90 hours/week for the next 4 months, you can have a **school-ready pilot product** and your first pilot classrooms by October 2026. But the first paid school contracts are more likely to close in Q4 2026 or Q1 2027.

To survive the 4-month runway, you likely need either:
- A small amount of service revenue alongside the product (e.g., paid teacher training, custom setup).
- A grant or veteran-founder program to bridge the gap.
- Pre-orders or early-adopter parent sales funding the school build.

This plan assumes you go all-in and targets **school pilots as the primary path**.

---

## 1. Why Schools Are a Good Target for LitTCG

### Market Data
- **Chromebooks dominate schools:** 60.1% of global K–12 devices, 93% of US districts buying more.
- **Literacy is a high-priority spending area:** Federal and state funds (Title I, ESSER successors, literacy grants) target reading and vocabulary.
- **Teachers need engaging tools:** Traditional drill apps bore students; entertainment games lack learning alignment.
- **Post-pandemic learning loss:** Schools are actively purchasing intervention tools for reading and vocabulary gaps.

### Why LitTCG Fits
- **Skill-building through play:** Spelling, synonyms, antonyms, grammar, and etymology are core mechanics.
- **Works on Chromebooks:** Web/WASM deployment means no IT installation friction.
- **COPPA/FERPA-friendly:** Local-first data, no accounts, no tracking.
- **Differentiated instruction:** Advanced students can tackle harder words; struggling students repeat at lower levels.
- **Evidence generation:** Save file and dashboard produce concrete progress data.

---

## 2. What Schools Actually Buy

### The School Buyer's Decision Chain
| Role | What They Care About | How to Win Them |
|------|---------------------|-----------------|
| **Teacher** | Engages students, easy to use, fits curriculum | Free pilot, lesson plan, quick setup |
| **Literacy Specialist / Instructional Coach** | Evidence of skill growth, standards alignment | Progress reports, case study data, skills matrix |
| **IT Director** | Runs on Chromebooks, no security/privacy risk | Web deployment, COPPA statement, no accounts |
| **Principal** | Budget-friendly, teacher demand, parent approval | Low per-student cost, teacher testimonials |
| **Curriculum Director / Superintendent** | District-wide impact, standards alignment, ROI | Pilot data, standards mapping, scalable pricing |

### What Schools Will Pay For
- **Per-student license:** $5–$15/year per student.
- **Per-classroom license:** $200–$500/year per classroom.
- **Site license:** $1,000–$10,000/year per school (depending on size).
- **District license:** $10,000–$100,000/year (multi-year deals).
- **Professional development:** $1,500–$5,000 per training session.
- **Custom setup / integration:** $2,500–$10,000 one-time.

### Typical Sales Cycle
| Phase | Timeline | What Happens |
|-------|----------|--------------|
| **Awareness** | Weeks 1–4 | Teacher or admin learns about LitTCG |
| **Pilot** | Weeks 5–12 | Free classroom trial, 2–4 teachers, 4–8 weeks of use |
| **Evaluation** | Weeks 13–16 | Review data, teacher feedback, student outcomes |
| **Procurement** | Weeks 17–36 | Budget approval, PO, contract signing |
| **Deployment** | Weeks 37–40 | Rollout to more classrooms/schools |

---

## 3. Product Pivot: From Game to Classroom Literacy Tool

To sell to schools, LitTCG must be framed as a **literacy intervention** first and a game second. The game stays the same, but the packaging changes.

### 3.1 New Positioning for Schools
> *"LitTCG is a vocabulary and spelling intervention that engages students through creature collection. Students spell words, build unique pets from their meanings, and use them in grammar and synonym battles. Teachers get real-time progress data without tests."*

### 3.2 School-Ready Features (What Must Be Built)

| Feature | Why Schools Need It | Effort | Priority |
|---------|---------------------|--------|----------|
| **Teacher Classroom Code** | Students join a class without accounts | Medium | **P0** |
| **Roster Management** | Add/remove students, see class list | Medium | **P0** |
| **Assignment Builder** | Teacher assigns specific words or skills | Medium | **P0** |
| **Standards Alignment Document** | Maps game to CCSS/state literacy standards | Low | **P0** |
| **Progress Reports (Teacher)** | Class and individual mastery reports | Medium | **P0** |
| **Progress Reports (Parent)** | Send home weekly updates | Medium | **P1** |
| **Web Demo for Classrooms** | Teacher can try instantly on Chromebook | Low | **P0** |
| **Session Length / Time Controls** | Teachers/parents set play limits | Low | **P1** |
| **Accessibility Mode** | Dyslexia font, colorblind, larger targets | Medium | **P1** |
| **Export to CSV/PDF** | Share data with admins, IEP teams | Low | **P1** |
| **School Dashboard** | Admin view of all classes and usage | High | **P2** |
| **SSO / Clever / Google Classroom Integration** | Easier large-scale deployment | High | **P2** |

### 3.3 What Stays the Same
- Core spelling, pet reveal, battle, and quest mechanics.
- COPPA-safe local-first design.
- Cross-platform web/Chromebook/desktop builds.
- 9,582-word curriculum and psycholinguistic data.

---

## 4. 90-Day Sprint Plan: School-Ready Pilot

Assuming 90 hours/week, here is an aggressive but achievable build plan.

### Month 1: Foundation (Weeks 1–4)
**Goal:** Stable, safe demo + teacher-facing landing page.

- Week 1: Close safety gaps (profanity blocklist, "intensity" terminology, compiler warnings).
- Week 2: Implement async JSON loading, web demo stability, and demo limit.
- Week 3: Build pet card reveal and pet collection screen.
- Week 4: Create teacher landing page with demo video, standards alignment doc, and pilot signup form.

**Deliverable:** A public web demo any teacher can try in a browser.

### Month 2: Classroom Features (Weeks 5–8)
**Goal:** Classroom-ready build with teacher controls.

- Week 5: Classroom code system and student roster.
- Week 6: Assignment builder (teacher selects words/skills).
- Week 7: Teacher progress report dashboard.
- Week 8: Parent weekly report email.

**Deliverable:** Pilot-ready build for 2–3 classrooms.

### Month 3: Pilots and Sales (Weeks 9–12)
**Goal:** Running pilots and identifying first paying customers.

- Week 9: Recruit 3–5 pilot teachers (homeschool co-ops, charter schools, rural districts).
- Week 10: Launch pilots, collect feedback, fix bugs daily.
- Week 11: Analyze pilot data, build case studies.
- Week 12: Begin pricing conversations with pilot schools and begin procurement conversations.

**Deliverable:** 3–5 pilot classrooms, 1–2 case studies, pilot data summary.

---

## 5. Pilot Program Design

### 5.1 Ideal Pilot Teacher Profile
- Teaches 3rd–6th grade (vocabulary acquisition sweet spot).
- Uses Chromebooks in the classroom.
- Open to trying new literacy tools.
- Willing to provide feedback and allow data collection.
- Has influence with administrators or curriculum coordinators.

### 5.2 Pilot Offer
> **"Free 4-week classroom pilot of LitTCG. We'll provide the web-based game, lesson plan, and weekly progress reports. In exchange, we ask for 30 minutes of teacher feedback and permission to share anonymized learning outcomes."**

### 5.3 Pilot Success Metrics
| Metric | Target | Why It Matters |
|--------|--------|---------------|
| Student words spelled per week | 10+ | Engagement proof |
| Words mastered per student | 5+ | Learning proof |
| Average session length | 12–20 minutes | Sustainable classroom use |
| Teacher NPS | 50+ | Willingness to recommend |
| % of students showing improvement | 70%+ | Outcome evidence |

---

## 6. Pricing Strategy for Schools

### 6.1 Recommended School Tiers
| Tier | Price | Best For |
|------|-------|----------|
| **Classroom Pilot** | Free for 4–8 weeks | Validation and case studies |
| **Classroom License** | $300/year | Single teacher, up to 30 students |
| **School Site License** | $2,000/year | Unlimited classrooms in one school |
| **District License** | $10,000–$50,000/year | Multi-school deployment, custom support |
| **PD + Setup** | $2,500 one-time | Training, onboarding, custom configuration |

### 6.2 Pricing Notes
- Start low to reduce friction and gather proof.
- Raise prices after 3–5 paid customers and strong case studies.
- Always offer annual billing; avoid monthly complexity for schools.
- Bundle PD and setup to increase deal size and ensure adoption.

---

## 7. Sales Motion: How to Get the First School Customers

### 7.1 Lead Sources
1. **Personal network:** Friends, family, or former colleagues who are teachers or administrators.
2. **Homeschool co-ops:** Fastest to pilot; less procurement friction.
3. **Charter schools:** Smaller, more agile, often innovation-friendly.
4. **Rural districts:** Your location in Maine is an advantage; Aroostook County schools may be eager for local EdTech.
5. **Teacher influencers:** Reach out to literacy educators on Instagram/TikTok/YouTube for pilots.
6. **Education conferences:** Local and regional events (virtual or in-person).
7. **SDVOSB federal contracting:** VA/DoD schools once certified.

### 7.2 Outreach Template
> **Subject:** Free classroom pilot — vocabulary game that runs on Chromebooks
>
> Hi [Name],
>
> I'm a Maine-based EdTech developer and veteran building LitTCG, a literacy intervention where students spell words to summon unique creatures and use them in grammar and synonym battles. It runs in any Chromebook browser, requires no installation, and is COPPA-safe.
>
> I'm looking for 3–5 teachers to run a free 4-week pilot this fall. I'd provide the game, a lesson plan, and weekly progress reports. All I need is your feedback and permission to share anonymized outcomes.
>
> Would you be open to a 15-minute call to see if it's a fit for your classroom?
>
> [Your name]
> [LDTAtkinson / LitTCG]

### 7.3 Sales Process Checklist
- [ ] Identify 20 pilot prospects (teachers, literacy specialists, principals).
- [ ] Send personalized outreach to 10 per week.
- [ ] Run 3–5 pilots simultaneously.
- [ ] Collect teacher testimonials and student outcome data.
- [ ] Convert successful pilots to paid classroom/site licenses.
- [ ] Use case studies to approach larger districts.

---

## 8. Bridging the 4-Month Runway

Going all-in on schools is the right long-term play, but school revenue is slow. Here is how to survive the gap.

### 8.1 Fast Cash Options That Do Not Distract from LitTCG
1. **Pre-orders / early-bird sales:** Offer parents and teachers a discounted lifetime classroom license before full school launch.
2. **Paid pilot program:** Charge a nominal $500 for pilot setup + PD instead of running entirely free pilots.
3. **Freelance EdTech development:** Use LitTCG as your portfolio to win EdTech freelance contracts while building the school product.
4. **Grant funding:** MTI, MEA, and SDVOSB programs can provide non-dilutive capital.
5. **Veteran founder programs:** PenFed Foundation, Warrior Rising, etc.

### 8.2 Suggested Hybrid Model for the 4-Month Sprint
| Week | LitTCG Focus | Income Bridge |
|------|--------------|---------------|
| 1–2 | Close safety gaps, ship demo | Set up Polar + freelance profile |
| 3–4 | Pet reveal + collection screen | Outreach for 2–3 freelance EdTech audits |
| 5–8 | Classroom features | Paid LitTCG parent pre-orders / early access |
| 9–12 | Pilots + case studies | First paid pilot setup fees or school licenses |

This keeps you 100% focused on EdTech while reducing the financial risk.

---

## 9. Risks and Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| School sales cycle is too slow | High | High | Run free pilots fast; charge setup fees; maintain income bridge |
| Product crashes in classroom | Medium | Critical | Extensive Chromebook testing before any pilot |
| Teachers do not see learning value | Medium | High | Build progress reports and collect pre/post data |
| No procurement budget available | Medium | High | Target charter schools, homeschool co-ops, and grants first |
| Competitor copies the concept | Medium | Medium | Build brand and community; protect FACES IP |
| Burnout from 90-hour weeks | High | High | Sprint in bursts, not indefinitely; automate/manual first |

---

## 10. The 90-Hour Week: How to Use It Wisely

90 hours/week is a sprint, not a lifestyle. To avoid burnout and wasted effort:

1. **Week 1–2: Define the school-ready MVP.** Do not build everything. Only build what gets a pilot signed.
2. **Week 3–8: Build the classroom loop.** Teacher code → student plays → progress report → teacher sees value.
3. **Week 9–12: Sell and support pilots.** Spend at least 20 hours/week on outreach and teacher support.
4. **Track daily output, not hours.** The goal is a signed pilot, not perfect code.
5. **Automate nothing until revenue is proven.** Manual roster management and manual reports are fine for the first 3 pilots.
6. **Take one full day off every 10 days.** Burnout kills the sprint.

---

## 11. Conclusion

Selling LitTCG to schools is a viable, high-ceiling strategy. The product is conceptually strong, the market is receptive, and the Chromebook channel is ideal. But school revenue is not a 4-month emergency parachute — it is a 6–18 month build-and-sell cycle.

**The all-in plan:**
1. Build the school-ready MVP in 90 days.
2. Run 3–5 free pilots.
3. Convert pilots to paid licenses.
4. Bridge the runway with pre-orders, setup fees, freelance EdTech work, or grants.

If you are willing to do the work, this is the most direct path to meaningful school revenue. But do not ignore the financial bridge. The best outcome is a stable product + stable income by month 4, not just a great demo and an empty bank account.

---

**Next step:** Decide whether to commit to this school sprint. If yes, the first 7 days should be: safety gaps, async JSON loading, web demo stability, and a teacher landing page with pilot signup.
