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
      "@semantic-release/github",
      {
        "assets": "dist/*",
        "successComment": false,
        "failComment": false
      }
    ],
    [
      "@semantic-release/git",
      {
        "message": "chore(release): ${nextRelease.version} [skip ci]\n\n${nextRelease.notes}"
      }
    ],
    [
      "@semantic-release/exec",
      {
        "verifyConditionsCmd": "semantic-release-cargo verify-conditions",
        "publishCmd": "semantic-release-cargo publish"
      }
    ]
  ]
}
