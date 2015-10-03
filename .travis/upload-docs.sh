set -e
if [ "$TRAVIS_PULL_REQUEST" = "false" ]; then
  cargo doc --no-deps
  pip install --user ghp-import
  /home/travis/.local/bin/ghp-import -n target/doc
  git push -qf https://${TOKEN}@github.com/${TRAVIS_REPO_SLUG}.git gh-pages
  echo "Doc upload finished"
fi
