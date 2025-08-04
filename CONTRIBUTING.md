# 🤝 Contributing Guidelines

Thank you for showing interest in contributing to **Rusticle**! 🦀✨  
We welcome contributions that make this interpreter more robust, user-friendly, and fun to use.

Whether you are fixing a bug, improving the interpreter, enhancing the documentation, or experimenting with new features, your contributions are highly valued!

---

## 🛠 How to Contribute

### 1. Fork the repository
Click on the **Fork** button at the top-right of this repository.

### 2. Clone your fork
Clone your forked repository to your local machine:

```bash
git clone https://github.com/<your-username>/rusticle.git
cd rusticle
```

### 3. Add the upstream remote (one-time setup)
To keep your fork updated with the original repository:

```bash
git remote add upstream https://github.com/thedevyashsaini/rusticle.git
```

### 4. Create a new branch
Always create a new branch before making changes:

```bash
git checkout -b feature-name
```

### 5. Make your changes
- Follow the existing folder and module structure.
- Test your code changes using:

```bash
cargo run --quiet -- example.lin
```

### 6. Commit your changes
Use clear and meaningful commit messages:

```bash
git add .
git commit -m "Add: description of your change"
```

### 7. Keep your branch updated
Before pushing, sync with upstream:

```bash
git fetch upstream
git rebase upstream/main
```

Resolve any conflicts if they appear.

### 8. Push and Open a Pull Request
Push your branch:

```bash
git push origin feature-name
```

Open a Pull Request (PR) from your fork to the **main** branch of the original repository.

---

## 💡 Where Can You Contribute?

- **Fix Bugs:** Resolve issues in the interpreter logic, CLI, or syntax handling.
- **Add Features:** Implement new language features or commands.
- **Improve Docs:** Make the README, Syntax guide, or Contributing guide better.
- **Refactor Code:** Clean up and optimize existing Rust code.
- **Testing:** Write additional tests to make the interpreter more stable.

---

## 🧪 Development Tips

- Use `cargo run` frequently to test your changes.
- Write modular and well-documented Rust code.
- Check `Syntax.md` for how the language is structured.

---

## 📜 Code of Conduct

By contributing to this repository, you agree to follow our [Code of Conduct](CODE_OF_CONDUCT.md) to maintain a friendly and inclusive community.

---

## 🤝 Need Help?

- Open a **GitHub Issue** for bugs or feature suggestions.
- Join discussions or contact maintainers if you have doubts.

---

## 🚀 Quick Checklist for PRs

- [ ] Code tested and works locally
- [ ] Clear and descriptive commit message
- [ ] Documentation updated if required

---

Thank you for contributing to **Rusticle**!  
Let’s build something cool together! 🎉
