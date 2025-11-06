<picture>
  <source srcset="/assets/Ofoo" media="(prefers-color-scheme: dark)">
  <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Ofoo

**A lightweight todo web app built with [Leptos](https://github.com/leptos-rs/leptos) and stable Rust.**

Ofoo is a minimalistic todo app that stores your tasks directly in your browser using local storage.
It doesn’t aim to be feature-rich — instead, it exists as a playground for me to deeply understand **Leptos**, Rust’s reactive web framework.

---

## 🧭 What I Learned

Building Ofoo helped me dive into Leptos’s core concepts and how they differ from traditional JS frameworks like React:

- **Signals, derived signals, and closures:** mastering how reactive state propagates without re-renders.
- **Fine-grained reactivity:** Leptos updates only what changes — no Virtual DOM needed.
- **Reactive tracking system:** every reactive value is subscribed to dependencies automatically, making the UI updates blazingly fast.
- **Stable Rust:** this project runs entirely on stable Rust (no nightly features required).
- **Component design:** structured components that feel natural and ergonomic in pure Rust.

---

## ⚙️ Running the Project

Make sure you have `cargo-leptos` installed, then:

```bash
cargo leptos watch
