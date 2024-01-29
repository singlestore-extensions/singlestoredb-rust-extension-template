# SingleStoreDB Rust Extension Template

This is a template for creating new SingleStoreDB extensions written in Rust.

It sets up
* A Rust project for your extension code
* A basic SQL install script
* A basic WIT interface file
* Database script testing folder
* GitHub Actions for testing and releasing your extension

## Set Up

To use it, follow these instructions.

1. Run the template
    1. Install Python if you don't have it
    2. Run: `pip install cookiecutter`
    3. Run: `python -m cookiecutter gh:singlestore-extensions/singlestoredb-rust-extension-template`
    4. Follow the prompts

2. Create a new repository
    1. Go to your organization or user page
    2. Click the green "New" button to make a new repository
    3. Follow the instructions there to push your extension

3. Set up your SingleStore DB license key for GitHub Actions
    1.  Create a new [environment](https://docs.github.com/en/actions/security-guides/using-secrets-in-github-actions#creating-secrets-for-an-environment) named `Testing` for your repository.
    2. Add a secret named `SINGLESTORE_LICENSE` to it with a self-managed key for SingleStore DB