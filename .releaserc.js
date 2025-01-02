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
    [
      "@semantic-release/release-notes-generator",
      {
        "preset": "angular",
        "writerOpts": {
          "headerPartial": "# {{#with context}}{{#if isServerUpdate}}⚙️ {{/if}}{{/with}}[{{currentTag}}]{{#if title}} {{title}}{{/if}}\n\n{{#if date}}_{{date}}_{{/if}}",
          "commitPartial": "* {{#if scope}}**{{scope}}**: {{/if}}{{subject}} ([view commit]({{@root.repository}}/commit/{{hash}}))\n",
          "transform": function (commit, context) {
            if (!context.context) context.context = {};

            if (commit.scope === 'server') {
              context.context.isServerUpdate = true;
            }

            return {};
          }
        }
      }
    ],
    "@semantic-release/changelog",
    [
      "@semantic-release/github",
      {
        "assets": "dist/*"
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
    ]
  ]
}
