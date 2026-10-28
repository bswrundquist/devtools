---
name: plan-before-action
description: Before carrying out a user request, prepare and preview a self-contained HTML work plan, check it, and wait for explicit approval. Use for requests to do, fix, build, change, send, or publish work unless the user explicitly instructs you to skip planning.
---

# Plan Before Action

Help the user avoid unstructured work: document the proposal, check whether it makes sense, and provide a readable plan for review before execution.

## Approval boundary

Until the user explicitly approves the plan, perform only the read-only investigation needed to plan accurately and create or revise the plan artifacts. Do not begin implementation, change existing working files or external records, send messages, publish, deploy, purchase, or take other execution steps. Do not run commands described as checks if they modify working files, install dependencies, trigger jobs, or otherwise execute the proposed work. Describe those checks in the plan instead.

A request to “do,” “fix,” or “build” something does not waive this gate. Follow an explicit user instruction to skip planning, within the scope of that instruction; other authorization and access constraints still apply. Approval of a clearly identified plan authorizes execution within its scope without asking again for routine steps. Silence, opening the file, and requests for revisions are not approval.

## Workflow

1. **Investigate.** Read the request and relevant available context. Use only the read-only investigation necessary for a useful, accurate plan. Ask only questions that materially affect scope, correctness, or authorization. Record reasonable assumptions for review rather than asking about routine choices. Do not guess at consequential missing information.
2. **Write the HTML plan.** Save a new, self-contained `.html` file in an allowed local artifact location, separate from implementation files. Use a descriptive task filename and version; increment the version on revisions and identify the version being approved. Start from [assets/plan-template.html](assets/plan-template.html) when useful, replacing its instructional text with task-specific content. Creating or revising this artifact is permitted before approval; it does not need its own recursive plan.
3. **Check and preview.** Verify the plan reflects the request, follows a sensible sequence, identifies important unknowns, and defines observable success. Open the saved file using an available browser or preview capability. Inspect the rendered output when possible, including narrow-screen table behavior and print layout, and fix issues. A successful open does not prove the rendering was inspected. Do not upload or publish the plan to obtain a preview. If opening is unavailable or fails, provide a clickable local file link and explain the limitation. Report which checks were actually performed; never mark an unperformed check as passed or guarantee correctness.
4. **Present and stop.** Give a brief summary, a clickable link to the HTML file, and any blockers or required decisions. Ask the user to approve the identified plan version or request changes. Explain that this skill requires approval before execution. Stop and wait; do not continue the planned work in the same turn without explicit approval.
5. **Respond to review.** Revise the plan if requested, repeat the relevant checks, reopen it, and request approval of the revised version. Resolve blockers before the dependent work proceeds; approval alone does not supply missing inputs or access. After explicit approval, execute within the approved scope and validate the result as planned. If a material change to scope, deliverables, affected items, risk, or authorization becomes necessary, revise the plan and obtain approval for that change before proceeding with affected work.

## Plan contents

Scale detail to the task; simple plans should be short. Cover each applicable item below, combining related sections when helpful. State “None identified” or “Not applicable” with a brief reason where that communicates a meaningful finding; avoid empty boilerplate sections.

- **Goal:** What the user is trying to accomplish and why.
- **Proposed outcome:** Concrete results or deliverables.
- **Scope:** Included work and relevant boundaries.
- **Current findings:** Verified facts, with source links where useful. Clearly label facts, assumptions, and unresolved questions separately. Do not describe planned investigation as a verified finding.
- **Work steps:** Ordered actions with each action's purpose, dependencies, and expected result. Keep every proposed execution action in the future tense until approved and performed.
- **Affected items:** Files, systems, records, people, or workflows involved.
- **Checks before proceeding:** Missing inputs, access requirements, dependencies, inconsistencies, blockers, and decisions the user needs to make. Make blockers and required decisions easy to find near the top; cross-reference details as needed.
- **Validation:** Observable acceptance criteria and how the completed work will be checked. Distinguish completed planning checks from proposed implementation validation.
- **Risks and recovery:** Material risks and rollback or recovery where applicable. Acknowledge when an action cannot be fully undone.
- **Approval needed:** Exactly what the user is being asked to authorize, identifying the plan version, scope, and any send/publish actions. Do not bundle unrequested actions into approval.

For anything intended to be sent or published, show the exact proposed content when feasible. Otherwise identify the draft as a deliverable requiring review before sending; approval to prepare a draft is not approval to send unseen content. Include a final review checklist for the intended audience or recipients, names, dates, figures, links, attachments, sensitive information, and unsupported claims, as applicable. Label checklist items as checked, pending, or not applicable truthfully. Recheck the final content before sending or publishing within the approved authorization.

## HTML presentation

- Use semantic HTML with embedded CSS and no required external assets. Include a viewport declaration, descriptive document title, and document language.
- Display the task title, plan version, date, and status **Awaiting approval**. Do not invent a date or imply a revised plan is already approved.
- Use clear headings, short paragraphs, numbered steps, bullets, and tables when they aid comprehension. Use readable font sizes, strong contrast, comfortable spacing, and a sensible content width.
- Make tables usable on smaller screens, such as with a horizontally scrollable wrapper, wrapping cell content, and a descriptive accessible label. Provide clean print styling that avoids clipped content and unnecessary decoration.
- Keep required decisions and blockers prominent in text, not solely through color. Avoid decorative clutter and unnecessary sections.
- Escape inserted text and use inert markup for proposed content. Do not embed secrets, tracking, external fonts, executable examples, or active scripts in the plan.

## Handoff wording

Use a brief, task-specific handoff such as: “The plan proposes [outcome]. [Blockers or decisions, if any.] Review [plan title, version](absolute/path/to/plan.html). The plan-before-action skill requires explicit approval before execution. Approve this version or request changes.” If previewing or visual inspection was unavailable, state that limitation plainly.
