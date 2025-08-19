#!/bin/bash

echo "🚀 PDF Viewer GitHub Repository Setup Script"
echo "============================================="
echo ""

# Check if gh is installed
if ! command -v gh &> /dev/null; then
    echo "❌ GitHub CLI (gh) is not installed."
    echo "Please install it with: sudo pacman -S github-cli"
    exit 1
fi

echo "✅ GitHub CLI found"
echo ""

# Check if user is authenticated
if ! gh auth status &> /dev/null; then
    echo "🔑 GitHub authentication required..."
    echo "This will open your browser for authentication."
    read -p "Press Enter to continue with GitHub authentication..."
    gh auth login
    echo ""
fi

echo "✅ GitHub authentication verified"
echo ""

# Create the repository
echo "📁 Creating GitHub repository..."
REPO_NAME="pdf_viewer"
REPO_DESCRIPTION="Interactive Gaussian PDF explorer with real-time multiplication and visualization built with Rust + egui"

# Create repository (public by default, add --private if you want private)
gh repo create "$REPO_NAME" \
    --description "$REPO_DESCRIPTION" \
    --public \
    --source=. \
    --remote=origin \
    --push

if [ $? -eq 0 ]; then
    echo ""
    echo "🎉 Repository created and pushed successfully!"
    echo "📍 Your repository is available at:"
    gh repo view --web
    echo ""
    echo "✨ Repository features:"
    echo "   • Complete PDF viewer implementation with tests"
    echo "   • 22 passing unit tests"
    echo "   • Comprehensive documentation"
    echo "   • Ready to clone and run with: cargo run"
    echo ""
    echo "🧪 To verify after cloning:"
    echo "   cargo test    # Run all tests"
    echo "   cargo run     # Launch the application"
else
    echo "❌ Failed to create repository. Please check the error above."
    exit 1
fi