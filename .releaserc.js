module.exports = {
  "branches": [
    "master",
    {
      "name": "next",
      "prerelease": "beta",
      "channel": "beta"
    }
  ],
  "repositoryUrl": "https://github.com/vyfor/cord.nvim",
  "plugins": [
    "@semantic-release/release-notes-generator",
    [
      "@semantic-release/commit-analyzer",
      {
        "preset": "angular",
        "releaseRules": [
          { "message": "*[trigger release]*", "release": "patch" },
          { "scope": "no-release", "release": false },
          { "type": "docs", "release": false },
          { "type": "refactor", "release": false },
          { "type": "style", "release": false },
          { "type": "test", "release": false },
          { "type": "chore", "release": false },
          { "type": "build", "release": false },
          { "type": "perf", "release": "patch" },
          { "type": "fix", "release": "patch" },
          { "type": "feat", "release": "patch" },
          { breaking: true, release: "minor" },
          { "type": "release", "scope": "major", "release": "major" }
        ]
      }
    ],
    [
      "@semantic-release/exec",
      {
        "verifyConditionsCmd": "semantic-release-cargo verify-conditions",
        "prepareCmd": `
          semantic-release-cargo prepare \${nextRelease.version}

          PREVIOUS_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "")
          if [ -n "$PREVIOUS_TAG" ] && git diff --quiet "$PREVIOUS_TAG" HEAD -- src/; then
            echo "skipping metadata update"
            exit 0
          fi

          echo "updating server metadata"
          echo "\${nextRelease.version}|\${process.env.RELEASE_TIMESTAMP}" > .github/server-metadata.txt
        `,
        "publishCmd": "semantic-release-cargo publish"
      }
    ],
    [
      "@semantic-release/git",
      {
        "assets": [
          "Cargo.toml",
          "Cargo.lock",
          ".github/server-metadata.txt"
        ],
        "message": "chore(release): ${nextRelease.version} [skip ci]\n\n${nextRelease.notes}"
      }
    ],
    [
      "@semantic-release/github",
      {
        "assets": "dist/*",
        "successComment": false,
        "failComment": false
      }
    ],
    "semantic-release-export-data"
  ]
}
