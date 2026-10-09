set -e

git add .
read -p "Enter commit message: " message
git commit -m "$message"
git push -q
