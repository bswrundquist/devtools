---
name: llm-evals
description: Use when building or improving evaluation for an LLM feature — designing an eval set, choosing graders (assertions vs LLM-judge), calibrating judges, wiring evals into CI, or deciding whether a prompt/model change is safe to ship.
tools: Bash, Read, Grep, Glob, Write, Edit
---

# LLM Evals

An eval is a regression test for behavior you've already seen matter: build
it from real failures and real requirements, not imagined ones. Twenty real
cases beat two hundred synthetic ones.

## Building the Eval Set

Sources, in order of value:

1. **Production failures** — every bug report or bad output becomes a case.
2. **Real traffic samples** — representative inputs, including the messy ones
   (empty fields, wrong language, huge inputs).
3. **Requirements as cases** — each "must always/never" clause gets explicit
   cases, including adversarial phrasings.
4. **Synthetic edge cases** — generated variations, last and least; label
   them as synthetic so nobody over-trusts them.

Every case: input, expected behavior (or rubric), and *why it's in the set*
(link the incident/requirement). Version the set in git; changing cases and
changing the prompt in one commit is how eval theater starts.

## Grader Ladder

Use the cheapest grader that can judge the property — most features need a
mix:

| Grader | For | Notes |
|--------|-----|-------|
| Exact/regex/schema assert | Structured output, required fields, forbidden strings | Free, deterministic — use maximally |
| Programmatic checks | Parseable code, valid SQL, citation resolves, length | Still free; runs the output |
| LLM judge, binary per criterion | Tone, faithfulness, instruction-following | One question per call: "Does the response use only facts from the context? yes/no + quote the violation" |
| LLM judge, pairwise | A/B between prompt versions | Stronger signal than absolute scores |
| Human review | Judge calibration, high-stakes samples | The ground truth the judges answer to |

Never a 1–10 scale judge — scores cluster at 7 and drift between runs.
Binary criteria with a required justification, aggregated into a scorecard.

## Judge Discipline

An uncalibrated judge is a random number generator with confidence:

- Calibrate: hand-label 30–50 outputs, run the judge, check agreement (aim
  >90% on binary criteria). Below that, fix the judge prompt — usually by
  making the criterion more concrete — and re-check.
- Judge prompt contains: the criterion, 2–3 worked examples (pass and fail
  with reasoning), the required output format. Require the quote/evidence
  *before* the verdict.
- Use a different model (or at least different prompt lineage) as judge than
  generator; same-model judging inflates scores.
- Re-calibrate the judge whenever it, or the criteria, change.

## Harness

pytest works fine as the runner — no framework required to start:

```python
@pytest.mark.parametrize("case", load_cases("evals/extraction.jsonl"))
def test_extraction(case, model_client):
    out = model_client.run(PROMPT, case["input"])
    data = json.loads(out)                      # assert: valid JSON
    assert set(data) == set(case["expected_fields"])
    assert data["amount"] == case["expected"]["amount"]
```

- Temperature 0 where the product allows; otherwise run each case n≥3 and
  report pass-rate per case, not single-shot pass/fail.
- Record per-run artifacts: model id, prompt version/hash, per-case
  transcripts. A score you can't drill into can't be debugged.
- Report per-slice (by case source, difficulty, language), never just the
  headline number — regressions hide in slices.

## Shipping Gate

For any prompt/model change:

1. Run the full set on old and new; diff per-case, not just aggregate.
2. Read every **newly-failing** transcript — no exceptions. Read a sample of
   newly-passing ones (did it improve, or did the judge get fooled?).
3. Gate: no regression on `must_pass` cases; aggregate within agreed
   threshold on the rest.
4. Log a one-line verdict with the run link next to the change.

## Rules

- Eval cases come with provenance — no case without a reason it exists.
- Never tune the prompt on eval cases, and never eval on the examples baked
  into the prompt. Held-out means held out.
- Assertions before judges; binary judges before scales; scales never.
- An LLM judge is unusable until agreement with human labels is measured.
- Non-deterministic feature → pass-rates over repeats, not single shots.
- Read transcripts on every regression. Aggregate scores are for spotting
  problems, transcripts are for understanding them.
- A saved production failure that isn't in the eval set yet is a bug in the
  eval set.
- Track eval cost/latency; a suite too slow for CI stops being run, and an
  unrun eval protects nothing.
