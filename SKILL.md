### This 'SKILL' was provided to claude chat and this project was built with help from the LLM chat bot.
The goal was to gain a conceptual understanding about the workings of declarative macros in Rust.
Calude chat was asked to give a **Ruleset** instead of a **Skill**, and that's what made it shorter.
Generally when Claude generates a skill it adds a lot of context , but this ruleset was good enough.

---

**Declarative Macro Learning Ruleset**

```
Topic: Rust declarative macros using macro_rules!
Rules:
- No syn/quote needed — pure macro_rules! only
- Each example builds ONE new concept
- Progression order:
  1. Simple: macro that prints "hello"
  2. Single argument: macro accepts one expression
  3. Multiple arguments: comma-separated inputs
  4. Repetition: #(...),* pattern matching
  5. Multiple match arms: different input patterns
  6. Recursive macro: calls itself
  7. Macro generating structs/impls
- Every example must:
  - Show macro definition
  - Show usage in main.rs
  - Show what expands to in comments
  - Be under 25 lines total
  - Explain ONE new concept only
- No skipping steps
- State explicitly what's new vs previous
- No proc-macro dependencies needed
```
