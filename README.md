## FibBot GitHub Action
### About
FibBot is a custom GitHub Action created with Rust that scans pull requests (PRs) for numbers, calculates the Fibonacci numbers for them, and posts the results as a comment on the PR. It’s a fun way to bring Fibonacci calculations directly into your GitHub workflow.

### Key Features
- Fibonacci Calculation: Finds numbers in modified PR files and calculates the Fibonacci numbers for them.
- GitHub API Integration: Automatically posts a comment with the Fibonacci results directly on the PR.
- Rust-based: Built with Rust for speed and efficiency.
- File Scanning: It scans the changed files in the PR to find any numerical values.
### How It Works
Every time a pull request is opened, synchronized, or updated, FibBot kicks in. It will look through the files that have been modified in the PR, calculate the Fibonacci numbers for any numbers it finds, and leave a comment on the PR with the results.

Parameters
Fibonacci Calculation Flag: This flag lets you turn the Fibonacci calculation on or off.
Threshold Limit: You can set a limit to the maximum number for which the Fibonacci calculation will be done.
How to Use It
- Fork the Repo: First, fork the repository to use the action.
- Create a New Branch: Make a new branch for your changes.
- Add/Modify Content: Add or modify the files in your new branch.
- Create a Pull Request: Open a PR to the main branch, and FibBot will trigger.
Once your PR is open, FibBot will go through the files, find any numbers, calculate their Fibonacci values, and then comment those results back on your PR.


12 2 5 8 
