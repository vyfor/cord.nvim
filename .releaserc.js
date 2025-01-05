module.exports = {
  "branches": [
    "master",
    {
      "name": "client-server",
      "prerelease": "beta",
      "channel": "beta"
    }
  ],
  "repositoryUrl": "https://github.com/vyfor/cord.nvim",
  "plugins": [
    "@semantic-release/commit-analyzer",
    "@semantic-release/release-notes-generator",
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
        "assets": [
          "CHANGELOG.md"
        ],
        "message": "chore(release): ${nextRelease.version} [skip ci]\n\n${nextRelease.notes}"
      }
    ],
    ["@semantic-release-cargo/semantic-release-cargo"],
    [
      "@semantic-release/exec",
      {
        "verifyConditionsCmd": "./semantic-release-cargo verify-conditions",
        "prepareCmd": "./semantic-release-cargo prepare ${nextRelease.version}",
        "publishCmd": "./semantic-release-cargo publish"
      }
    ]
  ]
}
