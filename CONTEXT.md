# Simon Says Terminal Game

The domain language for the Simon Says arcade mini-game in `example/app/` — the worked example this repo's workflow documentation is built around.

## Language

### Gameplay

**Pad**:
One of the four colored quadrants on the board, each bound to an arrow key.
_Avoid_: button, tile, cell

**Step**:
A single pad occurrence in the sequence.
_Avoid_: move, note, entry

**Sequence**:
The persistent, ordered list of steps for the current run; one uniformly-random step is appended each round.
_Avoid_: pattern, chain

**Round**:
One Watch-then-Echo cycle over the current sequence.
_Avoid_: level, turn

**Watch**:
The phase in which the sequence plays back; player input is locked.
_Avoid_: playback phase, demo

**Echo**:
The phase in which the player replays the sequence; each keypress is judged immediately against the expected step.
_Avoid_: input phase, response

**Mistake**:
A wrong pad, or an expired per-key timeout, during Echo.
_Avoid_: error, miss

**Sudden Death**:
The rule that a run ends on the first mistake.
_Avoid_: lives, strikes

**Run**:
One complete game, from the length-1 sequence to game over; endless — there is no win cap.
_Avoid_: game, session, attempt

**Speed Tier**:
A playback tempo bracket entered at fixed round thresholds; doubles as the score multiplier.
_Avoid_: level, stage, difficulty

**Score**:
Points accumulated in a run — each correct step is worth base points times the current speed tier's multiplier.
_Avoid_: points

**High Score**:
The best score across all runs, persisted between plays.
_Avoid_: best, record

### Screens & flow

**Hub**:
The board's center cell, where Round, Phase, and callouts read out.
_Avoid_: center panel, status box

**Sidebar**:
The stats column beside the board showing Score, High Score, Speed Tier, Round, Phase, and key hints.
_Avoid_: HUD, stats panel

**Title**:
The launch-only idle state, shown as an overlay on the board until the player starts a Run; never returned to afterward.
_Avoid_: menu, splash, home screen

**Get Ready**:
The brief beat between starting or restarting a Run and its first Watch.
_Avoid_: countdown, warm-up

**Round Break**:
The fixed pause between a completed Echo and the next round's Watch; where the SPEED UP! callout lands on tier-up rounds.
_Avoid_: intermission, interlude

**Death Freeze**:
The beat after a Mistake in which the board reveals the expected pad, before Game Over.
_Avoid_: fail animation, death screen

**Game Over**:
The end-of-Run state — an overlay reporting the Run's result, offering restart or quit.
_Avoid_: end screen, results screen
