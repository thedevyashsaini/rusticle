export default {
    extends: ["@commitlint/config-conventional"],
    rules: {
        // Ensure the subject is not empty
        "subject-empty": [2, "never"],
        // Ensure the subject doesn't end with a period
        "subject-full-stop": [2, "never", "."],
        // Ensure the subject is not too long
        "subject-max-length": [2, "always", 72],
        // Ensure the subject starts with lowercase
        "subject-case": [2, "always", "lower-case"],
        // Ensure the type is not empty
        "type-empty": [2, "never"],
        // Ensure the type is lowercase
        "type-case": [2, "always", "lower-case"],
        // Define allowed types
        "type-enum": [
            2,
            "always",
            [
                "feat", // New feature
                "fix", // Bug fix
                "docs", // Documentation
                "style", // Formatting, missing semi colons, etc
                "refactor", // Code restructuring without changing external behavior
                "perf", // Performance improvements
                "test", // Adding or updating tests
                "chore", // Maintenance tasks
                "ci", // CI/CD changes
                "build", // Build system changes
                "revert", // Reverting a previous commit
            ],
        ],
        // Optional scope format
        "scope-case": [2, "always", "lower-case"],
        // Body line length
        "body-max-line-length": [1, "always", 100],
        // Footer line length
        "footer-max-line-length": [1, "always", 100],
    },
};
