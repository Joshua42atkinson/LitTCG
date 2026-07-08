# LitTCG Product Direction Decision
## Core Game vs. Parent Co-Play vs. Google Agentic Homeschool Platform

**Date:** July 7, 2026  
**Purpose:** Evaluate the latest product direction — a Google-integrated, agentic, homeschool learning system — and decide whether to commit, modify, or reject it.

---

## The Idea on the Table

You proposed a fun learning and Mad Libs game for parents and kids, using VAAM learning matrices, async grading, timing and notifications, Google OAuth, Google Calendar integration, and a fully online agentic system with Gemini.

This is a creative vision, but it is a **major scope expansion** from the current LitTCG. This document compares it against the existing options and gives a clear recommendation.

---

## What Is Brilliant About the Idea

Several pieces of this vision are genuinely strong:

1. **Parent co-play.** Parents and kids playing together is a powerful, under-served market.
2. **Async notifications.** A parent gets a nudge: "Ask Emma about the word 'fortress' today." This is high-value.
3. **Google integration.** Homeschool families already live in Google Workspace, Calendar, and Gmail.
4. **VAAM learning matrices.** The psycholinguistic data we already have can produce meaningful learning insights.
5. **Mad Libs as family activity.** This is naturally social and fun.

These are **features worth building** — but not necessarily the foundation.

---

## What Is Risky About the Idea

### 1. It Stops Being a Game and Becomes a Learning Platform
A "fully online agentic system with Gemini" sounds more like a homeschool LMS than a creature-collection game. The thing that makes LitTCG special — **spell a word, get a unique pet** — gets buried under platform features.

### 2. It Violates the Core Trust Proposition
LitTCG's strongest marketing pillar is **local-first, COPPA-safe, no cloud accounts**. Google OAuth, Calendar sync, and Gemini integration mean:
- Cloud data.
- Account requirements.
- AI processing of children's data.
- Privacy policy complexity.
- Potential COPPA/FERPA review burden.

This removes the key advantage over Prodigy, Lexia, and school platforms.

### 3. It Is Not Achievable in 90 Days
Building a Google-integrated, agentic homeschool platform requires:
- Google OAuth and API integration.
- Backend infrastructure for sync and notifications.
- Gemini API integration and prompt engineering.
- Calendar/event scheduling logic.
- Async grading system.
- Parent and child role management.
- Online/offline sync conflict handling.
- Privacy/compliance review.

This is a 6–12 month engineering project, not a 90-day sprint.

### 4. It Adds Ongoing Costs
- Gemini API calls cost money per user.
- Google Cloud hosting costs money.
- Backend maintenance is ongoing.
- Customer support becomes harder.

A $9.99 one-time game cannot support significant AI backend costs. This forces a subscription model.

### 5. It Shifts the Competition
With the current LitTCG, competitors are Pokémon, Prodigy, and WonderLang. With the Google agentic platform, competitors become Google Classroom, Seesaw, ClassDojo, IXL, and full curriculum platforms. Those are much bigger, better-funded, and harder to displace.

---

## Three Product Direction Options

### Option A: Core LitTCG (Game-First)
**What it is:** Spell words, summon pets, battle typos, complete quests. Local-first, one-time purchase.

**Sales model:** Free demo + $9.99 full game + $4.99 expansions + optional $39.99/year dashboard.

**Pros:**
- Fastest to build.
- Strongest differentiation (word-to-creature IP).
- Local-first/COPPA-safe trust anchor.
- Lowest ongoing costs.
- Clear marketing message.

**Cons:**
- Lower LTV without subscriptions.
- No recurring revenue.

**Best for:** 90-day sprint, homeschool parent market, proving product-market fit.

---

### Option B: LitTCG + Parent Co-Play & Notifications
**What it is:** Core LitTCG plus optional family features: co-play mode, parent conversation prompts, weekly progress emails, optional cloud sync.

**Sales model:** Free demo + $9.99 full game + $39.99/year family features (sync, reports, co-play) + expansion packs.

**Pros:**
- Keeps the core game intact.
- Adds the best parts of your new idea without the platform complexity.
- Annual subscription is parent-justified by progress reports and family engagement.
- Still mostly local-first; cloud sync is optional.

**Cons:**
- More engineering than core game alone.
- Requires email/notification infrastructure.

**Best for:** Parents who want to play with their kids and track progress without giving up privacy.

---

### Option C: LitTCG Google Agentic Homeschool Platform
**What it is:** Full online homeschool system with Google integration, Calendar scheduling, Gemini agent, async grading, and LMS-style features.

**Sales model:** Freemium + $19.99–$39.99/month family subscription or $199/year.

**Pros:**
- Highest revenue potential if it works.
- Fits the Google ecosystem many homeschoolers use.
- AI assistant could be genuinely useful.

**Cons:**
- 6–12 month build time.
- High ongoing costs.
- Loses COPPA/local-first advantage.
- Competes with giant platforms.
- High risk of building something no one wants.
- Not survivable on a 4-month runway.

**Best for:** Year 2+ after Core LitTCG is proven and funded.

---

## Sales Comparison

| Dimension | Option A: Core | Option B: Parent Tools | Option C: Agentic Platform |
|-----------|----------------|------------------------|----------------------------|
| Time to first sale | 30–60 days | 60–90 days | 6–12 months |
| Price point | $9.99 | $9.99 + $39.99/year | $199/year |
| Market size | Large | Medium | Smaller but higher willingness-to-pay |
| Sales complexity | Low | Low | High |
| Marketing message | Simple | Clear | Complex |
| Trust barrier | Low | Low | Higher (cloud/AI concerns) |
| Ongoing costs | Low | Medium | High |
| Risk | Low | Medium | Very high |

---

## Recommendation: Do Not Shift Again. Build Option A, Then Option B.

The pattern right now is escalating scope without shipping: web → XR → school → Android → Google agentic platform. Every shift resets the 90-day clock. That is fatal with a 4-month runway.

**Recommended path:**

1. **Ship Option A (Core LitTCG) in 90 days.** Web demo, Android app, $9.99 unlock, optional dashboard. This proves the product and generates revenue.
2. **Add Option B (Parent Co-Play & Notifications) in months 4–6.** Optional family sync, weekly reports, conversation prompts. This increases LTV.
3. **Consider Option C only after Option A and B are working.** And only if customers are asking for it, not because it sounds cool.

The Google integration and Gemini agent are **features**, not the product. The product is the word-to-creature game. Everything else is a layer on top.

---

## What to Build From Your New Idea

Do not build the whole platform. Instead, extract the highest-value pieces and add them later:

| Feature | When to Build | Why |
|---------|---------------|-----|
| Parent co-play mode | Option B | Increases family engagement |
| Weekly progress email | Option B | Keeps parents engaged without app complexity |
| Optional Google account sync | Option B | Convenience for families already on Google |
| Google Calendar integration | Option C | Only if families ask for study scheduling |
| Gemini AI assistant | Option C | High cost, high complexity, low priority |
| Async grading | Option C | Shifts product toward LMS; defer |
| Full online agentic system | Option C | Not the foundation |

---

## The Decision

You need to pick one path and commit. The three options are mutually exclusive for a 90-day sprint.

**If you want revenue in 4 months:** Choose Option A.  
**If you want a family-focused product with recurring revenue:** Choose Option B.  
**If you want to build a platform company and can survive 6–12 months without product revenue:** Choose Option C.

Given your business plan and runway, **Option A with Option B as the next phase is the only rational choice.** The Google agentic platform is a good Year 2 vision, but building it now is a bet-the-runway move that will likely fail before it ships.

---

## Next Step

Decide today. Then stop considering new directions for 90 days.

If the answer is Option A, the next 7 days are:
1. Safety gaps (profanity blocklist, terminology).
2. Async JSON loading for web/Chromebook.
3. Pet card reveal.
4. Touch-first mobile UI.
5. Demo limit + paywall.

If the answer is Option C, you need to secure grant or investment funding first, because it will not generate revenue in time.
