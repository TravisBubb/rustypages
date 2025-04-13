# ✨ rustypages: Feature Overview

`rustypages` is a lightweight static site generator focused on simplicity, speed, and minimal configuration.

---

## ✅ Implemented

- [x] Markdown to HTML conversion
- [x] Basic CLI structure
- [x] File system traversal and output directory creation

---

## 🔧 In Progress / Planned

### Core Functionality
- [ ] Simple HTML templating (basic variable substitution)
- [ ] Layout inheritance (e.g., base.html + per-page templates)
- [ ] Live reload development server
- [ ] Front matter parsing (YAML/TOML in markdown headers)
- [ ] Static asset copying (images, CSS, JS)
- [ ] Incremental builds (only recompile changed files)
- [ ] Sitemap.xml generation

### CLI & Config
- [ ] Config file support (e.g., `rustypages.toml` or `config.yaml`)
- [ ] Log output verbosity levels

### Templating
- [ ] Conditional rendering in templates (e.g., if/else)
- [ ] Loops (e.g., for blog post listings)
- [ ] Global template variables (site name, footer, etc.)

---

## 💡 Ideas & Stretch Goals

- [ ] Plugin system (custom processors or generators)
- [ ] Blog-aware features (pagination, post metadata)
- [ ] RSS feed generation
- [ ] Multi-language site support
- [ ] Custom themes or theme marketplace
- [ ] Markdown extensions (syntax highlighting, diagrams)

---

## 🗺️ Roadmap (Tentative)

1. **v0.1** – Markdown + basic templates + static output + minimal config
2. **v0.2** – Config file, asset copying, front matter
3. **v0.3** – Layouts, live dev server, CLI improvements
4. **v1.0** – Plugins, themes, advanced templating, blog features

---

*This document is a living spec and will evolve as `rustypages` grows.*

